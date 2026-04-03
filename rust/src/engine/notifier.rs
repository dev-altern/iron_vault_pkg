use dashmap::DashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

/// Broadcast change notifier for table-level and row-level events.
///
/// When a write occurs, `notify()` increments the version counter for
/// that channel and wakes all waiting watchers. Watchers block on
/// `wait()` until a new version is available.
///
/// Channel keys: `"table:tenant_id"` or `"table:tenant_id:row_id"`.
///
/// Thread-safe: `DashMap` for concurrent channel access, `Condvar` for
/// blocking waits.
pub(crate) struct ChangeNotifier {
    channels: DashMap<String, Arc<ChannelState>>,
    closed: AtomicBool,
    /// Global version — incremented on every notification (for testing).
    global_version: AtomicU64,
}

struct ChannelState {
    version: Mutex<u64>,
    condvar: Condvar,
}

impl ChangeNotifier {
    pub(crate) fn new() -> Self {
        Self {
            channels: DashMap::new(),
            closed: AtomicBool::new(false),
            global_version: AtomicU64::new(0),
        }
    }

    /// Notify all watchers on a table-level channel.
    ///
    /// Key format: `"table:tenant_id"`.
    pub(crate) fn notify(&self, table: &str, tenant_id: &str) {
        if self.closed.load(Ordering::SeqCst) {
            return;
        }
        let key = format!("{}:{}", table, tenant_id);
        self.notify_key(&key);
        self.global_version.fetch_add(1, Ordering::SeqCst);
    }

    /// Notify watchers on a specific row-level channel.
    ///
    /// Key format: `"table:tenant_id:row_id"`.
    /// Also fires the table-level channel (row watchers are a subset).
    #[allow(dead_code)]
    pub(crate) fn notify_row(&self, table: &str, tenant_id: &str, row_id: &str) {
        if self.closed.load(Ordering::SeqCst) {
            return;
        }
        // Fire both table-level and row-level
        let table_key = format!("{}:{}", table, tenant_id);
        let row_key = format!("{}:{}:{}", table, tenant_id, row_id);
        self.notify_key(&table_key);
        self.notify_key(&row_key);
        self.global_version.fetch_add(1, Ordering::SeqCst);
    }

    /// Block until a notification fires on the given channel key.
    ///
    /// Returns `true` if a notification was received, `false` if the
    /// notifier was closed (DB shutting down).
    pub(crate) fn wait(&self, key: &str) -> bool {
        if self.closed.load(Ordering::SeqCst) {
            return false;
        }

        let state = self.get_or_create(key);
        let mut version = state.version.lock().unwrap();
        let current = *version;

        // Wait until version changes OR notifier is closed
        while *version == current && !self.closed.load(Ordering::SeqCst) {
            let result = state
                .condvar
                .wait_timeout(version, Duration::from_millis(500))
                .unwrap();
            version = result.0;
            // Timeout loop: re-check closed flag periodically
        }

        !self.closed.load(Ordering::SeqCst)
    }

    /// Get the current version for a channel (for testing).
    pub(crate) fn version(&self, key: &str) -> u64 {
        self.channels
            .get(key)
            .map(|state| *state.version.lock().unwrap())
            .unwrap_or(0)
    }

    /// Get the global notification count (for testing).
    #[allow(dead_code)]
    pub(crate) fn global_version(&self) -> u64 {
        self.global_version.load(Ordering::SeqCst)
    }

    /// Shut down: wake all waiters and prevent new waits.
    pub(crate) fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        for entry in self.channels.iter() {
            entry.value().condvar.notify_all();
        }
    }

    #[allow(dead_code)]
    pub(crate) fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    // ── Private ──────────────────────────────────────────────────

    fn notify_key(&self, key: &str) {
        let state = self.get_or_create(key);
        let mut version = state.version.lock().unwrap();
        *version += 1;
        state.condvar.notify_all();
    }

    fn get_or_create(&self, key: &str) -> Arc<ChannelState> {
        self.channels
            .entry(key.to_string())
            .or_insert_with(|| {
                Arc::new(ChannelState {
                    version: Mutex::new(0),
                    condvar: Condvar::new(),
                })
            })
            .clone()
    }
}

// ─── Unit Tests ──────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn notify_increments_version() {
        let n = ChangeNotifier::new();
        assert_eq!(n.version("users:t1"), 0);
        n.notify("users", "t1");
        assert_eq!(n.version("users:t1"), 1);
        n.notify("users", "t1");
        assert_eq!(n.version("users:t1"), 2);
    }

    #[test]
    fn different_channels_independent() {
        let n = ChangeNotifier::new();
        n.notify("users", "t1");
        n.notify("orders", "t1");
        n.notify("users", "t2");
        assert_eq!(n.version("users:t1"), 1);
        assert_eq!(n.version("orders:t1"), 1);
        assert_eq!(n.version("users:t2"), 1);
    }

    #[test]
    fn notify_row_fires_both_channels() {
        let n = ChangeNotifier::new();
        n.notify_row("users", "t1", "row_abc");
        assert_eq!(n.version("users:t1"), 1); // table-level
        assert_eq!(n.version("users:t1:row_abc"), 1); // row-level
        assert_eq!(n.version("users:t1:row_xyz"), 0); // other row unaffected
    }

    #[test]
    fn global_version_incremented() {
        let n = ChangeNotifier::new();
        assert_eq!(n.global_version(), 0);
        n.notify("users", "t1");
        assert_eq!(n.global_version(), 1);
        n.notify_row("orders", "t1", "r1");
        assert_eq!(n.global_version(), 2);
    }

    #[test]
    fn wait_unblocks_on_notify() {
        let n = Arc::new(ChangeNotifier::new());
        let n2 = n.clone();

        let handle = thread::spawn(move || {
            // This will block until notify fires
            n2.wait("users:t1")
        });

        // Small delay then notify
        thread::sleep(Duration::from_millis(20));
        n.notify("users", "t1");

        let got_notification = handle.join().unwrap();
        assert!(got_notification);
    }

    #[test]
    fn wait_returns_false_on_close() {
        let n = Arc::new(ChangeNotifier::new());
        let n2 = n.clone();

        let handle = thread::spawn(move || n2.wait("users:t1"));

        thread::sleep(Duration::from_millis(20));
        n.close();

        let got_notification = handle.join().unwrap();
        assert!(!got_notification, "wait should return false when closed");
    }

    #[test]
    fn multiple_waiters_all_notified() {
        let n = Arc::new(ChangeNotifier::new());
        let mut handles = Vec::new();

        for _ in 0..5 {
            let n2 = n.clone();
            handles.push(thread::spawn(move || n2.wait("users:t1")));
        }

        thread::sleep(Duration::from_millis(20));
        n.notify("users", "t1");

        for h in handles {
            assert!(h.join().unwrap());
        }
    }

    #[test]
    fn notify_on_closed_is_noop() {
        let n = ChangeNotifier::new();
        n.close();
        n.notify("users", "t1");
        assert_eq!(n.version("users:t1"), 0);
    }

    #[test]
    fn wait_on_closed_returns_immediately() {
        let n = ChangeNotifier::new();
        n.close();
        let start = std::time::Instant::now();
        let result = n.wait("users:t1");
        assert!(!result);
        assert!(start.elapsed() < Duration::from_millis(100));
    }
}

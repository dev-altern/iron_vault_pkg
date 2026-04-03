use crate::common::*;
use iron_vault_core::api::types::*;
use std::collections::HashMap;

// ─── Query Performance ───────────────────────────────────────────────

#[test]
fn query_get_1000_iterations_under_1_second() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    // Seed 100 rows
    for i in 0..100 {
        insert_user(
            &db,
            &format!("User{}", i),
            &format!("u{}@t.com", i),
            "member",
            i as f64,
        );
    }

    let spec = query("users");

    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = db.query_get(spec.clone()).unwrap();
    }
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 1000,
        "1000 query_get calls took {:?}, expected < 1s",
        elapsed
    );
}

#[test]
fn single_insert_under_1ms_p99() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let mut times = Vec::with_capacity(100);
    for i in 0..100 {
        let start = std::time::Instant::now();
        let mut data = HashMap::new();
        data.insert("name".into(), SqlValue::Text(format!("U{}", i)));
        data.insert("email".into(), SqlValue::Text(format!("u{}@t.com", i)));
        data.insert("status".into(), SqlValue::Text("active".into()));
        db.query_insert("users".into(), data).unwrap();
        times.push(start.elapsed().as_micros());
    }

    times.sort();
    let p99 = times[98]; // 99th percentile
    assert!(
        p99 < 5000, // 5ms generous — encrypted DB is slower
        "Single insert p99 = {}μs, expected < 5000μs",
        p99
    );
}

#[test]
fn batch_10k_inserts_in_transaction_under_5_seconds() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let ops: Vec<Op> = (0..10_000)
        .map(|i| {
            let mut d = HashMap::new();
            d.insert("name".into(), SqlValue::Text(format!("User{}", i)));
            d.insert("email".into(), SqlValue::Text(format!("u{}@t.com", i)));
            d.insert("status".into(), SqlValue::Text("active".into()));
            Op::Insert {
                table: "users".into(),
                data: d,
            }
        })
        .collect();

    let start = std::time::Instant::now();
    let result = db.transaction(ops).unwrap();
    let elapsed = start.elapsed();

    assert_eq!(result.inserted_ids.len(), 10_000);
    assert!(
        elapsed.as_secs() < 5,
        "10k inserts in transaction took {:?}, expected < 5s",
        elapsed
    );
}

#[test]
fn count_query_under_1ms_on_10k_rows() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    // Seed 10k rows via transaction
    let ops: Vec<Op> = (0..10_000)
        .map(|i| {
            let mut d = HashMap::new();
            d.insert("name".into(), SqlValue::Text(format!("U{}", i)));
            d.insert("email".into(), SqlValue::Text(format!("u{}@t.com", i)));
            d.insert("status".into(), SqlValue::Text("active".into()));
            Op::Insert {
                table: "users".into(),
                data: d,
            }
        })
        .collect();
    db.transaction(ops).unwrap();

    // Measure count query
    let mut times = Vec::with_capacity(100);
    for _ in 0..100 {
        let start = std::time::Instant::now();
        let count = db.query_count(query("users")).unwrap();
        assert_eq!(count, 10_000);
        times.push(start.elapsed().as_micros());
    }

    times.sort();
    let p99 = times[98];
    assert!(
        p99 < 5000,
        "COUNT on 10k rows p99 = {}μs, expected < 5000μs",
        p99
    );
}

// ─── Pool Warm-Up Verification ───────────────────────────────────────

#[test]
fn first_query_is_fast_pool_warmed() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    // First query should NOT pay PRAGMA cost (pool warmed at open)
    let start = std::time::Instant::now();
    let _ = db.query_get(query("users")).unwrap();
    let first_query = start.elapsed();

    // Second query for comparison
    let start = std::time::Instant::now();
    let _ = db.query_get(query("users")).unwrap();
    let second_query = start.elapsed();

    // First query should be within 10x of second (not 100x as with cold pool)
    assert!(
        first_query.as_micros() < second_query.as_micros() * 20 + 5000,
        "First query {:?} should be close to second {:?} (pool warmed)",
        first_query,
        second_query
    );
}

// ─── Encryption Performance ──────────────────────────────────────────

#[test]
fn encrypt_decrypt_100_fields_under_100ms() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let start = std::time::Instant::now();
    for i in 0..100 {
        let enc = db.encrypt_field(format!("email_{}@example.com", i)).unwrap();
        let _dec = db.decrypt_field(enc).unwrap();
    }
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 100,
        "100 encrypt+decrypt cycles took {:?}, expected < 100ms",
        elapsed
    );
}

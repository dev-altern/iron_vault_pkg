use crate::api::types::VectorClock;
use std::collections::HashMap;

impl VectorClock {
    pub fn new() -> Self {
        Self { clocks: HashMap::new() }
    }

    pub fn from_map(clocks: HashMap<String, u64>) -> Self {
        Self { clocks }
    }

    /// Increment this node's counter and return a new clock.
    pub fn increment(&self, node_id: &str) -> Self {
        let mut new_clocks = self.clocks.clone();
        let counter = new_clocks.entry(node_id.to_string()).or_insert(0);
        *counter += 1;
        Self { clocks: new_clocks }
    }

    /// Merge two clocks — take the max of each node's counter.
    pub fn merge(&self, other: &VectorClock) -> Self {
        let mut merged = self.clocks.clone();
        for (node, &counter) in &other.clocks {
            let entry = merged.entry(node.clone()).or_insert(0);
            *entry = (*entry).max(counter);
        }
        Self { clocks: merged }
    }

    /// Returns true if self happens-before other (self < other).
    ///
    /// Self happens-before other iff:
    /// - For all nodes in self: self[node] <= other[node]
    /// - There exists at least one node where self[node] < other[node]
    pub fn happens_before(&self, other: &VectorClock) -> bool {
        let mut strictly_less = false;

        // Check all nodes in self
        for (node, &self_val) in &self.clocks {
            let other_val = other.clocks.get(node).copied().unwrap_or(0);
            if self_val > other_val {
                return false; // self has a higher counter somewhere
            }
            if self_val < other_val {
                strictly_less = true;
            }
        }

        // Check nodes only in other (self has 0 for those)
        for (node, &other_val) in &other.clocks {
            if !self.clocks.contains_key(node) && other_val > 0 {
                strictly_less = true;
            }
        }

        strictly_less
    }

    /// Returns true if the two clocks are concurrent (neither happens-before the other).
    pub fn is_concurrent_with(&self, other: &VectorClock) -> bool {
        !self.happens_before(other) && !other.happens_before(self) && self != other
    }

    /// Serialize to JSON string.
    pub fn to_json(&self) -> String {
        let parts: Vec<String> = self.clocks.iter()
            .map(|(k, v)| format!("\"{}\":{}", k, v))
            .collect();
        format!("{{{}}}", parts.join(","))
    }

    /// Deserialize from JSON string.
    pub fn from_json(json: &str) -> Self {
        let trimmed = json.trim().trim_start_matches('{').trim_end_matches('}');
        if trimmed.is_empty() {
            return Self::new();
        }
        let mut clocks = HashMap::new();
        for pair in trimmed.split(',') {
            let kv: Vec<&str> = pair.split(':').collect();
            if kv.len() == 2 {
                let key = kv[0].trim().trim_matches('"').to_string();
                if let Ok(val) = kv[1].trim().parse::<u64>() {
                    clocks.insert(key, val);
                }
            }
        }
        Self { clocks }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vc(pairs: &[(&str, u64)]) -> VectorClock {
        VectorClock::from_map(pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect())
    }

    #[test]
    fn new_clock_is_empty() {
        let c = VectorClock::new();
        assert!(c.clocks.is_empty());
    }

    #[test]
    fn increment_new_node() {
        let c = VectorClock::new().increment("A");
        assert_eq!(c.clocks["A"], 1);
    }

    #[test]
    fn increment_existing_node() {
        let c = vc(&[("A", 3)]).increment("A");
        assert_eq!(c.clocks["A"], 4);
    }

    #[test]
    fn increment_preserves_other_nodes() {
        let c = vc(&[("A", 2), ("B", 1)]).increment("A");
        assert_eq!(c.clocks["A"], 3);
        assert_eq!(c.clocks["B"], 1);
    }

    #[test]
    fn merge_takes_max_per_node() {
        let a = vc(&[("A", 3), ("B", 1)]);
        let b = vc(&[("A", 1), ("B", 4), ("C", 2)]);
        let m = a.merge(&b);
        assert_eq!(m.clocks["A"], 3);
        assert_eq!(m.clocks["B"], 4);
        assert_eq!(m.clocks["C"], 2);
    }

    #[test]
    fn merge_with_empty() {
        let a = vc(&[("A", 2)]);
        let empty = VectorClock::new();
        assert_eq!(a.merge(&empty), a);
        assert_eq!(empty.merge(&a), a);
    }

    #[test]
    fn happens_before_detected() {
        let a = vc(&[("A", 2), ("B", 1)]);
        let b = vc(&[("A", 3), ("B", 2)]);
        assert!(a.happens_before(&b));
        assert!(!b.happens_before(&a));
    }

    #[test]
    fn happens_before_with_missing_nodes() {
        let a = vc(&[("A", 2)]);
        let b = vc(&[("A", 2), ("B", 1)]);
        assert!(a.happens_before(&b)); // A has no B, B>0
        assert!(!b.happens_before(&a));
    }

    #[test]
    fn not_happens_before_equal() {
        let a = vc(&[("A", 2)]);
        let b = vc(&[("A", 2)]);
        assert!(!a.happens_before(&b));
        assert!(!b.happens_before(&a));
    }

    #[test]
    fn concurrent_detected() {
        let a = vc(&[("A", 2), ("B", 0)]);
        let b = vc(&[("A", 0), ("B", 2)]);
        assert!(a.is_concurrent_with(&b));
        assert!(b.is_concurrent_with(&a));
    }

    #[test]
    fn concurrent_with_different_higher_nodes() {
        let a = vc(&[("A", 3), ("B", 1)]);
        let b = vc(&[("A", 1), ("B", 3)]);
        assert!(a.is_concurrent_with(&b));
    }

    #[test]
    fn not_concurrent_if_happens_before() {
        let a = vc(&[("A", 1)]);
        let b = vc(&[("A", 2)]);
        assert!(!a.is_concurrent_with(&b));
    }

    #[test]
    fn not_concurrent_if_equal() {
        let a = vc(&[("A", 2)]);
        let b = vc(&[("A", 2)]);
        assert!(!a.is_concurrent_with(&b));
    }

    #[test]
    fn json_round_trip() {
        let original = vc(&[("nodeA", 3), ("nodeB", 7)]);
        let json = original.to_json();
        let parsed = VectorClock::from_json(&json);
        assert_eq!(parsed.clocks["nodeA"], 3);
        assert_eq!(parsed.clocks["nodeB"], 7);
    }

    #[test]
    fn json_empty() {
        let c = VectorClock::new();
        let json = c.to_json();
        assert_eq!(json, "{}");
        let parsed = VectorClock::from_json("{}");
        assert!(parsed.clocks.is_empty());
    }

    #[test]
    fn increment_is_immutable() {
        let a = vc(&[("A", 1)]);
        let b = a.increment("A");
        assert_eq!(a.clocks["A"], 1); // original unchanged
        assert_eq!(b.clocks["A"], 2);
    }
}

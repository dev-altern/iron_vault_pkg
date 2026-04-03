// Phase 1: Core Engine
pub mod crud;
pub mod encryption;
pub mod lifecycle;
pub mod pool;
pub mod stats;
pub mod wal;

// Phase 2: Query Builder
pub mod audit;
pub mod auto_audit;
pub mod auto_fts;
pub mod backup_export;
pub mod cross_phase;
pub mod edge_cases;
pub mod encryption_crypto;
pub mod key_rotation;
pub mod memory_safety;
pub mod performance;
pub mod migrations;
pub mod query_conditions;
pub mod query_read;
pub mod query_write;
pub mod reactive;
pub mod search;
pub mod semantic;
pub mod stress_fuzz;
pub mod sync_engine;
pub mod tenant;
pub mod transactions;

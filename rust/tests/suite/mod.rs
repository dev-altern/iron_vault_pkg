// Phase 1: Core Engine
pub mod lifecycle;
pub mod encryption;
pub mod crud;
pub mod wal;
pub mod stats;
pub mod pool;

// Phase 2: Query Builder
pub mod query_conditions;
pub mod query_read;
pub mod query_write;
pub mod tenant;
pub mod audit;
pub mod backup_export;
pub mod cross_phase;
pub mod edge_cases;
pub mod encryption_crypto;
pub mod migrations;
pub mod reactive;
pub mod search;
pub mod semantic;
pub mod sync_engine;
pub mod transactions;

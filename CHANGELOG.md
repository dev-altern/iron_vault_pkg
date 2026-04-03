## 1.0.0

* Core: SQLCipher encrypted SQLite with read/write connection pools, WAL mode
* Query builder: 13 condition types, tenant isolation, soft-delete, pagination, aggregates, joins
* Migrations: versioned, SHA-256 checksummed, rollback-capable
* Transactions: ACID with savepoints and optimistic locking
* Encryption: Argon2id KDF, HKDF key isolation, AES-256-GCM field encryption
* Reactive: change notifications, StreamSink watch queries with distinct emission
* Audit: HMAC-SHA256 signed entries with tamper detection
* Backup: zstd + AES-256-GCM + BLAKE3 checksums; CSV/JSON/JSONL export
* Search: Tantivy FTS with phrase, boolean, snippets
* Sync: vector clocks, outbox, conflict detection with resolution strategies
* Semantic: vector storage, cosine similarity, hybrid FTS+semantic ranking
* ONNX: optional on-device embeddings (feature flag)
* CI: GitHub Actions with clippy, fmt, tests
* 384 tests, 0 failures

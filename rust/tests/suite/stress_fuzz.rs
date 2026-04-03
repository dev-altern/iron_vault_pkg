use crate::common::*;
use iron_vault_core::api::types::*;
use std::collections::HashMap;

// ─── Fuzz: SQL Injection via Query Builder ───────────────────────────

#[test]
fn fuzz_where_values_never_cause_injection() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Legit", "legit@t.com", "member", 1.0);

    let long_string = "a".repeat(50000);
    let payloads = vec![
        "'; DROP TABLE users; --",
        "' OR 1=1 --",
        "' UNION SELECT * FROM _audit_log --",
        "\x00\x01\x02\x03",
        long_string.as_str(),
        "{\"key\": \"value\"}",
        "<script>alert(1)</script>",
        "NULL",
        "undefined",
        "NaN",
        "1e999",
        "Robert'); DROP TABLE users;--",
        "' AND 1=0 UNION ALL SELECT sql FROM sqlite_master--",
        "\n\r\t",
        "\"double quotes\"",
        "'single quotes'",
        "back\\slash",
    ];

    for payload in &payloads {
        let mut spec = query("users");
        spec.conditions.push(Condition::Eq {
            column: "email".into(),
            value: SqlValue::Text(payload.to_string()),
        });
        let result = db.query_get(spec);
        assert!(result.is_ok(), "Payload '{}' caused error", payload);
        assert!(result.unwrap().is_empty(), "Payload '{}' returned rows", payload);
    }

    // Table must still exist with original data
    assert_eq!(db.query_count(query("users")).unwrap(), 1);

    // Also fuzz LIKE patterns
    for payload in &payloads {
        let mut spec = query("users");
        spec.conditions.push(Condition::Like {
            column: "name".into(),
            pattern: payload.to_string(),
        });
        let result = db.query_get(spec);
        assert!(result.is_ok());
    }

    // And IN values
    let mut spec = query("users");
    spec.conditions.push(Condition::In {
        column: "email".into(),
        values: payloads.iter().map(|p| SqlValue::Text(p.to_string())).collect(),
    });
    let result = db.query_get(spec);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

// ─── Fuzz: Random QuerySpec ──────────────────────────────────────────

#[test]
fn fuzz_random_condition_combinations() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    for i in 0..20 {
        insert_user(&db, &format!("U{}", i), &format!("u{}@t.com", i), "member", i as f64);
    }

    // Rapidly combine different condition types — must never crash
    for i in 0..50 {
        let mut spec = query("users");
        spec.conditions.push(Condition::Gte {
            column: "score".into(),
            value: SqlValue::Real(i as f64 % 20.0),
        });
        spec.conditions.push(Condition::Like {
            column: "name".into(),
            pattern: format!("%{}%", i % 10),
        });
        if i % 3 == 0 {
            spec.order_by.push(OrderBy::Desc { column: "score".into() });
        }
        if i % 5 == 0 {
            spec.limit = Some((i % 10 + 1) as u32);
        }
        let result = db.query_get(spec);
        assert!(result.is_ok(), "Combo {} failed", i);
    }
}

// ─── Fuzz: Vector Clock Merges ───────────────────────────────────────

#[test]
fn fuzz_vector_clock_merge_1000_iterations() {
    for i in 0..1000 {
        let a = VectorClock {
            clocks: {
                let mut m = HashMap::new();
                m.insert("a".into(), (i * 7 + 3) % 100);
                m.insert("b".into(), (i * 13 + 5) % 100);
                m
            },
        };
        let b = VectorClock {
            clocks: {
                let mut m = HashMap::new();
                m.insert("a".into(), (i * 11 + 1) % 100);
                m.insert("c".into(), (i * 17 + 7) % 100);
                m
            },
        };

        let merged = a.merge(&b);

        // Merged must have max of each node
        assert!(merged.clocks["a"] >= a.clocks["a"]);
        assert!(merged.clocks["a"] >= *b.clocks.get("a").unwrap_or(&0));
        assert!(merged.clocks.get("b").copied().unwrap_or(0) >= *a.clocks.get("b").unwrap_or(&0));
        assert!(merged.clocks.get("c").copied().unwrap_or(0) >= *b.clocks.get("c").unwrap_or(&0));

        // happens_before / concurrent should be consistent
        let hb_ab = a.happens_before(&b);
        let hb_ba = b.happens_before(&a);
        let conc = a.is_concurrent_with(&b);
        // At most one can be true (or all false if equal)
        let trues = [hb_ab, hb_ba, conc].iter().filter(|&&x| x).count();
        assert!(trues <= 1, "Iteration {}: hb_ab={} hb_ba={} conc={}", i, hb_ab, hb_ba, conc);
    }
}

// ─── Stress: Rapid Inserts ───────────────────────────────────────────

#[test]
fn stress_1000_sequential_inserts() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    for i in 0..1000 {
        let mut data = HashMap::new();
        data.insert("name".into(), SqlValue::Text(format!("User{}", i)));
        data.insert("email".into(), SqlValue::Text(format!("u{}@t.com", i)));
        data.insert("status".into(), SqlValue::Text("active".into()));
        db.query_insert("users".into(), data).unwrap();
    }

    assert_eq!(db.query_count(query("users")).unwrap(), 1000);

    // Verify auto-audit created 1000 entries
    let audit_count: i64 = db
        .query_raw(
            "SELECT count(*) as c FROM _audit_log WHERE table_name = 'users' AND tenant_id = 'tenant_test'".into(),
            vec![],
        )
        .unwrap()[0]
        .get("c")
        .and_then(|v| match v {
            SqlValue::Integer(i) => Some(*i),
            _ => None,
        })
        .unwrap_or(0);
    assert_eq!(audit_count, 1000, "Auto-audit should create 1000 entries");
}

// ─── Stress: Rapid Open/Close ────────────────────────────────────────

#[test]
fn stress_open_close_5_times() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("stress.db").to_str().unwrap().to_string();

    {
        let db = iron_vault_core::api::vault::IronVaultDb::open(
            path.clone(), test_key(), "t".into(), VaultConfig::test_config(),
        ).unwrap();
        create_users_table(&db);
    }

    for _ in 0..5 {
        let mut db = iron_vault_core::api::vault::IronVaultDb::open(
            path.clone(), test_key(), "t".into(), VaultConfig::test_config(),
        ).unwrap();
        let count = db.query_count(query("users")).unwrap();
        assert_eq!(count, 0);
        db.close().unwrap();
    }
}

// ─── Stress: Mixed Read/Write ────────────────────────────────────────

#[test]
fn stress_mixed_read_write_operations() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    // Insert 100
    for i in 0..100 {
        insert_user(&db, &format!("U{}", i), &format!("u{}@t.com", i), "member", i as f64);
    }

    // Interleave reads and writes
    for i in 0..50 {
        // Read
        let count = db.query_count(query("users")).unwrap();
        assert!(count >= 50); // at least 50 remaining

        // Update
        let mut spec = query("users");
        spec.conditions.push(Condition::Gte {
            column: "score".into(),
            value: SqlValue::Real(i as f64 * 2.0),
        });
        spec.limit = Some(1);
        if let Some(row) = db.query_first(spec).unwrap() {
            if let Some(SqlValue::Text(id)) = row.get("id") {
                let mut data = HashMap::new();
                data.insert("score".into(), SqlValue::Real(999.0));
                let _ = db.query_update("users".into(), id.clone(), data);
            }
        }
    }

    // All rows should still be accessible
    let final_count = db.query_count(query("users")).unwrap();
    assert_eq!(final_count, 100);
}

// ─── Stress: Encrypt/Decrypt Cycle ───────────────────────────────────

#[test]
fn stress_500_encrypt_decrypt_cycles() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    for i in 0..500 {
        let plaintext = format!("email_{}@example.com_{}", i, "x".repeat(i % 100));
        let encrypted = db.encrypt_field(plaintext.clone()).unwrap();
        let decrypted = db.decrypt_field(encrypted).unwrap();
        assert_eq!(decrypted, plaintext, "Cycle {} failed", i);
    }
}

// ─── Stress: Migration Cycle ─────────────────────────────────────────

#[test]
fn stress_migrate_rollback_remigrate_5_times() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let migrations = vec![
        VaultMigration {
            version: 1, name: "t1".into(),
            up: "CREATE TABLE stress_t (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL, deleted_at INTEGER)".into(),
            down: Some("DROP TABLE stress_t".into()),
        },
    ];

    for _ in 0..5 {
        let r = db.migrate(migrations.clone()).unwrap();
        assert_eq!(r.current_version, 1);

        db.rollback_to(0, migrations.clone()).unwrap();

        let r = db.migrate(migrations.clone()).unwrap();
        assert_eq!(r.applied, vec![1]);
    }
}

// ─── Benchmark: Paginated Query Throughput ───────────────────────────

#[test]
fn bench_paginate_100_pages() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    // Seed 1000 rows
    let ops: Vec<Op> = (0..1000)
        .map(|i| {
            let mut d = HashMap::new();
            d.insert("name".into(), SqlValue::Text(format!("User{:04}", i)));
            d.insert("email".into(), SqlValue::Text(format!("u{}@t.com", i)));
            d.insert("status".into(), SqlValue::Text("active".into()));
            Op::Insert { table: "users".into(), data: d }
        })
        .collect();
    db.transaction(ops).unwrap();

    let start = std::time::Instant::now();
    for page in 0..100 {
        let spec = query("users");
        let result = db.query_paginate(spec, page, 10).unwrap();
        if page < 100 {
            assert!(result.items.len() <= 10);
        }
    }
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 2000,
        "100 paginated queries took {:?}, expected < 2s",
        elapsed
    );
}

// ─── Soak: Sustained Read/Write ──────────────────────────────────────

#[test]
fn soak_sustained_crud_500_iterations() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    for i in 0..500 {
        // Insert
        let id = insert_user(
            &db,
            &format!("User{}", i),
            &format!("u{}@t.com", i),
            if i % 3 == 0 { "admin" } else { "member" },
            i as f64,
        );

        // Read back
        let mut spec = query("users");
        spec.conditions.push(Condition::Eq {
            column: "id".into(),
            value: SqlValue::Text(id.clone()),
        });
        let row = db.query_first(spec).unwrap();
        assert!(row.is_some());

        // Update every 3rd
        if i % 3 == 0 {
            let mut data = HashMap::new();
            data.insert("score".into(), SqlValue::Real(i as f64 * 2.0));
            db.query_update("users".into(), id.clone(), data).unwrap();
        }

        // Delete every 5th
        if i % 5 == 0 {
            db.query_delete("users".into(), id).unwrap();
        }

        // Count periodically
        if i % 50 == 0 {
            let count = db.query_count(query("users")).unwrap();
            assert!(count <= (i + 1) as u64);
        }
    }

    // Final count: 500 inserted - (500/5 = 100 deleted) = 400
    let final_count = db.query_count(query("users")).unwrap();
    assert_eq!(final_count, 400);

    // Audit integrity should pass for all auto-audited entries
    let report = db.verify_audit_integrity(None, None).unwrap();
    assert!(report.is_clean);
    assert!(report.total_checked >= 500); // at least 500 entries (inserts + updates + deletes)

    // Stats should be consistent
    let stats = db.stats().unwrap();
    assert_eq!(stats.total_tables, 1);
}

// ─── Benchmark: FTS Search Throughput ────────────────────────────────

#[test]
fn bench_fts_100_searches() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.execute_raw(
        "CREATE TABLE articles (id TEXT PRIMARY KEY, title TEXT, body TEXT, \
         tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
         updated_at INTEGER NOT NULL, deleted_at INTEGER)".into(),
        vec![],
    ).unwrap();

    db.build_search_index(
        "articles".into(),
        vec![
            SearchField { name: "title".into(), weight: 3.0, stored: true },
            SearchField { name: "body".into(), weight: 1.0, stored: true },
        ],
    ).unwrap();

    // Seed 500 articles
    for i in 0..500 {
        let mut data = HashMap::new();
        data.insert("title".into(), SqlValue::Text(format!("Article about topic {}", i % 50)));
        data.insert("body".into(), SqlValue::Text(format!("This is the body of article {} with searchable content about various subjects", i)));
        db.query_insert("articles".into(), data).unwrap();
    }

    let start = std::time::Instant::now();
    for i in 0..100 {
        let query_str = format!("topic {}", i % 50);
        let hits = db.search("articles".into(), query_str, 10, false).unwrap();
        assert!(!hits.is_empty());
    }
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_secs() < 5,
        "100 FTS searches on 500 docs took {:?}, expected < 5s",
        elapsed
    );
}

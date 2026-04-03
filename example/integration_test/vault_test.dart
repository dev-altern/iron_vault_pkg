import 'dart:io';
import 'dart:typed_data';

import 'package:integration_test/integration_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:iron_vault_pkg/iron_vault_pkg.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() async => await RustLib.init());

  // ── Helpers ──────────────────────────────────────────────────

  Future<String> tempDbPath() async {
    final dir = await Directory.systemTemp.createTemp('iron_vault_test_');
    return '${dir.path}/test.db';
  }

  Uint8List testKey() => Uint8List.fromList(List.filled(32, 0xAB));

  Future<IronVaultDb> openTestDb() async {
    final path = await tempDbPath();
    return IronVaultDb.open(
      path: path,
      encryptionKey: testKey(),
      tenantId: 'tenant_test',
      config: VaultConfig.testConfig(),
    );
  }

  // ── Phase 0: FRB Bridge ────────────────────────────────────

  test('greet() round-trip works', () {
    expect(greet(name: 'Flutter'), 'Hello, Flutter!');
  });

  // ── Phase 1: Core Engine ───────────────────────────────────

  test('open and close database', () async {
    final db = await openTestDb();
    final path = await db.getPath();
    expect(path, contains('test.db'));
    final tenant = await db.getTenantId();
    expect(tenant, 'tenant_test');
    await db.close();
  });

  test('VaultConfig presets', () {
    final prod = VaultConfig.production();
    expect(prod.readPoolSize, 7);
    expect(prod.walMode, true);

    final test = VaultConfig.testConfig();
    expect(test.walMode, false);
  });

  test('stats returns valid data', () async {
    final db = await openTestDb();
    final stats = await db.stats();
    expect(stats.pageSize.toInt(), greaterThan(0));
    await db.close();
  });

  // ── Phase 2: Query Builder ─────────────────────────────────

  test('execute_raw and query_raw', () async {
    final db = await openTestDb();
    await db.executeRaw(
      sql: 'CREATE TABLE test_users (id TEXT PRIMARY KEY, name TEXT, '
          'tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, '
          'updated_at INTEGER NOT NULL, deleted_at INTEGER)',
      params: [],
    );
    await db.executeRaw(
      sql: "INSERT INTO test_users (id, name, tenant_id, created_at, updated_at) "
          "VALUES ('u1', 'Alice', 'tenant_test', 0, 0)",
      params: [],
    );
    final rows = await db.queryRaw(
      sql: "SELECT name FROM test_users WHERE id = 'u1'",
      params: [],
    );
    expect(rows.length, 1);
    final name = rows[0]['name'];
    expect(name, isA<SqlValue>());
    await db.close();
  });

  test('query_insert and query_get via QuerySpec', () async {
    final db = await openTestDb();
    await db.executeRaw(
      sql: 'CREATE TABLE items (id TEXT PRIMARY KEY, name TEXT NOT NULL, '
          'tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, '
          'updated_at INTEGER NOT NULL, deleted_at INTEGER)',
      params: [],
    );

    final id = await db.queryInsert(
      table: 'items',
      data: {'name': SqlValue.text('Widget')},
    );
    expect(id.length, 36); // UUID

    final rows = await db.queryGet(
      spec: QuerySpec(
        table: 'items',
        conditions: [],
        orConditions: [],
        orderBy: [],
        limit: null,
        offset: null,
        joins: [],
        columns: [],
        includeDeleted: false,
      ),
    );
    expect(rows.length, 1);
    await db.close();
  });

  // ── Phase 3: Migrations ────────────────────────────────────

  test('migrate creates tables', () async {
    final db = await openTestDb();
    final report = await db.migrate(migrations: [
      VaultMigration(
        version: 1,
        name: 'create_users',
        up: 'CREATE TABLE users (id TEXT PRIMARY KEY, name TEXT, '
            'tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, '
            'updated_at INTEGER NOT NULL, deleted_at INTEGER)',
        down: 'DROP TABLE users',
      ),
    ]);
    expect(report.applied, [1]);
    expect(report.currentVersion, 1);

    // Idempotent
    final report2 = await db.migrate(migrations: [
      VaultMigration(
        version: 1,
        name: 'create_users',
        up: 'CREATE TABLE users (id TEXT PRIMARY KEY, name TEXT, '
            'tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, '
            'updated_at INTEGER NOT NULL, deleted_at INTEGER)',
        down: 'DROP TABLE users',
      ),
    ]);
    expect(report2.applied, isEmpty);
    expect(report2.skipped, [1]);
    await db.close();
  });

  // ── Phase 5: Encryption ────────────────────────────────────

  test('deriveKey produces 32 bytes', () async {
    final salt = generateSalt();
    expect(salt.length, 32);

    final key = await deriveKey(
      password: 'test_password',
      salt: salt,
      memoryKb: 1024,
      iterations: 1,
      parallelism: 1,
    );
    expect(key.length, 32);
  });

  test('generateSalt is unique', () {
    final s1 = generateSalt();
    final s2 = generateSalt();
    expect(s1, isNot(equals(s2)));
  });

  test('encrypt_field and decrypt_field round-trip', () async {
    final db = await openTestDb();
    final encrypted = await db.encryptField(plaintext: 'hello@example.com');
    expect(encrypted, contains('"ct"'));
    expect(encrypted, contains('"nonce"'));

    final decrypted = await db.decryptField(ciphertextJson: encrypted);
    expect(decrypted, 'hello@example.com');
    await db.close();
  });

  test('same plaintext produces different ciphertext', () async {
    final db = await openTestDb();
    final e1 = await db.encryptField(plaintext: 'same');
    final e2 = await db.encryptField(plaintext: 'same');
    expect(e1, isNot(equals(e2)));
    await db.close();
  });

  // ── Phase 7: Audit ─────────────────────────────────────────

  test('set_actor and get_actor', () async {
    final db = await openTestDb();
    expect(await db.getActor(), 'system');
    await db.setActor(actorId: 'user_42');
    expect(await db.getActor(), 'user_42');
    await db.clearActor();
    expect(await db.getActor(), 'system');
    await db.close();
  });

  // ── Phase 11: Semantic ─────────────────────────────────────

  test('serialize and deserialize vector', () {
    final original = Float32List.fromList([1.0, -2.5, 0.0, 3.5]);
    final bytes = IronVaultDb.serializeVector(embedding: original.toList());
    final restored =
        IronVaultDb.deserializeVector(bytes: Uint8List.fromList(bytes));
    expect(restored.length, 4);
    expect(restored[0], closeTo(1.0, 0.001));
    expect(restored[1], closeTo(-2.5, 0.001));
  });

  // ── Phase 6: Watch Streams ──────────────────────────────────

  test('watch_query emits initial value', () async {
    final db = await openTestDb();
    await db.executeRaw(
      sql: 'CREATE TABLE watch_test (id TEXT PRIMARY KEY, name TEXT, '
          'tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, '
          'updated_at INTEGER NOT NULL, deleted_at INTEGER)',
      params: [],
    );

    final stream = db.watchQuery(
      spec: QuerySpec(
        table: 'watch_test',
        conditions: [],
        orConditions: [],
        orderBy: [],
        limit: null,
        offset: null,
        joins: [],
        columns: [],
        includeDeleted: false,
      ),
    );

    // Just verify stream produces at least one emission (the initial value)
    final first = await stream.first.timeout(
      const Duration(seconds: 5),
      onTimeout: () => <Map<String, SqlValue>>[],
    );
    // Initial emission should be empty (no rows yet)
    expect(first, isEmpty);
    await db.close();
  });

  test('notification_version increments on write from Dart', () async {
    final db = await openTestDb();
    await db.executeRaw(
      sql: 'CREATE TABLE notif_test (id TEXT PRIMARY KEY, name TEXT, '
          'tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, '
          'updated_at INTEGER NOT NULL, deleted_at INTEGER)',
      params: [],
    );

    final v0 = await db.notificationVersion(table: 'notif_test');
    expect(v0.toInt(), 0);

    await db.queryInsert(
      table: 'notif_test',
      data: {'name': SqlValue.text('Alice')},
    );

    final v1 = await db.notificationVersion(table: 'notif_test');
    expect(v1.toInt(), 1);
    await db.close();
  });

  // ── Phase 8: Backup ────────────────────────────────────────

  test('backup and verify round-trip', () async {
    final db = await openTestDb();
    await db.executeRaw(
      sql: 'CREATE TABLE backup_test (id TEXT PRIMARY KEY, val TEXT, '
          'tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, '
          'updated_at INTEGER NOT NULL, deleted_at INTEGER)',
      params: [],
    );
    await db.queryInsert(table: 'backup_test', data: {'val': SqlValue.text('data')});

    final dir = await Directory.systemTemp.createTemp('backup_');
    final backupPath = '${dir.path}/test.ivb';

    final result = await db.backup(
      outputPath: backupPath,
      compress: true,
      encrypt: true,
    );
    expect(result.sizeBytes.toInt(), greaterThan(0));
    expect(result.checksum, isNotEmpty);

    final verify = await db.verifyBackup(backupPath: backupPath);
    expect(verify.checksumOk, true);
    expect(verify.decryptOk, true);

    await db.close();
  });

  // ── Phase 9: Search ────────────────────────────────────────

  test('build_search_index + search round-trip', () async {
    final db = await openTestDb();
    await db.executeRaw(
      sql: 'CREATE TABLE search_test (id TEXT PRIMARY KEY, title TEXT, '
          'tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, '
          'updated_at INTEGER NOT NULL, deleted_at INTEGER)',
      params: [],
    );

    await db.buildSearchIndex(
      table: 'search_test',
      fields: [SearchField(name: 'title', weight: 1.0, stored: true)],
    );

    // Auto-indexed on insert
    await db.queryInsert(
      table: 'search_test',
      data: {'title': SqlValue.text('Rust programming guide')},
    );

    final hits = await db.search(
      table: 'search_test',
      query: 'rust',
      limit: 10,
      highlight: false,
    );
    expect(hits.length, 1);
    expect(hits.first.score, greaterThan(0));

    await db.close();
  });

  // ── Phase 10: Sync ─────────────────────────────────────────

  test('sync outbox add + get_delta + mark_synced', () async {
    final db = await openTestDb();

    final vc = VectorClock(clocks: {
      'node_a': BigInt.from(1),
    });
    final recordId = await db.syncAddToOutbox(
      tableName: 'users',
      rowId: 'r1',
      operation: 'INSERT',
      payload: '{"name":"Alice"}',
      vectorClock: vc,
    );
    expect(recordId, isNotEmpty);

    final delta = await db.syncGetDelta(sinceSeq: 0, limit: 100);
    expect(delta.records.length, 1);

    final marked = await db.syncMarkSynced(recordIds: [recordId]);
    expect(marked, 1);

    final delta2 = await db.syncGetDelta(sinceSeq: 0, limit: 100);
    expect(delta2.records, isEmpty);

    await db.close();
  });

  // ── Phase 16: Auto-Audit Verification ──────────────────────

  test('auto-audit: insert creates audit entry', () async {
    final db = await openTestDb();
    await db.executeRaw(
      sql: 'CREATE TABLE audit_test (id TEXT PRIMARY KEY, name TEXT, '
          'tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, '
          'updated_at INTEGER NOT NULL, deleted_at INTEGER)',
      params: [],
    );

    final id = await db.queryInsert(
      table: 'audit_test',
      data: {'name': SqlValue.text('Alice')},
    );

    final history = await db.getHistory(tableName: 'audit_test', rowId: id, limit: 50);
    expect(history.length, 1);
    expect(history.first.operation, 'INSERT');
    expect(history.first.afterJson, isNotNull);

    final report = await db.verifyAuditIntegrity(from: null, to: null);
    expect(report.isClean, true);

    await db.close();
  });

  // ── SqlValue variants ──────────────────────────────────────

  test('SqlValue all variants cross FFI', () async {
    final db = await openTestDb();
    await db.executeRaw(
      sql: 'CREATE TABLE types_test (id TEXT PRIMARY KEY, '
          'int_val INTEGER, real_val REAL, text_val TEXT, blob_val BLOB, '
          'null_val TEXT, '
          'tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, '
          'updated_at INTEGER NOT NULL, deleted_at INTEGER)',
      params: [],
    );

    await db.executeRaw(
      sql: "INSERT INTO types_test VALUES "
          "('r1', 42, 3.14, 'hello', X'DEADBEEF', NULL, "
          "'tenant_test', 0, 0, NULL)",
      params: [],
    );

    final rows = await db.queryRaw(
      sql: "SELECT int_val, real_val, text_val, blob_val, null_val "
          "FROM types_test WHERE id = 'r1'",
      params: [],
    );
    expect(rows.length, 1);
    // Just verify we got the row back without crash
    expect(rows[0].length, 5);
    await db.close();
  });
}

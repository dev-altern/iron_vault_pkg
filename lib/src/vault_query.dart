import 'package:iron_vault_pkg/iron_vault_pkg.dart';

/// Fluent query builder for IronVault.
///
/// Provides a Dart-idiomatic API on top of the FRB-generated `IronVaultDb`
/// methods. Builds a [QuerySpec] and dispatches to the appropriate method.
///
/// ```dart
/// final users = await db.from('users')
///   .where_('status', SqlValue.text('active'))
///   .orderByDesc('created_at')
///   .limit(20)
///   .get();
/// ```
class VaultQuery {
  final IronVaultDb _db;
  final String _table;
  final List<Condition> _conditions = [];
  final List<List<Condition>> _orConditions = [];
  final List<OrderBy> _orderBy = [];
  final List<JoinSpec> _joins = [];
  final List<String> _columns = [];
  int? _limit;
  int? _offset;
  bool _includeDeleted = false;

  VaultQuery._(this._db, this._table);

  // ── Filtering ──────────────────────────────────────────────────

  /// `column = value`
  VaultQuery where_(String column, SqlValue value) {
    _conditions.add(Condition.eq(column: column, value: value));
    return this;
  }

  /// `column != value`
  VaultQuery whereNot(String column, SqlValue value) {
    _conditions.add(Condition.notEq(column: column, value: value));
    return this;
  }

  /// `column > value`
  VaultQuery whereGt(String column, SqlValue value) {
    _conditions.add(Condition.gt(column: column, value: value));
    return this;
  }

  /// `column >= value`
  VaultQuery whereGte(String column, SqlValue value) {
    _conditions.add(Condition.gte(column: column, value: value));
    return this;
  }

  /// `column < value`
  VaultQuery whereLt(String column, SqlValue value) {
    _conditions.add(Condition.lt(column: column, value: value));
    return this;
  }

  /// `column <= value`
  VaultQuery whereLte(String column, SqlValue value) {
    _conditions.add(Condition.lte(column: column, value: value));
    return this;
  }

  /// `column LIKE pattern`
  VaultQuery whereLike(String column, String pattern) {
    _conditions.add(Condition.like(column: column, pattern: pattern));
    return this;
  }

  /// `column BETWEEN low AND high`
  VaultQuery whereBetween(String column, SqlValue low, SqlValue high) {
    _conditions.add(Condition.between(column: column, low: low, high: high));
    return this;
  }

  /// `column IN (values...)`
  VaultQuery whereIn(String column, List<SqlValue> values) {
    _conditions.add(Condition.in_(column: column, values: values));
    return this;
  }

  /// `column NOT IN (values...)`
  VaultQuery whereNotIn(String column, List<SqlValue> values) {
    _conditions.add(Condition.notIn(column: column, values: values));
    return this;
  }

  /// `column IS NULL`
  VaultQuery whereNull(String column) {
    _conditions.add(Condition.isNull(column: column));
    return this;
  }

  /// `column IS NOT NULL`
  VaultQuery whereNotNull(String column) {
    _conditions.add(Condition.isNotNull(column: column));
    return this;
  }

  /// Raw SQL condition with parameterized values.
  VaultQuery whereRaw(String sql, [List<SqlValue> params = const []]) {
    _conditions.add(Condition.raw(sql: sql, params: params));
    return this;
  }

  /// OR group — conditions inside are ANDed, group is ORed with main.
  VaultQuery orWhere(void Function(VaultQueryGroup group) builder) {
    final group = VaultQueryGroup._();
    builder(group);
    _orConditions.add(group._conditions);
    return this;
  }

  // ── Selection ──────────────────────────────────────────────────

  /// Select specific columns (default: all).
  VaultQuery select(List<String> columns) {
    _columns.addAll(columns);
    return this;
  }

  // ── Ordering ───────────────────────────────────────────────────

  /// Order by column ascending.
  VaultQuery orderByAsc(String column) {
    _orderBy.add(OrderBy.asc(column: column));
    return this;
  }

  /// Order by column descending.
  VaultQuery orderByDesc(String column) {
    _orderBy.add(OrderBy.desc(column: column));
    return this;
  }

  /// Raw ORDER BY expression.
  VaultQuery orderByRaw(String expression) {
    _orderBy.add(OrderBy.raw(expression: expression));
    return this;
  }

  // ── Pagination ─────────────────────────────────────────────────

  /// Limit the number of results.
  VaultQuery limit(int n) {
    _limit = n;
    return this;
  }

  /// Skip rows before returning results.
  VaultQuery offset(int n) {
    _offset = n;
    return this;
  }

  // ── Joins ──────────────────────────────────────────────────────

  /// INNER JOIN.
  VaultQuery join(String table, String on) {
    _joins.add(JoinSpec.inner(table: table, on_: on));
    return this;
  }

  /// LEFT JOIN.
  VaultQuery leftJoin(String table, String on) {
    _joins.add(JoinSpec.left(table: table, on_: on));
    return this;
  }

  // ── Soft Delete ────────────────────────────────────────────────

  /// Include soft-deleted rows in results.
  VaultQuery withDeleted() {
    _includeDeleted = true;
    return this;
  }

  // ── Execution: Read ────────────────────────────────────────────

  /// Execute and return all matching rows.
  Future<List<Map<String, SqlValue>>> get() =>
      _db.queryGet(spec: _buildSpec());

  /// Execute and return the first matching row.
  Future<Map<String, SqlValue>?> first() =>
      _db.queryFirst(spec: _buildSpec());

  /// Count matching rows.
  Future<int> count() async {
    final c = await _db.queryCount(spec: _buildSpec());
    return c.toInt();
  }

  /// Check if any rows match.
  Future<bool> exists() => _db.queryExists(spec: _buildSpec());

  /// Execute a paginated query (page is 0-based).
  Future<Page> paginate(int page, int pageSize) =>
      _db.queryPaginate(spec: _buildSpec(), page: page, pageSize: pageSize);

  /// Execute an aggregate query.
  Future<Map<String, SqlValue>> aggregate(List<AggExpr> expressions) =>
      _db.queryAggregate(spec: _buildSpec(), expressions: expressions);

  // ── Execution: Write ───────────────────────────────────────────

  /// Insert a row. Returns the generated or provided id.
  Future<String> insert(Map<String, SqlValue> data) =>
      _db.queryInsert(table: _table, data: data);

  /// Update a row by id. Returns rows affected (0 or 1).
  Future<int> update(String id, Map<String, SqlValue> data) async {
    final n = await _db.queryUpdate(table: _table, id: id, data: data);
    return n.toInt();
  }

  /// Upsert (insert or update on conflict).
  Future<String> upsert(
    Map<String, SqlValue> data, {
    required String conflictColumn,
  }) =>
      _db.queryUpsert(
        table: _table,
        data: data,
        conflictColumn: conflictColumn,
      );

  /// Soft-delete a row. Returns rows affected.
  Future<int> delete(String id) async {
    final n = await _db.queryDelete(table: _table, id: id);
    return n.toInt();
  }

  /// Permanently delete a row. Returns rows affected.
  Future<int> hardDelete(String id) async {
    final n = await _db.queryHardDelete(table: _table, id: id);
    return n.toInt();
  }

  /// Batch insert. Returns list of ids.
  Future<List<String>> insertBatch(List<Map<String, SqlValue>> rows) =>
      _db.queryInsertBatch(table: _table, rows: rows);

  /// Batch update. Returns total rows affected.
  Future<int> updateBatch(List<UpdateEntry> updates) async {
    final n = await _db.queryUpdateBatch(table: _table, updates: updates);
    return n.toInt();
  }

  /// Batch soft-delete. Returns total rows affected.
  Future<int> deleteBatch(List<String> ids) async {
    final n = await _db.queryDeleteBatch(table: _table, ids: ids);
    return n.toInt();
  }

  // ── Internal ───────────────────────────────────────────────────

  QuerySpec _buildSpec() => QuerySpec(
        table: _table,
        conditions: _conditions,
        orConditions: _orConditions,
        orderBy: _orderBy,
        limit: _limit,
        offset: _offset,
        joins: _joins,
        columns: _columns,
        includeDeleted: _includeDeleted,
      );
}

/// Condition group builder for [VaultQuery.orWhere].
class VaultQueryGroup {
  final List<Condition> _conditions = [];

  VaultQueryGroup._();

  VaultQueryGroup where_(String column, SqlValue value) {
    _conditions.add(Condition.eq(column: column, value: value));
    return this;
  }

  VaultQueryGroup whereGt(String column, SqlValue value) {
    _conditions.add(Condition.gt(column: column, value: value));
    return this;
  }

  VaultQueryGroup whereGte(String column, SqlValue value) {
    _conditions.add(Condition.gte(column: column, value: value));
    return this;
  }

  VaultQueryGroup whereLt(String column, SqlValue value) {
    _conditions.add(Condition.lt(column: column, value: value));
    return this;
  }

  VaultQueryGroup whereLte(String column, SqlValue value) {
    _conditions.add(Condition.lte(column: column, value: value));
    return this;
  }

  VaultQueryGroup whereLike(String column, String pattern) {
    _conditions.add(Condition.like(column: column, pattern: pattern));
    return this;
  }

  VaultQueryGroup whereIn(String column, List<SqlValue> values) {
    _conditions.add(Condition.in_(column: column, values: values));
    return this;
  }

  VaultQueryGroup whereNull(String column) {
    _conditions.add(Condition.isNull(column: column));
    return this;
  }

  VaultQueryGroup whereNotNull(String column) {
    _conditions.add(Condition.isNotNull(column: column));
    return this;
  }

  VaultQueryGroup whereRaw(String sql, [List<SqlValue> params = const []]) {
    _conditions.add(Condition.raw(sql: sql, params: params));
    return this;
  }
}

/// Extension to add `from()` entry point on [IronVaultDb].
extension IronVaultDbQuery on IronVaultDb {
  /// Start building a query for a table.
  VaultQuery from(String table) => VaultQuery._(this, table);
}

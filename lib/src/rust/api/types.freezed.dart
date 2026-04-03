// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'types.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$AggExpr {
  String get column => throw _privateConstructorUsedError;
  String get alias => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, String alias) count,
    required TResult Function(String column, String alias) sum,
    required TResult Function(String column, String alias) avg,
    required TResult Function(String column, String alias) min,
    required TResult Function(String column, String alias) max,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, String alias)? count,
    TResult? Function(String column, String alias)? sum,
    TResult? Function(String column, String alias)? avg,
    TResult? Function(String column, String alias)? min,
    TResult? Function(String column, String alias)? max,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, String alias)? count,
    TResult Function(String column, String alias)? sum,
    TResult Function(String column, String alias)? avg,
    TResult Function(String column, String alias)? min,
    TResult Function(String column, String alias)? max,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AggExpr_Count value) count,
    required TResult Function(AggExpr_Sum value) sum,
    required TResult Function(AggExpr_Avg value) avg,
    required TResult Function(AggExpr_Min value) min,
    required TResult Function(AggExpr_Max value) max,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AggExpr_Count value)? count,
    TResult? Function(AggExpr_Sum value)? sum,
    TResult? Function(AggExpr_Avg value)? avg,
    TResult? Function(AggExpr_Min value)? min,
    TResult? Function(AggExpr_Max value)? max,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AggExpr_Count value)? count,
    TResult Function(AggExpr_Sum value)? sum,
    TResult Function(AggExpr_Avg value)? avg,
    TResult Function(AggExpr_Min value)? min,
    TResult Function(AggExpr_Max value)? max,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $AggExprCopyWith<AggExpr> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $AggExprCopyWith<$Res> {
  factory $AggExprCopyWith(AggExpr value, $Res Function(AggExpr) then) =
      _$AggExprCopyWithImpl<$Res, AggExpr>;
  @useResult
  $Res call({String column, String alias});
}

/// @nodoc
class _$AggExprCopyWithImpl<$Res, $Val extends AggExpr>
    implements $AggExprCopyWith<$Res> {
  _$AggExprCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? alias = null}) {
    return _then(
      _value.copyWith(
            column: null == column
                ? _value.column
                : column // ignore: cast_nullable_to_non_nullable
                      as String,
            alias: null == alias
                ? _value.alias
                : alias // ignore: cast_nullable_to_non_nullable
                      as String,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$AggExpr_CountImplCopyWith<$Res>
    implements $AggExprCopyWith<$Res> {
  factory _$$AggExpr_CountImplCopyWith(
    _$AggExpr_CountImpl value,
    $Res Function(_$AggExpr_CountImpl) then,
  ) = __$$AggExpr_CountImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String column, String alias});
}

/// @nodoc
class __$$AggExpr_CountImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_CountImpl>
    implements _$$AggExpr_CountImplCopyWith<$Res> {
  __$$AggExpr_CountImplCopyWithImpl(
    _$AggExpr_CountImpl _value,
    $Res Function(_$AggExpr_CountImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? alias = null}) {
    return _then(
      _$AggExpr_CountImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        alias: null == alias
            ? _value.alias
            : alias // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$AggExpr_CountImpl extends AggExpr_Count {
  const _$AggExpr_CountImpl({required this.column, required this.alias})
    : super._();

  @override
  final String column;
  @override
  final String alias;

  @override
  String toString() {
    return 'AggExpr.count(column: $column, alias: $alias)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_CountImpl &&
            (identical(other.column, column) || other.column == column) &&
            (identical(other.alias, alias) || other.alias == alias));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column, alias);

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_CountImplCopyWith<_$AggExpr_CountImpl> get copyWith =>
      __$$AggExpr_CountImplCopyWithImpl<_$AggExpr_CountImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, String alias) count,
    required TResult Function(String column, String alias) sum,
    required TResult Function(String column, String alias) avg,
    required TResult Function(String column, String alias) min,
    required TResult Function(String column, String alias) max,
  }) {
    return count(column, alias);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, String alias)? count,
    TResult? Function(String column, String alias)? sum,
    TResult? Function(String column, String alias)? avg,
    TResult? Function(String column, String alias)? min,
    TResult? Function(String column, String alias)? max,
  }) {
    return count?.call(column, alias);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, String alias)? count,
    TResult Function(String column, String alias)? sum,
    TResult Function(String column, String alias)? avg,
    TResult Function(String column, String alias)? min,
    TResult Function(String column, String alias)? max,
    required TResult orElse(),
  }) {
    if (count != null) {
      return count(column, alias);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AggExpr_Count value) count,
    required TResult Function(AggExpr_Sum value) sum,
    required TResult Function(AggExpr_Avg value) avg,
    required TResult Function(AggExpr_Min value) min,
    required TResult Function(AggExpr_Max value) max,
  }) {
    return count(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AggExpr_Count value)? count,
    TResult? Function(AggExpr_Sum value)? sum,
    TResult? Function(AggExpr_Avg value)? avg,
    TResult? Function(AggExpr_Min value)? min,
    TResult? Function(AggExpr_Max value)? max,
  }) {
    return count?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AggExpr_Count value)? count,
    TResult Function(AggExpr_Sum value)? sum,
    TResult Function(AggExpr_Avg value)? avg,
    TResult Function(AggExpr_Min value)? min,
    TResult Function(AggExpr_Max value)? max,
    required TResult orElse(),
  }) {
    if (count != null) {
      return count(this);
    }
    return orElse();
  }
}

abstract class AggExpr_Count extends AggExpr {
  const factory AggExpr_Count({
    required final String column,
    required final String alias,
  }) = _$AggExpr_CountImpl;
  const AggExpr_Count._() : super._();

  @override
  String get column;
  @override
  String get alias;

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AggExpr_CountImplCopyWith<_$AggExpr_CountImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_SumImplCopyWith<$Res>
    implements $AggExprCopyWith<$Res> {
  factory _$$AggExpr_SumImplCopyWith(
    _$AggExpr_SumImpl value,
    $Res Function(_$AggExpr_SumImpl) then,
  ) = __$$AggExpr_SumImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String column, String alias});
}

/// @nodoc
class __$$AggExpr_SumImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_SumImpl>
    implements _$$AggExpr_SumImplCopyWith<$Res> {
  __$$AggExpr_SumImplCopyWithImpl(
    _$AggExpr_SumImpl _value,
    $Res Function(_$AggExpr_SumImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? alias = null}) {
    return _then(
      _$AggExpr_SumImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        alias: null == alias
            ? _value.alias
            : alias // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$AggExpr_SumImpl extends AggExpr_Sum {
  const _$AggExpr_SumImpl({required this.column, required this.alias})
    : super._();

  @override
  final String column;
  @override
  final String alias;

  @override
  String toString() {
    return 'AggExpr.sum(column: $column, alias: $alias)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_SumImpl &&
            (identical(other.column, column) || other.column == column) &&
            (identical(other.alias, alias) || other.alias == alias));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column, alias);

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_SumImplCopyWith<_$AggExpr_SumImpl> get copyWith =>
      __$$AggExpr_SumImplCopyWithImpl<_$AggExpr_SumImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, String alias) count,
    required TResult Function(String column, String alias) sum,
    required TResult Function(String column, String alias) avg,
    required TResult Function(String column, String alias) min,
    required TResult Function(String column, String alias) max,
  }) {
    return sum(column, alias);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, String alias)? count,
    TResult? Function(String column, String alias)? sum,
    TResult? Function(String column, String alias)? avg,
    TResult? Function(String column, String alias)? min,
    TResult? Function(String column, String alias)? max,
  }) {
    return sum?.call(column, alias);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, String alias)? count,
    TResult Function(String column, String alias)? sum,
    TResult Function(String column, String alias)? avg,
    TResult Function(String column, String alias)? min,
    TResult Function(String column, String alias)? max,
    required TResult orElse(),
  }) {
    if (sum != null) {
      return sum(column, alias);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AggExpr_Count value) count,
    required TResult Function(AggExpr_Sum value) sum,
    required TResult Function(AggExpr_Avg value) avg,
    required TResult Function(AggExpr_Min value) min,
    required TResult Function(AggExpr_Max value) max,
  }) {
    return sum(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AggExpr_Count value)? count,
    TResult? Function(AggExpr_Sum value)? sum,
    TResult? Function(AggExpr_Avg value)? avg,
    TResult? Function(AggExpr_Min value)? min,
    TResult? Function(AggExpr_Max value)? max,
  }) {
    return sum?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AggExpr_Count value)? count,
    TResult Function(AggExpr_Sum value)? sum,
    TResult Function(AggExpr_Avg value)? avg,
    TResult Function(AggExpr_Min value)? min,
    TResult Function(AggExpr_Max value)? max,
    required TResult orElse(),
  }) {
    if (sum != null) {
      return sum(this);
    }
    return orElse();
  }
}

abstract class AggExpr_Sum extends AggExpr {
  const factory AggExpr_Sum({
    required final String column,
    required final String alias,
  }) = _$AggExpr_SumImpl;
  const AggExpr_Sum._() : super._();

  @override
  String get column;
  @override
  String get alias;

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AggExpr_SumImplCopyWith<_$AggExpr_SumImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_AvgImplCopyWith<$Res>
    implements $AggExprCopyWith<$Res> {
  factory _$$AggExpr_AvgImplCopyWith(
    _$AggExpr_AvgImpl value,
    $Res Function(_$AggExpr_AvgImpl) then,
  ) = __$$AggExpr_AvgImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String column, String alias});
}

/// @nodoc
class __$$AggExpr_AvgImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_AvgImpl>
    implements _$$AggExpr_AvgImplCopyWith<$Res> {
  __$$AggExpr_AvgImplCopyWithImpl(
    _$AggExpr_AvgImpl _value,
    $Res Function(_$AggExpr_AvgImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? alias = null}) {
    return _then(
      _$AggExpr_AvgImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        alias: null == alias
            ? _value.alias
            : alias // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$AggExpr_AvgImpl extends AggExpr_Avg {
  const _$AggExpr_AvgImpl({required this.column, required this.alias})
    : super._();

  @override
  final String column;
  @override
  final String alias;

  @override
  String toString() {
    return 'AggExpr.avg(column: $column, alias: $alias)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_AvgImpl &&
            (identical(other.column, column) || other.column == column) &&
            (identical(other.alias, alias) || other.alias == alias));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column, alias);

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_AvgImplCopyWith<_$AggExpr_AvgImpl> get copyWith =>
      __$$AggExpr_AvgImplCopyWithImpl<_$AggExpr_AvgImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, String alias) count,
    required TResult Function(String column, String alias) sum,
    required TResult Function(String column, String alias) avg,
    required TResult Function(String column, String alias) min,
    required TResult Function(String column, String alias) max,
  }) {
    return avg(column, alias);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, String alias)? count,
    TResult? Function(String column, String alias)? sum,
    TResult? Function(String column, String alias)? avg,
    TResult? Function(String column, String alias)? min,
    TResult? Function(String column, String alias)? max,
  }) {
    return avg?.call(column, alias);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, String alias)? count,
    TResult Function(String column, String alias)? sum,
    TResult Function(String column, String alias)? avg,
    TResult Function(String column, String alias)? min,
    TResult Function(String column, String alias)? max,
    required TResult orElse(),
  }) {
    if (avg != null) {
      return avg(column, alias);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AggExpr_Count value) count,
    required TResult Function(AggExpr_Sum value) sum,
    required TResult Function(AggExpr_Avg value) avg,
    required TResult Function(AggExpr_Min value) min,
    required TResult Function(AggExpr_Max value) max,
  }) {
    return avg(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AggExpr_Count value)? count,
    TResult? Function(AggExpr_Sum value)? sum,
    TResult? Function(AggExpr_Avg value)? avg,
    TResult? Function(AggExpr_Min value)? min,
    TResult? Function(AggExpr_Max value)? max,
  }) {
    return avg?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AggExpr_Count value)? count,
    TResult Function(AggExpr_Sum value)? sum,
    TResult Function(AggExpr_Avg value)? avg,
    TResult Function(AggExpr_Min value)? min,
    TResult Function(AggExpr_Max value)? max,
    required TResult orElse(),
  }) {
    if (avg != null) {
      return avg(this);
    }
    return orElse();
  }
}

abstract class AggExpr_Avg extends AggExpr {
  const factory AggExpr_Avg({
    required final String column,
    required final String alias,
  }) = _$AggExpr_AvgImpl;
  const AggExpr_Avg._() : super._();

  @override
  String get column;
  @override
  String get alias;

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AggExpr_AvgImplCopyWith<_$AggExpr_AvgImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_MinImplCopyWith<$Res>
    implements $AggExprCopyWith<$Res> {
  factory _$$AggExpr_MinImplCopyWith(
    _$AggExpr_MinImpl value,
    $Res Function(_$AggExpr_MinImpl) then,
  ) = __$$AggExpr_MinImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String column, String alias});
}

/// @nodoc
class __$$AggExpr_MinImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_MinImpl>
    implements _$$AggExpr_MinImplCopyWith<$Res> {
  __$$AggExpr_MinImplCopyWithImpl(
    _$AggExpr_MinImpl _value,
    $Res Function(_$AggExpr_MinImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? alias = null}) {
    return _then(
      _$AggExpr_MinImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        alias: null == alias
            ? _value.alias
            : alias // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$AggExpr_MinImpl extends AggExpr_Min {
  const _$AggExpr_MinImpl({required this.column, required this.alias})
    : super._();

  @override
  final String column;
  @override
  final String alias;

  @override
  String toString() {
    return 'AggExpr.min(column: $column, alias: $alias)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_MinImpl &&
            (identical(other.column, column) || other.column == column) &&
            (identical(other.alias, alias) || other.alias == alias));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column, alias);

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_MinImplCopyWith<_$AggExpr_MinImpl> get copyWith =>
      __$$AggExpr_MinImplCopyWithImpl<_$AggExpr_MinImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, String alias) count,
    required TResult Function(String column, String alias) sum,
    required TResult Function(String column, String alias) avg,
    required TResult Function(String column, String alias) min,
    required TResult Function(String column, String alias) max,
  }) {
    return min(column, alias);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, String alias)? count,
    TResult? Function(String column, String alias)? sum,
    TResult? Function(String column, String alias)? avg,
    TResult? Function(String column, String alias)? min,
    TResult? Function(String column, String alias)? max,
  }) {
    return min?.call(column, alias);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, String alias)? count,
    TResult Function(String column, String alias)? sum,
    TResult Function(String column, String alias)? avg,
    TResult Function(String column, String alias)? min,
    TResult Function(String column, String alias)? max,
    required TResult orElse(),
  }) {
    if (min != null) {
      return min(column, alias);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AggExpr_Count value) count,
    required TResult Function(AggExpr_Sum value) sum,
    required TResult Function(AggExpr_Avg value) avg,
    required TResult Function(AggExpr_Min value) min,
    required TResult Function(AggExpr_Max value) max,
  }) {
    return min(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AggExpr_Count value)? count,
    TResult? Function(AggExpr_Sum value)? sum,
    TResult? Function(AggExpr_Avg value)? avg,
    TResult? Function(AggExpr_Min value)? min,
    TResult? Function(AggExpr_Max value)? max,
  }) {
    return min?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AggExpr_Count value)? count,
    TResult Function(AggExpr_Sum value)? sum,
    TResult Function(AggExpr_Avg value)? avg,
    TResult Function(AggExpr_Min value)? min,
    TResult Function(AggExpr_Max value)? max,
    required TResult orElse(),
  }) {
    if (min != null) {
      return min(this);
    }
    return orElse();
  }
}

abstract class AggExpr_Min extends AggExpr {
  const factory AggExpr_Min({
    required final String column,
    required final String alias,
  }) = _$AggExpr_MinImpl;
  const AggExpr_Min._() : super._();

  @override
  String get column;
  @override
  String get alias;

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AggExpr_MinImplCopyWith<_$AggExpr_MinImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_MaxImplCopyWith<$Res>
    implements $AggExprCopyWith<$Res> {
  factory _$$AggExpr_MaxImplCopyWith(
    _$AggExpr_MaxImpl value,
    $Res Function(_$AggExpr_MaxImpl) then,
  ) = __$$AggExpr_MaxImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String column, String alias});
}

/// @nodoc
class __$$AggExpr_MaxImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_MaxImpl>
    implements _$$AggExpr_MaxImplCopyWith<$Res> {
  __$$AggExpr_MaxImplCopyWithImpl(
    _$AggExpr_MaxImpl _value,
    $Res Function(_$AggExpr_MaxImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? alias = null}) {
    return _then(
      _$AggExpr_MaxImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        alias: null == alias
            ? _value.alias
            : alias // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$AggExpr_MaxImpl extends AggExpr_Max {
  const _$AggExpr_MaxImpl({required this.column, required this.alias})
    : super._();

  @override
  final String column;
  @override
  final String alias;

  @override
  String toString() {
    return 'AggExpr.max(column: $column, alias: $alias)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_MaxImpl &&
            (identical(other.column, column) || other.column == column) &&
            (identical(other.alias, alias) || other.alias == alias));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column, alias);

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_MaxImplCopyWith<_$AggExpr_MaxImpl> get copyWith =>
      __$$AggExpr_MaxImplCopyWithImpl<_$AggExpr_MaxImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, String alias) count,
    required TResult Function(String column, String alias) sum,
    required TResult Function(String column, String alias) avg,
    required TResult Function(String column, String alias) min,
    required TResult Function(String column, String alias) max,
  }) {
    return max(column, alias);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, String alias)? count,
    TResult? Function(String column, String alias)? sum,
    TResult? Function(String column, String alias)? avg,
    TResult? Function(String column, String alias)? min,
    TResult? Function(String column, String alias)? max,
  }) {
    return max?.call(column, alias);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, String alias)? count,
    TResult Function(String column, String alias)? sum,
    TResult Function(String column, String alias)? avg,
    TResult Function(String column, String alias)? min,
    TResult Function(String column, String alias)? max,
    required TResult orElse(),
  }) {
    if (max != null) {
      return max(column, alias);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AggExpr_Count value) count,
    required TResult Function(AggExpr_Sum value) sum,
    required TResult Function(AggExpr_Avg value) avg,
    required TResult Function(AggExpr_Min value) min,
    required TResult Function(AggExpr_Max value) max,
  }) {
    return max(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AggExpr_Count value)? count,
    TResult? Function(AggExpr_Sum value)? sum,
    TResult? Function(AggExpr_Avg value)? avg,
    TResult? Function(AggExpr_Min value)? min,
    TResult? Function(AggExpr_Max value)? max,
  }) {
    return max?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AggExpr_Count value)? count,
    TResult Function(AggExpr_Sum value)? sum,
    TResult Function(AggExpr_Avg value)? avg,
    TResult Function(AggExpr_Min value)? min,
    TResult Function(AggExpr_Max value)? max,
    required TResult orElse(),
  }) {
    if (max != null) {
      return max(this);
    }
    return orElse();
  }
}

abstract class AggExpr_Max extends AggExpr {
  const factory AggExpr_Max({
    required final String column,
    required final String alias,
  }) = _$AggExpr_MaxImpl;
  const AggExpr_Max._() : super._();

  @override
  String get column;
  @override
  String get alias;

  /// Create a copy of AggExpr
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AggExpr_MaxImplCopyWith<_$AggExpr_MaxImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Condition {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ConditionCopyWith<$Res> {
  factory $ConditionCopyWith(Condition value, $Res Function(Condition) then) =
      _$ConditionCopyWithImpl<$Res, Condition>;
}

/// @nodoc
class _$ConditionCopyWithImpl<$Res, $Val extends Condition>
    implements $ConditionCopyWith<$Res> {
  _$ConditionCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$Condition_EqImplCopyWith<$Res> {
  factory _$$Condition_EqImplCopyWith(
    _$Condition_EqImpl value,
    $Res Function(_$Condition_EqImpl) then,
  ) = __$$Condition_EqImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column, SqlValue value});

  $SqlValueCopyWith<$Res> get value;
}

/// @nodoc
class __$$Condition_EqImplCopyWithImpl<$Res>
    extends _$ConditionCopyWithImpl<$Res, _$Condition_EqImpl>
    implements _$$Condition_EqImplCopyWith<$Res> {
  __$$Condition_EqImplCopyWithImpl(
    _$Condition_EqImpl _value,
    $Res Function(_$Condition_EqImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? value = null}) {
    return _then(
      _$Condition_EqImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        value: null == value
            ? _value.value
            : value // ignore: cast_nullable_to_non_nullable
                  as SqlValue,
      ),
    );
  }

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $SqlValueCopyWith<$Res> get value {
    return $SqlValueCopyWith<$Res>(_value.value, (value) {
      return _then(_value.copyWith(value: value));
    });
  }
}

/// @nodoc

class _$Condition_EqImpl extends Condition_Eq {
  const _$Condition_EqImpl({required this.column, required this.value})
    : super._();

  @override
  final String column;
  @override
  final SqlValue value;

  @override
  String toString() {
    return 'Condition.eq(column: $column, value: $value)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Condition_EqImpl &&
            (identical(other.column, column) || other.column == column) &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column, value);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Condition_EqImplCopyWith<_$Condition_EqImpl> get copyWith =>
      __$$Condition_EqImplCopyWithImpl<_$Condition_EqImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) {
    return eq(column, value);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) {
    return eq?.call(column, value);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) {
    if (eq != null) {
      return eq(column, value);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) {
    return eq(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) {
    return eq?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (eq != null) {
      return eq(this);
    }
    return orElse();
  }
}

abstract class Condition_Eq extends Condition {
  const factory Condition_Eq({
    required final String column,
    required final SqlValue value,
  }) = _$Condition_EqImpl;
  const Condition_Eq._() : super._();

  String get column;
  SqlValue get value;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Condition_EqImplCopyWith<_$Condition_EqImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Condition_NotEqImplCopyWith<$Res> {
  factory _$$Condition_NotEqImplCopyWith(
    _$Condition_NotEqImpl value,
    $Res Function(_$Condition_NotEqImpl) then,
  ) = __$$Condition_NotEqImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column, SqlValue value});

  $SqlValueCopyWith<$Res> get value;
}

/// @nodoc
class __$$Condition_NotEqImplCopyWithImpl<$Res>
    extends _$ConditionCopyWithImpl<$Res, _$Condition_NotEqImpl>
    implements _$$Condition_NotEqImplCopyWith<$Res> {
  __$$Condition_NotEqImplCopyWithImpl(
    _$Condition_NotEqImpl _value,
    $Res Function(_$Condition_NotEqImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? value = null}) {
    return _then(
      _$Condition_NotEqImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        value: null == value
            ? _value.value
            : value // ignore: cast_nullable_to_non_nullable
                  as SqlValue,
      ),
    );
  }

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $SqlValueCopyWith<$Res> get value {
    return $SqlValueCopyWith<$Res>(_value.value, (value) {
      return _then(_value.copyWith(value: value));
    });
  }
}

/// @nodoc

class _$Condition_NotEqImpl extends Condition_NotEq {
  const _$Condition_NotEqImpl({required this.column, required this.value})
    : super._();

  @override
  final String column;
  @override
  final SqlValue value;

  @override
  String toString() {
    return 'Condition.notEq(column: $column, value: $value)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Condition_NotEqImpl &&
            (identical(other.column, column) || other.column == column) &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column, value);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Condition_NotEqImplCopyWith<_$Condition_NotEqImpl> get copyWith =>
      __$$Condition_NotEqImplCopyWithImpl<_$Condition_NotEqImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) {
    return notEq(column, value);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) {
    return notEq?.call(column, value);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) {
    if (notEq != null) {
      return notEq(column, value);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) {
    return notEq(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) {
    return notEq?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (notEq != null) {
      return notEq(this);
    }
    return orElse();
  }
}

abstract class Condition_NotEq extends Condition {
  const factory Condition_NotEq({
    required final String column,
    required final SqlValue value,
  }) = _$Condition_NotEqImpl;
  const Condition_NotEq._() : super._();

  String get column;
  SqlValue get value;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Condition_NotEqImplCopyWith<_$Condition_NotEqImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Condition_GtImplCopyWith<$Res> {
  factory _$$Condition_GtImplCopyWith(
    _$Condition_GtImpl value,
    $Res Function(_$Condition_GtImpl) then,
  ) = __$$Condition_GtImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column, SqlValue value});

  $SqlValueCopyWith<$Res> get value;
}

/// @nodoc
class __$$Condition_GtImplCopyWithImpl<$Res>
    extends _$ConditionCopyWithImpl<$Res, _$Condition_GtImpl>
    implements _$$Condition_GtImplCopyWith<$Res> {
  __$$Condition_GtImplCopyWithImpl(
    _$Condition_GtImpl _value,
    $Res Function(_$Condition_GtImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? value = null}) {
    return _then(
      _$Condition_GtImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        value: null == value
            ? _value.value
            : value // ignore: cast_nullable_to_non_nullable
                  as SqlValue,
      ),
    );
  }

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $SqlValueCopyWith<$Res> get value {
    return $SqlValueCopyWith<$Res>(_value.value, (value) {
      return _then(_value.copyWith(value: value));
    });
  }
}

/// @nodoc

class _$Condition_GtImpl extends Condition_Gt {
  const _$Condition_GtImpl({required this.column, required this.value})
    : super._();

  @override
  final String column;
  @override
  final SqlValue value;

  @override
  String toString() {
    return 'Condition.gt(column: $column, value: $value)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Condition_GtImpl &&
            (identical(other.column, column) || other.column == column) &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column, value);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Condition_GtImplCopyWith<_$Condition_GtImpl> get copyWith =>
      __$$Condition_GtImplCopyWithImpl<_$Condition_GtImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) {
    return gt(column, value);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) {
    return gt?.call(column, value);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) {
    if (gt != null) {
      return gt(column, value);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) {
    return gt(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) {
    return gt?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (gt != null) {
      return gt(this);
    }
    return orElse();
  }
}

abstract class Condition_Gt extends Condition {
  const factory Condition_Gt({
    required final String column,
    required final SqlValue value,
  }) = _$Condition_GtImpl;
  const Condition_Gt._() : super._();

  String get column;
  SqlValue get value;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Condition_GtImplCopyWith<_$Condition_GtImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Condition_GteImplCopyWith<$Res> {
  factory _$$Condition_GteImplCopyWith(
    _$Condition_GteImpl value,
    $Res Function(_$Condition_GteImpl) then,
  ) = __$$Condition_GteImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column, SqlValue value});

  $SqlValueCopyWith<$Res> get value;
}

/// @nodoc
class __$$Condition_GteImplCopyWithImpl<$Res>
    extends _$ConditionCopyWithImpl<$Res, _$Condition_GteImpl>
    implements _$$Condition_GteImplCopyWith<$Res> {
  __$$Condition_GteImplCopyWithImpl(
    _$Condition_GteImpl _value,
    $Res Function(_$Condition_GteImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? value = null}) {
    return _then(
      _$Condition_GteImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        value: null == value
            ? _value.value
            : value // ignore: cast_nullable_to_non_nullable
                  as SqlValue,
      ),
    );
  }

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $SqlValueCopyWith<$Res> get value {
    return $SqlValueCopyWith<$Res>(_value.value, (value) {
      return _then(_value.copyWith(value: value));
    });
  }
}

/// @nodoc

class _$Condition_GteImpl extends Condition_Gte {
  const _$Condition_GteImpl({required this.column, required this.value})
    : super._();

  @override
  final String column;
  @override
  final SqlValue value;

  @override
  String toString() {
    return 'Condition.gte(column: $column, value: $value)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Condition_GteImpl &&
            (identical(other.column, column) || other.column == column) &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column, value);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Condition_GteImplCopyWith<_$Condition_GteImpl> get copyWith =>
      __$$Condition_GteImplCopyWithImpl<_$Condition_GteImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) {
    return gte(column, value);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) {
    return gte?.call(column, value);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) {
    if (gte != null) {
      return gte(column, value);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) {
    return gte(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) {
    return gte?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (gte != null) {
      return gte(this);
    }
    return orElse();
  }
}

abstract class Condition_Gte extends Condition {
  const factory Condition_Gte({
    required final String column,
    required final SqlValue value,
  }) = _$Condition_GteImpl;
  const Condition_Gte._() : super._();

  String get column;
  SqlValue get value;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Condition_GteImplCopyWith<_$Condition_GteImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Condition_LtImplCopyWith<$Res> {
  factory _$$Condition_LtImplCopyWith(
    _$Condition_LtImpl value,
    $Res Function(_$Condition_LtImpl) then,
  ) = __$$Condition_LtImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column, SqlValue value});

  $SqlValueCopyWith<$Res> get value;
}

/// @nodoc
class __$$Condition_LtImplCopyWithImpl<$Res>
    extends _$ConditionCopyWithImpl<$Res, _$Condition_LtImpl>
    implements _$$Condition_LtImplCopyWith<$Res> {
  __$$Condition_LtImplCopyWithImpl(
    _$Condition_LtImpl _value,
    $Res Function(_$Condition_LtImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? value = null}) {
    return _then(
      _$Condition_LtImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        value: null == value
            ? _value.value
            : value // ignore: cast_nullable_to_non_nullable
                  as SqlValue,
      ),
    );
  }

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $SqlValueCopyWith<$Res> get value {
    return $SqlValueCopyWith<$Res>(_value.value, (value) {
      return _then(_value.copyWith(value: value));
    });
  }
}

/// @nodoc

class _$Condition_LtImpl extends Condition_Lt {
  const _$Condition_LtImpl({required this.column, required this.value})
    : super._();

  @override
  final String column;
  @override
  final SqlValue value;

  @override
  String toString() {
    return 'Condition.lt(column: $column, value: $value)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Condition_LtImpl &&
            (identical(other.column, column) || other.column == column) &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column, value);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Condition_LtImplCopyWith<_$Condition_LtImpl> get copyWith =>
      __$$Condition_LtImplCopyWithImpl<_$Condition_LtImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) {
    return lt(column, value);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) {
    return lt?.call(column, value);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) {
    if (lt != null) {
      return lt(column, value);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) {
    return lt(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) {
    return lt?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (lt != null) {
      return lt(this);
    }
    return orElse();
  }
}

abstract class Condition_Lt extends Condition {
  const factory Condition_Lt({
    required final String column,
    required final SqlValue value,
  }) = _$Condition_LtImpl;
  const Condition_Lt._() : super._();

  String get column;
  SqlValue get value;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Condition_LtImplCopyWith<_$Condition_LtImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Condition_LteImplCopyWith<$Res> {
  factory _$$Condition_LteImplCopyWith(
    _$Condition_LteImpl value,
    $Res Function(_$Condition_LteImpl) then,
  ) = __$$Condition_LteImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column, SqlValue value});

  $SqlValueCopyWith<$Res> get value;
}

/// @nodoc
class __$$Condition_LteImplCopyWithImpl<$Res>
    extends _$ConditionCopyWithImpl<$Res, _$Condition_LteImpl>
    implements _$$Condition_LteImplCopyWith<$Res> {
  __$$Condition_LteImplCopyWithImpl(
    _$Condition_LteImpl _value,
    $Res Function(_$Condition_LteImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? value = null}) {
    return _then(
      _$Condition_LteImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        value: null == value
            ? _value.value
            : value // ignore: cast_nullable_to_non_nullable
                  as SqlValue,
      ),
    );
  }

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $SqlValueCopyWith<$Res> get value {
    return $SqlValueCopyWith<$Res>(_value.value, (value) {
      return _then(_value.copyWith(value: value));
    });
  }
}

/// @nodoc

class _$Condition_LteImpl extends Condition_Lte {
  const _$Condition_LteImpl({required this.column, required this.value})
    : super._();

  @override
  final String column;
  @override
  final SqlValue value;

  @override
  String toString() {
    return 'Condition.lte(column: $column, value: $value)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Condition_LteImpl &&
            (identical(other.column, column) || other.column == column) &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column, value);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Condition_LteImplCopyWith<_$Condition_LteImpl> get copyWith =>
      __$$Condition_LteImplCopyWithImpl<_$Condition_LteImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) {
    return lte(column, value);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) {
    return lte?.call(column, value);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) {
    if (lte != null) {
      return lte(column, value);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) {
    return lte(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) {
    return lte?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (lte != null) {
      return lte(this);
    }
    return orElse();
  }
}

abstract class Condition_Lte extends Condition {
  const factory Condition_Lte({
    required final String column,
    required final SqlValue value,
  }) = _$Condition_LteImpl;
  const Condition_Lte._() : super._();

  String get column;
  SqlValue get value;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Condition_LteImplCopyWith<_$Condition_LteImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Condition_LikeImplCopyWith<$Res> {
  factory _$$Condition_LikeImplCopyWith(
    _$Condition_LikeImpl value,
    $Res Function(_$Condition_LikeImpl) then,
  ) = __$$Condition_LikeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column, String pattern});
}

/// @nodoc
class __$$Condition_LikeImplCopyWithImpl<$Res>
    extends _$ConditionCopyWithImpl<$Res, _$Condition_LikeImpl>
    implements _$$Condition_LikeImplCopyWith<$Res> {
  __$$Condition_LikeImplCopyWithImpl(
    _$Condition_LikeImpl _value,
    $Res Function(_$Condition_LikeImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? pattern = null}) {
    return _then(
      _$Condition_LikeImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        pattern: null == pattern
            ? _value.pattern
            : pattern // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$Condition_LikeImpl extends Condition_Like {
  const _$Condition_LikeImpl({required this.column, required this.pattern})
    : super._();

  @override
  final String column;
  @override
  final String pattern;

  @override
  String toString() {
    return 'Condition.like(column: $column, pattern: $pattern)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Condition_LikeImpl &&
            (identical(other.column, column) || other.column == column) &&
            (identical(other.pattern, pattern) || other.pattern == pattern));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column, pattern);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Condition_LikeImplCopyWith<_$Condition_LikeImpl> get copyWith =>
      __$$Condition_LikeImplCopyWithImpl<_$Condition_LikeImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) {
    return like(column, pattern);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) {
    return like?.call(column, pattern);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) {
    if (like != null) {
      return like(column, pattern);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) {
    return like(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) {
    return like?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (like != null) {
      return like(this);
    }
    return orElse();
  }
}

abstract class Condition_Like extends Condition {
  const factory Condition_Like({
    required final String column,
    required final String pattern,
  }) = _$Condition_LikeImpl;
  const Condition_Like._() : super._();

  String get column;
  String get pattern;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Condition_LikeImplCopyWith<_$Condition_LikeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Condition_BetweenImplCopyWith<$Res> {
  factory _$$Condition_BetweenImplCopyWith(
    _$Condition_BetweenImpl value,
    $Res Function(_$Condition_BetweenImpl) then,
  ) = __$$Condition_BetweenImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column, SqlValue low, SqlValue high});

  $SqlValueCopyWith<$Res> get low;
  $SqlValueCopyWith<$Res> get high;
}

/// @nodoc
class __$$Condition_BetweenImplCopyWithImpl<$Res>
    extends _$ConditionCopyWithImpl<$Res, _$Condition_BetweenImpl>
    implements _$$Condition_BetweenImplCopyWith<$Res> {
  __$$Condition_BetweenImplCopyWithImpl(
    _$Condition_BetweenImpl _value,
    $Res Function(_$Condition_BetweenImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? low = null, Object? high = null}) {
    return _then(
      _$Condition_BetweenImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        low: null == low
            ? _value.low
            : low // ignore: cast_nullable_to_non_nullable
                  as SqlValue,
        high: null == high
            ? _value.high
            : high // ignore: cast_nullable_to_non_nullable
                  as SqlValue,
      ),
    );
  }

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $SqlValueCopyWith<$Res> get low {
    return $SqlValueCopyWith<$Res>(_value.low, (value) {
      return _then(_value.copyWith(low: value));
    });
  }

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $SqlValueCopyWith<$Res> get high {
    return $SqlValueCopyWith<$Res>(_value.high, (value) {
      return _then(_value.copyWith(high: value));
    });
  }
}

/// @nodoc

class _$Condition_BetweenImpl extends Condition_Between {
  const _$Condition_BetweenImpl({
    required this.column,
    required this.low,
    required this.high,
  }) : super._();

  @override
  final String column;
  @override
  final SqlValue low;
  @override
  final SqlValue high;

  @override
  String toString() {
    return 'Condition.between(column: $column, low: $low, high: $high)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Condition_BetweenImpl &&
            (identical(other.column, column) || other.column == column) &&
            (identical(other.low, low) || other.low == low) &&
            (identical(other.high, high) || other.high == high));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column, low, high);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Condition_BetweenImplCopyWith<_$Condition_BetweenImpl> get copyWith =>
      __$$Condition_BetweenImplCopyWithImpl<_$Condition_BetweenImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) {
    return between(column, low, high);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) {
    return between?.call(column, low, high);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) {
    if (between != null) {
      return between(column, low, high);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) {
    return between(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) {
    return between?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (between != null) {
      return between(this);
    }
    return orElse();
  }
}

abstract class Condition_Between extends Condition {
  const factory Condition_Between({
    required final String column,
    required final SqlValue low,
    required final SqlValue high,
  }) = _$Condition_BetweenImpl;
  const Condition_Between._() : super._();

  String get column;
  SqlValue get low;
  SqlValue get high;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Condition_BetweenImplCopyWith<_$Condition_BetweenImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Condition_InImplCopyWith<$Res> {
  factory _$$Condition_InImplCopyWith(
    _$Condition_InImpl value,
    $Res Function(_$Condition_InImpl) then,
  ) = __$$Condition_InImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column, List<SqlValue> values});
}

/// @nodoc
class __$$Condition_InImplCopyWithImpl<$Res>
    extends _$ConditionCopyWithImpl<$Res, _$Condition_InImpl>
    implements _$$Condition_InImplCopyWith<$Res> {
  __$$Condition_InImplCopyWithImpl(
    _$Condition_InImpl _value,
    $Res Function(_$Condition_InImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? values = null}) {
    return _then(
      _$Condition_InImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        values: null == values
            ? _value._values
            : values // ignore: cast_nullable_to_non_nullable
                  as List<SqlValue>,
      ),
    );
  }
}

/// @nodoc

class _$Condition_InImpl extends Condition_In {
  const _$Condition_InImpl({
    required this.column,
    required final List<SqlValue> values,
  }) : _values = values,
       super._();

  @override
  final String column;
  final List<SqlValue> _values;
  @override
  List<SqlValue> get values {
    if (_values is EqualUnmodifiableListView) return _values;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_values);
  }

  @override
  String toString() {
    return 'Condition.in_(column: $column, values: $values)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Condition_InImpl &&
            (identical(other.column, column) || other.column == column) &&
            const DeepCollectionEquality().equals(other._values, _values));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    column,
    const DeepCollectionEquality().hash(_values),
  );

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Condition_InImplCopyWith<_$Condition_InImpl> get copyWith =>
      __$$Condition_InImplCopyWithImpl<_$Condition_InImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) {
    return in_(column, values);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) {
    return in_?.call(column, values);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) {
    if (in_ != null) {
      return in_(column, values);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) {
    return in_(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) {
    return in_?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (in_ != null) {
      return in_(this);
    }
    return orElse();
  }
}

abstract class Condition_In extends Condition {
  const factory Condition_In({
    required final String column,
    required final List<SqlValue> values,
  }) = _$Condition_InImpl;
  const Condition_In._() : super._();

  String get column;
  List<SqlValue> get values;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Condition_InImplCopyWith<_$Condition_InImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Condition_NotInImplCopyWith<$Res> {
  factory _$$Condition_NotInImplCopyWith(
    _$Condition_NotInImpl value,
    $Res Function(_$Condition_NotInImpl) then,
  ) = __$$Condition_NotInImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column, List<SqlValue> values});
}

/// @nodoc
class __$$Condition_NotInImplCopyWithImpl<$Res>
    extends _$ConditionCopyWithImpl<$Res, _$Condition_NotInImpl>
    implements _$$Condition_NotInImplCopyWith<$Res> {
  __$$Condition_NotInImplCopyWithImpl(
    _$Condition_NotInImpl _value,
    $Res Function(_$Condition_NotInImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null, Object? values = null}) {
    return _then(
      _$Condition_NotInImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
        values: null == values
            ? _value._values
            : values // ignore: cast_nullable_to_non_nullable
                  as List<SqlValue>,
      ),
    );
  }
}

/// @nodoc

class _$Condition_NotInImpl extends Condition_NotIn {
  const _$Condition_NotInImpl({
    required this.column,
    required final List<SqlValue> values,
  }) : _values = values,
       super._();

  @override
  final String column;
  final List<SqlValue> _values;
  @override
  List<SqlValue> get values {
    if (_values is EqualUnmodifiableListView) return _values;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_values);
  }

  @override
  String toString() {
    return 'Condition.notIn(column: $column, values: $values)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Condition_NotInImpl &&
            (identical(other.column, column) || other.column == column) &&
            const DeepCollectionEquality().equals(other._values, _values));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    column,
    const DeepCollectionEquality().hash(_values),
  );

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Condition_NotInImplCopyWith<_$Condition_NotInImpl> get copyWith =>
      __$$Condition_NotInImplCopyWithImpl<_$Condition_NotInImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) {
    return notIn(column, values);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) {
    return notIn?.call(column, values);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) {
    if (notIn != null) {
      return notIn(column, values);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) {
    return notIn(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) {
    return notIn?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (notIn != null) {
      return notIn(this);
    }
    return orElse();
  }
}

abstract class Condition_NotIn extends Condition {
  const factory Condition_NotIn({
    required final String column,
    required final List<SqlValue> values,
  }) = _$Condition_NotInImpl;
  const Condition_NotIn._() : super._();

  String get column;
  List<SqlValue> get values;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Condition_NotInImplCopyWith<_$Condition_NotInImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Condition_IsNullImplCopyWith<$Res> {
  factory _$$Condition_IsNullImplCopyWith(
    _$Condition_IsNullImpl value,
    $Res Function(_$Condition_IsNullImpl) then,
  ) = __$$Condition_IsNullImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column});
}

/// @nodoc
class __$$Condition_IsNullImplCopyWithImpl<$Res>
    extends _$ConditionCopyWithImpl<$Res, _$Condition_IsNullImpl>
    implements _$$Condition_IsNullImplCopyWith<$Res> {
  __$$Condition_IsNullImplCopyWithImpl(
    _$Condition_IsNullImpl _value,
    $Res Function(_$Condition_IsNullImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null}) {
    return _then(
      _$Condition_IsNullImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$Condition_IsNullImpl extends Condition_IsNull {
  const _$Condition_IsNullImpl({required this.column}) : super._();

  @override
  final String column;

  @override
  String toString() {
    return 'Condition.isNull(column: $column)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Condition_IsNullImpl &&
            (identical(other.column, column) || other.column == column));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Condition_IsNullImplCopyWith<_$Condition_IsNullImpl> get copyWith =>
      __$$Condition_IsNullImplCopyWithImpl<_$Condition_IsNullImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) {
    return isNull(column);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) {
    return isNull?.call(column);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) {
    if (isNull != null) {
      return isNull(column);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) {
    return isNull(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) {
    return isNull?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (isNull != null) {
      return isNull(this);
    }
    return orElse();
  }
}

abstract class Condition_IsNull extends Condition {
  const factory Condition_IsNull({required final String column}) =
      _$Condition_IsNullImpl;
  const Condition_IsNull._() : super._();

  String get column;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Condition_IsNullImplCopyWith<_$Condition_IsNullImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Condition_IsNotNullImplCopyWith<$Res> {
  factory _$$Condition_IsNotNullImplCopyWith(
    _$Condition_IsNotNullImpl value,
    $Res Function(_$Condition_IsNotNullImpl) then,
  ) = __$$Condition_IsNotNullImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column});
}

/// @nodoc
class __$$Condition_IsNotNullImplCopyWithImpl<$Res>
    extends _$ConditionCopyWithImpl<$Res, _$Condition_IsNotNullImpl>
    implements _$$Condition_IsNotNullImplCopyWith<$Res> {
  __$$Condition_IsNotNullImplCopyWithImpl(
    _$Condition_IsNotNullImpl _value,
    $Res Function(_$Condition_IsNotNullImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null}) {
    return _then(
      _$Condition_IsNotNullImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$Condition_IsNotNullImpl extends Condition_IsNotNull {
  const _$Condition_IsNotNullImpl({required this.column}) : super._();

  @override
  final String column;

  @override
  String toString() {
    return 'Condition.isNotNull(column: $column)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Condition_IsNotNullImpl &&
            (identical(other.column, column) || other.column == column));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Condition_IsNotNullImplCopyWith<_$Condition_IsNotNullImpl> get copyWith =>
      __$$Condition_IsNotNullImplCopyWithImpl<_$Condition_IsNotNullImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) {
    return isNotNull(column);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) {
    return isNotNull?.call(column);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) {
    if (isNotNull != null) {
      return isNotNull(column);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) {
    return isNotNull(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) {
    return isNotNull?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (isNotNull != null) {
      return isNotNull(this);
    }
    return orElse();
  }
}

abstract class Condition_IsNotNull extends Condition {
  const factory Condition_IsNotNull({required final String column}) =
      _$Condition_IsNotNullImpl;
  const Condition_IsNotNull._() : super._();

  String get column;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Condition_IsNotNullImplCopyWith<_$Condition_IsNotNullImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Condition_RawImplCopyWith<$Res> {
  factory _$$Condition_RawImplCopyWith(
    _$Condition_RawImpl value,
    $Res Function(_$Condition_RawImpl) then,
  ) = __$$Condition_RawImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String sql, List<SqlValue> params});
}

/// @nodoc
class __$$Condition_RawImplCopyWithImpl<$Res>
    extends _$ConditionCopyWithImpl<$Res, _$Condition_RawImpl>
    implements _$$Condition_RawImplCopyWith<$Res> {
  __$$Condition_RawImplCopyWithImpl(
    _$Condition_RawImpl _value,
    $Res Function(_$Condition_RawImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? sql = null, Object? params = null}) {
    return _then(
      _$Condition_RawImpl(
        sql: null == sql
            ? _value.sql
            : sql // ignore: cast_nullable_to_non_nullable
                  as String,
        params: null == params
            ? _value._params
            : params // ignore: cast_nullable_to_non_nullable
                  as List<SqlValue>,
      ),
    );
  }
}

/// @nodoc

class _$Condition_RawImpl extends Condition_Raw {
  const _$Condition_RawImpl({
    required this.sql,
    required final List<SqlValue> params,
  }) : _params = params,
       super._();

  @override
  final String sql;
  final List<SqlValue> _params;
  @override
  List<SqlValue> get params {
    if (_params is EqualUnmodifiableListView) return _params;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_params);
  }

  @override
  String toString() {
    return 'Condition.raw(sql: $sql, params: $params)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Condition_RawImpl &&
            (identical(other.sql, sql) || other.sql == sql) &&
            const DeepCollectionEquality().equals(other._params, _params));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    sql,
    const DeepCollectionEquality().hash(_params),
  );

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Condition_RawImplCopyWith<_$Condition_RawImpl> get copyWith =>
      __$$Condition_RawImplCopyWithImpl<_$Condition_RawImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column, SqlValue value) eq,
    required TResult Function(String column, SqlValue value) notEq,
    required TResult Function(String column, SqlValue value) gt,
    required TResult Function(String column, SqlValue value) gte,
    required TResult Function(String column, SqlValue value) lt,
    required TResult Function(String column, SqlValue value) lte,
    required TResult Function(String column, String pattern) like,
    required TResult Function(String column, SqlValue low, SqlValue high)
    between,
    required TResult Function(String column, List<SqlValue> values) in_,
    required TResult Function(String column, List<SqlValue> values) notIn,
    required TResult Function(String column) isNull,
    required TResult Function(String column) isNotNull,
    required TResult Function(String sql, List<SqlValue> params) raw,
  }) {
    return raw(sql, params);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column, SqlValue value)? eq,
    TResult? Function(String column, SqlValue value)? notEq,
    TResult? Function(String column, SqlValue value)? gt,
    TResult? Function(String column, SqlValue value)? gte,
    TResult? Function(String column, SqlValue value)? lt,
    TResult? Function(String column, SqlValue value)? lte,
    TResult? Function(String column, String pattern)? like,
    TResult? Function(String column, SqlValue low, SqlValue high)? between,
    TResult? Function(String column, List<SqlValue> values)? in_,
    TResult? Function(String column, List<SqlValue> values)? notIn,
    TResult? Function(String column)? isNull,
    TResult? Function(String column)? isNotNull,
    TResult? Function(String sql, List<SqlValue> params)? raw,
  }) {
    return raw?.call(sql, params);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column, SqlValue value)? eq,
    TResult Function(String column, SqlValue value)? notEq,
    TResult Function(String column, SqlValue value)? gt,
    TResult Function(String column, SqlValue value)? gte,
    TResult Function(String column, SqlValue value)? lt,
    TResult Function(String column, SqlValue value)? lte,
    TResult Function(String column, String pattern)? like,
    TResult Function(String column, SqlValue low, SqlValue high)? between,
    TResult Function(String column, List<SqlValue> values)? in_,
    TResult Function(String column, List<SqlValue> values)? notIn,
    TResult Function(String column)? isNull,
    TResult Function(String column)? isNotNull,
    TResult Function(String sql, List<SqlValue> params)? raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(sql, params);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Condition_Eq value) eq,
    required TResult Function(Condition_NotEq value) notEq,
    required TResult Function(Condition_Gt value) gt,
    required TResult Function(Condition_Gte value) gte,
    required TResult Function(Condition_Lt value) lt,
    required TResult Function(Condition_Lte value) lte,
    required TResult Function(Condition_Like value) like,
    required TResult Function(Condition_Between value) between,
    required TResult Function(Condition_In value) in_,
    required TResult Function(Condition_NotIn value) notIn,
    required TResult Function(Condition_IsNull value) isNull,
    required TResult Function(Condition_IsNotNull value) isNotNull,
    required TResult Function(Condition_Raw value) raw,
  }) {
    return raw(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Condition_Eq value)? eq,
    TResult? Function(Condition_NotEq value)? notEq,
    TResult? Function(Condition_Gt value)? gt,
    TResult? Function(Condition_Gte value)? gte,
    TResult? Function(Condition_Lt value)? lt,
    TResult? Function(Condition_Lte value)? lte,
    TResult? Function(Condition_Like value)? like,
    TResult? Function(Condition_Between value)? between,
    TResult? Function(Condition_In value)? in_,
    TResult? Function(Condition_NotIn value)? notIn,
    TResult? Function(Condition_IsNull value)? isNull,
    TResult? Function(Condition_IsNotNull value)? isNotNull,
    TResult? Function(Condition_Raw value)? raw,
  }) {
    return raw?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Condition_Eq value)? eq,
    TResult Function(Condition_NotEq value)? notEq,
    TResult Function(Condition_Gt value)? gt,
    TResult Function(Condition_Gte value)? gte,
    TResult Function(Condition_Lt value)? lt,
    TResult Function(Condition_Lte value)? lte,
    TResult Function(Condition_Like value)? like,
    TResult Function(Condition_Between value)? between,
    TResult Function(Condition_In value)? in_,
    TResult Function(Condition_NotIn value)? notIn,
    TResult Function(Condition_IsNull value)? isNull,
    TResult Function(Condition_IsNotNull value)? isNotNull,
    TResult Function(Condition_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(this);
    }
    return orElse();
  }
}

abstract class Condition_Raw extends Condition {
  const factory Condition_Raw({
    required final String sql,
    required final List<SqlValue> params,
  }) = _$Condition_RawImpl;
  const Condition_Raw._() : super._();

  String get sql;
  List<SqlValue> get params;

  /// Create a copy of Condition
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Condition_RawImplCopyWith<_$Condition_RawImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$JoinSpec {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, String on_) inner,
    required TResult Function(String table, String on_) left,
    required TResult Function(String expression) raw,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, String on_)? inner,
    TResult? Function(String table, String on_)? left,
    TResult? Function(String expression)? raw,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, String on_)? inner,
    TResult Function(String table, String on_)? left,
    TResult Function(String expression)? raw,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JoinSpec_Inner value) inner,
    required TResult Function(JoinSpec_Left value) left,
    required TResult Function(JoinSpec_Raw value) raw,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JoinSpec_Inner value)? inner,
    TResult? Function(JoinSpec_Left value)? left,
    TResult? Function(JoinSpec_Raw value)? raw,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JoinSpec_Inner value)? inner,
    TResult Function(JoinSpec_Left value)? left,
    TResult Function(JoinSpec_Raw value)? raw,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $JoinSpecCopyWith<$Res> {
  factory $JoinSpecCopyWith(JoinSpec value, $Res Function(JoinSpec) then) =
      _$JoinSpecCopyWithImpl<$Res, JoinSpec>;
}

/// @nodoc
class _$JoinSpecCopyWithImpl<$Res, $Val extends JoinSpec>
    implements $JoinSpecCopyWith<$Res> {
  _$JoinSpecCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of JoinSpec
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$JoinSpec_InnerImplCopyWith<$Res> {
  factory _$$JoinSpec_InnerImplCopyWith(
    _$JoinSpec_InnerImpl value,
    $Res Function(_$JoinSpec_InnerImpl) then,
  ) = __$$JoinSpec_InnerImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String table, String on_});
}

/// @nodoc
class __$$JoinSpec_InnerImplCopyWithImpl<$Res>
    extends _$JoinSpecCopyWithImpl<$Res, _$JoinSpec_InnerImpl>
    implements _$$JoinSpec_InnerImplCopyWith<$Res> {
  __$$JoinSpec_InnerImplCopyWithImpl(
    _$JoinSpec_InnerImpl _value,
    $Res Function(_$JoinSpec_InnerImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of JoinSpec
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? table = null, Object? on_ = null}) {
    return _then(
      _$JoinSpec_InnerImpl(
        table: null == table
            ? _value.table
            : table // ignore: cast_nullable_to_non_nullable
                  as String,
        on_: null == on_
            ? _value.on_
            : on_ // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$JoinSpec_InnerImpl extends JoinSpec_Inner {
  const _$JoinSpec_InnerImpl({required this.table, required this.on_})
    : super._();

  @override
  final String table;
  @override
  final String on_;

  @override
  String toString() {
    return 'JoinSpec.inner(table: $table, on_: $on_)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$JoinSpec_InnerImpl &&
            (identical(other.table, table) || other.table == table) &&
            (identical(other.on_, on_) || other.on_ == on_));
  }

  @override
  int get hashCode => Object.hash(runtimeType, table, on_);

  /// Create a copy of JoinSpec
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$JoinSpec_InnerImplCopyWith<_$JoinSpec_InnerImpl> get copyWith =>
      __$$JoinSpec_InnerImplCopyWithImpl<_$JoinSpec_InnerImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, String on_) inner,
    required TResult Function(String table, String on_) left,
    required TResult Function(String expression) raw,
  }) {
    return inner(table, on_);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, String on_)? inner,
    TResult? Function(String table, String on_)? left,
    TResult? Function(String expression)? raw,
  }) {
    return inner?.call(table, on_);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, String on_)? inner,
    TResult Function(String table, String on_)? left,
    TResult Function(String expression)? raw,
    required TResult orElse(),
  }) {
    if (inner != null) {
      return inner(table, on_);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JoinSpec_Inner value) inner,
    required TResult Function(JoinSpec_Left value) left,
    required TResult Function(JoinSpec_Raw value) raw,
  }) {
    return inner(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JoinSpec_Inner value)? inner,
    TResult? Function(JoinSpec_Left value)? left,
    TResult? Function(JoinSpec_Raw value)? raw,
  }) {
    return inner?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JoinSpec_Inner value)? inner,
    TResult Function(JoinSpec_Left value)? left,
    TResult Function(JoinSpec_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (inner != null) {
      return inner(this);
    }
    return orElse();
  }
}

abstract class JoinSpec_Inner extends JoinSpec {
  const factory JoinSpec_Inner({
    required final String table,
    required final String on_,
  }) = _$JoinSpec_InnerImpl;
  const JoinSpec_Inner._() : super._();

  String get table;
  String get on_;

  /// Create a copy of JoinSpec
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$JoinSpec_InnerImplCopyWith<_$JoinSpec_InnerImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$JoinSpec_LeftImplCopyWith<$Res> {
  factory _$$JoinSpec_LeftImplCopyWith(
    _$JoinSpec_LeftImpl value,
    $Res Function(_$JoinSpec_LeftImpl) then,
  ) = __$$JoinSpec_LeftImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String table, String on_});
}

/// @nodoc
class __$$JoinSpec_LeftImplCopyWithImpl<$Res>
    extends _$JoinSpecCopyWithImpl<$Res, _$JoinSpec_LeftImpl>
    implements _$$JoinSpec_LeftImplCopyWith<$Res> {
  __$$JoinSpec_LeftImplCopyWithImpl(
    _$JoinSpec_LeftImpl _value,
    $Res Function(_$JoinSpec_LeftImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of JoinSpec
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? table = null, Object? on_ = null}) {
    return _then(
      _$JoinSpec_LeftImpl(
        table: null == table
            ? _value.table
            : table // ignore: cast_nullable_to_non_nullable
                  as String,
        on_: null == on_
            ? _value.on_
            : on_ // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$JoinSpec_LeftImpl extends JoinSpec_Left {
  const _$JoinSpec_LeftImpl({required this.table, required this.on_})
    : super._();

  @override
  final String table;
  @override
  final String on_;

  @override
  String toString() {
    return 'JoinSpec.left(table: $table, on_: $on_)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$JoinSpec_LeftImpl &&
            (identical(other.table, table) || other.table == table) &&
            (identical(other.on_, on_) || other.on_ == on_));
  }

  @override
  int get hashCode => Object.hash(runtimeType, table, on_);

  /// Create a copy of JoinSpec
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$JoinSpec_LeftImplCopyWith<_$JoinSpec_LeftImpl> get copyWith =>
      __$$JoinSpec_LeftImplCopyWithImpl<_$JoinSpec_LeftImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, String on_) inner,
    required TResult Function(String table, String on_) left,
    required TResult Function(String expression) raw,
  }) {
    return left(table, on_);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, String on_)? inner,
    TResult? Function(String table, String on_)? left,
    TResult? Function(String expression)? raw,
  }) {
    return left?.call(table, on_);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, String on_)? inner,
    TResult Function(String table, String on_)? left,
    TResult Function(String expression)? raw,
    required TResult orElse(),
  }) {
    if (left != null) {
      return left(table, on_);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JoinSpec_Inner value) inner,
    required TResult Function(JoinSpec_Left value) left,
    required TResult Function(JoinSpec_Raw value) raw,
  }) {
    return left(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JoinSpec_Inner value)? inner,
    TResult? Function(JoinSpec_Left value)? left,
    TResult? Function(JoinSpec_Raw value)? raw,
  }) {
    return left?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JoinSpec_Inner value)? inner,
    TResult Function(JoinSpec_Left value)? left,
    TResult Function(JoinSpec_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (left != null) {
      return left(this);
    }
    return orElse();
  }
}

abstract class JoinSpec_Left extends JoinSpec {
  const factory JoinSpec_Left({
    required final String table,
    required final String on_,
  }) = _$JoinSpec_LeftImpl;
  const JoinSpec_Left._() : super._();

  String get table;
  String get on_;

  /// Create a copy of JoinSpec
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$JoinSpec_LeftImplCopyWith<_$JoinSpec_LeftImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$JoinSpec_RawImplCopyWith<$Res> {
  factory _$$JoinSpec_RawImplCopyWith(
    _$JoinSpec_RawImpl value,
    $Res Function(_$JoinSpec_RawImpl) then,
  ) = __$$JoinSpec_RawImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String expression});
}

/// @nodoc
class __$$JoinSpec_RawImplCopyWithImpl<$Res>
    extends _$JoinSpecCopyWithImpl<$Res, _$JoinSpec_RawImpl>
    implements _$$JoinSpec_RawImplCopyWith<$Res> {
  __$$JoinSpec_RawImplCopyWithImpl(
    _$JoinSpec_RawImpl _value,
    $Res Function(_$JoinSpec_RawImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of JoinSpec
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? expression = null}) {
    return _then(
      _$JoinSpec_RawImpl(
        expression: null == expression
            ? _value.expression
            : expression // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$JoinSpec_RawImpl extends JoinSpec_Raw {
  const _$JoinSpec_RawImpl({required this.expression}) : super._();

  @override
  final String expression;

  @override
  String toString() {
    return 'JoinSpec.raw(expression: $expression)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$JoinSpec_RawImpl &&
            (identical(other.expression, expression) ||
                other.expression == expression));
  }

  @override
  int get hashCode => Object.hash(runtimeType, expression);

  /// Create a copy of JoinSpec
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$JoinSpec_RawImplCopyWith<_$JoinSpec_RawImpl> get copyWith =>
      __$$JoinSpec_RawImplCopyWithImpl<_$JoinSpec_RawImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, String on_) inner,
    required TResult Function(String table, String on_) left,
    required TResult Function(String expression) raw,
  }) {
    return raw(expression);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, String on_)? inner,
    TResult? Function(String table, String on_)? left,
    TResult? Function(String expression)? raw,
  }) {
    return raw?.call(expression);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, String on_)? inner,
    TResult Function(String table, String on_)? left,
    TResult Function(String expression)? raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(expression);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JoinSpec_Inner value) inner,
    required TResult Function(JoinSpec_Left value) left,
    required TResult Function(JoinSpec_Raw value) raw,
  }) {
    return raw(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JoinSpec_Inner value)? inner,
    TResult? Function(JoinSpec_Left value)? left,
    TResult? Function(JoinSpec_Raw value)? raw,
  }) {
    return raw?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JoinSpec_Inner value)? inner,
    TResult Function(JoinSpec_Left value)? left,
    TResult Function(JoinSpec_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(this);
    }
    return orElse();
  }
}

abstract class JoinSpec_Raw extends JoinSpec {
  const factory JoinSpec_Raw({required final String expression}) =
      _$JoinSpec_RawImpl;
  const JoinSpec_Raw._() : super._();

  String get expression;

  /// Create a copy of JoinSpec
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$JoinSpec_RawImplCopyWith<_$JoinSpec_RawImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Op {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, Map<String, SqlValue> data) insert,
    required TResult Function(
      String table,
      String id,
      Map<String, SqlValue> data,
    )
    update,
    required TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )
    upsert,
    required TResult Function(String table, String id) delete,
    required TResult Function(String table, String id) hardDelete,
    required TResult Function(String sql, List<SqlValue> params) raw,
    required TResult Function(String name) savepoint,
    required TResult Function(String name) releaseSavepoint,
    required TResult Function(String name) rollbackToSavepoint,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, Map<String, SqlValue> data)? insert,
    TResult? Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult? Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult? Function(String table, String id)? delete,
    TResult? Function(String table, String id)? hardDelete,
    TResult? Function(String sql, List<SqlValue> params)? raw,
    TResult? Function(String name)? savepoint,
    TResult? Function(String name)? releaseSavepoint,
    TResult? Function(String name)? rollbackToSavepoint,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, Map<String, SqlValue> data)? insert,
    TResult Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult Function(String table, String id)? delete,
    TResult Function(String table, String id)? hardDelete,
    TResult Function(String sql, List<SqlValue> params)? raw,
    TResult Function(String name)? savepoint,
    TResult Function(String name)? releaseSavepoint,
    TResult Function(String name)? rollbackToSavepoint,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Op_Insert value) insert,
    required TResult Function(Op_Update value) update,
    required TResult Function(Op_Upsert value) upsert,
    required TResult Function(Op_Delete value) delete,
    required TResult Function(Op_HardDelete value) hardDelete,
    required TResult Function(Op_Raw value) raw,
    required TResult Function(Op_Savepoint value) savepoint,
    required TResult Function(Op_ReleaseSavepoint value) releaseSavepoint,
    required TResult Function(Op_RollbackToSavepoint value) rollbackToSavepoint,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Op_Insert value)? insert,
    TResult? Function(Op_Update value)? update,
    TResult? Function(Op_Upsert value)? upsert,
    TResult? Function(Op_Delete value)? delete,
    TResult? Function(Op_HardDelete value)? hardDelete,
    TResult? Function(Op_Raw value)? raw,
    TResult? Function(Op_Savepoint value)? savepoint,
    TResult? Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult? Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Op_Insert value)? insert,
    TResult Function(Op_Update value)? update,
    TResult Function(Op_Upsert value)? upsert,
    TResult Function(Op_Delete value)? delete,
    TResult Function(Op_HardDelete value)? hardDelete,
    TResult Function(Op_Raw value)? raw,
    TResult Function(Op_Savepoint value)? savepoint,
    TResult Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OpCopyWith<$Res> {
  factory $OpCopyWith(Op value, $Res Function(Op) then) =
      _$OpCopyWithImpl<$Res, Op>;
}

/// @nodoc
class _$OpCopyWithImpl<$Res, $Val extends Op> implements $OpCopyWith<$Res> {
  _$OpCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$Op_InsertImplCopyWith<$Res> {
  factory _$$Op_InsertImplCopyWith(
    _$Op_InsertImpl value,
    $Res Function(_$Op_InsertImpl) then,
  ) = __$$Op_InsertImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String table, Map<String, SqlValue> data});
}

/// @nodoc
class __$$Op_InsertImplCopyWithImpl<$Res>
    extends _$OpCopyWithImpl<$Res, _$Op_InsertImpl>
    implements _$$Op_InsertImplCopyWith<$Res> {
  __$$Op_InsertImplCopyWithImpl(
    _$Op_InsertImpl _value,
    $Res Function(_$Op_InsertImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? table = null, Object? data = null}) {
    return _then(
      _$Op_InsertImpl(
        table: null == table
            ? _value.table
            : table // ignore: cast_nullable_to_non_nullable
                  as String,
        data: null == data
            ? _value._data
            : data // ignore: cast_nullable_to_non_nullable
                  as Map<String, SqlValue>,
      ),
    );
  }
}

/// @nodoc

class _$Op_InsertImpl extends Op_Insert {
  const _$Op_InsertImpl({
    required this.table,
    required final Map<String, SqlValue> data,
  }) : _data = data,
       super._();

  @override
  final String table;
  final Map<String, SqlValue> _data;
  @override
  Map<String, SqlValue> get data {
    if (_data is EqualUnmodifiableMapView) return _data;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(_data);
  }

  @override
  String toString() {
    return 'Op.insert(table: $table, data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Op_InsertImpl &&
            (identical(other.table, table) || other.table == table) &&
            const DeepCollectionEquality().equals(other._data, _data));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    table,
    const DeepCollectionEquality().hash(_data),
  );

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Op_InsertImplCopyWith<_$Op_InsertImpl> get copyWith =>
      __$$Op_InsertImplCopyWithImpl<_$Op_InsertImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, Map<String, SqlValue> data) insert,
    required TResult Function(
      String table,
      String id,
      Map<String, SqlValue> data,
    )
    update,
    required TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )
    upsert,
    required TResult Function(String table, String id) delete,
    required TResult Function(String table, String id) hardDelete,
    required TResult Function(String sql, List<SqlValue> params) raw,
    required TResult Function(String name) savepoint,
    required TResult Function(String name) releaseSavepoint,
    required TResult Function(String name) rollbackToSavepoint,
  }) {
    return insert(table, data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, Map<String, SqlValue> data)? insert,
    TResult? Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult? Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult? Function(String table, String id)? delete,
    TResult? Function(String table, String id)? hardDelete,
    TResult? Function(String sql, List<SqlValue> params)? raw,
    TResult? Function(String name)? savepoint,
    TResult? Function(String name)? releaseSavepoint,
    TResult? Function(String name)? rollbackToSavepoint,
  }) {
    return insert?.call(table, data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, Map<String, SqlValue> data)? insert,
    TResult Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult Function(String table, String id)? delete,
    TResult Function(String table, String id)? hardDelete,
    TResult Function(String sql, List<SqlValue> params)? raw,
    TResult Function(String name)? savepoint,
    TResult Function(String name)? releaseSavepoint,
    TResult Function(String name)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (insert != null) {
      return insert(table, data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Op_Insert value) insert,
    required TResult Function(Op_Update value) update,
    required TResult Function(Op_Upsert value) upsert,
    required TResult Function(Op_Delete value) delete,
    required TResult Function(Op_HardDelete value) hardDelete,
    required TResult Function(Op_Raw value) raw,
    required TResult Function(Op_Savepoint value) savepoint,
    required TResult Function(Op_ReleaseSavepoint value) releaseSavepoint,
    required TResult Function(Op_RollbackToSavepoint value) rollbackToSavepoint,
  }) {
    return insert(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Op_Insert value)? insert,
    TResult? Function(Op_Update value)? update,
    TResult? Function(Op_Upsert value)? upsert,
    TResult? Function(Op_Delete value)? delete,
    TResult? Function(Op_HardDelete value)? hardDelete,
    TResult? Function(Op_Raw value)? raw,
    TResult? Function(Op_Savepoint value)? savepoint,
    TResult? Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult? Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
  }) {
    return insert?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Op_Insert value)? insert,
    TResult Function(Op_Update value)? update,
    TResult Function(Op_Upsert value)? upsert,
    TResult Function(Op_Delete value)? delete,
    TResult Function(Op_HardDelete value)? hardDelete,
    TResult Function(Op_Raw value)? raw,
    TResult Function(Op_Savepoint value)? savepoint,
    TResult Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (insert != null) {
      return insert(this);
    }
    return orElse();
  }
}

abstract class Op_Insert extends Op {
  const factory Op_Insert({
    required final String table,
    required final Map<String, SqlValue> data,
  }) = _$Op_InsertImpl;
  const Op_Insert._() : super._();

  String get table;
  Map<String, SqlValue> get data;

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Op_InsertImplCopyWith<_$Op_InsertImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Op_UpdateImplCopyWith<$Res> {
  factory _$$Op_UpdateImplCopyWith(
    _$Op_UpdateImpl value,
    $Res Function(_$Op_UpdateImpl) then,
  ) = __$$Op_UpdateImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String table, String id, Map<String, SqlValue> data});
}

/// @nodoc
class __$$Op_UpdateImplCopyWithImpl<$Res>
    extends _$OpCopyWithImpl<$Res, _$Op_UpdateImpl>
    implements _$$Op_UpdateImplCopyWith<$Res> {
  __$$Op_UpdateImplCopyWithImpl(
    _$Op_UpdateImpl _value,
    $Res Function(_$Op_UpdateImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? table = null, Object? id = null, Object? data = null}) {
    return _then(
      _$Op_UpdateImpl(
        table: null == table
            ? _value.table
            : table // ignore: cast_nullable_to_non_nullable
                  as String,
        id: null == id
            ? _value.id
            : id // ignore: cast_nullable_to_non_nullable
                  as String,
        data: null == data
            ? _value._data
            : data // ignore: cast_nullable_to_non_nullable
                  as Map<String, SqlValue>,
      ),
    );
  }
}

/// @nodoc

class _$Op_UpdateImpl extends Op_Update {
  const _$Op_UpdateImpl({
    required this.table,
    required this.id,
    required final Map<String, SqlValue> data,
  }) : _data = data,
       super._();

  @override
  final String table;
  @override
  final String id;
  final Map<String, SqlValue> _data;
  @override
  Map<String, SqlValue> get data {
    if (_data is EqualUnmodifiableMapView) return _data;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(_data);
  }

  @override
  String toString() {
    return 'Op.update(table: $table, id: $id, data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Op_UpdateImpl &&
            (identical(other.table, table) || other.table == table) &&
            (identical(other.id, id) || other.id == id) &&
            const DeepCollectionEquality().equals(other._data, _data));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    table,
    id,
    const DeepCollectionEquality().hash(_data),
  );

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Op_UpdateImplCopyWith<_$Op_UpdateImpl> get copyWith =>
      __$$Op_UpdateImplCopyWithImpl<_$Op_UpdateImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, Map<String, SqlValue> data) insert,
    required TResult Function(
      String table,
      String id,
      Map<String, SqlValue> data,
    )
    update,
    required TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )
    upsert,
    required TResult Function(String table, String id) delete,
    required TResult Function(String table, String id) hardDelete,
    required TResult Function(String sql, List<SqlValue> params) raw,
    required TResult Function(String name) savepoint,
    required TResult Function(String name) releaseSavepoint,
    required TResult Function(String name) rollbackToSavepoint,
  }) {
    return update(table, id, data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, Map<String, SqlValue> data)? insert,
    TResult? Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult? Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult? Function(String table, String id)? delete,
    TResult? Function(String table, String id)? hardDelete,
    TResult? Function(String sql, List<SqlValue> params)? raw,
    TResult? Function(String name)? savepoint,
    TResult? Function(String name)? releaseSavepoint,
    TResult? Function(String name)? rollbackToSavepoint,
  }) {
    return update?.call(table, id, data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, Map<String, SqlValue> data)? insert,
    TResult Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult Function(String table, String id)? delete,
    TResult Function(String table, String id)? hardDelete,
    TResult Function(String sql, List<SqlValue> params)? raw,
    TResult Function(String name)? savepoint,
    TResult Function(String name)? releaseSavepoint,
    TResult Function(String name)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (update != null) {
      return update(table, id, data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Op_Insert value) insert,
    required TResult Function(Op_Update value) update,
    required TResult Function(Op_Upsert value) upsert,
    required TResult Function(Op_Delete value) delete,
    required TResult Function(Op_HardDelete value) hardDelete,
    required TResult Function(Op_Raw value) raw,
    required TResult Function(Op_Savepoint value) savepoint,
    required TResult Function(Op_ReleaseSavepoint value) releaseSavepoint,
    required TResult Function(Op_RollbackToSavepoint value) rollbackToSavepoint,
  }) {
    return update(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Op_Insert value)? insert,
    TResult? Function(Op_Update value)? update,
    TResult? Function(Op_Upsert value)? upsert,
    TResult? Function(Op_Delete value)? delete,
    TResult? Function(Op_HardDelete value)? hardDelete,
    TResult? Function(Op_Raw value)? raw,
    TResult? Function(Op_Savepoint value)? savepoint,
    TResult? Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult? Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
  }) {
    return update?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Op_Insert value)? insert,
    TResult Function(Op_Update value)? update,
    TResult Function(Op_Upsert value)? upsert,
    TResult Function(Op_Delete value)? delete,
    TResult Function(Op_HardDelete value)? hardDelete,
    TResult Function(Op_Raw value)? raw,
    TResult Function(Op_Savepoint value)? savepoint,
    TResult Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (update != null) {
      return update(this);
    }
    return orElse();
  }
}

abstract class Op_Update extends Op {
  const factory Op_Update({
    required final String table,
    required final String id,
    required final Map<String, SqlValue> data,
  }) = _$Op_UpdateImpl;
  const Op_Update._() : super._();

  String get table;
  String get id;
  Map<String, SqlValue> get data;

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Op_UpdateImplCopyWith<_$Op_UpdateImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Op_UpsertImplCopyWith<$Res> {
  factory _$$Op_UpsertImplCopyWith(
    _$Op_UpsertImpl value,
    $Res Function(_$Op_UpsertImpl) then,
  ) = __$$Op_UpsertImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String table, Map<String, SqlValue> data, String conflictColumn});
}

/// @nodoc
class __$$Op_UpsertImplCopyWithImpl<$Res>
    extends _$OpCopyWithImpl<$Res, _$Op_UpsertImpl>
    implements _$$Op_UpsertImplCopyWith<$Res> {
  __$$Op_UpsertImplCopyWithImpl(
    _$Op_UpsertImpl _value,
    $Res Function(_$Op_UpsertImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? table = null,
    Object? data = null,
    Object? conflictColumn = null,
  }) {
    return _then(
      _$Op_UpsertImpl(
        table: null == table
            ? _value.table
            : table // ignore: cast_nullable_to_non_nullable
                  as String,
        data: null == data
            ? _value._data
            : data // ignore: cast_nullable_to_non_nullable
                  as Map<String, SqlValue>,
        conflictColumn: null == conflictColumn
            ? _value.conflictColumn
            : conflictColumn // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$Op_UpsertImpl extends Op_Upsert {
  const _$Op_UpsertImpl({
    required this.table,
    required final Map<String, SqlValue> data,
    required this.conflictColumn,
  }) : _data = data,
       super._();

  @override
  final String table;
  final Map<String, SqlValue> _data;
  @override
  Map<String, SqlValue> get data {
    if (_data is EqualUnmodifiableMapView) return _data;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableMapView(_data);
  }

  @override
  final String conflictColumn;

  @override
  String toString() {
    return 'Op.upsert(table: $table, data: $data, conflictColumn: $conflictColumn)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Op_UpsertImpl &&
            (identical(other.table, table) || other.table == table) &&
            const DeepCollectionEquality().equals(other._data, _data) &&
            (identical(other.conflictColumn, conflictColumn) ||
                other.conflictColumn == conflictColumn));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    table,
    const DeepCollectionEquality().hash(_data),
    conflictColumn,
  );

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Op_UpsertImplCopyWith<_$Op_UpsertImpl> get copyWith =>
      __$$Op_UpsertImplCopyWithImpl<_$Op_UpsertImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, Map<String, SqlValue> data) insert,
    required TResult Function(
      String table,
      String id,
      Map<String, SqlValue> data,
    )
    update,
    required TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )
    upsert,
    required TResult Function(String table, String id) delete,
    required TResult Function(String table, String id) hardDelete,
    required TResult Function(String sql, List<SqlValue> params) raw,
    required TResult Function(String name) savepoint,
    required TResult Function(String name) releaseSavepoint,
    required TResult Function(String name) rollbackToSavepoint,
  }) {
    return upsert(table, data, conflictColumn);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, Map<String, SqlValue> data)? insert,
    TResult? Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult? Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult? Function(String table, String id)? delete,
    TResult? Function(String table, String id)? hardDelete,
    TResult? Function(String sql, List<SqlValue> params)? raw,
    TResult? Function(String name)? savepoint,
    TResult? Function(String name)? releaseSavepoint,
    TResult? Function(String name)? rollbackToSavepoint,
  }) {
    return upsert?.call(table, data, conflictColumn);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, Map<String, SqlValue> data)? insert,
    TResult Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult Function(String table, String id)? delete,
    TResult Function(String table, String id)? hardDelete,
    TResult Function(String sql, List<SqlValue> params)? raw,
    TResult Function(String name)? savepoint,
    TResult Function(String name)? releaseSavepoint,
    TResult Function(String name)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (upsert != null) {
      return upsert(table, data, conflictColumn);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Op_Insert value) insert,
    required TResult Function(Op_Update value) update,
    required TResult Function(Op_Upsert value) upsert,
    required TResult Function(Op_Delete value) delete,
    required TResult Function(Op_HardDelete value) hardDelete,
    required TResult Function(Op_Raw value) raw,
    required TResult Function(Op_Savepoint value) savepoint,
    required TResult Function(Op_ReleaseSavepoint value) releaseSavepoint,
    required TResult Function(Op_RollbackToSavepoint value) rollbackToSavepoint,
  }) {
    return upsert(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Op_Insert value)? insert,
    TResult? Function(Op_Update value)? update,
    TResult? Function(Op_Upsert value)? upsert,
    TResult? Function(Op_Delete value)? delete,
    TResult? Function(Op_HardDelete value)? hardDelete,
    TResult? Function(Op_Raw value)? raw,
    TResult? Function(Op_Savepoint value)? savepoint,
    TResult? Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult? Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
  }) {
    return upsert?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Op_Insert value)? insert,
    TResult Function(Op_Update value)? update,
    TResult Function(Op_Upsert value)? upsert,
    TResult Function(Op_Delete value)? delete,
    TResult Function(Op_HardDelete value)? hardDelete,
    TResult Function(Op_Raw value)? raw,
    TResult Function(Op_Savepoint value)? savepoint,
    TResult Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (upsert != null) {
      return upsert(this);
    }
    return orElse();
  }
}

abstract class Op_Upsert extends Op {
  const factory Op_Upsert({
    required final String table,
    required final Map<String, SqlValue> data,
    required final String conflictColumn,
  }) = _$Op_UpsertImpl;
  const Op_Upsert._() : super._();

  String get table;
  Map<String, SqlValue> get data;
  String get conflictColumn;

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Op_UpsertImplCopyWith<_$Op_UpsertImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Op_DeleteImplCopyWith<$Res> {
  factory _$$Op_DeleteImplCopyWith(
    _$Op_DeleteImpl value,
    $Res Function(_$Op_DeleteImpl) then,
  ) = __$$Op_DeleteImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String table, String id});
}

/// @nodoc
class __$$Op_DeleteImplCopyWithImpl<$Res>
    extends _$OpCopyWithImpl<$Res, _$Op_DeleteImpl>
    implements _$$Op_DeleteImplCopyWith<$Res> {
  __$$Op_DeleteImplCopyWithImpl(
    _$Op_DeleteImpl _value,
    $Res Function(_$Op_DeleteImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? table = null, Object? id = null}) {
    return _then(
      _$Op_DeleteImpl(
        table: null == table
            ? _value.table
            : table // ignore: cast_nullable_to_non_nullable
                  as String,
        id: null == id
            ? _value.id
            : id // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$Op_DeleteImpl extends Op_Delete {
  const _$Op_DeleteImpl({required this.table, required this.id}) : super._();

  @override
  final String table;
  @override
  final String id;

  @override
  String toString() {
    return 'Op.delete(table: $table, id: $id)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Op_DeleteImpl &&
            (identical(other.table, table) || other.table == table) &&
            (identical(other.id, id) || other.id == id));
  }

  @override
  int get hashCode => Object.hash(runtimeType, table, id);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Op_DeleteImplCopyWith<_$Op_DeleteImpl> get copyWith =>
      __$$Op_DeleteImplCopyWithImpl<_$Op_DeleteImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, Map<String, SqlValue> data) insert,
    required TResult Function(
      String table,
      String id,
      Map<String, SqlValue> data,
    )
    update,
    required TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )
    upsert,
    required TResult Function(String table, String id) delete,
    required TResult Function(String table, String id) hardDelete,
    required TResult Function(String sql, List<SqlValue> params) raw,
    required TResult Function(String name) savepoint,
    required TResult Function(String name) releaseSavepoint,
    required TResult Function(String name) rollbackToSavepoint,
  }) {
    return delete(table, id);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, Map<String, SqlValue> data)? insert,
    TResult? Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult? Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult? Function(String table, String id)? delete,
    TResult? Function(String table, String id)? hardDelete,
    TResult? Function(String sql, List<SqlValue> params)? raw,
    TResult? Function(String name)? savepoint,
    TResult? Function(String name)? releaseSavepoint,
    TResult? Function(String name)? rollbackToSavepoint,
  }) {
    return delete?.call(table, id);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, Map<String, SqlValue> data)? insert,
    TResult Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult Function(String table, String id)? delete,
    TResult Function(String table, String id)? hardDelete,
    TResult Function(String sql, List<SqlValue> params)? raw,
    TResult Function(String name)? savepoint,
    TResult Function(String name)? releaseSavepoint,
    TResult Function(String name)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (delete != null) {
      return delete(table, id);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Op_Insert value) insert,
    required TResult Function(Op_Update value) update,
    required TResult Function(Op_Upsert value) upsert,
    required TResult Function(Op_Delete value) delete,
    required TResult Function(Op_HardDelete value) hardDelete,
    required TResult Function(Op_Raw value) raw,
    required TResult Function(Op_Savepoint value) savepoint,
    required TResult Function(Op_ReleaseSavepoint value) releaseSavepoint,
    required TResult Function(Op_RollbackToSavepoint value) rollbackToSavepoint,
  }) {
    return delete(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Op_Insert value)? insert,
    TResult? Function(Op_Update value)? update,
    TResult? Function(Op_Upsert value)? upsert,
    TResult? Function(Op_Delete value)? delete,
    TResult? Function(Op_HardDelete value)? hardDelete,
    TResult? Function(Op_Raw value)? raw,
    TResult? Function(Op_Savepoint value)? savepoint,
    TResult? Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult? Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
  }) {
    return delete?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Op_Insert value)? insert,
    TResult Function(Op_Update value)? update,
    TResult Function(Op_Upsert value)? upsert,
    TResult Function(Op_Delete value)? delete,
    TResult Function(Op_HardDelete value)? hardDelete,
    TResult Function(Op_Raw value)? raw,
    TResult Function(Op_Savepoint value)? savepoint,
    TResult Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (delete != null) {
      return delete(this);
    }
    return orElse();
  }
}

abstract class Op_Delete extends Op {
  const factory Op_Delete({
    required final String table,
    required final String id,
  }) = _$Op_DeleteImpl;
  const Op_Delete._() : super._();

  String get table;
  String get id;

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Op_DeleteImplCopyWith<_$Op_DeleteImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Op_HardDeleteImplCopyWith<$Res> {
  factory _$$Op_HardDeleteImplCopyWith(
    _$Op_HardDeleteImpl value,
    $Res Function(_$Op_HardDeleteImpl) then,
  ) = __$$Op_HardDeleteImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String table, String id});
}

/// @nodoc
class __$$Op_HardDeleteImplCopyWithImpl<$Res>
    extends _$OpCopyWithImpl<$Res, _$Op_HardDeleteImpl>
    implements _$$Op_HardDeleteImplCopyWith<$Res> {
  __$$Op_HardDeleteImplCopyWithImpl(
    _$Op_HardDeleteImpl _value,
    $Res Function(_$Op_HardDeleteImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? table = null, Object? id = null}) {
    return _then(
      _$Op_HardDeleteImpl(
        table: null == table
            ? _value.table
            : table // ignore: cast_nullable_to_non_nullable
                  as String,
        id: null == id
            ? _value.id
            : id // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$Op_HardDeleteImpl extends Op_HardDelete {
  const _$Op_HardDeleteImpl({required this.table, required this.id})
    : super._();

  @override
  final String table;
  @override
  final String id;

  @override
  String toString() {
    return 'Op.hardDelete(table: $table, id: $id)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Op_HardDeleteImpl &&
            (identical(other.table, table) || other.table == table) &&
            (identical(other.id, id) || other.id == id));
  }

  @override
  int get hashCode => Object.hash(runtimeType, table, id);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Op_HardDeleteImplCopyWith<_$Op_HardDeleteImpl> get copyWith =>
      __$$Op_HardDeleteImplCopyWithImpl<_$Op_HardDeleteImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, Map<String, SqlValue> data) insert,
    required TResult Function(
      String table,
      String id,
      Map<String, SqlValue> data,
    )
    update,
    required TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )
    upsert,
    required TResult Function(String table, String id) delete,
    required TResult Function(String table, String id) hardDelete,
    required TResult Function(String sql, List<SqlValue> params) raw,
    required TResult Function(String name) savepoint,
    required TResult Function(String name) releaseSavepoint,
    required TResult Function(String name) rollbackToSavepoint,
  }) {
    return hardDelete(table, id);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, Map<String, SqlValue> data)? insert,
    TResult? Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult? Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult? Function(String table, String id)? delete,
    TResult? Function(String table, String id)? hardDelete,
    TResult? Function(String sql, List<SqlValue> params)? raw,
    TResult? Function(String name)? savepoint,
    TResult? Function(String name)? releaseSavepoint,
    TResult? Function(String name)? rollbackToSavepoint,
  }) {
    return hardDelete?.call(table, id);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, Map<String, SqlValue> data)? insert,
    TResult Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult Function(String table, String id)? delete,
    TResult Function(String table, String id)? hardDelete,
    TResult Function(String sql, List<SqlValue> params)? raw,
    TResult Function(String name)? savepoint,
    TResult Function(String name)? releaseSavepoint,
    TResult Function(String name)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (hardDelete != null) {
      return hardDelete(table, id);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Op_Insert value) insert,
    required TResult Function(Op_Update value) update,
    required TResult Function(Op_Upsert value) upsert,
    required TResult Function(Op_Delete value) delete,
    required TResult Function(Op_HardDelete value) hardDelete,
    required TResult Function(Op_Raw value) raw,
    required TResult Function(Op_Savepoint value) savepoint,
    required TResult Function(Op_ReleaseSavepoint value) releaseSavepoint,
    required TResult Function(Op_RollbackToSavepoint value) rollbackToSavepoint,
  }) {
    return hardDelete(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Op_Insert value)? insert,
    TResult? Function(Op_Update value)? update,
    TResult? Function(Op_Upsert value)? upsert,
    TResult? Function(Op_Delete value)? delete,
    TResult? Function(Op_HardDelete value)? hardDelete,
    TResult? Function(Op_Raw value)? raw,
    TResult? Function(Op_Savepoint value)? savepoint,
    TResult? Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult? Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
  }) {
    return hardDelete?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Op_Insert value)? insert,
    TResult Function(Op_Update value)? update,
    TResult Function(Op_Upsert value)? upsert,
    TResult Function(Op_Delete value)? delete,
    TResult Function(Op_HardDelete value)? hardDelete,
    TResult Function(Op_Raw value)? raw,
    TResult Function(Op_Savepoint value)? savepoint,
    TResult Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (hardDelete != null) {
      return hardDelete(this);
    }
    return orElse();
  }
}

abstract class Op_HardDelete extends Op {
  const factory Op_HardDelete({
    required final String table,
    required final String id,
  }) = _$Op_HardDeleteImpl;
  const Op_HardDelete._() : super._();

  String get table;
  String get id;

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Op_HardDeleteImplCopyWith<_$Op_HardDeleteImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Op_RawImplCopyWith<$Res> {
  factory _$$Op_RawImplCopyWith(
    _$Op_RawImpl value,
    $Res Function(_$Op_RawImpl) then,
  ) = __$$Op_RawImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String sql, List<SqlValue> params});
}

/// @nodoc
class __$$Op_RawImplCopyWithImpl<$Res>
    extends _$OpCopyWithImpl<$Res, _$Op_RawImpl>
    implements _$$Op_RawImplCopyWith<$Res> {
  __$$Op_RawImplCopyWithImpl(
    _$Op_RawImpl _value,
    $Res Function(_$Op_RawImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? sql = null, Object? params = null}) {
    return _then(
      _$Op_RawImpl(
        sql: null == sql
            ? _value.sql
            : sql // ignore: cast_nullable_to_non_nullable
                  as String,
        params: null == params
            ? _value._params
            : params // ignore: cast_nullable_to_non_nullable
                  as List<SqlValue>,
      ),
    );
  }
}

/// @nodoc

class _$Op_RawImpl extends Op_Raw {
  const _$Op_RawImpl({required this.sql, required final List<SqlValue> params})
    : _params = params,
      super._();

  @override
  final String sql;
  final List<SqlValue> _params;
  @override
  List<SqlValue> get params {
    if (_params is EqualUnmodifiableListView) return _params;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_params);
  }

  @override
  String toString() {
    return 'Op.raw(sql: $sql, params: $params)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Op_RawImpl &&
            (identical(other.sql, sql) || other.sql == sql) &&
            const DeepCollectionEquality().equals(other._params, _params));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    sql,
    const DeepCollectionEquality().hash(_params),
  );

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Op_RawImplCopyWith<_$Op_RawImpl> get copyWith =>
      __$$Op_RawImplCopyWithImpl<_$Op_RawImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, Map<String, SqlValue> data) insert,
    required TResult Function(
      String table,
      String id,
      Map<String, SqlValue> data,
    )
    update,
    required TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )
    upsert,
    required TResult Function(String table, String id) delete,
    required TResult Function(String table, String id) hardDelete,
    required TResult Function(String sql, List<SqlValue> params) raw,
    required TResult Function(String name) savepoint,
    required TResult Function(String name) releaseSavepoint,
    required TResult Function(String name) rollbackToSavepoint,
  }) {
    return raw(sql, params);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, Map<String, SqlValue> data)? insert,
    TResult? Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult? Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult? Function(String table, String id)? delete,
    TResult? Function(String table, String id)? hardDelete,
    TResult? Function(String sql, List<SqlValue> params)? raw,
    TResult? Function(String name)? savepoint,
    TResult? Function(String name)? releaseSavepoint,
    TResult? Function(String name)? rollbackToSavepoint,
  }) {
    return raw?.call(sql, params);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, Map<String, SqlValue> data)? insert,
    TResult Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult Function(String table, String id)? delete,
    TResult Function(String table, String id)? hardDelete,
    TResult Function(String sql, List<SqlValue> params)? raw,
    TResult Function(String name)? savepoint,
    TResult Function(String name)? releaseSavepoint,
    TResult Function(String name)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(sql, params);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Op_Insert value) insert,
    required TResult Function(Op_Update value) update,
    required TResult Function(Op_Upsert value) upsert,
    required TResult Function(Op_Delete value) delete,
    required TResult Function(Op_HardDelete value) hardDelete,
    required TResult Function(Op_Raw value) raw,
    required TResult Function(Op_Savepoint value) savepoint,
    required TResult Function(Op_ReleaseSavepoint value) releaseSavepoint,
    required TResult Function(Op_RollbackToSavepoint value) rollbackToSavepoint,
  }) {
    return raw(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Op_Insert value)? insert,
    TResult? Function(Op_Update value)? update,
    TResult? Function(Op_Upsert value)? upsert,
    TResult? Function(Op_Delete value)? delete,
    TResult? Function(Op_HardDelete value)? hardDelete,
    TResult? Function(Op_Raw value)? raw,
    TResult? Function(Op_Savepoint value)? savepoint,
    TResult? Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult? Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
  }) {
    return raw?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Op_Insert value)? insert,
    TResult Function(Op_Update value)? update,
    TResult Function(Op_Upsert value)? upsert,
    TResult Function(Op_Delete value)? delete,
    TResult Function(Op_HardDelete value)? hardDelete,
    TResult Function(Op_Raw value)? raw,
    TResult Function(Op_Savepoint value)? savepoint,
    TResult Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(this);
    }
    return orElse();
  }
}

abstract class Op_Raw extends Op {
  const factory Op_Raw({
    required final String sql,
    required final List<SqlValue> params,
  }) = _$Op_RawImpl;
  const Op_Raw._() : super._();

  String get sql;
  List<SqlValue> get params;

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Op_RawImplCopyWith<_$Op_RawImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Op_SavepointImplCopyWith<$Res> {
  factory _$$Op_SavepointImplCopyWith(
    _$Op_SavepointImpl value,
    $Res Function(_$Op_SavepointImpl) then,
  ) = __$$Op_SavepointImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String name});
}

/// @nodoc
class __$$Op_SavepointImplCopyWithImpl<$Res>
    extends _$OpCopyWithImpl<$Res, _$Op_SavepointImpl>
    implements _$$Op_SavepointImplCopyWith<$Res> {
  __$$Op_SavepointImplCopyWithImpl(
    _$Op_SavepointImpl _value,
    $Res Function(_$Op_SavepointImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? name = null}) {
    return _then(
      _$Op_SavepointImpl(
        name: null == name
            ? _value.name
            : name // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$Op_SavepointImpl extends Op_Savepoint {
  const _$Op_SavepointImpl({required this.name}) : super._();

  @override
  final String name;

  @override
  String toString() {
    return 'Op.savepoint(name: $name)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Op_SavepointImpl &&
            (identical(other.name, name) || other.name == name));
  }

  @override
  int get hashCode => Object.hash(runtimeType, name);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Op_SavepointImplCopyWith<_$Op_SavepointImpl> get copyWith =>
      __$$Op_SavepointImplCopyWithImpl<_$Op_SavepointImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, Map<String, SqlValue> data) insert,
    required TResult Function(
      String table,
      String id,
      Map<String, SqlValue> data,
    )
    update,
    required TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )
    upsert,
    required TResult Function(String table, String id) delete,
    required TResult Function(String table, String id) hardDelete,
    required TResult Function(String sql, List<SqlValue> params) raw,
    required TResult Function(String name) savepoint,
    required TResult Function(String name) releaseSavepoint,
    required TResult Function(String name) rollbackToSavepoint,
  }) {
    return savepoint(name);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, Map<String, SqlValue> data)? insert,
    TResult? Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult? Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult? Function(String table, String id)? delete,
    TResult? Function(String table, String id)? hardDelete,
    TResult? Function(String sql, List<SqlValue> params)? raw,
    TResult? Function(String name)? savepoint,
    TResult? Function(String name)? releaseSavepoint,
    TResult? Function(String name)? rollbackToSavepoint,
  }) {
    return savepoint?.call(name);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, Map<String, SqlValue> data)? insert,
    TResult Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult Function(String table, String id)? delete,
    TResult Function(String table, String id)? hardDelete,
    TResult Function(String sql, List<SqlValue> params)? raw,
    TResult Function(String name)? savepoint,
    TResult Function(String name)? releaseSavepoint,
    TResult Function(String name)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (savepoint != null) {
      return savepoint(name);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Op_Insert value) insert,
    required TResult Function(Op_Update value) update,
    required TResult Function(Op_Upsert value) upsert,
    required TResult Function(Op_Delete value) delete,
    required TResult Function(Op_HardDelete value) hardDelete,
    required TResult Function(Op_Raw value) raw,
    required TResult Function(Op_Savepoint value) savepoint,
    required TResult Function(Op_ReleaseSavepoint value) releaseSavepoint,
    required TResult Function(Op_RollbackToSavepoint value) rollbackToSavepoint,
  }) {
    return savepoint(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Op_Insert value)? insert,
    TResult? Function(Op_Update value)? update,
    TResult? Function(Op_Upsert value)? upsert,
    TResult? Function(Op_Delete value)? delete,
    TResult? Function(Op_HardDelete value)? hardDelete,
    TResult? Function(Op_Raw value)? raw,
    TResult? Function(Op_Savepoint value)? savepoint,
    TResult? Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult? Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
  }) {
    return savepoint?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Op_Insert value)? insert,
    TResult Function(Op_Update value)? update,
    TResult Function(Op_Upsert value)? upsert,
    TResult Function(Op_Delete value)? delete,
    TResult Function(Op_HardDelete value)? hardDelete,
    TResult Function(Op_Raw value)? raw,
    TResult Function(Op_Savepoint value)? savepoint,
    TResult Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (savepoint != null) {
      return savepoint(this);
    }
    return orElse();
  }
}

abstract class Op_Savepoint extends Op {
  const factory Op_Savepoint({required final String name}) = _$Op_SavepointImpl;
  const Op_Savepoint._() : super._();

  String get name;

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Op_SavepointImplCopyWith<_$Op_SavepointImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Op_ReleaseSavepointImplCopyWith<$Res> {
  factory _$$Op_ReleaseSavepointImplCopyWith(
    _$Op_ReleaseSavepointImpl value,
    $Res Function(_$Op_ReleaseSavepointImpl) then,
  ) = __$$Op_ReleaseSavepointImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String name});
}

/// @nodoc
class __$$Op_ReleaseSavepointImplCopyWithImpl<$Res>
    extends _$OpCopyWithImpl<$Res, _$Op_ReleaseSavepointImpl>
    implements _$$Op_ReleaseSavepointImplCopyWith<$Res> {
  __$$Op_ReleaseSavepointImplCopyWithImpl(
    _$Op_ReleaseSavepointImpl _value,
    $Res Function(_$Op_ReleaseSavepointImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? name = null}) {
    return _then(
      _$Op_ReleaseSavepointImpl(
        name: null == name
            ? _value.name
            : name // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$Op_ReleaseSavepointImpl extends Op_ReleaseSavepoint {
  const _$Op_ReleaseSavepointImpl({required this.name}) : super._();

  @override
  final String name;

  @override
  String toString() {
    return 'Op.releaseSavepoint(name: $name)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Op_ReleaseSavepointImpl &&
            (identical(other.name, name) || other.name == name));
  }

  @override
  int get hashCode => Object.hash(runtimeType, name);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Op_ReleaseSavepointImplCopyWith<_$Op_ReleaseSavepointImpl> get copyWith =>
      __$$Op_ReleaseSavepointImplCopyWithImpl<_$Op_ReleaseSavepointImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, Map<String, SqlValue> data) insert,
    required TResult Function(
      String table,
      String id,
      Map<String, SqlValue> data,
    )
    update,
    required TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )
    upsert,
    required TResult Function(String table, String id) delete,
    required TResult Function(String table, String id) hardDelete,
    required TResult Function(String sql, List<SqlValue> params) raw,
    required TResult Function(String name) savepoint,
    required TResult Function(String name) releaseSavepoint,
    required TResult Function(String name) rollbackToSavepoint,
  }) {
    return releaseSavepoint(name);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, Map<String, SqlValue> data)? insert,
    TResult? Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult? Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult? Function(String table, String id)? delete,
    TResult? Function(String table, String id)? hardDelete,
    TResult? Function(String sql, List<SqlValue> params)? raw,
    TResult? Function(String name)? savepoint,
    TResult? Function(String name)? releaseSavepoint,
    TResult? Function(String name)? rollbackToSavepoint,
  }) {
    return releaseSavepoint?.call(name);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, Map<String, SqlValue> data)? insert,
    TResult Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult Function(String table, String id)? delete,
    TResult Function(String table, String id)? hardDelete,
    TResult Function(String sql, List<SqlValue> params)? raw,
    TResult Function(String name)? savepoint,
    TResult Function(String name)? releaseSavepoint,
    TResult Function(String name)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (releaseSavepoint != null) {
      return releaseSavepoint(name);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Op_Insert value) insert,
    required TResult Function(Op_Update value) update,
    required TResult Function(Op_Upsert value) upsert,
    required TResult Function(Op_Delete value) delete,
    required TResult Function(Op_HardDelete value) hardDelete,
    required TResult Function(Op_Raw value) raw,
    required TResult Function(Op_Savepoint value) savepoint,
    required TResult Function(Op_ReleaseSavepoint value) releaseSavepoint,
    required TResult Function(Op_RollbackToSavepoint value) rollbackToSavepoint,
  }) {
    return releaseSavepoint(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Op_Insert value)? insert,
    TResult? Function(Op_Update value)? update,
    TResult? Function(Op_Upsert value)? upsert,
    TResult? Function(Op_Delete value)? delete,
    TResult? Function(Op_HardDelete value)? hardDelete,
    TResult? Function(Op_Raw value)? raw,
    TResult? Function(Op_Savepoint value)? savepoint,
    TResult? Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult? Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
  }) {
    return releaseSavepoint?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Op_Insert value)? insert,
    TResult Function(Op_Update value)? update,
    TResult Function(Op_Upsert value)? upsert,
    TResult Function(Op_Delete value)? delete,
    TResult Function(Op_HardDelete value)? hardDelete,
    TResult Function(Op_Raw value)? raw,
    TResult Function(Op_Savepoint value)? savepoint,
    TResult Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (releaseSavepoint != null) {
      return releaseSavepoint(this);
    }
    return orElse();
  }
}

abstract class Op_ReleaseSavepoint extends Op {
  const factory Op_ReleaseSavepoint({required final String name}) =
      _$Op_ReleaseSavepointImpl;
  const Op_ReleaseSavepoint._() : super._();

  String get name;

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Op_ReleaseSavepointImplCopyWith<_$Op_ReleaseSavepointImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Op_RollbackToSavepointImplCopyWith<$Res> {
  factory _$$Op_RollbackToSavepointImplCopyWith(
    _$Op_RollbackToSavepointImpl value,
    $Res Function(_$Op_RollbackToSavepointImpl) then,
  ) = __$$Op_RollbackToSavepointImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String name});
}

/// @nodoc
class __$$Op_RollbackToSavepointImplCopyWithImpl<$Res>
    extends _$OpCopyWithImpl<$Res, _$Op_RollbackToSavepointImpl>
    implements _$$Op_RollbackToSavepointImplCopyWith<$Res> {
  __$$Op_RollbackToSavepointImplCopyWithImpl(
    _$Op_RollbackToSavepointImpl _value,
    $Res Function(_$Op_RollbackToSavepointImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? name = null}) {
    return _then(
      _$Op_RollbackToSavepointImpl(
        name: null == name
            ? _value.name
            : name // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$Op_RollbackToSavepointImpl extends Op_RollbackToSavepoint {
  const _$Op_RollbackToSavepointImpl({required this.name}) : super._();

  @override
  final String name;

  @override
  String toString() {
    return 'Op.rollbackToSavepoint(name: $name)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Op_RollbackToSavepointImpl &&
            (identical(other.name, name) || other.name == name));
  }

  @override
  int get hashCode => Object.hash(runtimeType, name);

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Op_RollbackToSavepointImplCopyWith<_$Op_RollbackToSavepointImpl>
  get copyWith =>
      __$$Op_RollbackToSavepointImplCopyWithImpl<_$Op_RollbackToSavepointImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String table, Map<String, SqlValue> data) insert,
    required TResult Function(
      String table,
      String id,
      Map<String, SqlValue> data,
    )
    update,
    required TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )
    upsert,
    required TResult Function(String table, String id) delete,
    required TResult Function(String table, String id) hardDelete,
    required TResult Function(String sql, List<SqlValue> params) raw,
    required TResult Function(String name) savepoint,
    required TResult Function(String name) releaseSavepoint,
    required TResult Function(String name) rollbackToSavepoint,
  }) {
    return rollbackToSavepoint(name);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String table, Map<String, SqlValue> data)? insert,
    TResult? Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult? Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult? Function(String table, String id)? delete,
    TResult? Function(String table, String id)? hardDelete,
    TResult? Function(String sql, List<SqlValue> params)? raw,
    TResult? Function(String name)? savepoint,
    TResult? Function(String name)? releaseSavepoint,
    TResult? Function(String name)? rollbackToSavepoint,
  }) {
    return rollbackToSavepoint?.call(name);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String table, Map<String, SqlValue> data)? insert,
    TResult Function(String table, String id, Map<String, SqlValue> data)?
    update,
    TResult Function(
      String table,
      Map<String, SqlValue> data,
      String conflictColumn,
    )?
    upsert,
    TResult Function(String table, String id)? delete,
    TResult Function(String table, String id)? hardDelete,
    TResult Function(String sql, List<SqlValue> params)? raw,
    TResult Function(String name)? savepoint,
    TResult Function(String name)? releaseSavepoint,
    TResult Function(String name)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (rollbackToSavepoint != null) {
      return rollbackToSavepoint(name);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Op_Insert value) insert,
    required TResult Function(Op_Update value) update,
    required TResult Function(Op_Upsert value) upsert,
    required TResult Function(Op_Delete value) delete,
    required TResult Function(Op_HardDelete value) hardDelete,
    required TResult Function(Op_Raw value) raw,
    required TResult Function(Op_Savepoint value) savepoint,
    required TResult Function(Op_ReleaseSavepoint value) releaseSavepoint,
    required TResult Function(Op_RollbackToSavepoint value) rollbackToSavepoint,
  }) {
    return rollbackToSavepoint(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Op_Insert value)? insert,
    TResult? Function(Op_Update value)? update,
    TResult? Function(Op_Upsert value)? upsert,
    TResult? Function(Op_Delete value)? delete,
    TResult? Function(Op_HardDelete value)? hardDelete,
    TResult? Function(Op_Raw value)? raw,
    TResult? Function(Op_Savepoint value)? savepoint,
    TResult? Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult? Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
  }) {
    return rollbackToSavepoint?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Op_Insert value)? insert,
    TResult Function(Op_Update value)? update,
    TResult Function(Op_Upsert value)? upsert,
    TResult Function(Op_Delete value)? delete,
    TResult Function(Op_HardDelete value)? hardDelete,
    TResult Function(Op_Raw value)? raw,
    TResult Function(Op_Savepoint value)? savepoint,
    TResult Function(Op_ReleaseSavepoint value)? releaseSavepoint,
    TResult Function(Op_RollbackToSavepoint value)? rollbackToSavepoint,
    required TResult orElse(),
  }) {
    if (rollbackToSavepoint != null) {
      return rollbackToSavepoint(this);
    }
    return orElse();
  }
}

abstract class Op_RollbackToSavepoint extends Op {
  const factory Op_RollbackToSavepoint({required final String name}) =
      _$Op_RollbackToSavepointImpl;
  const Op_RollbackToSavepoint._() : super._();

  String get name;

  /// Create a copy of Op
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Op_RollbackToSavepointImplCopyWith<_$Op_RollbackToSavepointImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$OrderBy {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column) asc,
    required TResult Function(String column) desc,
    required TResult Function(String expression) raw,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column)? asc,
    TResult? Function(String column)? desc,
    TResult? Function(String expression)? raw,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column)? asc,
    TResult Function(String column)? desc,
    TResult Function(String expression)? raw,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(OrderBy_Asc value) asc,
    required TResult Function(OrderBy_Desc value) desc,
    required TResult Function(OrderBy_Raw value) raw,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(OrderBy_Asc value)? asc,
    TResult? Function(OrderBy_Desc value)? desc,
    TResult? Function(OrderBy_Raw value)? raw,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(OrderBy_Asc value)? asc,
    TResult Function(OrderBy_Desc value)? desc,
    TResult Function(OrderBy_Raw value)? raw,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OrderByCopyWith<$Res> {
  factory $OrderByCopyWith(OrderBy value, $Res Function(OrderBy) then) =
      _$OrderByCopyWithImpl<$Res, OrderBy>;
}

/// @nodoc
class _$OrderByCopyWithImpl<$Res, $Val extends OrderBy>
    implements $OrderByCopyWith<$Res> {
  _$OrderByCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of OrderBy
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$OrderBy_AscImplCopyWith<$Res> {
  factory _$$OrderBy_AscImplCopyWith(
    _$OrderBy_AscImpl value,
    $Res Function(_$OrderBy_AscImpl) then,
  ) = __$$OrderBy_AscImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column});
}

/// @nodoc
class __$$OrderBy_AscImplCopyWithImpl<$Res>
    extends _$OrderByCopyWithImpl<$Res, _$OrderBy_AscImpl>
    implements _$$OrderBy_AscImplCopyWith<$Res> {
  __$$OrderBy_AscImplCopyWithImpl(
    _$OrderBy_AscImpl _value,
    $Res Function(_$OrderBy_AscImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of OrderBy
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null}) {
    return _then(
      _$OrderBy_AscImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$OrderBy_AscImpl extends OrderBy_Asc {
  const _$OrderBy_AscImpl({required this.column}) : super._();

  @override
  final String column;

  @override
  String toString() {
    return 'OrderBy.asc(column: $column)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$OrderBy_AscImpl &&
            (identical(other.column, column) || other.column == column));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column);

  /// Create a copy of OrderBy
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$OrderBy_AscImplCopyWith<_$OrderBy_AscImpl> get copyWith =>
      __$$OrderBy_AscImplCopyWithImpl<_$OrderBy_AscImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column) asc,
    required TResult Function(String column) desc,
    required TResult Function(String expression) raw,
  }) {
    return asc(column);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column)? asc,
    TResult? Function(String column)? desc,
    TResult? Function(String expression)? raw,
  }) {
    return asc?.call(column);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column)? asc,
    TResult Function(String column)? desc,
    TResult Function(String expression)? raw,
    required TResult orElse(),
  }) {
    if (asc != null) {
      return asc(column);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(OrderBy_Asc value) asc,
    required TResult Function(OrderBy_Desc value) desc,
    required TResult Function(OrderBy_Raw value) raw,
  }) {
    return asc(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(OrderBy_Asc value)? asc,
    TResult? Function(OrderBy_Desc value)? desc,
    TResult? Function(OrderBy_Raw value)? raw,
  }) {
    return asc?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(OrderBy_Asc value)? asc,
    TResult Function(OrderBy_Desc value)? desc,
    TResult Function(OrderBy_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (asc != null) {
      return asc(this);
    }
    return orElse();
  }
}

abstract class OrderBy_Asc extends OrderBy {
  const factory OrderBy_Asc({required final String column}) = _$OrderBy_AscImpl;
  const OrderBy_Asc._() : super._();

  String get column;

  /// Create a copy of OrderBy
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$OrderBy_AscImplCopyWith<_$OrderBy_AscImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$OrderBy_DescImplCopyWith<$Res> {
  factory _$$OrderBy_DescImplCopyWith(
    _$OrderBy_DescImpl value,
    $Res Function(_$OrderBy_DescImpl) then,
  ) = __$$OrderBy_DescImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String column});
}

/// @nodoc
class __$$OrderBy_DescImplCopyWithImpl<$Res>
    extends _$OrderByCopyWithImpl<$Res, _$OrderBy_DescImpl>
    implements _$$OrderBy_DescImplCopyWith<$Res> {
  __$$OrderBy_DescImplCopyWithImpl(
    _$OrderBy_DescImpl _value,
    $Res Function(_$OrderBy_DescImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of OrderBy
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? column = null}) {
    return _then(
      _$OrderBy_DescImpl(
        column: null == column
            ? _value.column
            : column // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$OrderBy_DescImpl extends OrderBy_Desc {
  const _$OrderBy_DescImpl({required this.column}) : super._();

  @override
  final String column;

  @override
  String toString() {
    return 'OrderBy.desc(column: $column)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$OrderBy_DescImpl &&
            (identical(other.column, column) || other.column == column));
  }

  @override
  int get hashCode => Object.hash(runtimeType, column);

  /// Create a copy of OrderBy
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$OrderBy_DescImplCopyWith<_$OrderBy_DescImpl> get copyWith =>
      __$$OrderBy_DescImplCopyWithImpl<_$OrderBy_DescImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column) asc,
    required TResult Function(String column) desc,
    required TResult Function(String expression) raw,
  }) {
    return desc(column);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column)? asc,
    TResult? Function(String column)? desc,
    TResult? Function(String expression)? raw,
  }) {
    return desc?.call(column);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column)? asc,
    TResult Function(String column)? desc,
    TResult Function(String expression)? raw,
    required TResult orElse(),
  }) {
    if (desc != null) {
      return desc(column);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(OrderBy_Asc value) asc,
    required TResult Function(OrderBy_Desc value) desc,
    required TResult Function(OrderBy_Raw value) raw,
  }) {
    return desc(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(OrderBy_Asc value)? asc,
    TResult? Function(OrderBy_Desc value)? desc,
    TResult? Function(OrderBy_Raw value)? raw,
  }) {
    return desc?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(OrderBy_Asc value)? asc,
    TResult Function(OrderBy_Desc value)? desc,
    TResult Function(OrderBy_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (desc != null) {
      return desc(this);
    }
    return orElse();
  }
}

abstract class OrderBy_Desc extends OrderBy {
  const factory OrderBy_Desc({required final String column}) =
      _$OrderBy_DescImpl;
  const OrderBy_Desc._() : super._();

  String get column;

  /// Create a copy of OrderBy
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$OrderBy_DescImplCopyWith<_$OrderBy_DescImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$OrderBy_RawImplCopyWith<$Res> {
  factory _$$OrderBy_RawImplCopyWith(
    _$OrderBy_RawImpl value,
    $Res Function(_$OrderBy_RawImpl) then,
  ) = __$$OrderBy_RawImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String expression});
}

/// @nodoc
class __$$OrderBy_RawImplCopyWithImpl<$Res>
    extends _$OrderByCopyWithImpl<$Res, _$OrderBy_RawImpl>
    implements _$$OrderBy_RawImplCopyWith<$Res> {
  __$$OrderBy_RawImplCopyWithImpl(
    _$OrderBy_RawImpl _value,
    $Res Function(_$OrderBy_RawImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of OrderBy
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? expression = null}) {
    return _then(
      _$OrderBy_RawImpl(
        expression: null == expression
            ? _value.expression
            : expression // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$OrderBy_RawImpl extends OrderBy_Raw {
  const _$OrderBy_RawImpl({required this.expression}) : super._();

  @override
  final String expression;

  @override
  String toString() {
    return 'OrderBy.raw(expression: $expression)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$OrderBy_RawImpl &&
            (identical(other.expression, expression) ||
                other.expression == expression));
  }

  @override
  int get hashCode => Object.hash(runtimeType, expression);

  /// Create a copy of OrderBy
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$OrderBy_RawImplCopyWith<_$OrderBy_RawImpl> get copyWith =>
      __$$OrderBy_RawImplCopyWithImpl<_$OrderBy_RawImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String column) asc,
    required TResult Function(String column) desc,
    required TResult Function(String expression) raw,
  }) {
    return raw(expression);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String column)? asc,
    TResult? Function(String column)? desc,
    TResult? Function(String expression)? raw,
  }) {
    return raw?.call(expression);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String column)? asc,
    TResult Function(String column)? desc,
    TResult Function(String expression)? raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(expression);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(OrderBy_Asc value) asc,
    required TResult Function(OrderBy_Desc value) desc,
    required TResult Function(OrderBy_Raw value) raw,
  }) {
    return raw(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(OrderBy_Asc value)? asc,
    TResult? Function(OrderBy_Desc value)? desc,
    TResult? Function(OrderBy_Raw value)? raw,
  }) {
    return raw?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(OrderBy_Asc value)? asc,
    TResult Function(OrderBy_Desc value)? desc,
    TResult Function(OrderBy_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(this);
    }
    return orElse();
  }
}

abstract class OrderBy_Raw extends OrderBy {
  const factory OrderBy_Raw({required final String expression}) =
      _$OrderBy_RawImpl;
  const OrderBy_Raw._() : super._();

  String get expression;

  /// Create a copy of OrderBy
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$OrderBy_RawImplCopyWith<_$OrderBy_RawImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$SqlValue {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() null_,
    required TResult Function(int field0) integer,
    required TResult Function(double field0) real,
    required TResult Function(String field0) text,
    required TResult Function(Uint8List field0) blob,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? null_,
    TResult? Function(int field0)? integer,
    TResult? Function(double field0)? real,
    TResult? Function(String field0)? text,
    TResult? Function(Uint8List field0)? blob,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? null_,
    TResult Function(int field0)? integer,
    TResult Function(double field0)? real,
    TResult Function(String field0)? text,
    TResult Function(Uint8List field0)? blob,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SqlValue_Null value) null_,
    required TResult Function(SqlValue_Integer value) integer,
    required TResult Function(SqlValue_Real value) real,
    required TResult Function(SqlValue_Text value) text,
    required TResult Function(SqlValue_Blob value) blob,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SqlValue_Null value)? null_,
    TResult? Function(SqlValue_Integer value)? integer,
    TResult? Function(SqlValue_Real value)? real,
    TResult? Function(SqlValue_Text value)? text,
    TResult? Function(SqlValue_Blob value)? blob,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SqlValue_Null value)? null_,
    TResult Function(SqlValue_Integer value)? integer,
    TResult Function(SqlValue_Real value)? real,
    TResult Function(SqlValue_Text value)? text,
    TResult Function(SqlValue_Blob value)? blob,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SqlValueCopyWith<$Res> {
  factory $SqlValueCopyWith(SqlValue value, $Res Function(SqlValue) then) =
      _$SqlValueCopyWithImpl<$Res, SqlValue>;
}

/// @nodoc
class _$SqlValueCopyWithImpl<$Res, $Val extends SqlValue>
    implements $SqlValueCopyWith<$Res> {
  _$SqlValueCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$SqlValue_NullImplCopyWith<$Res> {
  factory _$$SqlValue_NullImplCopyWith(
    _$SqlValue_NullImpl value,
    $Res Function(_$SqlValue_NullImpl) then,
  ) = __$$SqlValue_NullImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$SqlValue_NullImplCopyWithImpl<$Res>
    extends _$SqlValueCopyWithImpl<$Res, _$SqlValue_NullImpl>
    implements _$$SqlValue_NullImplCopyWith<$Res> {
  __$$SqlValue_NullImplCopyWithImpl(
    _$SqlValue_NullImpl _value,
    $Res Function(_$SqlValue_NullImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$SqlValue_NullImpl extends SqlValue_Null {
  const _$SqlValue_NullImpl() : super._();

  @override
  String toString() {
    return 'SqlValue.null_()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$SqlValue_NullImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() null_,
    required TResult Function(int field0) integer,
    required TResult Function(double field0) real,
    required TResult Function(String field0) text,
    required TResult Function(Uint8List field0) blob,
  }) {
    return null_();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? null_,
    TResult? Function(int field0)? integer,
    TResult? Function(double field0)? real,
    TResult? Function(String field0)? text,
    TResult? Function(Uint8List field0)? blob,
  }) {
    return null_?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? null_,
    TResult Function(int field0)? integer,
    TResult Function(double field0)? real,
    TResult Function(String field0)? text,
    TResult Function(Uint8List field0)? blob,
    required TResult orElse(),
  }) {
    if (null_ != null) {
      return null_();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SqlValue_Null value) null_,
    required TResult Function(SqlValue_Integer value) integer,
    required TResult Function(SqlValue_Real value) real,
    required TResult Function(SqlValue_Text value) text,
    required TResult Function(SqlValue_Blob value) blob,
  }) {
    return null_(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SqlValue_Null value)? null_,
    TResult? Function(SqlValue_Integer value)? integer,
    TResult? Function(SqlValue_Real value)? real,
    TResult? Function(SqlValue_Text value)? text,
    TResult? Function(SqlValue_Blob value)? blob,
  }) {
    return null_?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SqlValue_Null value)? null_,
    TResult Function(SqlValue_Integer value)? integer,
    TResult Function(SqlValue_Real value)? real,
    TResult Function(SqlValue_Text value)? text,
    TResult Function(SqlValue_Blob value)? blob,
    required TResult orElse(),
  }) {
    if (null_ != null) {
      return null_(this);
    }
    return orElse();
  }
}

abstract class SqlValue_Null extends SqlValue {
  const factory SqlValue_Null() = _$SqlValue_NullImpl;
  const SqlValue_Null._() : super._();
}

/// @nodoc
abstract class _$$SqlValue_IntegerImplCopyWith<$Res> {
  factory _$$SqlValue_IntegerImplCopyWith(
    _$SqlValue_IntegerImpl value,
    $Res Function(_$SqlValue_IntegerImpl) then,
  ) = __$$SqlValue_IntegerImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$SqlValue_IntegerImplCopyWithImpl<$Res>
    extends _$SqlValueCopyWithImpl<$Res, _$SqlValue_IntegerImpl>
    implements _$$SqlValue_IntegerImplCopyWith<$Res> {
  __$$SqlValue_IntegerImplCopyWithImpl(
    _$SqlValue_IntegerImpl _value,
    $Res Function(_$SqlValue_IntegerImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$SqlValue_IntegerImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as int,
      ),
    );
  }
}

/// @nodoc

class _$SqlValue_IntegerImpl extends SqlValue_Integer {
  const _$SqlValue_IntegerImpl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'SqlValue.integer(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SqlValue_IntegerImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SqlValue_IntegerImplCopyWith<_$SqlValue_IntegerImpl> get copyWith =>
      __$$SqlValue_IntegerImplCopyWithImpl<_$SqlValue_IntegerImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() null_,
    required TResult Function(int field0) integer,
    required TResult Function(double field0) real,
    required TResult Function(String field0) text,
    required TResult Function(Uint8List field0) blob,
  }) {
    return integer(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? null_,
    TResult? Function(int field0)? integer,
    TResult? Function(double field0)? real,
    TResult? Function(String field0)? text,
    TResult? Function(Uint8List field0)? blob,
  }) {
    return integer?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? null_,
    TResult Function(int field0)? integer,
    TResult Function(double field0)? real,
    TResult Function(String field0)? text,
    TResult Function(Uint8List field0)? blob,
    required TResult orElse(),
  }) {
    if (integer != null) {
      return integer(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SqlValue_Null value) null_,
    required TResult Function(SqlValue_Integer value) integer,
    required TResult Function(SqlValue_Real value) real,
    required TResult Function(SqlValue_Text value) text,
    required TResult Function(SqlValue_Blob value) blob,
  }) {
    return integer(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SqlValue_Null value)? null_,
    TResult? Function(SqlValue_Integer value)? integer,
    TResult? Function(SqlValue_Real value)? real,
    TResult? Function(SqlValue_Text value)? text,
    TResult? Function(SqlValue_Blob value)? blob,
  }) {
    return integer?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SqlValue_Null value)? null_,
    TResult Function(SqlValue_Integer value)? integer,
    TResult Function(SqlValue_Real value)? real,
    TResult Function(SqlValue_Text value)? text,
    TResult Function(SqlValue_Blob value)? blob,
    required TResult orElse(),
  }) {
    if (integer != null) {
      return integer(this);
    }
    return orElse();
  }
}

abstract class SqlValue_Integer extends SqlValue {
  const factory SqlValue_Integer(final int field0) = _$SqlValue_IntegerImpl;
  const SqlValue_Integer._() : super._();

  int get field0;

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SqlValue_IntegerImplCopyWith<_$SqlValue_IntegerImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SqlValue_RealImplCopyWith<$Res> {
  factory _$$SqlValue_RealImplCopyWith(
    _$SqlValue_RealImpl value,
    $Res Function(_$SqlValue_RealImpl) then,
  ) = __$$SqlValue_RealImplCopyWithImpl<$Res>;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class __$$SqlValue_RealImplCopyWithImpl<$Res>
    extends _$SqlValueCopyWithImpl<$Res, _$SqlValue_RealImpl>
    implements _$$SqlValue_RealImplCopyWith<$Res> {
  __$$SqlValue_RealImplCopyWithImpl(
    _$SqlValue_RealImpl _value,
    $Res Function(_$SqlValue_RealImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$SqlValue_RealImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as double,
      ),
    );
  }
}

/// @nodoc

class _$SqlValue_RealImpl extends SqlValue_Real {
  const _$SqlValue_RealImpl(this.field0) : super._();

  @override
  final double field0;

  @override
  String toString() {
    return 'SqlValue.real(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SqlValue_RealImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SqlValue_RealImplCopyWith<_$SqlValue_RealImpl> get copyWith =>
      __$$SqlValue_RealImplCopyWithImpl<_$SqlValue_RealImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() null_,
    required TResult Function(int field0) integer,
    required TResult Function(double field0) real,
    required TResult Function(String field0) text,
    required TResult Function(Uint8List field0) blob,
  }) {
    return real(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? null_,
    TResult? Function(int field0)? integer,
    TResult? Function(double field0)? real,
    TResult? Function(String field0)? text,
    TResult? Function(Uint8List field0)? blob,
  }) {
    return real?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? null_,
    TResult Function(int field0)? integer,
    TResult Function(double field0)? real,
    TResult Function(String field0)? text,
    TResult Function(Uint8List field0)? blob,
    required TResult orElse(),
  }) {
    if (real != null) {
      return real(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SqlValue_Null value) null_,
    required TResult Function(SqlValue_Integer value) integer,
    required TResult Function(SqlValue_Real value) real,
    required TResult Function(SqlValue_Text value) text,
    required TResult Function(SqlValue_Blob value) blob,
  }) {
    return real(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SqlValue_Null value)? null_,
    TResult? Function(SqlValue_Integer value)? integer,
    TResult? Function(SqlValue_Real value)? real,
    TResult? Function(SqlValue_Text value)? text,
    TResult? Function(SqlValue_Blob value)? blob,
  }) {
    return real?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SqlValue_Null value)? null_,
    TResult Function(SqlValue_Integer value)? integer,
    TResult Function(SqlValue_Real value)? real,
    TResult Function(SqlValue_Text value)? text,
    TResult Function(SqlValue_Blob value)? blob,
    required TResult orElse(),
  }) {
    if (real != null) {
      return real(this);
    }
    return orElse();
  }
}

abstract class SqlValue_Real extends SqlValue {
  const factory SqlValue_Real(final double field0) = _$SqlValue_RealImpl;
  const SqlValue_Real._() : super._();

  double get field0;

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SqlValue_RealImplCopyWith<_$SqlValue_RealImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SqlValue_TextImplCopyWith<$Res> {
  factory _$$SqlValue_TextImplCopyWith(
    _$SqlValue_TextImpl value,
    $Res Function(_$SqlValue_TextImpl) then,
  ) = __$$SqlValue_TextImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$SqlValue_TextImplCopyWithImpl<$Res>
    extends _$SqlValueCopyWithImpl<$Res, _$SqlValue_TextImpl>
    implements _$$SqlValue_TextImplCopyWith<$Res> {
  __$$SqlValue_TextImplCopyWithImpl(
    _$SqlValue_TextImpl _value,
    $Res Function(_$SqlValue_TextImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$SqlValue_TextImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$SqlValue_TextImpl extends SqlValue_Text {
  const _$SqlValue_TextImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'SqlValue.text(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SqlValue_TextImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SqlValue_TextImplCopyWith<_$SqlValue_TextImpl> get copyWith =>
      __$$SqlValue_TextImplCopyWithImpl<_$SqlValue_TextImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() null_,
    required TResult Function(int field0) integer,
    required TResult Function(double field0) real,
    required TResult Function(String field0) text,
    required TResult Function(Uint8List field0) blob,
  }) {
    return text(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? null_,
    TResult? Function(int field0)? integer,
    TResult? Function(double field0)? real,
    TResult? Function(String field0)? text,
    TResult? Function(Uint8List field0)? blob,
  }) {
    return text?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? null_,
    TResult Function(int field0)? integer,
    TResult Function(double field0)? real,
    TResult Function(String field0)? text,
    TResult Function(Uint8List field0)? blob,
    required TResult orElse(),
  }) {
    if (text != null) {
      return text(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SqlValue_Null value) null_,
    required TResult Function(SqlValue_Integer value) integer,
    required TResult Function(SqlValue_Real value) real,
    required TResult Function(SqlValue_Text value) text,
    required TResult Function(SqlValue_Blob value) blob,
  }) {
    return text(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SqlValue_Null value)? null_,
    TResult? Function(SqlValue_Integer value)? integer,
    TResult? Function(SqlValue_Real value)? real,
    TResult? Function(SqlValue_Text value)? text,
    TResult? Function(SqlValue_Blob value)? blob,
  }) {
    return text?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SqlValue_Null value)? null_,
    TResult Function(SqlValue_Integer value)? integer,
    TResult Function(SqlValue_Real value)? real,
    TResult Function(SqlValue_Text value)? text,
    TResult Function(SqlValue_Blob value)? blob,
    required TResult orElse(),
  }) {
    if (text != null) {
      return text(this);
    }
    return orElse();
  }
}

abstract class SqlValue_Text extends SqlValue {
  const factory SqlValue_Text(final String field0) = _$SqlValue_TextImpl;
  const SqlValue_Text._() : super._();

  String get field0;

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SqlValue_TextImplCopyWith<_$SqlValue_TextImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SqlValue_BlobImplCopyWith<$Res> {
  factory _$$SqlValue_BlobImplCopyWith(
    _$SqlValue_BlobImpl value,
    $Res Function(_$SqlValue_BlobImpl) then,
  ) = __$$SqlValue_BlobImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$SqlValue_BlobImplCopyWithImpl<$Res>
    extends _$SqlValueCopyWithImpl<$Res, _$SqlValue_BlobImpl>
    implements _$$SqlValue_BlobImplCopyWith<$Res> {
  __$$SqlValue_BlobImplCopyWithImpl(
    _$SqlValue_BlobImpl _value,
    $Res Function(_$SqlValue_BlobImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$SqlValue_BlobImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as Uint8List,
      ),
    );
  }
}

/// @nodoc

class _$SqlValue_BlobImpl extends SqlValue_Blob {
  const _$SqlValue_BlobImpl(this.field0) : super._();

  @override
  final Uint8List field0;

  @override
  String toString() {
    return 'SqlValue.blob(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SqlValue_BlobImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SqlValue_BlobImplCopyWith<_$SqlValue_BlobImpl> get copyWith =>
      __$$SqlValue_BlobImplCopyWithImpl<_$SqlValue_BlobImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() null_,
    required TResult Function(int field0) integer,
    required TResult Function(double field0) real,
    required TResult Function(String field0) text,
    required TResult Function(Uint8List field0) blob,
  }) {
    return blob(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? null_,
    TResult? Function(int field0)? integer,
    TResult? Function(double field0)? real,
    TResult? Function(String field0)? text,
    TResult? Function(Uint8List field0)? blob,
  }) {
    return blob?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? null_,
    TResult Function(int field0)? integer,
    TResult Function(double field0)? real,
    TResult Function(String field0)? text,
    TResult Function(Uint8List field0)? blob,
    required TResult orElse(),
  }) {
    if (blob != null) {
      return blob(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SqlValue_Null value) null_,
    required TResult Function(SqlValue_Integer value) integer,
    required TResult Function(SqlValue_Real value) real,
    required TResult Function(SqlValue_Text value) text,
    required TResult Function(SqlValue_Blob value) blob,
  }) {
    return blob(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SqlValue_Null value)? null_,
    TResult? Function(SqlValue_Integer value)? integer,
    TResult? Function(SqlValue_Real value)? real,
    TResult? Function(SqlValue_Text value)? text,
    TResult? Function(SqlValue_Blob value)? blob,
  }) {
    return blob?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SqlValue_Null value)? null_,
    TResult Function(SqlValue_Integer value)? integer,
    TResult Function(SqlValue_Real value)? real,
    TResult Function(SqlValue_Text value)? text,
    TResult Function(SqlValue_Blob value)? blob,
    required TResult orElse(),
  }) {
    if (blob != null) {
      return blob(this);
    }
    return orElse();
  }
}

abstract class SqlValue_Blob extends SqlValue {
  const factory SqlValue_Blob(final Uint8List field0) = _$SqlValue_BlobImpl;
  const SqlValue_Blob._() : super._();

  Uint8List get field0;

  /// Create a copy of SqlValue
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SqlValue_BlobImplCopyWith<_$SqlValue_BlobImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

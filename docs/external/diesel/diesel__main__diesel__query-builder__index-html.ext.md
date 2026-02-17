----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/query_builder/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, query builder, index html
- Summary: [Source](../../src/diesel/query_builder/mod.rs.html#1-449)
----

Source: https://docs.diesel.rs/main/diesel/query_builder/index.html

[diesel](../index.html)

# Module query_builder

[Source](../../src/diesel/query_builder/mod.rs.html#1-449)

Expand description

Contains traits responsible for the actual construction of SQL statements

The types in this module are part of Diesel’s public API, but are generally
only useful for implementing Diesel plugins. Applications should generally
not need to care about the types inside of this module.

## Modules[§](#modules)

[bind_collector](bind_collector/index.html)Types related to managing bind parameters during query construction.

## Structs[§](#structs)

[All](struct.All.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Keep duplicate rows in the result[AstPass](struct.AstPass.html)The primary type used when walking a Diesel AST during query execution.[AstPassToSqlOptions](struct.AstPassToSqlOptions.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This is used to pass down additional settings to the `AstPass`
when rendering the sql string.[BatchInsert](struct.BatchInsert.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This type represents a batch insert clause, which allows
to insert multiple rows at once.[BoxedLimitOffsetClause](struct.BoxedLimitOffsetClause.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`A boxed variant of [`LimitOffsetClause`](struct.LimitOffsetClause.html)[BoxedSelectStatement](struct.BoxedSelectStatement.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This type represents a boxed select query[BoxedSqlQuery](struct.BoxedSqlQuery.html)See [`SqlQuery::into_boxed`](struct.SqlQuery.html#method.into_boxed).[CollectedQuery](struct.CollectedQuery.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`A SQL query variant with already collected bind data which can be moved[ConflictTarget](struct.ConflictTarget.html)[DebugQuery](struct.DebugQuery.html)A struct that implements `fmt::Display` and `fmt::Debug` to show the SQL
representation of a query.[DefaultValues](struct.DefaultValues.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`[DeleteStatement](struct.DeleteStatement.html)Represents a SQL `DELETE` statement.[Distinct](struct.Distinct.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Remove duplicate rows in the result, this is the default behavior of `UNION`, `INTERSECT` and `EXCEPT`[Except](struct.Except.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Computes the set difference of the rows returned by the involved `SELECT` statements using SQL `EXCEPT`[FromClause](struct.FromClause.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`[IncompleteInsertStatement](struct.IncompleteInsertStatement.html)The structure returned by [`insert_into`](../fn.insert_into.html).[InsertOrIgnore](struct.InsertOrIgnore.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`A marker type for insert or ignore statements[InsertStatement](struct.InsertStatement.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`A fully constructed insert statement.[Intersect](struct.Intersect.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Computes the set intersection of the rows returned by the involved `SELECT` statements using SQL `INTERSECT`[LimitClause](struct.LimitClause.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`A query node representing a limit clause[LimitOffsetClause](struct.LimitOffsetClause.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`A helper query node that contains both limit and offset clauses[NoFromClause](struct.NoFromClause.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This type represents a not existing from clause[NoLimitClause](struct.NoLimitClause.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`A query node indicating the absence of a limit clause[NoOffsetClause](struct.NoOffsetClause.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`A query node indicating the absence of an offset clause[NoOrderClause](struct.NoOrderClause.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`DSL node that represents that no order clause is set[OffsetClause](struct.OffsetClause.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`A query node representing an offset clause[Only](struct.Only.html)`postgres_backend`Represents a query with an `ONLY` clause.[OrderClause](struct.OrderClause.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`DSL node that represents that an order clause is set[ParenthesisWrapper](struct.ParenthesisWrapper.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Wrapper used to wrap rhs sql in parenthesis when supported by backend[Replace](struct.Replace.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`A marker type for replace statements[ReturningClause](struct.ReturningClause.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This type represents a SQL `Returning` clause[SelectStatement](struct.SelectStatement.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This type represents a select query[SqlQuery](struct.SqlQuery.html)The return value of `sql_query`.[Tablesample](struct.Tablesample.html)`postgres_backend`Represents a query with a `TABLESAMPLE` clause.[Union](struct.Union.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Computes the set union of the rows returned by the involved `SELECT` statements using SQL `UNION`[UpdateStatement](struct.UpdateStatement.html)Represents a complete `UPDATE` statement.[UpdateTarget](struct.UpdateTarget.html)[ValuesClause](struct.ValuesClause.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This type represents a values clause used as part of insert statements

## Traits[§](#traits)

[AsChangeset](trait.AsChangeset.html)Types which can be passed to
[`update.set`](struct.UpdateStatement.html#method.set).[AsQuery](trait.AsQuery.html)Types that can be converted into a complete, typed SQL query.[BindCollector](trait.BindCollector.html)A type which manages serializing bind parameters during query construction.[DecoratableTarget](trait.DecoratableTarget.html)Interface to add information to conflict targets.
Designed to be open for further additions to conflict targets like constraints[IntoBoxedClause](trait.IntoBoxedClause.html)A trait used to construct type erased boxed variant of the current query node[IntoConflictValueClause](trait.IntoConflictValueClause.html)Represents a type that can be converted into a value clause for an
`ON CONFLICT` statement.[IntoUpdateTarget](trait.IntoUpdateTarget.html)A type which can be passed to [`update`](../fn.update.html) or [`delete`](../fn.delete.html).[MoveableBindCollector](trait.MoveableBindCollector.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`A movable version of the bind collector which allows it to be extracted, moved and refilled.[OnConflictTarget](trait.OnConflictTarget.html)Represents the target of an `ON CONFLICT` clause.[Query](trait.Query.html)A complete SQL query with a return type.[QueryBuilder](trait.QueryBuilder.html)Constructs a SQL query from a Diesel AST.[QueryFragment](trait.QueryFragment.html)An untyped fragment of SQL.[QueryId](trait.QueryId.html)Uniquely identifies queries by their type for the purpose of prepared
statement caching.[SelectClauseExpression](trait.SelectClauseExpression.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Specialised variant of `Expression` for select clause types[SelectQuery](trait.SelectQuery.html)Indicates that a type is a `SELECT` statement.[SupportsCombinationClause](trait.SupportsCombinationClause.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Marker trait used to indicate whenever a backend supports given combination[UndecoratedInsertRecord](trait.UndecoratedInsertRecord.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Marker trait to indicate that no additional operations have been added
to a record for insert.

## Functions[§](#functions)

[debug_query](fn.debug_query.html)Takes a query `QueryFragment` expression as an argument and returns a type
that implements `fmt::Display` and `fmt::Debug` to show the query.

## Type Aliases[§](#types)

[BoxedDeleteStatement](type.BoxedDeleteStatement.html)A `DELETE` statement with a boxed `WHERE` clause[BoxedUpdateStatement](type.BoxedUpdateStatement.html)An `UPDATE` statement with a boxed `WHERE` clause.[BuildQueryResult](type.BuildQueryResult.html)A specialized Result type used with the query builder.[IncompleteInsertOrIgnoreStatement](type.IncompleteInsertOrIgnoreStatement.html)Represents the return type of [`diesel::insert_or_ignore_into`](../fn.insert_or_ignore_into.html)[IncompleteReplaceStatement](type.IncompleteReplaceStatement.html)Represents the return type of [`diesel::replace_into`](../fn.replace_into.html)[InsertOrIgnoreStatement](type.InsertOrIgnoreStatement.html)Represents a complete `INSERT OR IGNORE` statement.[ReplaceStatement](type.ReplaceStatement.html)Represents a complete `INSERT OR REPLACE` statement.

## Derive Macros[§](#derives)

[AsChangeset](derive.AsChangeset.html)Implements `AsChangeset`[QueryId](derive.QueryId.html)Implements `QueryId`

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

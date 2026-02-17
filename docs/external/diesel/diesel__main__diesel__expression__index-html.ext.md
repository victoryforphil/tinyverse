----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/expression/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, expression, index html
- Summary: [Source](../../src/diesel/expression/mod.rs.html#1-1123)
----

Source: https://docs.diesel.rs/main/diesel/expression/index.html

[diesel](../index.html)

# Module expression

[Source](../../src/diesel/expression/mod.rs.html#1-1123)

Expand description

AST types representing various typed SQL expressions.

Almost all types implement either [`Expression`](trait.Expression.html) or
[`AsExpression`](trait.AsExpression.html).

The most common expression to work with is a
[`Column`](../query_source/trait.Column.html). There are various methods
that you can call on these, found in
[`expression_methods`](../expression_methods/index.html).

You can also use numeric operators such as `+` on expressions of the
appropriate type.

Any primitive which implements [`ToSql`](../serialize/trait.ToSql.html) will
also implement [`AsExpression`](trait.AsExpression.html), allowing it to be
used as an argument to any of the methods described here.

## Modules[§](#modules)

[array_comparison](array_comparison/index.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This module contains the query dsl node definitions
for array comparison operations like `IN` and `NOT IN`[exists](exists/index.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This module contains the query dsl node definition
for `EXISTS` expressions[expression_types](expression_types/index.html)Possible types for []`Expression::SqlType`][functions](functions/index.html)Helper macros to define custom sql functions[is_aggregate](is_aggregate/index.html)Possible values for `ValidGrouping::IsAggregate`

## Structs[§](#structs)

[CaseWhen](struct.CaseWhen.html)A SQL `CASE WHEN ... END` expression[Collate](struct.Collate.html)Represents the SQL `COLLATE` operator[Concat](struct.Concat.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This type represents a string concat operator[SqlLiteral](struct.SqlLiteral.html)Returned by the [`sql()`](../dsl/fn.sql.html) function.[UncheckedBind](struct.UncheckedBind.html)Returned by the [`SqlLiteral::bind()`](struct.SqlLiteral.html#method.bind) method when binding a value to a fragment of SQL.

## Traits[§](#traits)

[AppearsOnTable](trait.AppearsOnTable.html)Indicates that all elements of an expression are valid given a from clause.[AsExpression](trait.AsExpression.html)Converts a type to its representation for use in Diesel’s query builder.[AsExpressionList](trait.AsExpressionList.html)Deprecated`with-deprecated` and non-`without-deprecated`Converts a tuple of values into a tuple of Diesel expressions.[BoxableExpression](trait.BoxableExpression.html)Helper trait used when boxing expressions.[CastsTo](trait.CastsTo.html)Marker trait: this SQL type (`Self`) can be cast to the target SQL type
(`ST`) using `CAST(expr AS target_sql_type)`[Expression](trait.Expression.html)Represents a typed fragment of SQL.[FallibleCastsTo](trait.FallibleCastsTo.html)Marker trait: this SQL type (`Self`) can be cast to the target SQL type, but some values can be invalid[IntoSql](trait.IntoSql.html)Converts a type to its representation for use in Diesel’s query builder.[KnownCastSqlTypeName](trait.KnownCastSqlTypeName.html)We know what to write as `sql_type` in the `CAST(expr AS sql_type)` SQL for
`Self`[MixedAggregates](trait.MixedAggregates.html)Can two `IsAggregate` types appear in the same expression?[NonAggregate](trait.NonAggregate.html)Non-`unstable`Trait alias to represent an expression that isn’t aggregate by default.[QueryMetadata](trait.QueryMetadata.html)A helper to translate type level sql type information into
runtime type information for specific queries[Selectable](trait.Selectable.html)Trait indicating that a record can be selected and queried from the database.[SelectableExpression](trait.SelectableExpression.html)Indicates that an expression can be selected from a source.[SelectableHelper](trait.SelectableHelper.html)This helper trait provides several methods for
constructing a select or returning clause based on a
[`Selectable`](trait.Selectable.html) implementation.[TypedExpressionType](trait.TypedExpressionType.html)Marker trait for possible types of [`Expression::SqlType`](trait.Expression.html#associatedtype.SqlType)[ValidGrouping](trait.ValidGrouping.html)Is this expression valid for a given group by clause?

## Derive Macros[§](#derives)

[AsExpression](derive.AsExpression.html)Implements all required variants of `AsExpression`[Selectable](derive.Selectable.html)Implements `Selectable`[ValidGrouping](derive.ValidGrouping.html)Implements `ValidGrouping`

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

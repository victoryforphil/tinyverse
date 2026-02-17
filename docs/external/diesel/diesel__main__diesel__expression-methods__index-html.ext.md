----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/expression_methods/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, expression methods, index html
- Summary: [Source](../../src/diesel/expression_methods/mod.rs.html#1-50)
----

Source: https://docs.diesel.rs/main/diesel/expression_methods/index.html

[diesel](../index.html)

# Module expression_methods

[Source](../../src/diesel/expression_methods/mod.rs.html#1-50)

Expand description

Adds various methods to construct new expressions. These traits are exported
by default, and implemented automatically.

You can rely on the methods provided by this trait existing on any
`Expression` of the appropriate type. You should not rely on the specific
traits existing, their names, or their organization.

## Structs[§](#structs)

[OffsetFollowing](struct.OffsetFollowing.html)A following frame clause expression with a fixed offset[OffsetPreceding](struct.OffsetPreceding.html)A preceding frame clause expression with a fixed offset

## Traits[§](#traits)

[AggregateExpressionMethods](trait.AggregateExpressionMethods.html)Expression methods to build aggregate function expressions[AnyJsonExpressionMethods](trait.AnyJsonExpressionMethods.html)`__sqlite-shared` or `postgres_backend`PostgreSQL specific methods present on JSON and JSONB expressions.[BoolExpressionMethods](trait.BoolExpressionMethods.html)Methods present on boolean expressions[EscapeExpressionMethods](trait.EscapeExpressionMethods.html)Adds the `escape` method to `LIKE` and `NOT LIKE`. This is used to specify
the escape character for the pattern.[ExpressionMethods](trait.ExpressionMethods.html)Methods present on all expressions, except tuples[FrameBoundDsl](trait.FrameBoundDsl.html)Construct a frame clause for window functions from an integer[FrameClauseDsl](trait.FrameClauseDsl.html)Construct a frame clause for window functions[FrameClauseEndBound](trait.FrameClauseEndBound.html)A marker trait for possible end frame expressions[FrameClauseExclusion](trait.FrameClauseExclusion.html)A marker trait for possible frame exclusion expressions[FrameClauseStartBound](trait.FrameClauseStartBound.html)A marker trait for possible start frame expressions[JsonIndex](trait.JsonIndex.html)`__sqlite-shared` or `postgres_backend`A marker trait indicating which types can be used as index into a json field[NullableExpressionMethods](trait.NullableExpressionMethods.html)Methods present on all expressions[PgAnyJsonExpressionMethods](trait.PgAnyJsonExpressionMethods.html)`postgres_backend`PostgreSQL specific methods present on JSON and JSONB expressions.[PgArrayExpressionMethods](trait.PgArrayExpressionMethods.html)`postgres_backend`PostgreSQL specific methods present on array expressions.[PgBinaryExpressionMethods](trait.PgBinaryExpressionMethods.html)`postgres_backend`PostgreSQL specific methods present on Binary expressions.[PgExpressionMethods](trait.PgExpressionMethods.html)`postgres_backend`PostgreSQL specific methods which are present on all expressions.[PgJsonbExpressionMethods](trait.PgJsonbExpressionMethods.html)`postgres_backend`PostgreSQL specific methods present on JSONB expressions.[PgNetExpressionMethods](trait.PgNetExpressionMethods.html)`postgres_backend`PostgreSQL specific methods present between CIDR/INET expressions[PgRangeExpressionMethods](trait.PgRangeExpressionMethods.html)`postgres_backend`PostgreSQL specific methods present on range expressions.[PgSortExpressionMethods](trait.PgSortExpressionMethods.html)`postgres_backend`PostgreSQL expression methods related to sorting.[PgTextExpressionMethods](trait.PgTextExpressionMethods.html)`postgres_backend`PostgreSQL specific methods present on text expressions.[PgTimestampExpressionMethods](trait.PgTimestampExpressionMethods.html)`postgres_backend`PostgreSQL specific methods present on timestamp expressions.[PreferredBoolSqlType](trait.PreferredBoolSqlType.html)Allow ~type inference on [And](../dsl/type.And.html) and [Or](../dsl/type.Or.html)
helper types[SqliteAnyJsonExpressionMethods](trait.SqliteAnyJsonExpressionMethods.html)`__sqlite-shared`SQLite specific methods present on JSON and JSONB expressions.[SqliteExpressionMethods](trait.SqliteExpressionMethods.html)`__sqlite-shared`Sqlite specific methods which are present on all expressions.[TextExpressionMethods](trait.TextExpressionMethods.html)Methods present on text expressions[UntypedExpressionMethods](trait.UntypedExpressionMethods.html)Methods present on untyped expressions.[WindowExpressionMethods](trait.WindowExpressionMethods.html)Methods to construct a window function call

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

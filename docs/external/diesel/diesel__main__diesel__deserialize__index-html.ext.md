----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/deserialize/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, deserialize, index html
- Summary: [Source](../../src/diesel/deserialize.rs.html#1-622)
----

Source: https://docs.diesel.rs/main/diesel/deserialize/index.html

[diesel](../index.html)

# Module deserialize

[Source](../../src/diesel/deserialize.rs.html#1-622)

Expand description

Types and traits related to deserializing values from the database

## Traits[§](#traits)

[FromSql](trait.FromSql.html)Deserialize a single field of a given SQL type.[FromSqlRow](trait.FromSqlRow.html)Deserialize a database row into a rust data structure[FromStaticSqlRow](trait.FromStaticSqlRow.html)A helper trait to deserialize a statically sized row into a tuple[Queryable](trait.Queryable.html)Trait indicating that a record can be queried from the database.[QueryableByName](trait.QueryableByName.html)Deserializes the result of a query constructed with [`sql_query`](../fn.sql_query.html).[StaticallySizedRow](trait.StaticallySizedRow.html)A marker trait indicating that the corresponding type consumes a static at
compile time known number of field

## Type Aliases[§](#types)

[Result](type.Result.html)A specialized result type representing the result of deserializing
a value from the database.

## Derive Macros[§](#derives)

[FromSqlRow](derive.FromSqlRow.html)Implements `Queryable` for types that correspond to a single SQL type. The type must implement `FromSql`.[Queryable](derive.Queryable.html)Implements `Queryable` to load the result of statically typed queries[QueryableByName](derive.QueryableByName.html)Implements `QueryableByName` for untyped sql queries, such as that one generated
by `sql_query`

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

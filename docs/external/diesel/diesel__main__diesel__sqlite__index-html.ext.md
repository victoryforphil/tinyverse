----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/sqlite/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, sqlite, index html
- Summary: [Source](../../src/diesel/sqlite/mod.rs.html#1-59)
----

Source: https://docs.diesel.rs/main/diesel/sqlite/index.html

[diesel](../index.html)

# Module sqlite

[Source](../../src/diesel/sqlite/mod.rs.html#1-59)

Available on crate feature `__sqlite-shared` only.

Expand description

Provides types and functions related to working with SQLite

Much of this module is re-exported from database agnostic locations.
However, if you are writing code specifically to extend Diesel on
SQLite, you may need to work with this module directly.

## Re-exports[§](#reexports)

`pub use self::query_builder::SqliteQueryBuilder;`

## Modules[§](#modules)

[expression](expression/index.html)Sqlite related query builder extensions.[query_builder](query_builder/index.html)The SQLite query builder[sql_types](sql_types/index.html)SQLite specific sql types

## Structs[§](#structs)

[SerializedDatabase](struct.SerializedDatabase.html)`SerializedDatabase` is a wrapper for a serialized database that is dynamically allocated by calling `sqlite3_serialize`.
This RAII wrapper is necessary to deallocate the memory when it goes out of scope with `sqlite3_free`.[Sqlite](struct.Sqlite.html)The SQLite backend[SqliteBindValue](struct.SqliteBindValue.html)This type represents a value bound to
a sqlite prepared statement[SqliteConnection](struct.SqliteConnection.html)Connections for the SQLite backend. Unlike other backends, SQLite supported
connection URLs are:[SqliteValue](struct.SqliteValue.html)Raw sqlite value as received from the database

## Enums[§](#enums)

[JsonValidFlag](enum.JsonValidFlag.html)Flags for the `json_valid` function[SqliteType](enum.SqliteType.html)Determines how a bind parameter is given to SQLite

## Traits[§](#traits)

[SqliteAggregateFunction](trait.SqliteAggregateFunction.html)Trait for the implementation of a SQLite aggregate function

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

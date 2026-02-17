----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/upsert/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, upsert, index html
- Summary: [Source](../../src/diesel/upsert/mod.rs.html#1-31)
----

Source: https://docs.diesel.rs/main/diesel/upsert/index.html

[diesel](../index.html)

# Module upsert

[Source](../../src/diesel/upsert/mod.rs.html#1-31)

Expand description

Types and functions related to PG’s and Sqlite’s `ON CONFLICT` clause

Upsert is currently supported by diesel for the following database systems:

- PostgreSQL version 9.5 or newer

- Sqlite3 version 3.24.0 or newer

- MySQL version 5.7 or newer

See [the methods on `InsertStatement`](../query_builder/struct.InsertStatement.html#impl-2)
for usage examples.

Constructing an upsert statement from an existing select statement
requires a where clause on sqlite due to a ambiguity in their
parser. See [the corresponding documentation](https://www.sqlite.org/lang_UPSERT.html)
for details.

## Structs[§](#structs)

[IncompleteDoUpdate](struct.IncompleteDoUpdate.html)A partially constructed `ON CONFLICT DO UPDATE` clause.[IncompleteOnConflict](struct.IncompleteOnConflict.html)A partially constructed `ON CONFLICT` clause.

## Traits[§](#traits)

[DecoratableTarget](trait.DecoratableTarget.html)Interface to add information to conflict targets.
Designed to be open for further additions to conflict targets like constraints

## Functions[§](#functions)

[excluded](fn.excluded.html)Represents `excluded.column` in an `ON CONFLICT DO UPDATE` clause.[on_constraint](fn.on_constraint.html)`postgres_backend`Used to specify the constraint name for an upsert statement in the form `ON CONFLICT ON CONSTRAINT`. Note that `constraint_name` must be the name of a
unique constraint, not the name of an index.

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

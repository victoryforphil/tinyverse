----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/serialize/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, serialize, index html
- Summary: [Source](../../src/diesel/serialize.rs.html#1-335)
----

Source: https://docs.diesel.rs/main/diesel/serialize/index.html

[diesel](../index.html)

# Module serialize

[Source](../../src/diesel/serialize.rs.html#1-335)

Expand description

Types and traits related to serializing values for the database

## Structs[§](#structs)

[Output](struct.Output.html)Wraps a buffer to be written by `ToSql` with additional backend specific
utilities.

## Enums[§](#enums)

[IsNull](enum.IsNull.html)Tiny enum to make the return type of `ToSql` more descriptive

## Traits[§](#traits)

[ToSql](trait.ToSql.html)Serializes a single value to be sent to the database.[WriteTuple](trait.WriteTuple.html)`postgres_backend`Helper trait for writing tuples as named composite types

## Type Aliases[§](#types)

[Result](type.Result.html)A specialized result type representing the result of serializing
a value for the database.

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

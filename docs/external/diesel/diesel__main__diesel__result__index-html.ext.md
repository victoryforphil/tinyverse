----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/result/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, result, index html
- Summary: [Source](../../src/diesel/result.rs.html#1-565)
----

Source: https://docs.diesel.rs/main/diesel/result/index.html

[diesel](../index.html)

# Module result

[Source](../../src/diesel/result.rs.html#1-565)

Expand description

Errors, type aliases, and functions related to working with `Result`.

## Structs[§](#structs)

[DeserializeFieldError](struct.DeserializeFieldError.html)An error occurred while deserializing a field[EmptyChangeset](struct.EmptyChangeset.html)Expected when an update has no changes to save.[EmptyQuery](struct.EmptyQuery.html)Expected when you try to execute an empty query[UnexpectedEndOfRow](struct.UnexpectedEndOfRow.html)Expected more fields then present in the current row while deserializing results[UnexpectedNullError](struct.UnexpectedNullError.html)An unexpected `NULL` was encountered during deserialization

## Enums[§](#enums)

[ConnectionError](enum.ConnectionError.html)Errors which can occur during [`Connection::establish`](../connection/trait.Connection.html#tymethod.establish)[DatabaseErrorKind](enum.DatabaseErrorKind.html)The kind of database error that occurred.[Error](enum.Error.html)Represents all the ways that a query can fail.

## Traits[§](#traits)

[DatabaseErrorInformation](trait.DatabaseErrorInformation.html)Information about an error that was returned by the database.[OptionalEmptyChangesetExtension](trait.OptionalEmptyChangesetExtension.html)See the [method documentation](trait.OptionalEmptyChangesetExtension.html#tymethod.optional_empty_changeset).[OptionalExtension](trait.OptionalExtension.html)See the [method documentation](trait.OptionalExtension.html#tymethod.optional).

## Type Aliases[§](#types)

[ConnectionResult](type.ConnectionResult.html)A specialized result type for establishing connections.[QueryResult](type.QueryResult.html)A specialized result type for queries.

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/backend/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, backend, index html
- Summary: [Source](../../src/diesel/backend.rs.html#1-657)
----

Source: https://docs.diesel.rs/main/diesel/backend/index.html

[diesel](../index.html)

# Module backend

[Source](../../src/diesel/backend.rs.html#1-657)

Expand description

Types which represent various database backends

## Modules[§](#modules)

[sql_dialect](sql_dialect/index.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This module contains all options provided by diesel to configure the [`SqlDialect`](trait.SqlDialect.html) trait.

## Traits[§](#traits)

[Backend](trait.Backend.html)A database backend[DieselReserveSpecialization](trait.DieselReserveSpecialization.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This is a marker trait which indicates that
diesel may specialize a certain [`QueryFragment`](../query_builder/trait.QueryFragment.html)
impl in a later version. If you as a user encounter, where rustc
suggests adding this a bound to a type implementing `Backend`
consider adding the following bound instead
`YourQueryType: QueryFragment` (the concrete bound
is likely mentioned by rustc as part of a `note: …`)[SqlDialect](trait.SqlDialect.html)This trait provides various options to configure the
generated SQL for a specific backend.[TrustedBackend](trait.TrustedBackend.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This trait just indicates that none implements
[`SqlDialect`](trait.SqlDialect.html) without enabling the
`i-implement-a-third-party-backend-and-opt-into-breaking-changes`
feature flag.

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

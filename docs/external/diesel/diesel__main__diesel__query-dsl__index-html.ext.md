----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/query_dsl/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, query dsl, index html
- Summary: [Source](../../src/diesel/query_dsl/mod.rs.html#1-1846)
----

Source: https://docs.diesel.rs/main/diesel/query_dsl/index.html

[diesel](../index.html)

# Module query_dsl

[Source](../../src/diesel/query_dsl/mod.rs.html#1-1846)

Expand description

Traits that construct SELECT statements

Traits in this module have methods that generally map to the keyword for the corresponding clause in SQL,
unless it conflicts with a Rust keyword (such as `WHERE`/`where`).

Methods for constructing queries lives on the [`QueryDsl`](trait.QueryDsl.html) trait.
Methods for executing queries live on [`RunQueryDsl`](trait.RunQueryDsl.html).

See also [`expression_methods`](../expression_methods/index.html) and [`dsl`](../dsl/index.html).

## Modules[§](#modules)

[methods](methods/index.html)The traits used by `QueryDsl`.

## Traits[§](#traits)

[BelongingToDsl](trait.BelongingToDsl.html)Constructs a query that finds record(s) based on directional association with other record(s).[CombineDsl](trait.CombineDsl.html)Extension trait to combine queries using a combinator like `UNION`, `INTERSECT` or `EXCEPT`
with or without `ALL` rule for duplicates[CompatibleType](trait.CompatibleType.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`[JoinOnDsl](trait.JoinOnDsl.html)Specify the `ON` clause for a join statement. This will override
any implicit `ON` clause that would come from [`joinable!`](../macro.joinable.html)[QueryDsl](trait.QueryDsl.html)Methods used to construct select statements.[RunQueryDsl](trait.RunQueryDsl.html)Methods used to execute queries.[SaveChangesDsl](trait.SaveChangesDsl.html)Sugar for types which implement both `AsChangeset` and `Identifiable`[UpdateAndFetchResults](trait.UpdateAndFetchResults.html)A trait defining how to update a record and fetch the updated entry
on a certain backend.

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

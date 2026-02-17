----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/pg/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, pg, index html
- Summary: [Source](../../src/diesel/pg/mod.rs.html#1-70)
----

Source: https://docs.diesel.rs/main/diesel/pg/index.html

[diesel](../index.html)

# Module pg

[Source](../../src/diesel/pg/mod.rs.html#1-70)

Available on crate feature `postgres_backend` only.

Expand description

Provides types and functions related to working with PostgreSQL

Much of this module is re-exported from database agnostic locations.
However, if you are writing code specifically to extend Diesel on
PostgreSQL, you may need to work with this module directly.

## Modules[§](#modules)

[data_types](data_types/index.html)Data structures for PG types which have no corresponding Rust type[expression](expression/index.html)PostgreSQL related query builder extensions[sql_types](sql_types/index.html)PostgreSQL specific SQL types

## Structs[§](#structs)

[CopyFromQuery](struct.CopyFromQuery.html)The structure returned by [`copy_from`](../fn.copy_from.html)[CopyToQuery](struct.CopyToQuery.html)The structure returned by [`copy_to`](../fn.copy_to.html)[DistinctOnClause](struct.DistinctOnClause.html)Represents `DISTINCT ON (...)`[FailedToLookupTypeError](struct.FailedToLookupTypeError.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This error indicates that a type lookup for a custom
postgres type failed[Pg](struct.Pg.html)The PostgreSQL backend[PgConnection](struct.PgConnection.html)`postgres`The connection string expected by `PgConnection::establish`
should be a PostgreSQL connection string, as documented at
[https://www.postgresql.org/docs/9.4/static/libpq-connect.html#LIBPQ-CONNSTRING](https://www.postgresql.org/docs/9.4/static/libpq-connect.html#LIBPQ-CONNSTRING)[PgMetadataCache](struct.PgMetadataCache.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Cache for the [OIDs](https://www.postgresql.org/docs/current/static/datatype-oid.html) of custom Postgres types[PgMetadataCacheKey](struct.PgMetadataCacheKey.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`The key used to lookup cached type oid’s inside of
a [PgMetadataCache](struct.PgMetadataCache.html).[PgNotification](struct.PgNotification.html)See Postgres documentation for SQL Commands NOTIFY and LISTEN[PgQueryBuilder](struct.PgQueryBuilder.html)The PostgreSQL query builder[PgRowByRowLoadingMode](struct.PgRowByRowLoadingMode.html)`postgres`A [`PgConnection`](struct.PgConnection.html) specific loading mode to load rows one by one[PgTypeMetadata](struct.PgTypeMetadata.html)The [OIDs](https://www.postgresql.org/docs/current/static/datatype-oid.html) for a SQL type[PgValue](struct.PgValue.html)Raw postgres value as received from the database[TransactionBuilder](struct.TransactionBuilder.html)Used to build a transaction, specifying additional details.

## Enums[§](#enums)

[CopyFormat](enum.CopyFormat.html)Describes the format used by `COPY FROM` or `COPY TO`
statements[CopyHeader](enum.CopyHeader.html)Describes the different possible settings for the `HEADER` option
for `COPY FROM` statements

## Traits[§](#traits)

[CopyTarget](trait.CopyTarget.html)A expression that could be used as target/source for `COPY FROM` and `COPY TO` commands[GetPgMetadataCache](trait.GetPgMetadataCache.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Gets the `PgMetadataCache` for a `Connection`
so that the lookup of user defined types, or types which come from an extension can be cached.[OrderDecorator](trait.OrderDecorator.html)A decorator trait for `OrderClause`
It helps to have bounds on either Col, Asc and Desc.[PgMetadataLookup](trait.PgMetadataLookup.html)Determines the OID of types at runtime[TypeOidLookup](trait.TypeOidLookup.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This is a helper trait to defer a type oid
lookup to a later point in time

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

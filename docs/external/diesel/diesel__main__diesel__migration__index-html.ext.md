----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/migration/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, migration, index html
- Summary: [Source](../../src/diesel/migration/mod.rs.html#1-227)
----

Source: https://docs.diesel.rs/main/diesel/migration/index.html

[diesel](../index.html)

# Module migration

[Source](../../src/diesel/migration/mod.rs.html#1-227)

Expand description

Representation of migrations

## Structs[§](#structs)

[MigrationVersion](struct.MigrationVersion.html)A migration version identifier

## Constants[§](#constants)

[CREATE_MIGRATIONS_TABLE](constant.CREATE_MIGRATIONS_TABLE.html)Create table statement for the `__diesel_schema_migrations` used
used by the postgresql, sqlite and mysql backend

## Traits[§](#traits)

[Migration](trait.Migration.html)Represents a migration that interacts with diesel[MigrationConnection](trait.MigrationConnection.html)A trait indicating that a connection could be used to manage migrations[MigrationMetadata](trait.MigrationMetadata.html)This trait is designed to customize the behaviour
of the default migration harness of diesel[MigrationName](trait.MigrationName.html)Represents the name of a migration[MigrationSource](trait.MigrationSource.html)A migration source is an entity that can be used
to receive a number of migrations from.

## Type Aliases[§](#types)

[Result](type.Result.html)A specialized result type representing the result of
a migration operation

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

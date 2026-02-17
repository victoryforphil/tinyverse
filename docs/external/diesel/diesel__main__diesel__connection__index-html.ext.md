----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/connection/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, connection, index html
- Summary: [Source](../../src/diesel/connection/mod.rs.html#1-613)
----

Source: https://docs.diesel.rs/main/diesel/connection/index.html

[diesel](../index.html)

# Module connection

[Source](../../src/diesel/connection/mod.rs.html#1-613)

Expand description

Types related to database connections

## Modules[§](#modules)

[statement_cache](statement_cache/index.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Helper types for prepared statement caching

## Structs[§](#structs)

[AnsiTransactionManager](struct.AnsiTransactionManager.html)An implementation of `TransactionManager` which can be used for backends
which use ANSI standard syntax for savepoints such as SQLite and PostgreSQL.[DefaultLoadingMode](struct.DefaultLoadingMode.html)The default loading mode provided by a [`Connection`](trait.Connection.html).[DynInstrumentation](struct.DynInstrumentation.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`An optional dyn instrumentation.[InTransactionStatus](struct.InTransactionStatus.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Various status fields to track the status of
a transaction manager with a started transaction[StrQueryHelper](struct.StrQueryHelper.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`A helper type that allows printing out str slices[ValidTransactionManagerStatus](struct.ValidTransactionManagerStatus.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Valid transaction status for the manager. Can return the current transaction depth

## Enums[§](#enums)

[CacheSize](enum.CacheSize.html)Set cache size for a connection[InstrumentationEvent](enum.InstrumentationEvent.html)This enum describes possible connection events
that can be handled by an [`Instrumentation`](trait.Instrumentation.html) implementation[TransactionDepthChange](enum.TransactionDepthChange.html)Represents a change to apply to the depth of a transaction[TransactionManagerStatus](enum.TransactionManagerStatus.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Status of the transaction manager

## Traits[§](#traits)

[BoxableConnection](trait.BoxableConnection.html)A variant of the [`Connection`](trait.Connection.html) trait that is
usable with dynamic dispatch[Connection](trait.Connection.html)A connection to a database[ConnectionSealed](trait.ConnectionSealed.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This trait restricts who can implement `Connection`[DebugQuery](trait.DebugQuery.html)A helper trait for opaque query representations
which allows to get a `Display` and `Debug`
representation of the underlying type without
exposing type specific details[Instrumentation](trait.Instrumentation.html)A type that provides an connection `Instrumentation`[LoadConnection](trait.LoadConnection.html)The specific part of a [`Connection`](trait.Connection.html) which actually loads data from the database[MultiConnectionHelper](trait.MultiConnectionHelper.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`This trait provides helper methods to convert a database lookup type
to/from an `std::any::Any` reference. This is used internally by the `#[derive(MultiConnection)]`
implementation[SimpleConnection](trait.SimpleConnection.html)Perform simple operations on a backend.[TransactionManager](trait.TransactionManager.html)Manages the internal transaction state for a connection.[WithMetadataLookup](trait.WithMetadataLookup.html)`i-implement-a-third-party-backend-and-opt-into-breaking-changes`Describes a connection with an underlying [`crate::sql_types::TypeMetadata::MetadataLookup`](../sql_types/trait.TypeMetadata.html#associatedtype.MetadataLookup)

## Functions[§](#functions)

[get_default_instrumentation](fn.get_default_instrumentation.html)Get an instance of the default [`Instrumentation`](trait.Instrumentation.html)[set_default_instrumentation](fn.set_default_instrumentation.html)`std`Set a custom constructor for the default [`Instrumentation`](trait.Instrumentation.html)
used by new connections

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

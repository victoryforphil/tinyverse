----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/sql_types/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, sql types, index html
- Summary: [Source](../../src/diesel/sql_types/mod.rs.html#1-823)
----

Source: https://docs.diesel.rs/main/diesel/sql_types/index.html

[diesel](../index.html)

# Module sql_types

[Source](../../src/diesel/sql_types/mod.rs.html#1-823)

Expand description

Types which represent a SQL data type.

The structs in this module are only used as markers to represent a SQL type.
They should never be used in your structs.
If you’d like to know the rust types which can be used for a given SQL type,
see the documentation for that SQL type.
Additional types may be provided by other crates.

To see which SQL type can be used with a given Rust type,
see the “Implementors” section of [`FromSql`](../deserialize/trait.FromSql.html).

Any backend specific types are re-exported through this module

## Modules[§](#modules)

[is_nullable](is_nullable/index.html)Possible values for `SqlType::IsNullable`[ops](ops/index.html)Represents the output of numeric operators in SQL

## Structs[§](#structs)

[Array](struct.Array.html)`postgres_backend`The [`Array`](https://www.postgresql.org/docs/current/arrays.html) SQL type.[BigInt](struct.BigInt.html)The big integer SQL type.[Binary](struct.Binary.html)The binary SQL type.[Bool](struct.Bool.html)The boolean SQL type.[CChar](struct.CChar.html)`postgres_backend`The [`"char"`](https://www.postgresql.org/docs/current/datatype-character.html#DATATYPE-CHARACTER-SPECIAL-TABLE) SQL type. This is a PostgreSQL specific type. Used for e.g. [setweight](https://www.postgresql.org/docs/current/functions-textsearch.html). [Do not use in user tables](https://www.postgresql.org/docs/current/datatype-character.html#DATATYPE-CHARACTER-SPECIAL-TABLE).[Cidr](struct.Cidr.html)`postgres_backend`The [`CIDR`](https://www.postgresql.org/docs/postgresql/static/datatype-net-types.html) SQL type. This type can only be used with `feature = "network-address"` or `feature = "ipnet-address"`.[Citext](struct.Citext.html)`postgres_backend`The [`Citext`](https://www.postgresql.org/docs/current/citext.html) SQL type. This is a PostgreSQL specific type.[Date](struct.Date.html)The date SQL type.[Datetime](struct.Datetime.html)`mysql_backend`Represents the MySQL datetime type.[Double](struct.Double.html)The double precision float SQL type.[Float](struct.Float.html)The float SQL type.[Inet](struct.Inet.html)`postgres_backend`The [`INET`](https://www.postgresql.org/docs/current/static/datatype-net-types.html) SQL type. This type can only be used with `feature = "network-address"` or `feature = "ipnet-address"`.[Integer](struct.Integer.html)The integer SQL type.[Interval](struct.Interval.html)The interval SQL type.[Json](struct.Json.html)The JSON SQL type.  This type can only be used with `feature = "serde_json"`[Jsonb](struct.Jsonb.html)The [`jsonb`](https://www.postgresql.org/docs/current/datatype-json.html) SQL type.  This type can only be used with `feature = "serde_json"`[MacAddr](struct.MacAddr.html)`postgres_backend`The [`MACADDR`](https://www.postgresql.org/docs/current/static/datatype-net-types.html) SQL type.[MacAddr8](struct.MacAddr8.html)`postgres_backend`The [`MACADDR8`](https://www.postgresql.org/docs/current/static/datatype-net-types.html) SQL type.[Money](struct.Money.html)`postgres_backend`The PostgreSQL [Money](https://www.postgresql.org/docs/current/static/datatype-money.html) type.[Multirange](struct.Multirange.html)`postgres_backend`The [`Multirange`](https://www.postgresql.org/docs/current/rangetypes.html) SQL type.[NullValueTreatmentEnum](struct.NullValueTreatmentEnum.html)`postgres_backend`This is a wrapper for [`NullValueTreatment`](enum.NullValueTreatment.html) to represent null_value_treatment for jsonb_seet_lax:
‘raise_exception’ ‘use_json_null’ ‘delete_key’ ‘return_target’
used in functions jsonb_set_lax[Nullable](struct.Nullable.html)The nullable SQL type.[Numeric](struct.Numeric.html)The arbitrary precision numeric SQL type.[Oid](struct.Oid.html)`postgres_backend`The [`OID`](https://www.postgresql.org/docs/current/datatype-oid.html) SQL type. This is a PostgreSQL specific type.[PgLsn](struct.PgLsn.html)`postgres_backend`The [`pg_lsn`](https://www.postgresql.org/docs/current/datatype-pg-lsn.html) SQL type. This is a PostgreSQL specific type. Encodes a position in the PostgreSQL Write Ahead Log (WAL).[Range](struct.Range.html)`postgres_backend`The [`Range`](https://www.postgresql.org/docs/current/rangetypes.html) SQL type.[RangeBoundEnum](struct.RangeBoundEnum.html)`postgres_backend`This is a wrapper for [`RangeBound`](enum.RangeBound.html) to represent range bounds: ‘[]’, ‘(]’, ‘[)’, ‘()’,
used in functions int4range, int8range, numrange, tsrange, tstzrange, daterange.[Record](struct.Record.html)`postgres_backend`The [`Record`](https://www.postgresql.org/docs/current/rowtypes.html) (a.k.a. tuple) SQL type.[SmallInt](struct.SmallInt.html)The small integer SQL type.[Text](struct.Text.html)The text SQL type.[Time](struct.Time.html)The time SQL type.[Timestamp](struct.Timestamp.html)The timestamp SQL type.[Timestamptz](struct.Timestamptz.html)`postgres_backend`The [“timestamp with time zone” SQL type](https://www.postgresql.org/docs/current/datatype-datetime.html), which PostgreSQL abbreviates
to `timestamptz`.[TimestamptzSqlite](struct.TimestamptzSqlite.html)`__sqlite-shared`The SQLite timestamp with time zone type[TinyInt](struct.TinyInt.html)The tiny integer SQL type.[Unsigned](struct.Unsigned.html)`mysql_backend`Represents the MySQL unsigned type.[Untyped](struct.Untyped.html)Query nodes with this expression type do not have a statically at compile
time known expression type.[Uuid](struct.Uuid.html)`postgres_backend`The [`UUID`](https://www.postgresql.org/docs/current/datatype-uuid.html) SQL type. This type can only be used with `feature = "uuid"`

## Enums[§](#enums)

[NullValueTreatment](enum.NullValueTreatment.html)`postgres_backend`Represent null_value_treatment for jsonb_seet_lax:
‘raise_exception’ ‘use_json_null’ ‘delete_key’ ‘return_target’
used in functions jsonb_seet_lax.[RangeBound](enum.RangeBound.html)`postgres_backend`Represent postgres range bounds: ‘[]’, ‘(]’, ‘[)’, ‘()’,
used in functions int4range, int8range, numrange, tsrange, tstzrange, daterange.

## Traits[§](#traits)

[AllAreNullable](trait.AllAreNullable.html)Are both values of `IsNull` are nullable?[BoolOrNullableBool](trait.BoolOrNullableBool.html)A marker trait for accepting expressions of the type `Bool` and
`Nullable` in the same place[Foldable](trait.Foldable.html)Represents SQL types which can be used with `SUM` and `AVG`[HasSqlType](trait.HasSqlType.html)Indicates that a SQL type exists for a backend.[IntoNotNullable](trait.IntoNotNullable.html)Converts a type which may or may not be nullable into its not nullable
representation.[IntoNullable](trait.IntoNullable.html)Converts a type which may or may not be nullable into its nullable
representation.[MaybeNullableType](trait.MaybeNullableType.html)A type level constructor for maybe nullable types[OneIsNullable](trait.OneIsNullable.html)Is one value of `IsNull` nullable?[SingleValue](trait.SingleValue.html)A marker trait indicating that a SQL type represents a single value, as
opposed to a list of values.[SqlOrd](trait.SqlOrd.html)Marker trait for types which can be used with `MAX` and `MIN`[SqlType](trait.SqlType.html)A marker trait for SQL types[TypeMetadata](trait.TypeMetadata.html)Information about how a backend stores metadata about given SQL types

## Type Aliases[§](#types)

[BigSerial](type.BigSerial.html)`postgres_backend`Alias for [`BigInt`](struct.BigInt.html)[Bpchar](type.Bpchar.html)`postgres_backend`[Bytea](type.Bytea.html)`postgres_backend`Alias for `Binary`, to ensure `diesel print-schema` works[Datemultirange](type.Datemultirange.html)`postgres_backend`[Daterange](type.Daterange.html)`postgres_backend`[Decimal](type.Decimal.html)Alias for `Numeric`[Int4multirange](type.Int4multirange.html)`postgres_backend`[Int4range](type.Int4range.html)`postgres_backend`[Int8multirange](type.Int8multirange.html)`postgres_backend`[Int8range](type.Int8range.html)`postgres_backend`[Macaddr](type.Macaddr.html)`postgres_backend`Alias for `MacAddr` to be able to use it with `diesel print-schema`.[Macaddr8](type.Macaddr8.html)`postgres_backend`Alias for `MacAddr` to be able to use it with `diesel print-schema`.[Nummultirange](type.Nummultirange.html)`postgres_backend`[Numrange](type.Numrange.html)`postgres_backend`[Serial](type.Serial.html)`postgres_backend`Alias for [`Integer`](struct.Integer.html)[SmallSerial](type.SmallSerial.html)`postgres_backend`Alias for [`SmallInt`](struct.SmallInt.html)[Tsmultirange](type.Tsmultirange.html)`postgres_backend`[Tsrange](type.Tsrange.html)`postgres_backend`[Tstzmultirange](type.Tstzmultirange.html)`postgres_backend`[Tstzrange](type.Tstzrange.html)`postgres_backend`[VarChar](type.VarChar.html)The SQL `VARCHAR` type

## Derive Macros[§](#derives)

[DieselNumericOps](derive.DieselNumericOps.html)Implement numeric operators for the current query node[SqlType](derive.SqlType.html)Implement necessary traits for adding a new sql type

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

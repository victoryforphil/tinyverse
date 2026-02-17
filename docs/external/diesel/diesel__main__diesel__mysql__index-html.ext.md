----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/mysql/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, mysql, index html
- Summary: [Source](../../src/diesel/mysql/mod.rs.html#1-34)
----

Source: https://docs.diesel.rs/main/diesel/mysql/index.html

[diesel](../index.html)

# Module mysql

[Source](../../src/diesel/mysql/mod.rs.html#1-34)

Available on crate feature `mysql_backend` only.

Expand description

Provides types and functions related to working with MySQL

Much of this module is re-exported from database agnostic locations.
However, if you are writing code specifically to extend Diesel on
MySQL, you may need to work with this module directly.

## Modules[§](#modules)

[data_types](data_types/index.html)Data structures for MySQL types which have no corresponding Rust type[sql_types](sql_types/index.html)MySQL specific sql types

## Structs[§](#structs)

[Mysql](struct.Mysql.html)The MySQL backend[MysqlConnection](struct.MysqlConnection.html)`mysql`A connection to a MySQL database. Connection URLs should be in the form
`mysql://[user[:password]@]host/database_name[?unix_socket=socket-path&ssl_mode=SSL_MODE*&ssl_ca=/etc/ssl/certs/ca-certificates.crt&ssl_cert=/etc/ssl/certs/client-cert.crt&ssl_key=/etc/ssl/certs/client-key.crt]`[MysqlQueryBuilder](struct.MysqlQueryBuilder.html)The MySQL query builder[MysqlValue](struct.MysqlValue.html)Raw mysql value as received from the database

## Enums[§](#enums)

[MysqlType](enum.MysqlType.html)Represents possible types, that can be transmitted as via the
Mysql wire protocol[NumericRepresentation](enum.NumericRepresentation.html)Represents all possible forms MySQL transmits integers

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/r2d2/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, r2d2, index html
- Summary: [Source](../../src/diesel/r2d2.rs.html#1-709)
----

Source: https://docs.diesel.rs/main/diesel/r2d2/index.html

[diesel](../index.html)

# Module r2d2

[Source](../../src/diesel/r2d2.rs.html#1-709)

Available on crate feature `r2d2` only.

Expand description

Connection pooling via r2d2.

Note: This module requires enabling the `r2d2` feature

## [§](#example)Example

The below snippet is a contrived example emulating a web application,
where one would first initialize the pool in the `main()` function
(at the start of a long-running process). One would then pass this
pool struct around as shared state, which, here, we’ve emulated using
threads instead of routes.

```
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;
use std::thread;

pub fn get_connection_pool() -> Pool> {
    let url = database_url_for_env();
    let manager = ConnectionManager::::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn create_user(conn: &mut DbConnection, user_name: &str) -> Result {
    use schema::users::dsl::*;

    diesel::insert_into(users)
        .values(name.eq(user_name))
        .execute(conn)
}

fn main() {
    let pool = get_connection_pool();
    let mut threads = vec![];
    let max_users_to_create = 1;

    for i in 0..max_users_to_create {
        let pool = pool.clone();
        threads.push(thread::spawn({
            move || {
                let conn = &mut pool.get().unwrap();
                let name = format!("Person {}", i);
                create_user(conn, &name).unwrap();
            }
        }))
    }

    for handle in threads {
        handle.join().unwrap();
    }
}
```

## [§](#a-note-on-error-handling)A note on error handling

When used inside a pool, if an individual connection becomes
broken (as determined by the [R2D2Connection::is_broken](trait.R2D2Connection.html#method.is_broken) method)
then, when the connection goes out of scope, `r2d2` will close
and return the connection to the DB.

`diesel` determines broken connections by whether or not the current
thread is panicking or if individual `Connection` structs are
broken (determined by the `is_broken()` method). Generically, these
are left to individual backends to implement themselves.

For SQLite, PG, and MySQL backends `is_broken()` is determined
by whether or not the `TransactionManagerStatus` (as a part
of the `AnsiTransactionManager` struct) is in an `InError` state
or contains an open transaction when the connection goes out of scope.

## [§](#testing-with-connections-pools)Testing with connections pools

When testing with connection pools, it is recommended to set the pool size to 1,
and use a customizer to ensure that the transactions are never committed.
The tests using a pool prepared this way can be run in parallel, because
the changes are never committed to the database and are local to each test.

## [§](#example-1)Example

```
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::CustomizeConnection;
use diesel::r2d2::TestCustomizer;
use diesel::r2d2::Pool;
use diesel::result::Error;
use std::thread;

pub fn get_testing_pool() -> Pool> {
    let url = database_url_for_env();
    let manager = ConnectionManager::::new(url);

    Pool::builder()
        .test_on_check_out(true)
        .max_size(1) // Max pool size set to 1
        .connection_customizer(Box::new(TestCustomizer)) // Test customizer
        .build(manager)
        .expect("Could not build connection pool")
}

table! {
    users {
        id -> Integer,
        name -> Text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[diesel_test_helper::test]
    fn test_1() {
        let pool = get_testing_pool();
        let mut conn = pool.get().unwrap();

        crate::sql_query(
            "CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name TEXT NOT NULL)",
        )
        .execute(&mut conn)
        .unwrap();

        crate::insert_into(users::table)
            .values(users::name.eq("John"))
            .execute(&mut conn)
            .unwrap();
    }

    #[diesel_test_helper::test]
    fn test_2() {
        let pool = get_testing_pool();
        let mut conn = pool.get().unwrap();

        crate::sql_query(
            "CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name TEXT NOT NULL)",
        )
        .execute(&mut conn)
        .unwrap();

        let user_count = users::table.count().get_result::(&mut conn).unwrap();
        assert_eq!(user_count, 0); // Because the transaction from test_1 was never committed
    }
}
```

## Modules[§](#modules)

[event](event/index.html)Event subscriptions.

## Structs[§](#structs)

[Builder](struct.Builder.html)A builder for a connection pool.[ConnectionManager](struct.ConnectionManager.html)An r2d2 connection manager for use with Diesel.[Extensions](struct.Extensions.html)A “type map” used to associate data with pooled connections.[LoggingErrorHandler](struct.LoggingErrorHandler.html)A `HandleError` implementation which logs at the error level.[NopConnectionCustomizer](struct.NopConnectionCustomizer.html)A `CustomizeConnection` which does nothing.[NopErrorHandler](struct.NopErrorHandler.html)A `HandleError` implementation which does nothing.[NopEventHandler](struct.NopEventHandler.html)A `HandleEvent` implementation which does nothing.[Pool](struct.Pool.html)A generic connection pool.[PooledConnection](struct.PooledConnection.html)A smart pointer wrapping a connection.[State](struct.State.html)Information about the state of a `Pool`.[TestCustomizer](struct.TestCustomizer.html)A connection customizer designed for use in tests. Implements
[CustomizeConnection](trait.CustomizeConnection.html) in a way that ensures transactions
in a pool customized by it are never committed.

## Enums[§](#enums)

[Error](enum.Error.html)The error used when managing connections with `r2d2`.

## Traits[§](#traits)

[CustomizeConnection](trait.CustomizeConnection.html)A trait which allows for customization of connections.[HandleError](trait.HandleError.html)A trait which handles errors reported by the `ManageConnection`.[HandleEvent](trait.HandleEvent.html)A trait which is provided with information about events in a connection pool.[ManageConnection](trait.ManageConnection.html)A trait which provides connection-specific functionality.[R2D2Connection](trait.R2D2Connection.html)A trait indicating a connection could be used inside a r2d2 pool

## Type Aliases[§](#types)

[PoolError](type.PoolError.html)A re-export of [`r2d2::Error`](../../r2d2/struct.Error.html), which is only used by methods on [`r2d2::Pool`](struct.Pool.html).

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

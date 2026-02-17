----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /guides/all-about-inserts.html
- Keywords: diesel, rust, orm, guides, api, guides, all about inserts html
- Summary: Most applications fall into a category called “CRUD” apps. CRUD
----

Source: https://diesel.rs/guides/all-about-inserts.html

## All About Inserts

Most applications fall into a category called “CRUD” apps. CRUD
stands for “Create, Read, Update, Delete”. Diesel provides support for
all four pieces, but in this guide we’re going to look at the different
ways to go about creating `INSERT` statements.

The examples for this guide are going to be shown for PostgreSQL, but
you can follow along with any backend. The full code examples for all
backends are linked at the bottom of this guide.

An insert statement always starts with [`insert_into`](https://docs.diesel.rs/2.3.x/diesel/fn.insert_into.html).
The first argument to this function is the table you’re inserting
into.

For this guide, our schema will look like this:

src/lib.rs

View on GitHub

```
diesel::table! {
    users {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
```

Since our functions are going to only operate on the
`users` table, we can put
`use schema::users::dsl::*;` at the top of our function,
which will let us write `insert_into(users)` instead of
`insert_into(users::table)`. If you’re importing
`table::dsl::*`, make sure it’s always inside a function, not
the top of your module.

If all of the columns on a table have a default, the simplest thing
we can do is call [`.default_values`](https://docs.diesel.rs/2.3.x/diesel/query_builder/insert_statement/struct.IncompleteInsertStatement.html#method.default_values).
We could write a function that ran that query like this:

src/lib.rs

View on GitHub

```
use schema::users::dsl::*;

insert_into(users).default_values().execute(conn)
```

It’s worth noting that this code will still compile, even if you
don’t have default values on all of your columns. Diesel will ensure
that the value you’re assigning has the right type, but it can’t
validate whether the column has a default, any constraints that could
fail, or any triggers that could fire.

We can use [`debug_query`](https://docs.diesel.rs/2.3.x/diesel/fn.debug_query.html)
to inspect the generated SQL. The exact SQL that is generated may differ
depending on the backend you’re using. If we run
`println!("{}", debug_query::
(&our_query));`,
we’ll see the following:

Generated SQL

View on GitHub

```
INSERT INTO "users" DEFAULT VALUES -- binds: []
```

If we want to actually provide values, we can call [`.values`](https://docs.diesel.rs/2.3.x/diesel/query_builder/insert_statement/struct.IncompleteInsertStatement.html#method.values)
instead. There are a lot of different arguments we can provide here. The
simplest is a single column/value pair using [`.eq`](https://docs.diesel.rs/2.3.x/diesel/expression_methods/trait.ExpressionMethods.html#method.eq).

src/lib.rs

View on GitHub

```
use schema::users::dsl::*;

insert_into(users).values(name.eq("Sean")).execute(conn)
```

This will generate the following SQL:

Generated SQL

View on GitHub

```
INSERT INTO "users" ("name") VALUES ($1)
-- binds ["Sean"]
```

If we want to provide values for more than one column, we can pass a
tuple.

src/lib.rs

View on GitHub

```
insert_into(users)
    .values((name.eq("Tess"), hair_color.eq("Brown")))
    .execute(conn)
```

This will generate the following SQL:

Generated SQL

View on GitHub

```
INSERT INTO "users" ("name", "hair_color") VALUES ($1, $2)
-- binds: ["Tess", "Brown"]
```

## Insertable

Working with tuples is the typical way to do an insert if you just
have some values that you want to stick in the database. But what if
your data is coming from another source, like a web form deserialized by
Serde? It’d be annoying to have to write
`(name.eq(user.name), hair_color.eq(user.hair_color))`.

Diesel provides the [`Insertable`](https://docs.diesel.rs/2.3.x/diesel/prelude/trait.Insertable.html)
trait for this case. `Insertable` maps your struct to columns
in the database. We can derive this automatically by adding [`#[derive(Insertable)]`](https://docs.diesel.rs/2.3.x/diesel/prelude/derive.Insertable.html)
to our type.

src/lib.rs

View on GitHub

```
use schema::users;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct UserForm {
    name: &'a str,
    hair_color: Option,
}
```

src/lib.rs

View on GitHub

```
use schema::users::dsl::*;

let json = r#"{ "name": "Sean", "hair_color": "Black" }"#;
let user_form = serde_json::from_str::(json)?;

insert_into(users).values(&user_form).execute(conn)?;

Ok(())
```

This will generate the same SQL as if we had used a tuple.

Generated SQL

View on GitHub

```
INSERT INTO "users" ("name", "hair_color") VALUES ($1, $2)
-- binds: ["Sean", "Black"]
```

If one of the fields is `None`, the default value will be
inserted for that field.

src/lib.rs

View on GitHub

```
use schema::users::dsl::*;

let json = r#"{ "name": "Ruby", "hair_color": null }"#;
let user_form = serde_json::from_str::(json)?;

insert_into(users).values(&user_form).execute(conn)?;

Ok(())
```

That will generate the following SQL:

Generated SQL

View on GitHub

```
INSERT INTO "users" ("name", "hair_color") VALUES ($1, DEFAULT)
-- binds: ["Ruby"]
```

## Batch Insert

If we want to insert more than one row at a time, we can do that by
passing a `&Vec` or slice of any of the forms used above.
Keep in mind that you’re always passing a reference here.

On backends that support the `DEFAULT` keyword (all
backends except SQLite), the data will be inserted in a single query. On
SQLite, one query will be performed per row.

Customizing the usage of `DEFAULT` values:

You can use the
`#[diesel(treat_none_as_default_value = false)]` attribute on
a struct to disable using `DEFAULT` in place of
`None` values and rather use `NULL` values for
such fields. This enables for example real batch inserts with the SQLite
backend.

For example, if we wanted to insert two rows with a single value, we
can just use a `Vec`.

src/lib.rs

View on GitHub

```
use schema::users::dsl::*;

insert_into(users)
    .values(&vec![name.eq("Sean"), name.eq("Tess")])
    .execute(conn)
```

Which generates the following SQL:

Generated SQL

View on GitHub

```
INSERT INTO "users" ("name") VALUES ($1), ($2)
-- binds ["Sean", "Tess"]
```

If we wanted to use `DEFAULT` for some of our rows, we can
use an option here.

src/lib.rs

View on GitHub

```
use schema::users::dsl::*;

insert_into(users)
    .values(&vec![Some(name.eq("Sean")), None])
    .execute(conn)
```

Note that the type here is
`Option>` not
`Eq>`. Doing
`column.eq(None)` would insert `NULL` not
`DEFAULT`. This generates the following SQL:

Generated SQL

View on GitHub

```
INSERT INTO "users" ("name") VALUES ($1), (DEFAULT)
-- binds ["Sean"]
```

We can do the same thing with tuples.

src/lib.rs

View on GitHub

```
use schema::users::dsl::*;

insert_into(users)
    .values(&vec![
        (name.eq("Sean"), hair_color.eq("Black")),
        (name.eq("Tess"), hair_color.eq("Brown")),
    ])
    .execute(conn)
```

Which generates the following SQL:

Generated SQL

View on GitHub

```
INSERT INTO "users" ("name", "hair_color")
VALUES ($1, $2), ($3, $4)
-- binds: ["Sean", "Black", "Tess", "Brown"]
```

Once again, we can use an `Option` for any of the fields
to insert `DEFAULT`.

src/lib.rs

View on GitHub

```
use schema::users::dsl::*;

insert_into(users)
    .values(&vec![
        (name.eq("Sean"), Some(hair_color.eq("Black"))),
        (name.eq("Ruby"), None),
    ])
    .execute(conn)
```

Which generates the following SQL:

Generated SQL

View on GitHub

```
INSERT INTO "users" ("name", "hair_color")
VALUES ($1, $2), ($3, DEFAULT)
-- binds: ["Sean", "Black", "Ruby"]
```

Finally, `Insertable` structs can be used for batch insert
as well.

src/lib.rs

View on GitHub

```
use schema::users::dsl::*;

let json = r#"[
    { "name": "Sean", "hair_color": "Black" },
    { "name": "Tess", "hair_color": "Brown" }
]"#;
let user_form = serde_json::from_str::>(json)?;

insert_into(users).values(&user_form).execute(conn)?;

Ok(())
```

This generates the same SQL as if we had used a tuple:

Generated SQL

View on GitHub

```
INSERT INTO "users" ("name", "hair_color")
VALUES ($1, $2), ($3, $4)
-- binds: ["Sean", "Black", "Tess", "Brown"]
```

## The `RETURNING` Clause

On backends that support the `RETURNING` clause (such as
PostgreSQL and SQLite), we can get data back from our insert as well. On
the SQLite backend, support for the `RETURNING` clause can be
enabled with a feature flag,
`returning_clauses_for_sqlite_3_35`. MySQL does not support
`RETURNING` clauses. To get back all of the inserted rows, we
can call [`.get_results`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html#method.get_results)
instead of [`.execute`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html#method.execute).

Given this struct:

src/lib.rs

View on GitHub

```
#[derive(Queryable, PartialEq, Debug)]
struct User {
    id: i32,
    name: String,
    hair_color: Option,
    created_at: SystemTime,
    updated_at: SystemTime,
}
```

We can use `get_results` with this test:

src/lib.rs

View on GitHub

```
use diesel::select;
use schema::users::dsl::*;

let now = select(diesel::dsl::now).get_result::(conn)?;

let inserted_users = insert_into(users)
    .values(&vec![
        (id.eq(1), name.eq("Sean")),
        (id.eq(2), name.eq("Tess")),
    ])
    .get_results(conn)?;

let expected_users = vec![
    User {
        id: 1,
        name: "Sean".into(),
        hair_color: None,
        created_at: now,
        updated_at: now,
    },
    User {
        id: 2,
        name: "Tess".into(),
        hair_color: None,
        created_at: now,
        updated_at: now,
    },
];
assert_eq!(expected_users, inserted_users);
```

To inspect the SQL generated by `.get_results` or
`.get_result`, we will need to call `.as_query`
before passing it to `debug_query`. The query in the last
test generates the following SQL:

Generated SQL

View on GitHub

```
INSERT INTO "users" ("id", "name") VALUES ($1, $2), ($3, $4)
RETURNING "users"."id", "users"."name", "users"."hair_color",
          "users"."created_at", "users"."updated_at"
-- binds: [1, "Sean", 2, "Tess"]
```

You’ll notice that we’ve never given an explicit value for
`created_at` and `updated_at` in any of our
examples. With Diesel, you typically won’t set those values in Rust.
Typically these columns get set with
`DEFAULT CURRENT_TIMESTAMP`, and a trigger is used to change
`updated_at` on updates. If you’re using PostgreSQL, you can
use a built-in trigger by running
`SELECT diesel_manage_updated_at('users');` in a
migration.

If we expect one row instead of multiple, we can call
`.get_result` instead of `.get_results`.

src/lib.rs

View on GitHub

```
use diesel::select;
use schema::users::dsl::*;

let now = select(diesel::dsl::now).get_result::(conn)?;

let inserted_user = insert_into(users)
    .values((id.eq(3), name.eq("Ruby")))
    .get_result(conn)?;

let expected_user = User {
    id: 3,
    name: "Ruby".into(),
    hair_color: None,
    created_at: now,
    updated_at: now,
};
assert_eq!(expected_user, inserted_user);
```

This generates the same SQL as `get_results`:

Generated SQL

View on GitHub

```
INSERT INTO "users" ("id", "name") VALUES ($1, $2)
RETURNING "users"."id", "users"."name", "users"."hair_color",
          "users"."created_at", "users"."updated_at"
-- binds: [3, "Ruby"]
```

Finally, if we only want a single column back, we can call
`.returning()` explicitly. This code would return the
inserted ID:

src/lib.rs

View on GitHub

```
use schema::users::dsl::*;

insert_into(users)
    .values(name.eq("Ruby"))
    .returning(id)
    .get_result(conn)
```

Which generates the following SQL:

Generated SQL

View on GitHub

```
INSERT INTO "users" ("name") VALUES ($1)
RETURNING "users"."id"
-- binds: ["Ruby"]
```

## “Upsert”

Every type of insert statement covered in this guide can also be used
for “insert or update” queries, also known as “upsert”. The specifics of
upsert are covered extensively in the API documentation.

For PostgreSQL and SQLite, see the [`diesel::upsert`](https://docs.diesel.rs/2.3.x/diesel/upsert/index.html)
module. For MySQL, upsert is done via `REPLACE`. See [`replace_into`](https://docs.diesel.rs/2.3.x/diesel/fn.replace_into.html)
for details.

## Conclusion

While there are a lot of examples in this guide, ultimately the only
difference between various kinds of insert statements is the argument
passed to `.values`.

All examples in this guide are run as part of Diesel’s test suite.
You can find the full code examples for each backend at these links:

- [PostgreSQL](https://github.com/diesel-rs/diesel/tree/2.3.x/examples/postgres/all_about_inserts)

- [MySQL](https://github.com/diesel-rs/diesel/tree/2.3.x/examples/mysql/all_about_inserts)

- [SQLite](https://github.com/diesel-rs/diesel/tree/2.3.x/examples/sqlite/all_about_inserts)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

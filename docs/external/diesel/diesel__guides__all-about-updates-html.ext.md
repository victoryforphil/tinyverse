----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /guides/all-about-updates.html
- Keywords: diesel, rust, orm, guides, api, guides, all about updates html
- Summary: Most applications fall into a category called “CRUD” apps. CRUD
----

Source: https://diesel.rs/guides/all-about-updates.html

## All About Updates

Most applications fall into a category called “CRUD” apps. CRUD
stands for “Create, Read, Update, Delete”. Diesel provides support for
all four pieces, but in this guide we’re going to look at all the
different ways to go about updating records.

An update statement is constructed by calling
`diesel::update(target).set(changes)`. The resulting
statement is then run by calling either `execute`,
`get_result`, or `get_results`.

If you look at the documentation for [`update`](https://docs.diesel.rs/2.3.x/diesel/fn.update.html),
you’ll notice that the type of the argument is any type `T`
which implements `IntoUpdateTarget`. You don’t need to worry
about what this trait does, but it is important to know which types
implement it. There are three kinds which implement this trait. The
first is tables.

If we have a table that looks like this:

src/lib.rs

View on GitHub

```
table! {
    posts {
        id -> BigInt,
        title -> Text,
        body -> Text,
        draft -> Bool,
        publish_at -> Timestamp,
        visit_count -> Integer,
    }
}
```

We could write a query that publishes all posts by doing:

src/lib.rs

View on GitHub

```
use crate::posts::dsl::*;

diesel::update(posts).set(draft.eq(false)).execute(conn)
```

We can use the [`debug_query`](https://docs.diesel.rs/2.3.x/diesel/fn.debug_query.html)
function to inspect the generated SQL. The output you see may slightly
differ from this guide, depending on which backend you’re using. If we
run
`println!("{}", debug_query::
(&our_query));`,
we’ll see the following:

Generated SQL

View on GitHub

```
UPDATE "posts" SET "draft" = $1 -- binds: [false]
```

This is pretty much one-to-one with the Rust code (the
`$1` denotes a bound parameter in PostgreSQL, in SQLite/MySQL
it would be `?`, which will be substituted with
`false` here). It’s quite rare to want to update an entire
table, though. So let’s look at how we can scope that down. The second
kind that you can pass to `update` is any query which has
only had `.filter` called on it. We could scope our update to
only touch posts where `publish_at` is in the past like
so:

src/lib.rs

View on GitHub

```
use crate::posts::dsl::*;
use diesel::dsl::now;

diesel::update(posts)
    .filter(publish_at.lt(now))
    .set(draft.eq(false))
    .execute(conn)
```

That would generate the following SQL:

Generated SQL

View on GitHub

```
UPDATE "posts" SET "draft" = $1 WHERE ("posts"."publish_at" View on GitHub

```
#[derive(Queryable, Identifiable, AsChangeset)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub draft: bool,
    pub publish_at: SystemTime,
    pub visit_count: i32,
}
```

The struct has one field per database column, but what’s important
for `Identifiable` is that it has the `id` field,
which is the primary key of our table. Since our struct name is just the
table name without an `s`, we don’t have to provide the table
name explicitly. If our struct were named something different, or if
pluralizing it was more complex than putting an `s` on the
end, we would have to specify the table name by adding
`#[diesel(table_name = posts)]`. We’re using
`SystemTime` here since it’s in the standard library, but in
a real application we’d probably want to use a more full-featured type
like one from `chrono`, which you can do by enabling the
`chrono` feature on Diesel.

If we wanted to publish just this post, we could do it like this:

src/lib.rs

View on GitHub

```
diesel::update(post)
    .set(posts::draft.eq(false))
    .execute(conn)
```

It’s important to note that we always pass a reference to the post,
not the post itself. When we write `update(post)`, that’s
equivalent to writing `update(posts.find(post.id))`, or
`update(posts.filter(id.eq(post.id)))`. We can see this in
the generated SQL:

Generated SQL

View on GitHub

```
UPDATE "posts" SET "draft" = $1 WHERE ("posts"."id" = $2) -- binds: [false, 1]
```

Now that we’ve seen all the ways to specify what we want to update,
let’s look at the different ways to provide the data to update it with.
We’ve already seen the first way, which is to pass
`column.eq(value)` directly. So far we’ve just been passing
Rust values here, but we can actually use any Diesel expression. For
example, we could increment a column:

src/lib.rs

View on GitHub

```
use crate::posts::dsl::*;

diesel::update(posts)
    .set(visit_count.eq(visit_count + 1))
    .execute(conn)
```

That would generate this SQL:

Generated SQL

View on GitHub

```
UPDATE "posts" SET "visit_count" = ("posts"."visit_count" + $1) -- binds: [1]
```

Assigning values directly is great for small, simple changes. If we
wanted to update multiple columns this way, we can pass a tuple.

src/lib.rs

View on GitHub

```
use crate::posts::dsl::*;

diesel::update(posts)
    .set((
        title.eq("[REDACTED]"),
        body.eq("This post has been classified"),
    ))
    .execute(conn)
```

This will generate exactly the SQL you’d expect:

Generated SQL

View on GitHub

```
UPDATE "posts" SET "title" = $1, "body" = $2 -- binds: ["[REDACTED]", "This post has been classified"]
```

## AsChangeset

While it’s nice to have the ability to update columns directly like
this, it can quickly get cumbersome when dealing with forms that have
more than a handful of fields. If we look at the signature of [`.set`](https://docs.diesel.rs/2.3.x/diesel/query_builder/struct.UpdateStatement.html#method.set),
you’ll notice that the constraint is for a trait called [`AsChangeset`](https://docs.diesel.rs/2.3.x/diesel/query_builder/trait.AsChangeset.html).
This is another trait that `diesel` can derive for us. We can
add [`#[derive(AsChangeset)]`](https://docs.diesel.rs/2.3.x/diesel/prelude/derive.AsChangeset.html)
to our `Post` struct, which will let us pass a
`&Post` to `set`.

src/lib.rs

View on GitHub

```
diesel::update(posts::table).set(post).execute(conn)
```

The SQL will set every field present on the `Post` struct
except for the primary key.

Generated SQL

View on GitHub

```
UPDATE "posts" SET "title" = $1, "body" = $2, "draft" = $3, "publish_at" = $4, "visit_count" = $5 -- binds: ["", "", false, now, 0]
```

Changing the primary key of an existing row is almost never something
that you want to do, so `#[derive(AsChangeset)]` assumes that
you want to ignore it. The only way to change the primary key is to
explicitly do it with `.set(id.eq(new_id))`. However, note
that `#[derive(AsChangeset)]` doesn’t have the information
from your table definition. If the primary key is something other than
`id`, you’ll need to put
`#[diesel(primary_key(your_primary_key))]` on the struct as
well.

If the struct has any optional fields on it, these will also have
special behavior. By default, `#[derive(AsChangeset)]` will
assume that `None` means that you don’t wish to assign that
field. For example, if we had the following code:

src/lib.rs

View on GitHub

```
#[derive(AsChangeset)]
#[diesel(table_name = posts)]
struct PostForm {
    title: Option,
    body: Option,
}

diesel::update(posts::table)
    .set(&PostForm {
        title: None,
        body: Some("My new post"),
    })
    .execute(conn)
```

That would generate the following SQL:

Generated SQL

View on GitHub

```
UPDATE "posts" SET "body" = $1 -- binds: ["My new post"]
```

If you wanted to assign `NULL` instead, you can either
specify `#[diesel(treat_none_as_null = true)]` on the struct,
or you can have the field be of type
`Option>`. Diesel doesn’t currently
provide a way to explicitly assign a field to its default value, though
it may be provided in the future.

If you are using PostgreSQL or SQLite, all of these options will work
with `INSERT ON CONFLICT DO UPDATE` as well. See the [upsert docs](https://docs.diesel.rs/2.3.x/diesel/upsert/index.html) for more details.

## Executing your query

Once you’ve constructed your query, we need to actually execute it.
There are several different methods to do this, depending on what type
you’d like back.

The simplest method for running your query is [`execute`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html#tymethod.execute).
This method will run your query, and return the number of rows that were
affected. This is the method you should use if you simply want to ensure
that the query executed successfully, and don’t care about getting
anything back from the database.

For queries where you do want to get data back from the database, we
need to use [`get_result`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html#method.get_result)
or [`get_results`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html#method.get_results).
If you haven’t explicitly called [`returning`](https://docs.diesel.rs/2.3.x/diesel/query_builder/update_statement/struct.UpdateStatement.html#method.returning),
these methods will return all of the columns on the table. Similar to [`load`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html#method.load)
on a select statement, you will need to specify the type you’d like to
deserialize to (either a tuple or a struct with
`#[derive(Queryable)]`). You should use [`get_results`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html#method.get_results)
when you are expecting more than one record back. If you are only
expecting a single record, you can call [`get_result`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html#method.get_result)
instead.

It should be noted that receiving 0 rows from [`get_result`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html#method.get_result)
is considered an error condition by default. If you want to get back 0
or 1 row (e.g. have a return type of
`QueryResult>`), then you will need to
call `.get_result(...).optional()`.

Finally, if your struct has both `#[derive(AsChangeset)]`
and `#[derive(Identifiable)]`, you will be able to use the [`save_changes`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.SaveChangesDsl.html#method.save_changes)
method. Unlike the other methods mentioned in this guide, you do not
explicitly build a query when using [`save_changes`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.SaveChangesDsl.html#method.save_changes).
Doing `foo.save_changes(&conn)` is equivalent to doing
`diesel::update(&foo).set(&foo).get_result(&conn)`.
Like [`get_result`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html#method.get_result)
and [`get_results`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html#method.get_results),
you will need to specify the type you’d like to get back.

All of the code for this guide can be found in executable form in [this Diesel example](https://github.com/diesel-rs/diesel/blob/2.3.x/examples/postgres/all_about_updates/src/lib.rs).

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /guides/extending-diesel.html
- Keywords: diesel, rust, orm, guides, api, guides, extending diesel html
- Summary: Diesel provides a lot of capabilities out of the box. However, it
----

Source: https://diesel.rs/guides/extending-diesel.html

## Extending Diesel

Diesel provides a lot of capabilities out of the box. However, it
doesn’t necessarily provide everything your app may want to use. One of
Diesel’s greatest strengths is that it can be extended to add new
functionality.

In this guide we’ll look at several ways to hook into Diesel’s query
builder, both to add new capabilities, and to introduce new
abstractions.

This guide is only going to cover extending the query builder. How to
add support for new SQL types will be covered in a future guide.

## `sql_function!`

The easiest and most common way to extend Diesel’s query builder is
by declaring a new SQL function. This can be used for functions defined
by your database, or for built-in functions that Diesel doesn’t support
out of the box.

Functions in SQL often have multiple signatures, making them
difficult or impossible to represent in Rust. Because of this, Diesel
only provides support for a small number of built-in SQL functions.
Consider `COALESCE`. This function can take any number of
arguments, and its return type changes based on whether any arguments
are `NOT NULL`. While we can’t easily represent that in Rust,
we can use [`sql_function!`](https://docs.diesel.rs/2.3.x/diesel/expression/functions/macro.sql_function.html)
to declare it with the exact signature we’re using.

Example

```
use diesel::sql_types::{Nullable, Text};
sql_function! { fn coalesce(x: Nullable, y: Text) -> Text; }

users.select(coalesce(hair_color, "blue"))
```

As this example shows, [`sql_function!`](https://docs.diesel.rs/2.3.x/diesel/expression/functions/macro.sql_function.html)
converts its argument like other parts of the query builder. This means
that the generated function can take both Diesel expressions, and Rust
values to be sent with the query.

The macro takes one argument: a function definition. However, the
types in the function signature are SQL types, not concrete Rust types.
This is what allows us to pass both columns and Rust strings. If we
defined this function manually, it would look like this:

Example

```
pub fn coalesce(x: X, y: Y) -> coalesce::HelperType
where
    X: AsExpression>,
    Y: AsExpression,
{
    ...
}

pub(crate) mod coalesce {
    pub type HelperType = ...;
}
```

A helper type is generated with the same name as the function. This
helper type handles Diesel’s argument conversion. This lets us write
`coalesce`. More information can
be found in [the API documentation](https://docs.diesel.rs/2.3.x/diesel/expression/functions/macro.sql_function.html)

## Using Custom SQL and How to Extend the Query DSL

Often times it’s useful to encapsulate a common SQL pattern. For
example, if you’re doing pagination on your queries, PostgreSQL is
capable of loading the total count in a single query. The query you
would want to execute would look like this:

Example

```
SELECT *, COUNT(*) OVER () FROM (subselect t) as paged_query_with LIMIT $1 OFFSET $2
```

However, as of version 2.0, Diesel doesn’t support window functions,
or selecting from a subselect. Even if Diesel’s query builder supported
those things, this is a case that is easier to reason about in terms of
the SQL we want to generate.

Let’s look at how we would go about adding a `paginate`
method to Diesel’s query builder, to generate that query. Let’s assume
for the time being that we have a struct `Paginated`
already. We’ll look at the specifics of this struct shortly.

If you are creating a struct where you want to manually define the
SQL, you will need to implement a trait called [`QueryFragment`](https://docs.diesel.rs/2.3.x/diesel/query_builder/trait.QueryFragment.html).
The implementation will look like this:

src/pagination.rs

View on GitHub

```
impl QueryFragment
 for Paginated
where
    T: QueryFragment
,
{
    fn walk_ast(&'b self, mut out: AstPass) -> QueryResult {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") as paged_query_with LIMIT ");
        out.push_bind_param::(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::(&self.offset)?;
        Ok(())
    }
}
```

For details on what each method does, see the documentation for [`AstPass`](https://docs.diesel.rs/2.3.x/diesel/query_builder/struct.AstPass.html).
One important question to ask whenever you implement
`QueryFragment` is whether you are generating a query that is
safe to cache. The way to answer this question is by asking “does this
struct generate an unlimited number of potential SQL queries”? Typically
that is only the case if the body of `walk_ast` contains a
for loop. If your query is not safe to cache, you must call [`out.unsafe_to_cache_prepared`](https://docs.diesel.rs/2.3.x/diesel/query_builder/struct.AstPass.html#method.unsafe_to_cache_prepared).

Whenever you implement `QueryFragment`, you also need to
implement [`QueryId`](https://docs.diesel.rs/2.3.x/diesel/query_builder/trait.QueryId.html).
- We can use [`#[derive(QueryId)]`](https://docs.diesel.rs/2.3.x/diesel/query_builder/derive.QueryId.html)
for this. Since this struct represents a full query which can be
executed, we will implement [`RunQueryDsl`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html)
which adds methods like [`execute`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html#method.execute)
and [`load`](https://docs.diesel.rs/2.3.x/diesel/query_dsl/trait.RunQueryDsl.html#method.load).
Since this query has a return type, we’ll implement [`Query`](https://docs.diesel.rs/2.3.x/diesel/query_builder/trait.Query.html)
which states the return type as well.

src/pagination.rs

View on GitHub

```
impl Query for Paginated {
    type SqlType = (T::SqlType, BigInt);
}

impl RunQueryDsl
 for Paginated {}
```

Now that we’ve implemented all of these things, let’s look at how we
would go about constructing this. We’ll want to add a
`paginate` method to all Diesel queries, which specifies
which page we’re on, as well as a `per_page` method which
specifies the number of elements per page.

In order to add new methods to existing types, we can use a
trait.

src/pagination.rs

View on GitHub

```
pub trait Paginate: Sized {
    fn paginate(self, page: i64) -> Paginated;
}

impl Paginate for T {
    fn paginate(self, page: i64) -> Paginated {
        Paginated {
            query: self,
            per_page: DEFAULT_PER_PAGE,
            page,
            offset: (page - 1) * DEFAULT_PER_PAGE,
        }
    }
}

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated {
    query: T,
    page: i64,
    per_page: i64,
    offset: i64,
}

impl Paginated {
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated {
            per_page,
            offset: (self.page - 1) * per_page,
            ..self
        }
    }
}
```

Now we can get the third page of a query with 25 elements per page
like this:

Example

```
let results: Vec = users::table
        .paginate(3)
        .per_page(25)
        .get_results(&conn)
        .expect("error");
```

With this code, we could load any query into a
`Vec`, but we can do better. When doing
pagination, you usually want the records and the total number of pages.
We can write that method.

src/pagination.rs

View on GitHub

```
impl Paginated {
    pub fn load_and_count_pages(self, conn: &mut PgConnection) -> QueryResult, i64)>
    where
        Self: LoadQuery,
    {
        let per_page = self.per_page;
        let results = self.load::(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok((records, total_pages))
    }
}
```

This is one of the rare cases where we want to define a function that
takes a connection. One benefit of defining the function this way is
that if we wanted to support backends other than PostgreSQL, we could
have this function execute two queries.

You can find the full code for this example in [the “advanced blog” example](https://github.com/diesel-rs/diesel/tree/2.3.x/examples/postgres/advanced-blog-cli).

## Custom Operators

If you’re adding support for a new type to Diesel, or working with a
type that has incomplete support, you may wish to add support for the
operators associated with that type. The term operator refers to
anything that uses one of these syntaxes:

- Infix (e.g. `left OP right`)

- Prefix (e.g. `OP expr`)

- Postfix (e.g. `expr OP`)

Diesel provides helper macros for defining each of these kinds of
operators. In fact, Diesel uses these macros to declare nearly all of
the operators supported by the main crate. The macros are [`diesel::infix_operator!`](https://docs.diesel.rs/2.3.x/diesel/macro.infix_operator.html),
[`diesel::postfix_operator!`](https://docs.diesel.rs/2.3.x/diesel/macro.postfix_operator.html)
and [`diesel::prefix_operator!`](https://docs.diesel.rs/2.3.x/diesel/macro.prefix_operator.html).

All of these macros have the same signature. They take between two
and four arguments.

The first is the name of the struct you want to represent this
operator.

The second is the actual SQL for this operator.

The third argument is optional, and is the SQL type of the operator.
If the SQL type is not specified, it will default to `Bool`.
You can also pass the “magic” type `ReturnBasedOnArgs`, which
will cause the SQL type to be the same as the type of its arguments.
Diesel uses this to make the string concatenation operator
`||` return `Nullable` if the
arguments are nullable, or `Text` if they are not null.

The fourth argument (or third if you didn’t specify the SQL type) is
the backend this operator is used for. If you don’t specify a backend,
the operator can be used on all backends.

Let’s look at some example usage from Diesel:

Example

```
// A simple operator. It returns `Bool` and works on all backends.
diesel::infix_operator!(Eq, " = ");

// Here we've specified the SQL type.
// Since this operator is only used for ordering, and we don't want it used
// elsewhere, we've made it `()` which is normally useless.
diesel::postfix_operator!(Asc, " ASC", ());

// Concat uses the magic `ReturnBasedOnArgs` return type
// so it can work with both `Text` and `Nullable`.
diesel::infix_operator!(Concat, " || ", ReturnBasedOnArgs);

// This operator is PG specific, so we specify the backend
diesel::infix_operator!(IsDistinctFrom, " IS DISTINCT FROM ", backend: Pg);

// This operator is PG specific, and we are also specifying the SQL type.
diesel::postfix_operator!(NullsFirst, " NULLS FIRST", (), backend: Pg);
```

Diesel provides a proof-of-concept crate showing how to add new SQL
types called `diesel_full_text_search`. These are the
operators as they are defined in that crate. You’ll notice all of the
operators specify the backend, and many of them specify the return
type.

src/lib.rs

View on GitHub

```
diesel::infix_operator!(Matches, " @@ ", backend: Pg);
diesel::infix_operator!(Concat, " || ", TsVector, backend: Pg);
diesel::infix_operator!(And, " && ", TsQuery, backend: Pg);
diesel::infix_operator!(Or, " || ", TsQuery, backend: Pg);
diesel::infix_operator!(Contains, " @> ", backend: Pg);
diesel::infix_operator!(ContainedBy, " View on GitHub

```
pub trait ExpressionMethods: Expression + Sized {
    fn eq(self, other: T) -> dsl::Eq
    where
        Self::SqlType: SqlType,
        T: AsExpression,
    {
        Grouped(Eq::new(self, other.as_expression()))
    }
}

impl ExpressionMethods for T {}
```

It’s important to note that these methods are where you should put
any type constraints. The structs defined by
`diesel::*_operator!` don’t know or care about what the types
of the arguments should be. The `=` operator requires that
both sides be of the same type, so we represent that in the type of
`ExpressionMethods::eq`.

You’ll also notice that our argument is [`AsExpression`](https://docs.diesel.rs/2.3.x/diesel/expression/trait.AsExpression.html),
not [`Expression`](https://docs.diesel.rs/2.3.x/diesel/prelude/trait.Expression.html).
This allows Rust values to be passed as well as Diesel expressions. For
example, we can do `text_col.eq(other_text_col)`, or
`text_col.eq("Some Rust string")`.

If the operator is specific to only one SQL type, we can represent
that in our trait.

src/expression_methods/global_expression_methods.rs

View on GitHub

```
pub trait BoolExpressionMethods
where
    Self: Expression + Sized,
{
    fn and(self, other: T) -> dsl::And
    where
        Self::SqlType: SqlType,
        ST: SqlType + TypedExpressionType,
        T: AsExpression,
        And: Expression,
    {
        Grouped(And::new(self, other.as_expression()))
    }
}

impl BoolExpressionMethods for T
where
    T: Expression,
    T::SqlType: BoolOrNullableBool,
{
}
```

Prefix operators are usually defined as bare functions. The code is
very similar, but without the trait. Here’s how [`not`](https://docs.diesel.rs/2.3.x/diesel/dsl/fn.not.html)
is defined in Diesel.

src/expression/not.rs

View on GitHub

```
pub fn not(expr: T) -> not
where
    T: Expression,
    ::SqlType: BoolOrNullableBool,
{
    super::operators::Not::new(Grouped(expr))
}
```

In this case we’re using `Grouped` (which is currently
undocumented in Diesel and only used internally) to add parenthesis
around our argument. This ensures that the operator precedence in SQL
matches what’s expected. For example, we would expect
`not(true.and(false))` to return `true`. However,
`SELECT NOT TRUE AND FALSE` returns `FALSE`.
Diesel does the same thing for most of the built-in operators.

It’s also a best practice to expose a “helper type” for your method,
which does the same type conversion as the method itself. Nobody wants
to write
`Eq>::Expression>`.
Instead, we provide a type that lets you write [`Eq`](https://docs.diesel.rs/2.3.x/diesel/helper_types/type.Eq.html).

src/expression/helper_types.rs

View on GitHub

```
pub type Eq = Grouped>>;
```

For defining these types, you’ll usually want to make use of [`SqlTypeOf`](https://docs.diesel.rs/2.3.x/diesel/helper_types/type.SqlTypeOf.html),
[`AsExpr`](https://docs.diesel.rs/2.3.x/diesel/helper_types/type.AsExpr.html),
and [`AsExprOf`](https://docs.diesel.rs/2.3.x/diesel/helper_types/type.AsExprOf.html).

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

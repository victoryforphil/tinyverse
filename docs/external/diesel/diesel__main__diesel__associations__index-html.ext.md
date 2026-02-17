----
## External Docs Snapshot // diesel

- Captured: 2026-02-17T03:18:52.150Z
- Source root: https://diesel.rs/guides/
- Source page: /main/diesel/associations/index.html
- Keywords: diesel, rust, orm, guides, api, main, diesel, associations, index html
- Summary: [Source](../../src/diesel/associations/mod.rs.html#1-544)
----

Source: https://docs.diesel.rs/main/diesel/associations/index.html

[diesel](../index.html)

# Module associations

[Source](../../src/diesel/associations/mod.rs.html#1-544)

Expand description

Traits related to relationships between multiple tables.

Associations in Diesel are always child-to-parent.
You can declare an association between two records with `#[diesel(belongs_to)]`.
Unlike other ORMs, Diesel has no concept of `has many`

```
use schema::{posts, users};

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct User {
    id: i32,
    name: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = posts)]
pub struct Post {
    id: i32,
    user_id: i32,
    title: String,
}

let user = users.find(2).get_result::(connection)?;
let users_post = Post::belonging_to(&user).first(connection)?;
let expected = Post {
    id: 3,
    user_id: 2,
    title: "My first post too".into(),
};
assert_eq!(expected, users_post);
```

Note that in addition to the `#[diesel(belongs_to)]` annotation, we also need to
`#[derive(Associations)]`

`#[diesel(belongs_to)]` is given the name of the struct that represents the parent.
Both the parent and child must implement [`Identifiable`](trait.Identifiable.html).
The struct given to `#[diesel(belongs_to)]` must be in scope,
so you will need `use some_module::User` if `User` is defined in another module.

If the parent record is generic over lifetimes, they can be written as `'_`.
You will also need to wrap the type in quotes until
`unrestricted_attribute_tokens` is stable.

```
#[derive(Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    id: i32,
    name: Cow,
}

#[derive(Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = posts)]
pub struct Post {
    id: i32,
    user_id: i32,
    title: String,
}
```

By default, Diesel assumes that your foreign keys will follow the convention `table_name_id`.
If your foreign key has a different name,
you can provide the `foreign_key` argument to `#[diesel(belongs_to)]`.
For example, `#[diesel(belongs_to(Foo, foreign_key = mykey))]`.

Associated data is typically loaded in multiple queries (one query per table).
This is usually more efficient than using a join,
especially if 3 or more tables are involved.
For most datasets,
using a join to load in a single query transmits so much duplicate data
that it costs more time than the extra round trip would have.

You can load the children for one or more parents using
[`belonging_to`](../query_dsl/trait.BelongingToDsl.html#tymethod.belonging_to)

```
let user = users
    .find(1)
    .first::(connection)
    .expect("Error loading user");
let post_list = Post::belonging_to(&user)
    .load::
(connection)
    .expect("Error loading posts");
let expected = vec![
    Post {
        id: 1,
        user_id: 1,
        title: "My first post".to_string(),
    },
    Post {
        id: 2,
        user_id: 1,
        title: "About Rust".to_string(),
    },
];

assert_eq!(post_list, expected);
```

If you’re coming from other ORMs, you’ll notice that this design is quite different from most.
There you would have an instance method on the parent, or have the children stored somewhere on
the posts. This design leads to many problems, including [N+1 query bugs](https://stackoverflow.com/q/97197/1254484), and runtime errors when accessing an
association that isn’t there.

In Diesel, data and its associations are considered to be separate. If you want to pass around
a user and all of its posts, that type is `(User, Vec
)`.

Next lets look at how to load the children for more than one parent record.
[`belonging_to`](../query_dsl/trait.BelongingToDsl.html#tymethod.belonging_to) can be used to load the data, but we’ll also need to group it
with its parents. For this we use an additional method [`grouped_by`](trait.GroupedBy.html#tymethod.grouped_by).

```
let sean = users.filter(name.eq("Sean")).first::(connection)?;
let tess = users.filter(name.eq("Tess")).first::(connection)?;

let seans_posts = Post::belonging_to(&sean)
    .select(title)
    .load::(connection)?;
assert_eq!(vec!["My first post", "About Rust"], seans_posts);

// A vec or slice can be passed as well
let more_posts = Post::belonging_to(&vec![sean, tess])
    .select(title)
    .load::(connection)?;
assert_eq!(
    vec!["My first post", "About Rust", "My first post too"],
    more_posts
);
```

Typically you will want to group up the children with their parents.
In other ORMs, this is often called a `has_many` relationship.
Diesel provides support for doing this grouping, once the data has been
loaded.

[`grouped_by`](trait.GroupedBy.html#tymethod.grouped_by) is called on a `Vec` with a `&[Parent]`.
The return value will be `Vec>` indexed to match their parent.
Or to put it another way, the returned data can be passed to `zip`,
and it will be combined with its parent.

```
let users = users::table.load::(connection)?;
let posts = Post::belonging_to(&users)
    .load::
(connection)?
    .grouped_by(&users);
let data = users.into_iter().zip(posts).collect::>();

let expected_data = vec![
    (
        User {
            id: 1,
            name: "Sean".into(),
        },
        vec![
            Post {
                id: 1,
                user_id: 1,
                title: "My first post".into(),
            },
            Post {
                id: 2,
                user_id: 1,
                title: "About Rust".into(),
            },
        ],
    ),
    (
        User {
            id: 2,
            name: "Tess".into(),
        },
        vec![Post {
            id: 3,
            user_id: 2,
            title: "My first post too".into(),
        }],
    ),
];

assert_eq!(expected_data, data);
```

[`grouped_by`](trait.GroupedBy.html#tymethod.grouped_by) can be called multiple times
if you have multiple children or grandchildren.

For example, this code will load some users,
all of their posts,
and all of the comments on those posts.
Explicit type annotations have been added
to make each line a bit more clear.

```
let users: Vec = users::table
    .load::(connection)
    .expect("error loading users");
let posts: Vec
 = Post::belonging_to(&users)
    .load::
(connection)
    .expect("error loading posts");
let comments: Vec = Comment::belonging_to(&posts)
    .load::(connection)
    .expect("Error loading comments");
let grouped_comments: Vec> = comments.grouped_by(&posts);
let posts_and_comments: Vec)>> =
    posts.into_iter().zip(grouped_comments).grouped_by(&users);
let result: Vec)>)> =
    users.into_iter().zip(posts_and_comments).collect();
let expected = vec![
    (
        User {
            id: 1,
            name: "Sean".to_string(),
        },
        vec![
            (
                Post {
                    id: 1,
                    user_id: 1,
                    title: "My first post".to_string(),
                },
                vec![Comment {
                    id: 1,
                    post_id: 1,
                    body: "Great post".to_string(),
                }],
            ),
            (
                Post {
                    id: 2,
                    user_id: 1,
                    title: "About Rust".to_string(),
                },
                vec![Comment {
                    id: 2,
                    post_id: 2,
                    body: "Yay! I am learning Rust".to_string(),
                }],
            ),
        ],
    ),
    (
        User {
            id: 2,
            name: "Tess".to_string(),
        },
        vec![(
            Post {
                id: 3,
                user_id: 2,
                title: "My first post too".to_string(),
            },
            vec![Comment {
                id: 3,
                post_id: 3,
                body: "I enjoyed your post".to_string(),
            }],
        )],
    ),
];

assert_eq!(result, expected);
```

And that’s it.
It may seem odd to have load, group, and zip be explicit separate steps
if you are coming from another ORM.
However, the goal is to provide simple building blocks which can
be used to construct the complex behavior applications need.

## Structs[§](#structs)

[TryGroupedByError](struct.TryGroupedByError.html)A type of error which can be returned when attempting to group
a list of records.

## Traits[§](#traits)

[BelongsTo](trait.BelongsTo.html)Indicates that a type belongs to `Parent`[GroupedBy](trait.GroupedBy.html)The `grouped_by` function groups records by their parent.[HasTable](trait.HasTable.html)This trait indicates that a struct is associated with a single database table.[Identifiable](trait.Identifiable.html)This trait indicates that a struct represents a single row in a database table.

## Derive Macros[§](#derives)

[Associations](derive.Associations.html)Implement required traits for the associations API[Identifiable](derive.Identifiable.html)Implements `Identifiable` for references of the current type

----
## Notes / Comments / Lessons

- Collection method: sitemap-first guide discovery with seed URLs and API index fallback.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

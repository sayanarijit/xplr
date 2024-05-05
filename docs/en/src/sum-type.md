# Sum Type

> This section isn't specific to xplr. However, since xplr configuration makes
> heavy use of this particular data type, even though it isn't available in
> most of the mainstream programming languages (yet), making it a wild or
> unfamiliar concept for many, it's worth doing a quick introduction here.
>
> If you're already familiar with [Sum Type / Tagged Union][1] (e.g. Rust's
> enum), you can skip ahead.

While reading this doc, you'll come across some data types like [Layout][2],
[Color][4], [Message][3] etc. that says something like "x is a sum type that
can be any of the following", and then you'll see a list of strings and/or lua
tables just below.

Yes, they are actually sum types, i.e. they can be any of the given set of
tagged variants listed there.

Notice the word "be". Unlike classes or structs (aka product types), they can't
"have" values, they can only "be" the value, or rather, be one of the possible
set of values.

Also notice the word "tagged". Unlike the single variant `null`, or the dual
variant `boolean` types, the variants of sum types are tagged (i.e. named), and
may further have, or be, value or values of any data type.

A simple example of a sum type is an enum. Many programming languages have
them, but only a few modern programming languages allow nesting other types
into a sum type.

```rust
enum Color {
    Red,
    Green,
}
```

Here, `Color` can be one of two possible set of values: `Red` and `Green`, just
like `boolean`, but unlike `boolean`, being tagged allows `Color` to have more
than two variants if required, by changing the definition.

e.g.

```rust
enum Color {
    Red,
    Green,
    Blue,
}
```

We'd document it here as:

> Result is a sum type that can be one of the following:
>
> - "Red"
> - "Green"
> - "Blue"

But some languages (like Rust, Haskell, Elm etc.) go even further, allowing us
to associate each branch of the enum with further nested types like:

```rust
enum Layout {
    Table,
    HelpMenu,
    Horizontal {
        config: LayoutConfig,  // A product type (similar to class/struct)
        splits: Vec<Layout>  // A list of "Layout"s (i.e. list of sum types)
    },
}
```

Here, as we can see, unlike the first example, some of `Layout`'s possible
variants can have further nested types associated with them. Note that
`Horizontal` here can have a sum type (e.g. enum), or a product type (e.g.
class/struct), or both (any number of them actually) nested in it. But the
nested values will only exist when `Layout` is `Horizontal`.

We'd document it here as:

> Layout is a sum type that can be one of the following:
>
> - "Table"
> - "HelpMenu"
> - { Horizontal = { config = Layout Config, splits = { Layout, ... } }

And then we'd go on documenting whatever `Layout Config` is.

So, there you go. This is exactly what sum types are - glorified enums that can
have nested types in each branch.

[1]: https://en.wikipedia.org/wiki/Tagged_union
[2]: layout.md
[3]: message.md
[4]: style.md#color

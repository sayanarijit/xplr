# Sum Type

> This section isn't specific to xplr. However, since xplr configuration makes
> heavy use of this particular data type, even though it isn't available in
> most of the mainstream programming languages (yet), making it a wild or
> unfamilier concept for many, it's worth doing a quick introduction here.
>
> If you're already familiar with [Sum Type / Tagged Union][1] (e.g. Rust's
> enum), you can skip ahead.

While reading this doc, you'll come across some data types like [Layout][2],
[Color][4], [Message][3] etc. that says something like "x can be any of the
following", and then you'll see a list of strings and/or lua tables just below.

Yes, they are actually sum types, i.e. they can be any of the finite set of
values listed there.

Note the word "finite". Many conventional programming languages have only two
kinds of data types with finite set of values:

- `null` - aka "unit type", can be only one thing - i.e. `null`.
- `bool` - can be one of the two possible values - `true` or `false`.

Other types like `integer`, `float`, `string` etc. can have infinite number of
possible values (ideally).

Many popular programming languages come with the concept of `enum`, which lets
us define a type with finite set of possible values like:

```rust
enum Result {
    Ok,
    Err,
}
```

Here, `Result` has only two possible values (just like `bool`): `Ok` and `Err`.

We'd document it here as:

> Result is a sum type that can be one of the following:
>
> - "Ok"
> - "Err"

But some languages (like Rust, Haskell, Elm etc.) go even further, allowing us
to associate each branch of the enum with further nested types like:

```rust
enum Result {
    Ok(bool),
    Err(String),
}
```

We'd document it here as:

> Result is a sum type that can be one of the following:
>
> - { Ok = bool }
> - { Err = "string" }

Here, `Result` still has only two possibilities, but unlike bool, each
possibility here has further finite or infinite set of possible values.

And there you go. This is exactly what sum types are - glorified enums that can
have nested types in each branch.

[1]: https://en.wikipedia.org/wiki/Tagged_union
[2]: layout.md
[3]: message.md
[4]: style.md#color

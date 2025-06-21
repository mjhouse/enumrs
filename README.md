# Overview

This crate provides a derive macro called `Tagged` and a `tag` attribute for enums so that data can be associated with each variant. The derive macro will automatically generate methods to access the associated tag data. This allows us to avoid writing long match functions.

## Getting started

Install the crate using cargo:

```bash
cargo add enumrs
```

Or by updating your Cargo.toml:

```toml
[dependencies]
enumrs = "0.2.1"
```

Derive the `Tagged` macro for your enum, and associate data with the `tag` attribute:

```rust
use enumrs::Tagged;

#[derive(Tagged)]
pub enum Country {

    #[tag(name, "Afghanistan")]
    #[tag(description, "Description of Afghanistan")]
	AFG = 1,

    #[tag(name, "Albania")]
    #[tag(description, "Description of Albania")]
	ALB = 2,

    // ...
}
```

Access the associated data using the generated functions:

```rust
use enumrs::Tagged;

#[derive(Tagged)]
pub enum Country {

    #[tag(name, "Afghanistan")]
    #[tag(description, "Description of Afghanistan")]
	AFG = 1,

    #[tag(name, "Albania")]
    #[tag(description, "Description of Albania")]
	ALB = 2,

    // ...
}

let variant = Country::AFG;
let name = variant.name();
assert_eq!(name,Some("Afghanistan"));
```

## Detailed usage

In a `tag` declaration, the first value must be a plain identifier without quotations. This is the tag name, and it's required. Everything that comes after the name is executed as an expression. Expressions must be relatively simple mathematical expressions or bare values.

### Examples

#### Right

Return type for `name()` will be `Option<&'static str>`, and it will return `Some("Name")`:

```rust
use enumrs::Tagged;

#[derive(Tagged)]
pub enum MyEnum {
    #[tag( name, "Name" )]
    Variant1,
}
```

Return type for `offset()` will be `Option<f64>` and it will return `Some(0.45)`:

```rust
use enumrs::Tagged;

#[derive(Tagged)]
pub enum MyEnum {
    #[tag( offset, 0.45 )]
    Variant2,
}
```

The return type for `total_width()` will be `Option<i64>` and it will return `Some(7)`:

```rust
use enumrs::Tagged;

#[derive(Tagged)]
pub enum MyEnum {
    #[tag( width, 5 )]
    #[tag( padding, 1 )]
    #[tag( total_width, width + (padding * 2) )]
    Variant3,
}
```

#### Wrong

Can't evaluate because 'String' is not in scope at compile time:

```compile_fail
use enumrs::Tagged;

#[derive(Tagged)]
pub enum MyEnum {
    #[tag( name, String::from("Name") )]
    Variant4,
}
```

Can't evaluate because 'my_custom_func' is not in scope at compile time:

```compile_fail
use enumrs::Tagged;

#[derive(Tagged)]
pub enum MyEnum {
    #[tag( name, my_custom_func() )]
    Variant5,
}
```

No tag with the name 'other' is defined:

```compile_fail
use enumrs::Tagged;

#[derive(Tagged)]
pub enum MyEnum {
    // #[tag(other, 1)]
    #[tag( name, other + 3 )]
    Variant6,
}
```

### Types

The result of the expression must be one of the following simple types, and will be returned from functions as the associated rust value type, wrapped in an `Option`. Any variants that don't have a particular attribute will return `None`.

| Simple type | Rust type    | Return type            |
| --          | --           | --                     |
| Float       | f64          | `Option<f64>`          |
| Integer     | i64          | `Option<i64>`          |
| String      | &'static str | `Option<&'static str>` |
| Boolean     | bool         | `Option<bool>`         | 

### Operators

The expressions in the `tag` attributes can use any of the operators available from the [evalexpr](https://github.com/ISibboI/evalexpr?tab=readme-ov-file#operators) crate. The only caveat is that the context for the expression will be empty at evaluation time *except* for other attributes on the same enum variant.

So this works:

```rust
use enumrs::Tagged;

#[derive(Tagged)]
pub enum MyEnum {
    #[tag( id, 3 )]
    #[tag( index, id - 1 )]
    Variant,
}
```

But this does not:

```compile_fail
use enumrs::Tagged;

pub const VALUE: i64 = 3;

#[derive(Tagged)]
pub enum MyEnum {
    #[tag( id, VALUE + 1 )]
    Variant1,
    // ...
}
```

# Contributing

Anyone is welcome to contribute. It's a small crate, so your contributions are likely to have a large impact on the future of this library. I'll review and discuss any pull requests, but there may be a bit of a delay. Don't hesitate to ping me over and over for a review until I respond.

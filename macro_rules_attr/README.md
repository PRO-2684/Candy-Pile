# `macro_rules_attr`

[![GitHub License](https://img.shields.io/github/license/PRO-2684/macro_rules_attr?logo=opensourceinitiative)](https://github.com/PRO-2684/macro_rules_attr/blob/main/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/macro_rules_attr?logo=rust)](https://crates.io/crates/macro_rules_attr)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/macro_rules_attr?logo=rust)](https://crates.io/crates/macro_rules_attr)
[![docs.rs](https://img.shields.io/docsrs/macro_rules_attr?logo=rust)](https://docs.rs/macro_rules_attr)

Use declarative macros as proc_macro attributes. (`#[apply]` your `macro_rules!`)

## Usage

First, bring the `apply` attribute macro into scope:

```rust
use macro_rules_attr::apply;
```

Then, define your macro using `macro_rules!`:

```rust
# use macro_rules_attr::apply;
#
/// Your macro that you want to use as an attribute.
macro_rules! the_macro {
    // Your macro implementation here.
#     ($($tt:tt)*) => {}; // Matches and discards everything.
}
```

Finally, annotate your item with the `apply` attribute macro:

```rust
# use macro_rules_attr::apply;
#
# /// Your macro that you want to use as an attribute.
# macro_rules! the_macro {
#     // Your macro implementation here.
#     ($($tt:tt)*) => {}; // Matches and discards everything.
# }
#
#[apply(the_macro)]
struct One {}
// Expands to:
the_macro! { struct One {} }
```

Additional tokens are appended after the annotated item:

```rust
# use macro_rules_attr::apply;
#
# /// Your macro that you want to use as an attribute.
# macro_rules! the_macro {
#     // Your macro implementation here.
#     ($($tt:tt)*) => {}; // Matches and discards everything.
# }
#
#[apply(the_macro, "additional tokens", anything, (you - like))]
struct Another {}
// Expands to:
the_macro! { struct Another {}, "additional tokens", anything, (you - like) }
```

## Example

```rust
use macro_rules_attr::apply;

/// Simple macro that generates a `hello` function for given struct, which returns `Hello, {name}!`. If given a second argument, it will replace `{name}`.
macro_rules! make_hello {
( // Matches a struct definition (some details omitted for brevity)
#     $(#[$struct_meta:meta])*
#     $struct_vis:vis
    struct $StructName:ident {
        // ...
#         $(
#             $(#[$field_meta:meta])*
#             $field_vis:vis
#             $field_name:ident : $field_ty:ty
#         ),* $(,)?
    }$(, $replacement:expr)?
) => {
    // Repeat the struct definition
#     $(#[$struct_meta])*
#     $struct_vis
    struct $StructName {
        // ...
#         $(
#             $(#[$field_meta])*
#             $field_vis:vis $field_name: $field_ty,
#         )*
    }

    // Implement the `hello` function
    impl $StructName {
        fn hello() -> String {
            let name = stringify!($StructName);
            $(let name = $replacement;)? // Shadow `name` if a replacement was provided
            format!("Hello, {name}!")
        }
    }
};
}

#[apply(make_hello)] // No additional tokens
struct WithoutReplacement {}
assert_eq!(WithoutReplacement::hello(), "Hello, WithoutReplacement!");

#[apply(make_hello, "World")] // Additional tokens
struct WithReplacement {}
assert_eq!(WithReplacement::hello(), "Hello, World!");
```

## Cargo Features

- `log`: Enable logging with the `log` crate. (Requires `log` as a dependency)

## Comparison

This crate is heavily inspired by [macro_rules_attribute](https://crates.io/crates/macro_rules_attribute), but differs in the following ways:

- `macro_rules_attr` is more lightweight and has no dependencies by default.
    - Less than 100 lines of code. (Excluding tests and documentation)
    - You can enable logging with the `log` feature, which requires `log` as a dependency.
    - `paste` is required as a dev-dependency for the tests.
- `macro_rules_attr` only has one attribute: `#[apply]`, while `macro_rules_attribute` provides more.
- `macro_rules_attr` allows you to append any tokens after the annotated item, while `macro_rules_attribute` does not.

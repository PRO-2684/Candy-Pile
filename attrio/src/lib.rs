#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::cargo)]
#![allow(clippy::test_attr_in_doctest, reason = "For demonstration purposes")]

/// Generate a macro that annotates items with given attributes. See the [crate-level documentation](crate) for more details.
///
/// ---
///
/// The dollar hack is attributed to [rust#95860](https://github.com/rust-lang/rust/pull/95860).
#[macro_export]
macro_rules! attrio {(
    $dollar:tt $macro_name:ident,
    $(#[$attr:meta])*
) => {
    macro_rules! $macro_name {(
        $dollar item:item
    ) => {
        $(#[$attr])*
        $dollar item
    }}
}}

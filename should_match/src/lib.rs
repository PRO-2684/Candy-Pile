#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::cargo)]
#![allow(clippy::test_attr_in_doctest, reason = "For demonstration purposes")]

// `should_match` and `test_match`

/// Wraps a function that takes nothing and returns something, panicking if the result does not match the expected pattern.
///
/// See the [crate-level documentation](crate) for more information.
#[macro_export]
macro_rules! should_match {(
    $(#[$attr:meta])*
    $vis:vis fn $name:ident() -> $ret_ty:ty $body:block,
    pattern = $pattern:pat
    $(, message = $message:literal)? $(,)?
) => {
    $(#[$attr])*
    $vis fn $name() {
        fn inner() -> $ret_ty $body
        let result = inner();
        let is_match = ::core::matches!(result, $pattern);
        let message = ::core::concat!("Expected to match `", stringify!($pattern), "`");
        // Shadow the message if it was provided
        $(let message = $message;)?
        ::core::assert!(is_match, "{message}");
    }
}}

/// [`should_match!`] + `#[test]`.
#[macro_export]
macro_rules! test_match {(
    $($target:tt)*
) => {
    $crate::should_match! {
        #[test]
        $($target)*
    }
}}

// Helper macro

/// Makes a pair of macros, built on top of [`should_match!`] and [`test_match`]. Used internally.
///
/// ## Arguments
///
/// - `$should_macro_name`: The name of the `should_*` macro.
/// - `$dollar`: The dollar sign `$`.
/// - `$test_macro_name`: The name of the `test_*` macro.
/// - `$full_pattern`: The full pattern to match.
/// - `$display_pattern`: The pattern to display in the documentation.
/// - `$message`: The message to display if the pattern does not match.
/// - `$footnote`: A footnote to display in the documentation. Optional.
///
/// ---
///
/// The dollar hack is attributed to [rust#95860](https://github.com/rust-lang/rust/pull/95860).
macro_rules! make_macro_pair {(
    $should_macro_name:ident $dollar:tt $test_macro_name:ident,
    $full_pattern:pat,
    $display_pattern:literal,
    $message:literal
    $(, $footnote:literal)?
    $(,)?
) => {
    #[doc = "Shortcut for [`should_match!`], with:"]
    #[doc = "### Pattern"]
    #[doc = "```rust ignore"]
    #[doc = $display_pattern]
    #[doc = "```"]
    #[doc = "### Message"]
    #[doc = "```text"]
    #[doc = $message]
    #[doc = "```"]
    $(#[doc = "---"] #[doc = $footnote])?
    #[macro_export]
    macro_rules! $should_macro_name {(
        $dollar($target:tt)*
    ) => {
        $crate::should_match! {
            $dollar($target)*,
            pattern = $full_pattern,
            message = $message
        }
    }}

    #[doc = "Shortcut for [`test_match!`], with:"]
    #[doc = "### Pattern"]
    #[doc = "```rust ignore"]
    #[doc = $display_pattern]
    #[doc = "```"]
    #[doc = "### Message"]
    #[doc = "```text"]
    #[doc = $message]
    #[doc = "```"]
    $(#[doc = "---"] #[doc = $footnote])?
    #[macro_export]
    macro_rules! $test_macro_name {(
        $dollar($target:tt)*
    ) => {
        $crate::test_match! {
            $dollar($target)*,
            pattern = $full_pattern,
            message = $message
        }
    }}
}}

// Shortcuts for common patterns

make_macro_pair! {
    should_ok $ test_ok,
    ::core::result::Result::Ok(_),
    "Ok(_)",
    "Expected `Ok`, but got `Err`",
    "You probably don't need this, since this is supported by `#[test]` directly. It exists solely for consistency."
}

make_macro_pair! {
    should_err $ test_err,
    ::core::result::Result::Err(_),
    "Err(_)",
    "Expected `Err`, but got `Ok`"
}

make_macro_pair! {
    should_some $ test_some,
    ::core::option::Option::Some(_),
    "Some(_)",
    "Expected `Some`, but got `None`"
}

make_macro_pair! {
    should_none $ test_none,
    ::core::option::Option::None,
    "None",
    "Expected `None`, but got `Some`"
}

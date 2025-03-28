#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::cargo)]
#![allow(clippy::test_attr_in_doctest, reason = "For demonstration purposes")]

// `should_*` macros

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

/// Wraps a function that takes nothing and returns something, panicking if the result is not `Ok`. Shortcut for [`should_match!`].
///
/// ```rust ignore
/// // This macro invocation:
/// should_ok! { ... }
/// // Is equivalent to:
/// should_match! { ..., pattern = Ok(_), message = "Expected `Ok`, but got `Err`" }
/// ```
///
/// You probably don't need this, since this is supported by `#[test]` directly. It exists solely for consistency.
#[macro_export]
macro_rules! should_ok {(
    $($target:tt)*
) => {
    $crate::should_match! {
        $($target)*,
        pattern = ::core::result::Result::Ok(_),
        message = "Expected `Ok`, but got `Err`"
    }
}}

/// Wraps a function that takes nothing and returns something, panicking if the result is not `Err`. Shortcut for [`should_match!`].
///
/// ```rust ignore
/// // This macro invocation:
/// should_err! { ... }
/// // Is equivalent to:
/// should_match! { ..., pattern = Err(_), message = "Expected `Err`, but got `Ok`" }
/// ```
#[macro_export]
macro_rules! should_err {(
    $($target:tt)*
) => {
    $crate::should_match! {
        $($target)*,
        pattern = ::core::result::Result::Err(_),
        message = "Expected `Err`, but got `Ok`"
    }
}}

/// Wraps a function that takes nothing and returns something, panicking if the result is not `Some`. Shortcut for [`should_match!`].
///
/// ```rust ignore
/// // This macro invocation:
/// should_some! { ... }
/// // Is equivalent to:
/// should_match! { ..., pattern = Some(_), message = "Expected `Some`, but got `None`" }
/// ```
#[macro_export]
macro_rules! should_some {(
    $($target:tt)*
) => {
    $crate::should_match! {
        $($target)*,
        pattern = ::core::option::Option::Some(_),
        message = "Expected `Some`, but got `None`"
    }
}}

/// Wraps a function that takes nothing and returns something, panicking if the result is not `None`. Shortcut for [`should_match!`].
///
/// ```rust ignore
/// // This macro invocation:
/// should_none! { ... }
/// // Is equivalent to:
/// should_match! { ..., pattern = None, message = "Expected `None`, but got `Some`" }
/// ```
#[macro_export]
macro_rules! should_none {(
    $($target:tt)*
) => {
    $crate::should_match! {
        $($target)*,
        pattern = ::core::option::Option::None,
        message = "Expected `None`, but got `Some`"
    }
}}

// `test_*` macros

/// Wraps a function that takes nothing and returns something, failing the test if the result is not `Ok`. Shortcut for [`should_match!`].
///
/// ```rust ignore
/// // This macro invocation:
/// test_match! { ... }
/// // Is equivalent to:
/// should_match! { #[test] ... }
/// ```
#[macro_export]
macro_rules! test_match {(
    $($target:tt)*
) => {
    $crate::should_match! {
        #[test]
        $($target)*
    }
}}

/// Wraps a function that takes nothing and returns something, failing the test if the result is not `Ok`. Shortcut for [`should_ok!`].
///
/// ```rust ignore
/// // This macro invocation:
/// test_ok! { ... }
/// // Is equivalent to:
/// should_ok! { #[test] ... }
/// ```
///
/// You probably don't need this, since this is supported by `#[test]` directly. It exists solely for consistency.
#[macro_export]
macro_rules! test_ok {(
    $($target:tt)*
) => {
    $crate::should_ok! {
        #[test]
        $($target)*
    }
}}

/// Wraps a function that takes nothing and returns something, failing the test if the result is not `Err`. Shortcut for [`should_err!`].
///
/// ```rust ignore
/// // This macro invocation:
/// test_err! { ... }
/// // Is equivalent to:
/// should_err! { #[test] ... }
/// ```
#[macro_export]
macro_rules! test_err {(
    $($target:tt)*
) => {
    $crate::should_err! {
        #[test]
        $($target)*
    }
}}

/// Wraps a function that takes nothing and returns something, failing the test if the result is not `Some`. Shortcut for [`should_some!`].
///
/// ```rust ignore
/// // This macro invocation:
/// test_some! { ... }
/// // Is equivalent to:
/// should_some! { #[test] ... }
/// ```
#[macro_export]
macro_rules! test_some {(
    $($target:tt)*
) => {
    $crate::should_some! {
        #[test]
        $($target)*
    }
}}

/// Wraps a function that takes nothing and returns something, failing the test if the result is not `None`. Shortcut for [`should_none!`].
///
/// ```rust ignore
/// // This macro invocation:
/// test_none! { ... }
/// // Is equivalent to:
/// should_none! { #[test] ... }
/// ```
#[macro_export]
macro_rules! test_none {(
    $($target:tt)*
) => {
    $crate::should_none! {
        #[test]
        $($target)*
    }
}}

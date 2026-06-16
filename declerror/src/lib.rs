#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

/// Impl `Display` and `Error` for an enum.
#[macro_export]
macro_rules! error_enum {
    (
        $(#[$enum_attr:meta])*
        $vis:vis enum $name:ident {
            $(
                #[error = $doc:literal]
                $(#[$variant_attr:meta])*
                $variant:ident $({ $($field:ident : $field_ty:ty),* $(,)? })?
            ),* $(,)?
        }
    ) => {
        $(#[$enum_attr])*
        #[derive(::core::fmt::Debug)]
        $vis enum $name {
            $(
                $(#[$variant_attr])*
                $variant $({ $($field : $field_ty),* })?
            ),*
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(
                        $crate::error_enum_pat!($variant $({ $($field : $field_ty),* })?) => $crate::error_enum_fmt!($variant $({ $($field : $field_ty),* })?, formatter, $doc)
                        // Have to use two helper macros, because Rust macros cannot expand to incomplete AST nodes: https://github.com/rust-lang/rust/issues/12832#issuecomment-408640734
                    ),*
                }
            }
        }

        impl ::core::error::Error for $name {}
    };
}

/// Helper macro to match an enum variant.
#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_pat {
    // Unit variant
    ($variant:ident) => {
        Self::$variant
    };
    // Struct variant with named fields
    ($variant:ident { $($field:ident : $field_ty:ty),* }) => {
        Self::$variant { $($field),* }
    };
}

/// Helper macro to format an enum variant.
#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_fmt {
    // Unit variant
    ($variant:ident, $formatter:ident, $doc:literal) => {
        ::core::write!($formatter, $doc)
    };
    // Struct variant with named fields
    ($variant:ident { $($field:ident : $field_ty:ty),* }, $formatter:ident, $doc:literal) => {
        ::core::write!($formatter, $doc, $($field = $field),*)
    };
}

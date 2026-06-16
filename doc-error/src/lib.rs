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
                #[doc = $doc:literal]
                $(#[$variant_attr:meta])*
                $variant:ident $({ $($field:ident : $field_ty:ty),* $(,)? })?
            ),* $(,)?
        }
    ) => {
        $(#[$enum_attr])*
        #[derive(::core::fmt::Debug)]
        $vis enum $name {
            $(
                #[doc = $doc]
                $(#[$variant_attr])*
                $variant $({ $($field : $field_ty),* })?
            ),*
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(
                        #[doc = $doc]
                        $crate::error_enum_pat!($variant $({ $($field : $field_ty),* })?) => $crate::error_enum_fmt!($variant $({ $($field : $field_ty),* })?, formatter, $doc)
                    ),*
                }
            }
        }

        impl ::core::error::Error for $name {}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_pat {
    ($variant:ident { $($field:ident : $field_ty:ty),* }) => {
        Self::$variant { $($field),* }
    };
    ($variant:ident) => {
        Self::$variant
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_fmt {
    ($variant:ident { $($field:ident : $field_ty:ty),* }, $formatter:ident, $doc:literal) => {
        ::core::write!($formatter, $doc, $($field = $field),*)
    };
    ($variant:ident, $formatter:ident, $doc:literal) => {
        ::core::write!($formatter, $doc)
    };
}

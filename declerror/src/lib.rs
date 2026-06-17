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
                $variant:ident
                $({ $($field:ident : $field_ty:ty),* $(,)? })?
                $(($($tuple_field_ty:ty),* $(,)?))?
            ),* $(,)?
        }
    ) => {
        $(#[$enum_attr])*
        #[derive(::core::fmt::Debug)]
        $vis enum $name {
            $(
                $(#[$variant_attr])*
                $variant
                $({ $($field : $field_ty),* })?
                $(($($tuple_field_ty),*))?
            ),*
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(
                        $crate::error_enum_pat!(
                            $variant
                            $({ $($field : $field_ty),* })?
                            $(($($tuple_field_ty),*))?;
                            [
                                field_0, field_1, field_2, field_3, field_4, field_5,
                                field_6, field_7, field_8, field_9, field_10, field_11
                            ]
                        ) => $crate::error_enum_fmt!(
                            $variant
                            $({ $($field : $field_ty),* })?
                            $(($($tuple_field_ty),*))?;
                            [
                                field_0, field_1, field_2, field_3, field_4, field_5,
                                field_6, field_7, field_8, field_9, field_10, field_11
                            ],
                            formatter,
                            $doc
                        )
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
    ($variant:ident; [$($tuple_field:ident),*]) => {
        Self::$variant
    };
    // Struct variant with named fields
    ($variant:ident { $($field:ident : $field_ty:ty),* }; [$($tuple_field:ident),*]) => {
        Self::$variant { $($field),* }
    };
    // Tuple variant with unnamed fields
    ($variant:ident ($($field_ty:ty),*); [$($tuple_field:ident),*]) => {
        $crate::error_enum_tuple_pat!($variant; [$($tuple_field),*]; $($field_ty),*)
    };
}

/// Helper macro to match a tuple enum variant.
#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_tuple_pat {
    ($variant:ident; [$($tuple_field:ident),*]; $($field_ty:ty),*) => {
        $crate::error_enum_tuple_pat!(@collect $variant; []; [$($tuple_field),*]; $($field_ty),*)
    };
    (@collect $variant:ident; [$($field:ident,)*]; [$next_field:ident $(, $tuple_field:ident)*]; $field_ty:ty $(, $rest_ty:ty)*) => {
        $crate::error_enum_tuple_pat!(
            @collect
            $variant;
            [$($field,)* $next_field,];
            [$($tuple_field),*];
            $($rest_ty),*
        )
    };
    (@collect $variant:ident; [$($field:ident,)*]; [$($tuple_field:ident),*];) => {
        Self::$variant($($field),*)
    };
}

/// Helper macro to format an enum variant.
#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_fmt {
    // Unit variant
    ($variant:ident; [$($tuple_field:ident),*], $formatter:ident, $doc:literal) => {
        ::core::write!($formatter, $doc)
    };
    // Struct variant with named fields
    ($variant:ident { $($field:ident : $field_ty:ty),* }; [$($tuple_field:ident),*], $formatter:ident, $doc:literal) => {
        ::core::write!($formatter, $doc, $($field = $field),*)
    };
    // Tuple variant with unnamed fields
    ($variant:ident ($($field_ty:ty),*); [$($tuple_field:ident),*], $formatter:ident, $doc:literal) => {
        $crate::error_enum_tuple_fmt!($formatter, $doc; [$($tuple_field),*]; $($field_ty),*)
    };
}

/// Helper macro to format a tuple enum variant.
#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_tuple_fmt {
    ($formatter:ident, $doc:literal; [$($tuple_field:ident),*]; $($field_ty:ty),*) => {
        $crate::error_enum_tuple_fmt!(@collect $formatter, $doc; []; [$($tuple_field),*]; $($field_ty),*)
    };
    (@collect $formatter:ident, $doc:literal; [$($field:ident,)*]; [$next_field:ident $(, $tuple_field:ident)*]; $field_ty:ty $(, $rest_ty:ty)*) => {
        $crate::error_enum_tuple_fmt!(
            @collect
            $formatter,
            $doc;
            [$($field,)* $next_field,];
            [$($tuple_field),*];
            $($rest_ty),*
        )
    };
    (@collect $formatter:ident, $doc:literal; [$($field:ident,)*]; [$($tuple_field:ident),*];) => {
        ::core::write!($formatter, $doc $(, $field)*)
    };
}

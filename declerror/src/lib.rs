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
                            $(($($tuple_field_ty),*))?
                        ) => $crate::error_enum_fmt!(
                            $variant
                            $({ $($field : $field_ty),* })?
                            $(($($tuple_field_ty),*))?,
                            self,
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
    ($variant:ident) => {
        Self::$variant
    };
    // Struct variant with named fields
    ($variant:ident { $($field:ident : $field_ty:ty),* }) => {
        Self::$variant { $($field),* }
    };
    // Tuple variant with unnamed fields
    ($variant:ident ($($field_ty:ty),*)) => {
        $crate::error_enum_tuple_pat!($variant($($field_ty),*))
    };
}

/// Helper macro to match a tuple enum variant.
#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_tuple_pat {
    ($variant:ident()) => {
        Self::$variant()
    };
    ($variant:ident($($field_ty:ty),+)) => {
        Self::$variant(..)
    };
}

/// Helper macro to format an enum variant.
#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_fmt {
    // Unit variant
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal) => {
        ::core::write!($formatter, $doc)
    };
    // Struct variant with named fields
    ($variant:ident { $($field:ident : $field_ty:ty),* }, $self:ident, $formatter:ident, $doc:literal) => {
        ::core::write!($formatter, $doc, $($field = $field),*)
    };
    // Tuple variant with unnamed fields
    ($variant:ident ($($field_ty:ty),*), $self:ident, $formatter:ident, $doc:literal) => {
        $crate::error_enum_tuple_fmt!($variant, $self, $formatter, $doc; $($field_ty),*)
    };
}

/// Helper macro to format a tuple enum variant.
#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_tuple_fmt {
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal;) => {{ ::core::write!($formatter, $doc) }};
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal; $field_0_ty:ty) => {{
        let Self::$variant(field_0) = $self else {
            ::core::unreachable!()
        };
        ::core::write!($formatter, $doc, field_0)
    }};
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal; $field_0_ty:ty, $field_1_ty:ty) => {{
        let Self::$variant(field_0, field_1) = $self else {
            ::core::unreachable!()
        };
        ::core::write!($formatter, $doc, field_0, field_1)
    }};
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal; $field_0_ty:ty, $field_1_ty:ty, $field_2_ty:ty) => {{
        let Self::$variant(field_0, field_1, field_2) = $self else {
            ::core::unreachable!()
        };
        ::core::write!($formatter, $doc, field_0, field_1, field_2)
    }};
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal; $field_0_ty:ty, $field_1_ty:ty, $field_2_ty:ty, $field_3_ty:ty) => {{
        let Self::$variant(field_0, field_1, field_2, field_3) = $self else {
            ::core::unreachable!()
        };
        ::core::write!($formatter, $doc, field_0, field_1, field_2, field_3)
    }};
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal; $field_0_ty:ty, $field_1_ty:ty, $field_2_ty:ty, $field_3_ty:ty, $field_4_ty:ty) => {{
        let Self::$variant(field_0, field_1, field_2, field_3, field_4) = $self else {
            ::core::unreachable!()
        };
        ::core::write!(
            $formatter, $doc, field_0, field_1, field_2, field_3, field_4
        )
    }};
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal; $field_0_ty:ty, $field_1_ty:ty, $field_2_ty:ty, $field_3_ty:ty, $field_4_ty:ty, $field_5_ty:ty) => {{
        let Self::$variant(field_0, field_1, field_2, field_3, field_4, field_5) = $self else {
            ::core::unreachable!()
        };
        ::core::write!(
            $formatter, $doc, field_0, field_1, field_2, field_3, field_4, field_5
        )
    }};
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal; $field_0_ty:ty, $field_1_ty:ty, $field_2_ty:ty, $field_3_ty:ty, $field_4_ty:ty, $field_5_ty:ty, $field_6_ty:ty) => {{
        let Self::$variant(field_0, field_1, field_2, field_3, field_4, field_5, field_6) = $self
        else {
            ::core::unreachable!()
        };
        ::core::write!(
            $formatter, $doc, field_0, field_1, field_2, field_3, field_4, field_5, field_6
        )
    }};
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal; $field_0_ty:ty, $field_1_ty:ty, $field_2_ty:ty, $field_3_ty:ty, $field_4_ty:ty, $field_5_ty:ty, $field_6_ty:ty, $field_7_ty:ty) => {{
        let Self::$variant(field_0, field_1, field_2, field_3, field_4, field_5, field_6, field_7) =
            $self
        else {
            ::core::unreachable!()
        };
        ::core::write!(
            $formatter, $doc, field_0, field_1, field_2, field_3, field_4, field_5, field_6,
            field_7
        )
    }};
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal; $field_0_ty:ty, $field_1_ty:ty, $field_2_ty:ty, $field_3_ty:ty, $field_4_ty:ty, $field_5_ty:ty, $field_6_ty:ty, $field_7_ty:ty, $field_8_ty:ty) => {{
        let Self::$variant(
            field_0,
            field_1,
            field_2,
            field_3,
            field_4,
            field_5,
            field_6,
            field_7,
            field_8,
        ) = $self
        else {
            ::core::unreachable!()
        };
        ::core::write!(
            $formatter, $doc, field_0, field_1, field_2, field_3, field_4, field_5, field_6,
            field_7, field_8
        )
    }};
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal; $field_0_ty:ty, $field_1_ty:ty, $field_2_ty:ty, $field_3_ty:ty, $field_4_ty:ty, $field_5_ty:ty, $field_6_ty:ty, $field_7_ty:ty, $field_8_ty:ty, $field_9_ty:ty) => {{
        let Self::$variant(
            field_0,
            field_1,
            field_2,
            field_3,
            field_4,
            field_5,
            field_6,
            field_7,
            field_8,
            field_9,
        ) = $self
        else {
            ::core::unreachable!()
        };
        ::core::write!(
            $formatter, $doc, field_0, field_1, field_2, field_3, field_4, field_5, field_6,
            field_7, field_8, field_9
        )
    }};
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal; $field_0_ty:ty, $field_1_ty:ty, $field_2_ty:ty, $field_3_ty:ty, $field_4_ty:ty, $field_5_ty:ty, $field_6_ty:ty, $field_7_ty:ty, $field_8_ty:ty, $field_9_ty:ty, $field_10_ty:ty) => {{
        let Self::$variant(
            field_0,
            field_1,
            field_2,
            field_3,
            field_4,
            field_5,
            field_6,
            field_7,
            field_8,
            field_9,
            field_10,
        ) = $self
        else {
            ::core::unreachable!()
        };
        ::core::write!(
            $formatter, $doc, field_0, field_1, field_2, field_3, field_4, field_5, field_6,
            field_7, field_8, field_9, field_10
        )
    }};
    ($variant:ident, $self:ident, $formatter:ident, $doc:literal; $field_0_ty:ty, $field_1_ty:ty, $field_2_ty:ty, $field_3_ty:ty, $field_4_ty:ty, $field_5_ty:ty, $field_6_ty:ty, $field_7_ty:ty, $field_8_ty:ty, $field_9_ty:ty, $field_10_ty:ty, $field_11_ty:ty) => {{
        let Self::$variant(
            field_0,
            field_1,
            field_2,
            field_3,
            field_4,
            field_5,
            field_6,
            field_7,
            field_8,
            field_9,
            field_10,
            field_11,
        ) = $self
        else {
            ::core::unreachable!()
        };
        ::core::write!(
            $formatter, $doc, field_0, field_1, field_2, field_3, field_4, field_5, field_6,
            field_7, field_8, field_9, field_10, field_11
        )
    }};
}

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
                #[error($doc:literal $(, $format_arg:expr)* $(,)?)]
                $(#[$variant_attr:meta])*
                $variant:ident
                // Keep struct and tuple forms explicit. Capturing an optional
                // `$tt` after `$variant` is too broad and can make the variant
                // list locally ambiguous around commas and the enum closing brace.
                $({ $($(#[$field_attr:meta])* $field:ident : $field_ty:ty),* $(,)? })?
                $(($($(#[$tuple_field_attr:meta])* $tuple_field_ty:ty),* $(,)?))?
            ),* $(,)?
        }
    ) => {
        $(#[$enum_attr])*
        #[derive(::core::fmt::Debug)]
        $vis enum $name {
            $(
                $(#[$variant_attr])*
                $variant
                $({ $($(#[$field_attr])* $field : $field_ty),* })?
                $(($($(#[$tuple_field_attr])* $tuple_field_ty),*))?
            ),*
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(
                        // Tuple variants do not provide field names, so pass a fixed binding-name pool into both helpers.
                        // Because the pattern and format expression receive identifiers from the same expansion, the formatter can refer to the bindings introduced by the pattern.
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
                            $doc;
                            $($format_arg),*
                        )
                        // Pattern and expression are separate helpers because `macro_rules!` cannot expand to an incomplete match arm.
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
    // Consume one tuple type and one generated binding name at a time. The type is only used as a counter; the emitted pattern needs the binding name.
    (@collect $variant:ident; [$($field:ident,)*]; [$next_field:ident $(, $tuple_field:ident)*]; $field_ty:ty $(, $rest_ty:ty)*) => {
        $crate::error_enum_tuple_pat!(
            @collect
            $variant;
            [$($field,)* $next_field,];
            [$($tuple_field),*];
            $($rest_ty),*
        )
    };
    // When no tuple types remain, emit the tuple variant pattern with exactly the collected bindings. This also handles the zero-field tuple case.
    (@collect $variant:ident; [$($field:ident,)*]; [$($tuple_field:ident),*];) => {
        Self::$variant($($field),*)
    };
}

/// Helper macro to format an enum variant.
#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_fmt {
    // Unit variant
    ($variant:ident; [$($tuple_field:ident),*], $formatter:ident, $doc:literal; $($format_arg:expr),*) => {
        ::core::write!($formatter, $doc $(, $format_arg)*)
    };
    // Struct variant with named fields
    ($variant:ident { $($field:ident : $field_ty:ty),* }; [$($tuple_field:ident),*], $formatter:ident, $doc:literal; $($format_arg:expr),*) => {
        ::core::write!($formatter, $doc $(, $format_arg)*)
    };
    // Tuple variant with unnamed fields
    ($variant:ident ($($field_ty:ty),*); [$($tuple_field:ident),*], $formatter:ident, $doc:literal; $($format_arg:expr),*) => {
        $crate::error_enum_tuple_fmt!(
            $formatter,
            $doc;
            fields [$($tuple_field),*];
            types [$($field_ty),*];
            args [$($format_arg),*]
        )
    };
}

/// Helper macro to format a tuple enum variant.
#[doc(hidden)]
#[macro_export]
macro_rules! error_enum_tuple_fmt {
    ($formatter:ident, $doc:literal; fields [$($tuple_field:ident),*]; types [$($field_ty:ty),*]; args []) => {
        $crate::error_enum_tuple_fmt!(@collect $formatter, $doc; []; [$($tuple_field),*]; $($field_ty),*)
    };
    ($formatter:ident, $doc:literal; fields [$($tuple_field:ident),*]; types [$($field_ty:ty),*]; args [$first_format_arg:expr $(, $format_arg:expr)*]) => {
        ::core::compile_error!("tuple variants do not support explicit #[error(...)] format arguments; use positional placeholders like {0} and {1}")
    };
    // Mirror `error_enum_tuple_pat!`: consume one type per generated binding so positional format strings receive the same fields bound by the pattern.
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
    // Emit no extra arguments for zero-field tuple variants, otherwise forward the collected bindings as positional formatting arguments.
    (@collect $formatter:ident, $doc:literal; [$($field:ident,)*]; [$($tuple_field:ident),*];) => {
        ::core::write!($formatter, $doc $(, $field)*)
    };
}

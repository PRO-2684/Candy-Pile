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
                $crate::error_enum!(
                    @display_match
                    self,
                    formatter;
                    $(
                        #[doc = $doc]
                        $variant $({ $($field : $field_ty),* })?
                    ),*
                )
            }
        }

        impl ::core::error::Error for $name {}
    };

    (@display_match $self:ident, $formatter:ident;) => {
        ::core::unreachable!()
    };

    (
        @display_match
        $self:ident,
        $formatter:ident;
        #[doc = $doc:literal]
        $variant:ident,
        $($tail:tt)*
    ) => {
        match $self {
            Self::$variant => ::core::write!($formatter, $doc),
            _ => $crate::error_enum!(@display_match $self, $formatter; $($tail)*),
        }
    };

    (
        @display_match
        $self:ident,
        $formatter:ident;
        #[doc = $doc:literal]
        $variant:ident
    ) => {
        match $self {
            Self::$variant => ::core::write!($formatter, $doc),
            _ => ::core::unreachable!(),
        }
    };

    (
        @display_match
        $self:ident,
        $formatter:ident;
        #[doc = $doc:literal]
        $variant:ident { $($field:ident : $field_ty:ty),* $(,)? },
        $($tail:tt)*
    ) => {
        match $self {
            Self::$variant { $($field),* } => {
                ::core::write!($formatter, $doc, $($field = $field),*)
            }
            _ => $crate::error_enum!(@display_match $self, $formatter; $($tail)*),
        }
    };

    (
        @display_match
        $self:ident,
        $formatter:ident;
        #[doc = $doc:literal]
        $variant:ident { $($field:ident : $field_ty:ty),* $(,)? }
    ) => {
        match $self {
            Self::$variant { $($field),* } => {
                ::core::write!($formatter, $doc, $($field = $field),*)
            }
            _ => ::core::unreachable!(),
        }
    };
}

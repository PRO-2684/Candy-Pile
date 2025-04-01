#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]

/// A macro to generate a new struct with fields wrapped in `Option`.
#[macro_export]
macro_rules! optfield {(
    $(#[$attr:meta])*
    $vis:vis struct $name:ident {
        $(
            $(#[$field_attr:meta])*
            $field_vis:vis
            $field:ident : $ty:ty
        ),* $(,)?
    }
    ($new:ident)
) => {
    // Original struct
    $(#[$attr])*
    $vis struct $name {
        $(
            $(#[$field_attr])*
            $field_vis
            $field: $ty,
        )*
    }
    // Generated struct
    $(#[$attr])*
    $vis struct $new {
        $(
            $(#[$field_attr])*
            $field_vis
            $field: ::core::option::Option<$ty>,
        )*
    }
}}

#[cfg(test)]
mod tests {
    use super::*;
    use macro_rules_attr::apply;

    #[apply(optfield(OptTest))]
    /// My test struct
    struct Test {
        pub a: u32,
        b: u32,
    }

    #[test]
    fn test_original_struct() {
        let test = Test { a: 1, b: 2 };
        assert_eq!(test.a, 1);
        assert_eq!(test.b, 2);
    }

    #[test]
    fn test_generated_struct() {
        let opt_test = OptTest {
            a: Some(1),
            b: None,
        };
        assert_eq!(opt_test.a.unwrap(), 1);
        assert!(opt_test.b.is_none());
    }
}

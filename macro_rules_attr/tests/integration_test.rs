use macro_rules_attr::apply;
use paste::paste;

macro_rules! make_getters {
(
    $(#[$struct_meta:meta])*
    $struct_vis:vis
    struct $StructName:ident {
        $(
            $(#[$field_meta:meta])*
            $field_vis:vis // this visibility will be applied to the getters instead
            $field_name:ident : $field_ty:ty
        ),* $(,)?
    }
) => (
    // First, generate the struct definition we have been given, but with
    // private fields instead.
    $(#[$struct_meta])*
    $struct_vis
    struct $StructName {
        $(
            $(#[$field_meta])*
            // notice the lack of visibility => private fields
            $field_name: $field_ty,
        )*
    }

    // Then, implement the getters:
    impl $StructName {
        $(
            #[inline]
            $field_vis
            fn $field_name (self: &'_ Self)
              -> &'_ $field_ty
            {
                &self.$field_name
            }
        )*
    }
);
(
    $(#[$struct_meta:meta])*
    $struct_vis:vis
    struct $StructName:ident {
        $(
            $(#[$field_meta:meta])*
            $field_vis:vis // this visibility will be applied to the getters instead
            $field_name:ident : $field_ty:ty
        ),* $(,)?
    },
    $prefix:ident
) => (
    // First, generate the struct definition we have been given, but with
    // private fields instead.
    $(#[$struct_meta])*
    $struct_vis
    struct $StructName {
        $(
            $(#[$field_meta])*
            // notice the lack of visibility => private fields
            $field_name: $field_ty,
        )*
    }

    // Then, implement the getters:
    paste! {
        impl $StructName {
            $(
                #[inline]
                $field_vis
                fn [<$prefix $field_name>] (self: &'_ Self)
                  -> &'_ $field_ty
                {
                    &self.$field_name
                }
            )*
        }
    }
)}

#[test]
fn test_apply_without_args() {
    #[apply(make_getters)]
    struct MyStruct {
        pub a: i32,
        pub b: i32,
    }

    let my_struct = MyStruct { a: 1, b: 2 };

    assert_eq!(*my_struct.a(), 1);
    assert_eq!(*my_struct.b(), 2);
}

#[test]
fn test_apply_with_args() {
    #[apply(make_getters, get_)]
    struct MyStruct {
        pub a: i32,
        pub b: i32,
    }

    let my_struct = MyStruct { a: 1, b: 2 };

    assert_eq!(*my_struct.get_a(), 1);
    assert_eq!(*my_struct.get_b(), 2);
}

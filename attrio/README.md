# `attrio`

A tiny library for easily generating macros that annotate items with given attributes.

## Usage

Recommended to work with [`macro_rules_attr`](https://crates.io/crates/macro_rules_attr), which provides nice syntactic sugar:

```rust
use attrio::attrio;
use macro_rules_attr::apply;

// Generate a macro `derive_cmp` that annotates items with `#[derive(PartialEq, Eq, PartialOrd, Ord)]`
attrio! {
    $derive_cmp,
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
}

// Apply the generated macro to a struct
#[apply(derive_cmp)]
#[derive(Debug)]
struct Foo(i32);

// Testing that it works
let foo1 = Foo(1);
let foo2 = Foo(2);
assert!(foo1 < foo2);

let another_foo1 = Foo(1);
assert_eq!(foo1, another_foo1);
```

However, you can also use it directly:

```rust
# use attrio::attrio;
#
# attrio! {
#    $derive_cmp,
#    #[derive(PartialEq, Eq, PartialOrd, Ord)]
# }
#
// ...
// Apply the generated macro to a struct
derive_cmp! {
    #[derive(Debug)]
    struct Foo(i32);
}
// Testing that it works
// ...
# let foo1 = Foo(1);
# let foo2 = Foo(2);
# assert!(foo1 < foo2);
# let another_foo1 = Foo(1);
# assert_eq!(foo1, another_foo1);
```

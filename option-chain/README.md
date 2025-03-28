<img alt="Logo" src="./logo.svg" align="right" style="width: 4em; height: 4em;"></img>

# option-chain

A macro for using `?` operator in functions that don't return `Option`.

## Features

- 🪶 **Lightweight**: Just a single `macro_rules!`, without any dependency - even the standard library!

## Examples

```rust
use option_chain::opt;

struct Test1 {
    a: Option<Test2>,
}
struct Test2 {
    b: Option<Test3>,
}
struct Test3 {
    c: i32,
}

let v = Test1 {
    a: Some(Test2 { b: Some(Test3 { c: 42 }) }),
};
let c = opt!(v.a?.b?.c);
assert_eq!(c.unwrap(), 42);
```

## Why?

Consider the following scenario:

```rust
struct Test1 {
    a: Option<Test2>,
}
struct Test2 {
    b: Option<Test3>,
}
struct Test3 {
    c: i32,
}

fn main() {
    let v = Test1 {
        a: Some(Test2 { b: Some(Test3 { c: 42 }) }),
    };
    // We want to get the value of the `c` field, returning `None` if any member in the "chain of fields" is `None`.
}
```

Essentially, we're looking for Rust-equivalent of the following JavaScript code:

```javascript
const c = v?.a?.b?.c;
```

Usually you'd utilize the [`and_then`](`Option::and_then`) and [`map`](`Option::map`) methods:

```rust
# struct Test1 {
#     a: Option<Test2>,
# }
# struct Test2 {
#     b: Option<Test3>,
# }
# struct Test3 {
#     c: i32,
# }
#
# fn main() {
#     let v = Test1 {
#         a: Some(Test2 { b: Some(Test3 { c: 42 }) }),
#     };
let c = v.a.and_then(|a| a.b).map(|b| b.c);
assert_eq!(c.unwrap(), 42);
# }
```

Which looks quite verbose. [`flatten`](`Option::flatten`) is also a good choice:

```rust
# struct Test1 {
#     a: Option<Test2>,
# }
# struct Test2 {
#     b: Option<Test3>,
# }
# struct Test3 {
#     c: i32,
# }
#
# fn main() {
#     let v = Test1 {
#         a: Some(Test2 { b: Some(Test3 { c: 42 }) }),
#     };
let c = v.a.map(|a| a.b).flatten().map(|b| b.c);
assert_eq!(c.unwrap(), 42);
# }
```

But it's still not as concise as the JavaScript code. Also, you might think of creating a helper function:

```rust
# struct Test1 {
#     a: Option<Test2>,
# }
# struct Test2 {
#     b: Option<Test3>,
# }
# struct Test3 {
#     c: i32,
# }
#
# fn main() {
#     let v = Test1 {
#         a: Some(Test2 { b: Some(Test3 { c: 42 }) }),
#     };
fn get_c(v: Test1) -> Option<i32> {
    Some(v.a?.b?.c)
}

let c = get_c(v);
assert_eq!(c.unwrap(), 42);
# }
```

Which is better, but you'll need to create a function for every different chain of fields you want to access.

This is where `option-chain` comes in:

```rust
# use option_chain::opt;
#
# struct Test1 {
#     a: Option<Test2>,
# }
# struct Test2 {
#     b: Option<Test3>,
# }
# struct Test3 {
#     c: i32,
# }
#
# fn main() {
#     let v = Test1 {
#         a: Some(Test2 { b: Some(Test3 { c: 42 }) }),
#     };
let c = opt!(v.a?.b?.c);
assert_eq!(c.unwrap(), 42);
# }
```

## How?

It just wraps the expression in a closure which returns `Option`, and immediately calls it:

```rust
macro_rules! opt {
    ($e:expr) => {{ || -> Option<_> { Some($e) }() }};
}
```

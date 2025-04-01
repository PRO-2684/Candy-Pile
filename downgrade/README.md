# `downgrade`

Downgrade a mutable reference to an immutable one.

## Usage

```rust
use downgrade::Downgrade;

let mut x = 5;
let y = &mut x; // `y` is a mutable reference to `x` (`&mut x`)
let z = y.downgrade(); // `z` is an immutable reference to `x` (`&x`)
```

## Why?

### Brief

Some functions return a mutable reference to a value, but you only need an immutable reference. A mutable reference cannot be `.clone()`d, while an immutable reference can, so this method might come in handy.

### Example

Consider the following example:

```rust compile_fail
use std::thread;

fn process(foo: &i32, i: i32) {
    // Do some jobs
    println!("[#{i}] Processing: {foo}");
}

fn main() {
    let foo = 5;
    let leaked = Box::leak(Box::new(foo));
    *leaked += 1; // Modify the value
    // Note that `leaked` is a mutable reference, which does not implement `Clone`
    for i in 0..4 {
        thread::spawn(move || {
            process(leaked, i);
        });
    }
}
```

This would not compile, since `leaked` is a mutable reference, which does not implement `Clone`. However, the compiler does not know that we will only use the reference in a read-only way, so we must tell it explicitly. Normally, you can:

```rust
# use std::thread;
#
# fn process(foo: &i32, i: i32) {
#     // Do some jobs
#     println!("[#{i}] Processing: {foo}");
# }
#
# fn main() {
#     let foo = 5;
let leaked = Box::leak(Box::new(foo));
*leaked += 1; // Modify the value
let leaked: &_ = leaked; // Downgrade the mutable reference to an immutable one using a type annotation
#     for i in 0..4 {
#         thread::spawn(move || {
#             process(leaked, i);
#         });
#     }
# }
```

Which looks a bit ugly. By using `downgrade`, we can make it a bit cleaner:

```rust
use downgrade::Downgrade;
# use std::thread;
#
# fn process(foo: &i32, i: i32) {
#     // Do some jobs
#     println!("[#{i}] Processing: {foo}");
# }
#
# fn main() {
#     let foo = 5;
let leaked = Box::leak(Box::new(foo));
*leaked += 1; // Modify the value
let leaked = leaked.downgrade(); // We've downgraded the mutable reference to an immutable one
#     for i in 0..4 {
#         thread::spawn(move || {
#             process(leaked, i);
#         });
#     }
# }
```

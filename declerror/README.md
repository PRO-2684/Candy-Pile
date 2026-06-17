# `declerror`

[![GitHub License](https://img.shields.io/github/license/PRO-2684/Candy-Pile?logo=opensourceinitiative)](https://github.com/PRO-2684/Candy-Pile/blob/main/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/declerror?logo=rust)](https://crates.io/crates/declerror)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/declerror?logo=rust)](https://crates.io/crates/declerror)
[![docs.rs](https://img.shields.io/docsrs/declerror?logo=rust)](https://docs.rs/declerror)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)

Declarative macros to "derive" `Error`.

## Usage

```rust
use declerror::error_enum;
use macro_rules_attr::apply;

#[apply(error_enum)]
pub enum MyError {
    #[error = "Unit variants"]
    SimpleError,
    #[error = "Referencing named fields ({code} {message}) in struct variants"]
    ErrorWithMessageAndCode { message: String, code: i32 },
    #[error = "Referencing tuple fields ({0} {1}) in tuple variants"]
    ErrorWithUnnamedFields(String, i32),
}

let error1 = MyError::SimpleError;
let error2 = MyError::ErrorWithMessageAndCode {
    message: "Something went wrong".to_string(),
    code: 404,
};
let error3 = MyError::ErrorWithUnnamedFields("Bad input".to_string(), 400);

assert_eq!(error1.to_string(), "Unit variants");
assert_eq!(error2.to_string(), "Referencing named fields (404 Something went wrong) in struct variants");
assert_eq!(error3.to_string(), "Referencing tuple fields (Bad input 400) in tuple variants");
```

## How?

Checkout [`./docs/HOW.md`](./docs/HOW.md) for a detailed explanation.

## TODO

- [x] Tuple variants ([up to 12 fields](<https://doc.rust-lang.org/std/primitive.tuple.html#impl-From%3C%5BT;+1%5D%3E-for-(T,)>)).
- [ ] Generics and lifetimes.
- [ ] Support for `#[source]` and `#[backtrace]` attributes?

## Related

- [`thiserror`](https://github.com/dtolnay/thiserror)
- [`thiserror_lite`](https://github.com/kangalio/thiserror_lite)

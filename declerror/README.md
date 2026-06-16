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
}

let error1 = MyError::SimpleError;
let error2 = MyError::ErrorWithMessageAndCode {
    message: "Something went wrong".to_string(),
    code: 404,
};
assert_eq!(error1.to_string(), "Unit variants");
assert_eq!(error2.to_string(), "Referencing named fields (404 Something went wrong) in struct variants");
```

Tuple variants are not supported.

## Related

- [`thiserror`](https://github.com/dtolnay/thiserror)
- [`thiserror_lite`](https://github.com/kangalio/thiserror_lite)

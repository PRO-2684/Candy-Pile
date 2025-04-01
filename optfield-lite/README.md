# `optfield-lite`

[![GitHub License](https://img.shields.io/github/license/PRO-2684/Candy-Pile?logo=opensourceinitiative)](https://github.com/PRO-2684/Candy-Pile/blob/main/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/optfield-lite?logo=rust)](https://crates.io/crates/optfield-lite)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/optfield-lite?logo=rust)](https://crates.io/crates/optfield-lite)
[![docs.rs](https://img.shields.io/docsrs/optfield-lite?logo=rust)](https://docs.rs/optfield-lite)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)

A macro to generate a new struct with fields wrapped in `Option`. Lite version of [`optfield`](https://crates.io/crates/optfield).

## Usage

Recommended to work with [`macro_rules_attr`](https://crates.io/crates/macro_rules_attr), which provides nice syntactic sugar:

```rust
use optfield_lite::optfield;
use macro_rules_attr::apply;

#[apply(optfield(OptTest))]
/// My test struct
struct Test {
    pub a: u32,
    b: u32,
}
```

This will generate a struct `OptTest` with the following fields:

```rust
/// My test struct
struct OptTest {
    pub a: Option<u32>,
    b: Option<u32>,
}
```

Note that the generated struct will have the same attributes and visibility as the original struct. You can also use it directly, which produces the same result:

```rust
use optfield_lite::optfield;

optfield! {
    /// My test struct
    struct Test {
        pub a: u32,
        b: u32,
    }(OptTest)
}
```

## Comparison

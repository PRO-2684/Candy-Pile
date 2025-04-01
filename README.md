# Candy-Pile

[![GitHub License](https://img.shields.io/github/license/PRO-2684/Candy-Pile?logo=opensourceinitiative)](https://github.com/PRO-2684/Candy-Pile/blob/main/LICENSE)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)

🦀 Collection of lightweight syntactic sugar for Rust.

## Members

- [`attrio`](./attrio/README.md): A tiny library for easily generating macros that annotate items with given attributes.
- [`downgrade`](./downgrade/README.md): Downgrade a mutable reference to an immutable one.
- [`macro_rules_attr`](./macro_rules_attr/README.md): Use declarative macros as proc_macro attributes. (`#[apply]` your `macro_rules!`)
- [`optfield-lite`](./optfield-lite/README.md): A macro to generate a new struct with fields wrapped in `Option`. Lite version of [`optfield`](https://crates.io/crates/optfield).
- [`option-chain`](./option-chain/README.md): A macro for using `?` operator in functions that don't return `Option`.
- [`should_match`](./should_match/README.md): Pass a test if the output matches a pattern.

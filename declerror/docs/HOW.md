# Tuple Variants in `declerror`

`declerror::error_enum!` supports unit variants, struct variants, and tuple variants. Tuple variants are accepted with explicit parentheses:

```rust
#[apply(error_enum)]
enum MyError {
    #[error = "Unit variant"]
    Unit,
    #[error = "Struct variant: {message}"]
    Struct { message: String },
    #[error = "Tuple variant: {0} {1}"]
    Tuple(String, i32),
}
```

Tuple variants support up to 12 fields. Format strings use Rust's positional formatting syntax: `{0}`, `{1}`, and so on.

## Variant Matching

The top-level matcher keeps the variant forms explicit:

```rust
$variant:ident
$({ $($field:ident : $field_ty:ty),* $(,)? })?
$(($($tuple_field_ty:ty),* $(,)?))?
```

This avoids capturing an arbitrary optional token tree after the variant name. A matcher such as `$variant:ident $($variant_args:tt)?` is too broad and can introduce local ambiguity when the macro also needs to parse commas, variant attributes, and the end of the enum body.

The explicit matcher works because the two data-bearing variant forms start with distinct delimiters:

- Struct variants start with `{ ... }`.
- Tuple variants start with `( ... )`.
- Unit variants have neither.

## Pattern and Format Helpers

`Display::fmt` expands to a `match self` expression. Each match arm is assembled from two helpers:

- `error_enum_pat!` emits the pattern for the variant.
- `error_enum_fmt!` emits the `write!` expression for the arm body.

Struct variants are straightforward because named fields from the input can be reused directly:

```rust
Self::Variant { message } => write!(formatter, "{message}", message = message)
```

Tuple variants need generated binding names because the enum syntax only provides types:

```rust
Tuple(String, i32)
```

There are no user-provided field identifiers to bind.

## Hygiene Strategy

The tuple implementation passes a fixed pool of binding identifiers from `error_enum!` into both tuple helpers:

```rust
[
    field_0, field_1, field_2, field_3, field_4, field_5,
    field_6, field_7, field_8, field_9, field_10, field_11
]
```

`error_enum_tuple_pat!` consumes one identifier from that pool for each tuple field type and emits the match pattern:

```rust
Self::Tuple(field_0, field_1)
```

`error_enum_tuple_fmt!` consumes the same number of identifiers and emits the positional format arguments:

```rust
write!(formatter, doc, field_0, field_1)
```

The binding names come from the same outer macro expansion, so the identifiers used in the arm body refer to the bindings introduced by the match pattern.

This avoids a second destructuring step such as:

```rust
let Self::Tuple(field_0, field_1) = self else {
    unreachable!()
};
```

It also keeps the tuple arity logic in helper macros instead of repeating 12 full match-arm implementations.

## TT Munching

The tuple helpers use a small TT-munching pattern:

1. Keep an accumulator of selected binding identifiers.
2. Keep a remaining pool of available binding identifiers.
3. Consume one tuple field type at a time.
4. Stop when no tuple field types remain.

Conceptually:

```rust
types:  String, i32
pool:   field_0, field_1, field_2, ...
result: field_0, field_1
```

The helper never needs to inspect the tuple field types. It only counts them by consuming one type token at a time. That count determines how many binding names are used in the generated pattern and `write!` call.

## Tradeoffs

This implementation keeps the public syntax compact and avoids procedural macros, but it is still limited by `macro_rules!`:

- `macro_rules!` cannot synthesize identifiers such as `field_7` from a repetition count.
- It cannot directly zip `$( $field_ty:ty ),*` with a fixed identifier list in a flat repetition.
- A recursive helper is the least repetitive way to map tuple arity to generated field names.

The alternative is explicit arity arms for 0 through 12 fields, which is easier to expand mentally but much more repetitive in source.

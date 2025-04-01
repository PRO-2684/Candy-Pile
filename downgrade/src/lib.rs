#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]

/// A trait for downgrading mutable references to immutable ones.
pub trait Downgrade {
    /// Downgrade a mutable reference to an immutable one.
    fn downgrade(self: &mut Self) -> &Self {
        self
    }
}

// Blank implementation for all types
impl<T> Downgrade for T {}

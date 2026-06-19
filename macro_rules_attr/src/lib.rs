#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::cargo)]

#[cfg(feature = "log")]
use log::debug;

use proc_macro::{Delimiter, Group, Punct, Spacing, TokenStream, TokenTree};

/// Emit a compile error with the given message
macro_rules! error {
    ($msg:literal) => {{
        ::core::stringify! {
            ::core::compile_error!($msg)
        }
        .parse()
        .unwrap()
    }};
}

/// Errors that can occur when splitting the macro name and append tokens.
#[derive(Debug)]
enum MacroError {
    /// The input is empty, so no macro name was found.
    EmptyInput,
    /// The first token is not an identifier, so it cannot be a valid macro name.
    InvalidMacroName,
}

impl MacroError {
    /// Emit a compile error for this macro error.
    fn into_token_stream(self) -> TokenStream {
        match self {
            Self::EmptyInput => {
                error!("Expected an identifier, found nothing")
            }
            Self::InvalidMacroName => {
                error!("Expected an identifier, found something else")
            }
        }
    }
}

/// Apply the given macro to the annotated item, appending additional tokens if provided.
///
/// # Example
///
/// ```rust
/// use macro_rules_attr::apply;
///
/// macro_rules! rename_to_unit {
///     ($vis:vis struct $Name:ident;) => {
///         $vis struct Unit;
///     };
/// }
///
/// #[apply(rename_to_unit)]
/// struct Original;
///
/// let _ = Unit;
/// ```
#[proc_macro_attribute]
pub fn apply(attrs: TokenStream, input: TokenStream) -> TokenStream {
    // Split the attributes into the macro name and the additional tokens to append
    let (macro_name, macro_append) = match split_macro_name_and_append(attrs) {
        Ok(result) => result,
        Err(err) => return err.into_token_stream(),
    };

    // Invoke the macro
    invoke_macro(macro_name.into(), macro_append, input)
}

/// Extend the annotated item by applying the given macro and appending additional tokens if provided.
///
/// # Example
///
/// ```rust
/// use macro_rules_attr::extend;
///
/// macro_rules! impl_new {
///     ($vis:vis struct $Name:ident;) => {
///         impl $Name {
///             fn new() -> Self {
///                 Self
///             }
///         }
///     };
/// }
///
/// #[extend(impl_new)]
/// struct Unit;
///
/// let _ = Unit::new();
/// ```
#[proc_macro_attribute]
pub fn extend(attrs: TokenStream, mut input: TokenStream) -> TokenStream {
    // Split the attributes into the macro name and the additional tokens to append
    let (macro_name, macro_append) = match split_macro_name_and_append(attrs) {
        Ok(result) => result,
        Err(err) => return err.into_token_stream(),
    };

    // Invoke the macro
    let invoked = invoke_macro(macro_name.into(), macro_append, input.clone());

    // Combine the original input with the invoked macro output
    input.extend(invoked);
    input
}

/// Try to split the given `TokenStream` into an identifier followed by additional tokens.
fn split_macro_name_and_append(input: TokenStream) -> Result<(TokenTree, TokenStream), MacroError> {
    let mut tts = input.into_iter();
    let macro_name = tts.next().ok_or(MacroError::EmptyInput)?;
    if !matches!(macro_name, TokenTree::Ident(_)) {
        return Err(MacroError::InvalidMacroName);
    }
    let macro_append = tts.collect();

    #[cfg(feature = "log")]
    debug!("macro_name: {macro_name:?}");
    #[cfg(feature = "log")]
    debug!("macro_append: {macro_append:?}");

    Ok((macro_name, macro_append))
}

/// Invoke `macro_name` with given `input`, appending `macro_append` at the end.
fn invoke_macro(
    macro_name: TokenStream,
    macro_append: TokenStream,
    input: TokenStream,
) -> TokenStream {
    // Adapted from https://github.com/danielhenrymantilla/macro_rules_attribute-rs/blob/fa6f120939c7757dec23589e2687d5e8480fa1ce/src/proc_macro/mod.rs#L27-L53
    let mut macro_input = input;
    let mut ret = macro_name;

    macro_input.extend(macro_append); // Append `macro_append` to `macro_input`
    ret.extend([TokenTree::Punct(Punct::new('!', Spacing::Alone))]); // Append `!` to invoke the macro
    ret.extend([TokenTree::Group(Group::new(Delimiter::Brace, macro_input))]); // Wrap with `{}`

    #[cfg(feature = "log")]
    debug!("ret: {ret:?}");
    ret
}

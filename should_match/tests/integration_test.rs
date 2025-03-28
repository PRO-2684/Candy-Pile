#![no_std]

use macro_rules_attr::apply;
use should_match::{should_match, should_ok, should_err, should_some, should_none, test_match, test_ok, test_err, test_some, test_none};

// Testing `should_match!`

should_match! {
    #[test]
    fn test_match_direct() -> Result<(), &'static str> {
        Err("error")
    },
    pattern = Err("error")
}

#[apply(should_match, pattern = 42)]
#[test]
fn test_should_match_apply_first() -> i32 {
    42
}

#[test]
#[apply(should_match, pattern = 42)]
fn test_should_match_apply_second() -> i32 {
    42
}

#[test]
#[apply(should_match, pattern = Err("error"))]
fn test_should_match_with_return() -> Result<(), &'static str> {
    return Err("error");
}

#[test]
#[apply(should_match, pattern = Err("error"))]
fn test_should_match_with_question_mark() -> Result<(), &'static str> {
    Err("error")?;
    Ok(())
}

#[test]
#[apply(should_match, pattern = Err("error"), message = "Expected `Err`, but got `Ok`")]
fn test_should_match_with_message() -> Result<(), &'static str> {
    Err("error")
}

#[apply(should_match, pattern = Err("error"))]
#[test]
#[should_panic(expected = "Expected to match `Err(\"error\")`")]
fn test_should_match_panic() -> Result<(), &'static str> {
    Ok(())
}

// Testing `should_*` shortcuts

#[test]
#[apply(should_ok)]
fn test_should_ok() -> Result<(), &'static str> {
    Ok(())
}

#[apply(should_ok)]
#[test]
#[should_panic(expected = "Expected `Ok`, but got `Err`")]
fn test_should_ok_panic() -> Result<(), &'static str> {
    Err("error")
}

#[test]
#[apply(should_err)]
fn test_should_err() -> Result<(), &'static str> {
    Err("error")
}

#[apply(should_err)]
#[test]
#[should_panic(expected = "Expected `Err`, but got `Ok`")]
fn test_should_err_panic() -> Result<(), &'static str> {
    Ok(())
}

#[test]
#[apply(should_some)]
fn test_should_some() -> Option<()> {
    Some(())
}

#[apply(should_some)]
#[test]
#[should_panic(expected = "Expected `Some`, but got `None`")]
fn test_should_some_panic() -> Option<()> {
    None
}

#[test]
#[apply(should_none)]
fn test_should_none() -> Option<()> {
    None
}

#[apply(should_none)]
#[test]
#[should_panic(expected = "Expected `None`, but got `Some`")]
fn test_should_none_panic() -> Option<()> {
    Some(())
}

// Testing `test_*` shortcuts

#[apply(test_match, pattern = Err("error"))]
fn test_test_match() -> Result<(), &'static str> {
    Err("error")
}

#[apply(test_ok)]
fn test_test_ok() -> Result<(), &'static str> {
    Ok(())
}

#[apply(test_ok)]
#[should_panic(expected = "Expected `Ok`, but got `Err`")]
fn test_test_ok_panic() -> Result<(), &'static str> {
    Err("error")
}

#[apply(test_err)]
fn test_test_err() -> Result<(), &'static str> {
    Err("error")
}

#[apply(test_err)]
#[should_panic(expected = "Expected `Err`, but got `Ok`")]
fn test_test_err_panic() -> Result<(), &'static str> {
    Ok(())
}

#[apply(test_some)]
fn test_test_some() -> Option<()> {
    Some(())
}

#[apply(test_some)]
#[should_panic(expected = "Expected `Some`, but got `None`")]
fn test_test_some_panic() -> Option<()> {
    None
}

#[apply(test_none)]
fn test_test_none() -> Option<()> {
    None
}

#[apply(test_none)]
#[should_panic(expected = "Expected `None`, but got `Some`")]
fn test_test_none_panic() -> Option<()> {
    Some(())
}

// Testing custom enums & shortcuts

#[allow(dead_code, reason = "This is a test")]
enum Custom {
    One,
    Two,
    Three,
}

#[test]
#[apply(should_match, pattern = Custom::One)]
fn test_should_match_custom() -> Custom {
    Custom::One
}

macro_rules! should_three {(
    $($target:tt)*
) => {
    ::should_match::should_match! {
        $($target)*,
        pattern = $crate::Custom::Three,
        message = "Expected `Three`, but got something else"
    }
}}

#[test]
#[apply(should_three)]
fn test_should_three() -> Custom {
    Custom::Three
}

macro_rules! test_three {(
    $($target:tt)*
) => {
    ::should_match::test_match! {
        $($target)*,
        pattern = $crate::Custom::Three,
        message = "Expected `Three`, but got something else"
    }
}}

#[apply(test_three)]
fn test_test_three() -> Custom {
    Custom::Three
}

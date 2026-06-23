use std::path::PathBuf;

use declerror::error_enum;
use macro_rules_attr::apply;

#[apply(error_enum)]
enum MyError {
    /// Doc comment before `#[error]`
    #[error("This is a simple error without additional data")]
    SimpleError,
    #[error("This is an error with a message: {message}")]
    /// Doc comment after `#[error]`
    ErrorWithMessage { message: String },
    #[error("This is an error with a code: {code}")]
    ErrorWithCode {
        /// The error code associated with this error
        code: i32,
    },
    #[error("This is an error with a message: {message} and a code: {code}")]
    ErrorWithMessageAndCode { message: String, code: i32 },
    #[error("This is an error with a path: {}", path.display())]
    ErrorWithPath { path: PathBuf },
    #[error("This is an error with unnamed fields: {0}, {1}")]
    ErrorWithUnnamedFields(
        String,
        /// Some integer value associated with this error
        i32,
    ),
    #[error("{0}{1}{2}{3}{4}{5}{6}{7}{8}{9}{10}{11}")]
    ErrorWithTwelveFields(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),
}

#[test]
fn test_errors() {
    let simple_error = MyError::SimpleError;
    let error_with_message = MyError::ErrorWithMessage {
        message: "Something went wrong".to_string(),
    };
    let error_with_code = MyError::ErrorWithCode { code: 404 };
    let error_with_message_and_code = MyError::ErrorWithMessageAndCode {
        message: "Something went wrong".to_string(),
        code: 404,
    };
    let error_with_path = MyError::ErrorWithPath {
        path: PathBuf::from("/tmp/input.txt"),
    };
    let error_with_unnamed_fields = MyError::ErrorWithUnnamedFields("message".to_string(), 500);
    let error_with_twelve_fields =
        MyError::ErrorWithTwelveFields(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

    assert_eq!(
        simple_error.to_string(),
        "This is a simple error without additional data"
    );
    assert_eq!(
        error_with_message.to_string(),
        "This is an error with a message: Something went wrong"
    );
    assert_eq!(
        error_with_code.to_string(),
        "This is an error with a code: 404"
    );
    assert_eq!(
        error_with_message_and_code.to_string(),
        "This is an error with a message: Something went wrong and a code: 404"
    );
    assert_eq!(
        error_with_path.to_string(),
        "This is an error with a path: /tmp/input.txt"
    );
    assert_eq!(
        error_with_unnamed_fields.to_string(),
        "This is an error with unnamed fields: message, 500"
    );
    assert_eq!(error_with_twelve_fields.to_string(), "01234567891011");
}

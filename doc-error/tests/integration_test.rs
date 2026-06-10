use doc_error::error_enum;
use macro_rules_attr::apply;

#[apply(error_enum)]
pub enum MyError {
    ///This is a simple error without additional data
    SimpleError,
    ///This is an error with a message: {message}
    ErrorWithMessage { message: String },
    ///This is an error with a code: {code}
    ErrorWithCode { code: i32 },
    ///This is an error with a message: {message} and a code: {code}
    ErrorWithMessageAndCode { message: String, code: i32 },
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
}

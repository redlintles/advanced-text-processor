enum ErrorCode {
    FileNotFound(String),
    TokenNotFound(String),
    TokenArrayNotFound(String),
    BytecodeNotFound(String),
    TextParsingError(String),
    BytecodeParsingError(String),
    InvalidIndex(String),
    IndexOutOfRange(String),
    InvalidOperands(String),
}

impl ErrorCode {
    pub fn get_error_code(&self) -> i32 {
        match self {
            ErrorCode::FileNotFound(_) => 100,
            ErrorCode::TokenNotFound(_) => 101,
            ErrorCode::TokenArrayNotFound(_) => 102,
            ErrorCode::BytecodeNotFound(_) => 103,
            ErrorCode::InvalidOperands(_) => 200,
            ErrorCode::IndexOutOfRange(_) => 201,
            ErrorCode::InvalidIndex(_) => 202,
            ErrorCode::TextParsingError(_) => 300,
            ErrorCode::BytecodeParsingError(_) => 301,
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            | Self::FileNotFound(x)
            | Self::IndexOutOfRange(x)
            | Self::InvalidIndex(x)
            | Self::InvalidOperands(x)
            | Self::TextParsingError(x)
            | Self::TokenNotFound(x)
            | Self::BytecodeNotFound(x)
            | Self::BytecodeParsingError(x)
            | Self::TokenArrayNotFound(x) => x.to_string(),
        }
    }
}

pub fn token_array_not_found(identifier: &str) -> impl Fn() -> String {
    let message =
        format!("Token array not found, is {} a valid identifier for this processor?", identifier);
    move || message.clone()
}

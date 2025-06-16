#[derive(Default, Clone)]
pub struct ErrorManager {
    panic_with_error: bool,
    show_warnings: bool,
    error_vec: Vec<AtpError>,
}

#[derive(Clone)]
pub struct AtpError {
    error_code: AtpErrorCode,
    instruction: String,
    input: String,
}

impl AtpError {
    pub fn new(error_code: AtpErrorCode, instruction: String, input: String) -> Self {
        AtpError { error_code, instruction, input }
    }
}

impl ErrorManager {
    pub fn will_panic(&mut self, val: bool) {
        self.panic_with_error = val;
    }
    pub fn will_log(&mut self, val: bool) {
        self.show_warnings = val;
    }
    pub fn add_error(&mut self, err: AtpError) {
        self.error_vec.push(err);
    }

    pub fn print_errors(&self) {
        for (error, index) in self.error_vec.iter().zip(0..self.error_vec.len()) {
            println!(
                "[{}] => {}:\n\tInstruction: {}\n\t Message: {}\n\tProcessing: {}",
                index,
                error.error_code.get_error_code(),
                error.instruction,
                error.error_code.get_message(),
                error.input
            );
        }
    }
}
#[derive(Clone)]
pub enum AtpErrorCode {
    FileNotFound(String),
    TokenNotFound(String),
    TokenArrayNotFound(String),
    FileReadingError(String),
    FileWritingError(String),
    FileOpeningError(String),
    BytecodeNotFound(String),
    TextParsingError(String),
    BytecodeParsingError(String),
    InvalidIndex(String),
    IndexOutOfRange(String),
    InvalidOperands(String),
    InvalidParameters(String),
    ValidationError(String),
}

impl AtpErrorCode {
    pub fn get_error_code(&self) -> i32 {
        match self {
            Self::FileNotFound(_) => 100,
            Self::TokenNotFound(_) => 101,
            Self::TokenArrayNotFound(_) => 102,
            Self::FileReadingError(_) => 103,
            Self::FileWritingError(_) => 104,
            Self::FileOpeningError(_) => 105,
            Self::BytecodeNotFound(_) => 106,
            Self::InvalidOperands(_) => 200,
            Self::IndexOutOfRange(_) => 201,
            Self::InvalidIndex(_) => 202,
            Self::InvalidParameters(_) => 203,
            Self::TextParsingError(_) => 300,
            Self::BytecodeParsingError(_) => 301,
            Self::ValidationError(_) => 302,
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
            | Self::InvalidParameters(x)
            | Self::ValidationError(x)
            | Self::FileReadingError(x)
            | Self::FileWritingError(x)
            | Self::FileOpeningError(x)
            | Self::TokenArrayNotFound(x) => x.to_string(),
        }
    }
}

pub fn token_array_not_found(identifier: &str) -> impl Fn() -> String {
    let message =
        format!("Token array not found, is {} a valid identifier for this processor?", identifier);
    move || message.clone()
}

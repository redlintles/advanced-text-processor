use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;

#[derive(Default, Clone)]
pub struct ErrorManager {
    panic_with_error: bool,
    show_warnings: bool,
    error_vec: Vec<AtpError>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AtpError {
    pub error_code: AtpErrorCode,
    pub instruction: Cow<'static, str>,
    pub input: Cow<'static, str>,
}

impl Error for AtpError {}

impl Display for AtpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Erro: {}\nInstruction: {}\n,Input: {}\n",
            self.error_code,
            self.instruction,
            self.input
        )
    }
}

impl AtpError {
    pub fn new<I, T>(error_code: AtpErrorCode, instruction: I, input: T) -> Self
        where I: Into<Cow<'static, str>>, T: Into<Cow<'static, str>>
    {
        AtpError {
            error_code,
            instruction: instruction.into(),
            input: input.into(),
        }
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
#[derive(Clone, Debug, PartialEq)]
pub enum AtpErrorCode {
    FileNotFound(Cow<'static, str>),
    BytecodeParamNotRecognized(Cow<'static, str>),
    TokenNotFound(Cow<'static, str>),
    TokenArrayNotFound(Cow<'static, str>),
    BlockNotFound(Cow<'static, str>),
    VariableNotFound(Cow<'static, str>),
    NonMutableVariableError(Cow<'static, str>),
    FileReadingError(Cow<'static, str>),
    FileWritingError(Cow<'static, str>),
    FileOpeningError(Cow<'static, str>),
    BytecodeNotFound(Cow<'static, str>),
    TextParsingError(Cow<'static, str>),
    BytecodeParsingError(Cow<'static, str>),
    BytecodeParamParsingError(Cow<'static, str>),
    InvalidIndex(Cow<'static, str>),
    IndexOutOfRange(Cow<'static, str>),
    InvalidOperands(Cow<'static, str>),
    InvalidParameters(Cow<'static, str>),
    ValidationError(Cow<'static, str>),
    InvalidArgumentNumber(Cow<'static, str>),
    ZeroDivisionError(Cow<'static, str>),
}

impl Display for AtpErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n\tCódigo: {}\n\tMensagem: {}\n", self.get_error_code(), self.get_message())
    }
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
            Self::BlockNotFound(_) => 107,
            Self::VariableNotFound(_) => 108,
            Self::NonMutableVariableError(_) => 109,
            Self::InvalidOperands(_) => 200,
            Self::IndexOutOfRange(_) => 201,
            Self::InvalidIndex(_) => 202,
            Self::InvalidParameters(_) => 203,
            Self::InvalidArgumentNumber(_) => 204,
            Self::BytecodeParamNotRecognized(_) => 205,
            Self::TextParsingError(_) => 300,
            Self::BytecodeParsingError(_) => 301,
            Self::BytecodeParamParsingError(_) => 302,
            Self::ValidationError(_) => 303,
            Self::ZeroDivisionError(_) => 304,
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
            | Self::InvalidArgumentNumber(x)
            | Self::FileReadingError(x)
            | Self::FileWritingError(x)
            | Self::FileOpeningError(x)
            | Self::BytecodeParamParsingError(x)
            | Self::ZeroDivisionError(x)
            | Self::TokenArrayNotFound(x)
            | Self::BlockNotFound(x)
            | Self::VariableNotFound(x)
            | Self::NonMutableVariableError(x)
            | Self::BytecodeParamNotRecognized(x) => x.to_string(),
        }
    }
}

pub fn token_array_not_found(identifier: &str) -> impl Fn() -> AtpError {
    let message = AtpError::new(
        AtpErrorCode::TokenArrayNotFound(
            format!("Token array not found, is {} a valid identifier for this processor?", identifier).into()
        ),
        Cow::Borrowed("get identifier"),
        Cow::Borrowed("")
    );
    move || message.clone()
}

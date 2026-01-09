use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;

// Colored formatting (presentation only)
use colored::{ Color, Colorize };

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
        // Presentation only. No semantic changes.
        let title = "Erro".bold().red();
        let instr_label = "Instruction:".bold().cyan();
        let input_label = "Input:".bold().dimmed();

        write!(
            f,
            "{}\n{}\n{} {}\n{} {}",
            title,
            self.error_code,
            instr_label,
            self.instruction.as_ref().cyan(),
            input_label,
            self.input.as_ref().dimmed()
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

    // Cheap helpers (do not change public fields; just convenience)
    pub fn instruction_str(&self) -> &str {
        self.instruction.as_ref()
    }
    pub fn input_str(&self) -> &str {
        self.input.as_ref()
    }
}

impl ErrorManager {
    pub fn will_panic(&mut self, val: bool) {
        self.panic_with_error = val;
    }

    pub fn will_log(&mut self, val: bool) {
        self.show_warnings = val;

        // Tie "log mode" to color output by default.
        colored::control::set_override(val);
    }

    pub fn add_error(&mut self, err: AtpError) {
        self.error_vec.push(err);
    }

    pub fn has_errors(&self) -> bool {
        !self.error_vec.is_empty()
    }

    /// Optional: reserve capacity when you expect many errors in a big pipeline.
    pub fn reserve_errors(&mut self, additional: usize) {
        self.error_vec.reserve(additional);
    }

    /// Keeps existing API: prints to stdout.
    /// Internally uses a writer so it can be unit-tested and buffered.
    pub fn print_errors(&self) {
        use std::io::{ self, BufWriter };
        let stdout = io::stdout();
        let mut w = BufWriter::new(stdout.lock());
        let _ = self.write_errors(&mut w);
    }

    /// Internal helper: write formatted errors into any writer (test-friendly).
    fn write_errors<W: std::io::Write>(&self, mut w: W) -> std::io::Result<()> {
        for (index, error) in self.error_vec.iter().enumerate() {
            writeln!(
                w,
                "[{}] {} {}\n\t{} {}\n\t{} {}\n\t{} {}",
                index.to_string().blue(),
                "Code:".bold().red(),
                error.error_code.get_error_code().to_string().red().bold(),
                "Instruction:".bold().cyan(),
                error.instruction.as_ref().cyan(),
                "Message:".bold().yellow(),
                // IMPORTANT: avoid allocating; get_message returns &Cow now.
                error.error_code.get_message().as_ref().yellow(),
                "Processing:".bold().dimmed(),
                error.input.as_ref().dimmed()
            )?;
        }
        Ok(())
    }

    /// Optional: consistent behavior when you want to both store and act on an error.
    /// Does not change any existing behavior unless you choose to use it.
    pub fn handle_error(&mut self, err: AtpError) {
        self.add_error(err.clone());

        if self.show_warnings {
            println!("{err}");
        }

        if self.panic_with_error {
            panic!("{err}");
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
        // Use severity to color the code/message, but keep raw values available via getters.
        let severity = self.severity_color();

        let code_label = "Código:".bold().red();
        let msg_label = "Mensagem:".bold().yellow();

        let code_val = self.get_error_code().to_string().color(severity).bold();
        let msg_val = self.message().as_ref().color(severity);

        write!(f, "{} {}\n{} {}", code_label, code_val, msg_label, msg_val)
    }
}

impl AtpErrorCode {
    pub fn get_error_code(&self) -> u16 {
        match self {
            Self::FileNotFound(_) => 1u16,
            Self::TokenNotFound(_) => 2u16,
            Self::TokenArrayNotFound(_) => 3u16,
            Self::FileReadingError(_) => 4u16,
            Self::FileWritingError(_) => 5u16,
            Self::FileOpeningError(_) => 6u16,
            Self::BytecodeNotFound(_) => 7u16,
            Self::BlockNotFound(_) => 8u16,
            Self::VariableNotFound(_) => 9u16,
            Self::NonMutableVariableError(_) => 10u16,
            Self::InvalidOperands(_) => 11u16,
            Self::IndexOutOfRange(_) => 12u16,
            Self::InvalidIndex(_) => 13u16,
            Self::InvalidParameters(_) => 14u16,
            Self::InvalidArgumentNumber(_) => 15u16,
            Self::BytecodeParamNotRecognized(_) => 16u16,
            Self::TextParsingError(_) => 17u16,
            Self::BytecodeParsingError(_) => 18u16,
            Self::BytecodeParamParsingError(_) => 19u16,
            Self::ValidationError(_) => 20u16,
            Self::ZeroDivisionError(_) => 21u16,
        }
    }

    /// Kept name, but now returns borrowed data (no allocation).
    pub fn get_message(&self) -> &Cow<'static, str> {
        self.message()
    }

    /// Zero-allocation access to the inner message.
    pub fn message(&self) -> &Cow<'static, str> {
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
            | Self::BytecodeParamNotRecognized(x) => x,
        }
    }

    fn severity_color(&self) -> Color {
        match self {
            // "Hard" errors
            | Self::ZeroDivisionError(_)
            | Self::InvalidOperands(_)
            | Self::ValidationError(_)
            | Self::InvalidParameters(_)
            | Self::InvalidArgumentNumber(_)
            | Self::BytecodeParsingError(_)
            | Self::BytecodeParamParsingError(_)
            | Self::TextParsingError(_) => Color::Red,

            // "Missing things" / lookup failures
            | Self::FileNotFound(_)
            | Self::FileOpeningError(_)
            | Self::FileReadingError(_)
            | Self::FileWritingError(_)
            | Self::TokenNotFound(_)
            | Self::TokenArrayNotFound(_)
            | Self::BlockNotFound(_)
            | Self::VariableNotFound(_)
            | Self::NonMutableVariableError(_)
            | Self::BytecodeNotFound(_)
            | Self::BytecodeParamNotRecognized(_) => Color::Yellow,

            // Index-ish
            Self::InvalidIndex(_) | Self::IndexOutOfRange(_) => Color::Magenta,
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

#[cfg(feature = "test_access")]
#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn atp_error_display_no_weird_comma_and_contains_sections() {
        colored::control::set_override(false); // stable tests (no ANSI)
        let err = AtpError::new(
            AtpErrorCode::InvalidIndex(Cow::Borrowed("index inválido")),
            "dlc",
            "banana"
        );

        let s = format!("{err}");
        assert!(s.contains("Erro"));
        assert!(s.contains("Código:"));
        assert!(s.contains("Mensagem:"));
        assert!(s.contains("Instruction:"));
        assert!(s.contains("Input:"));
        assert!(!s.contains("\n,Input"), "Não deve existir '\\n,Input' no Display");
    }

    #[test]
    fn error_code_message_borrowed_and_get_message_matches() {
        let code = AtpErrorCode::TokenNotFound(Cow::Borrowed("no token"));
        assert_eq!(code.message().as_ref(), "no token");
        assert_eq!(code.get_message().as_ref(), "no token");
    }

    #[test]
    fn print_errors_writer_helper_is_testable() {
        colored::control::set_override(false);

        let mut mgr = ErrorManager::default();
        mgr.add_error(AtpError::new(AtpErrorCode::TokenNotFound(Cow::Borrowed("x")), "inst", "in"));

        let mut buf: Vec<u8> = vec![];
        mgr.write_errors(&mut buf).unwrap();

        let out = String::from_utf8(buf).unwrap();
        assert!(out.contains("Code:"));
        assert!(out.contains("Instruction:"));
        assert!(out.contains("Message:"));
        assert!(out.contains("Processing:"));
    }

    #[test]
    fn token_array_not_found_closure_returns_stable_clone() {
        colored::control::set_override(false);

        let f = token_array_not_found("abc");
        let e1 = f();
        let e2 = f();

        assert_eq!(e1, e2);
        assert_eq!(e1.instruction, Cow::Borrowed("get identifier"));
        assert_eq!(e1.input, Cow::Borrowed(""));

        match &e1.error_code {
            AtpErrorCode::TokenArrayNotFound(m) => {
                assert!(m.contains("Token array not found"));
                assert!(m.contains("abc"));
            }
            _ => panic!("Esperado TokenArrayNotFound"),
        }
    }

    #[test]
    fn get_error_code_is_u16_and_matches_expected_values() {
        assert_eq!(AtpErrorCode::FileNotFound(Cow::Borrowed("x")).get_error_code(), 1u16);
        assert_eq!(AtpErrorCode::ZeroDivisionError(Cow::Borrowed("x")).get_error_code(), 21u16);
    }
}

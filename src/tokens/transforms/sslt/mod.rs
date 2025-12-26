#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use regex::Regex;

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;
use crate::utils::validations::check_vec_len;
use crate::{ tokens::TokenMethods, utils::transforms::string_to_usize };

use crate::utils::errors::{ AtpError, AtpErrorCode };

/// SSLT - Split Select
///
/// Splits `input` by `pattern and return `index` of the resulting vec,
/// *discarding* the rest of the text in the process.
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::sslt::Sslt};
///
/// let token = Sslt::params("_", 1).unwrap();
///
/// assert_eq!(token.transform("foobar_foo_bar_bar_foo_barfoo"), Ok("foo".to_string()));
///
/// ```
#[derive(Clone)]
pub struct Sslt {
    pub pattern: Regex,
    pub index: usize,
}

impl Sslt {
    pub fn params(pattern: &str, index: usize) -> Result<Self, AtpError> {
        let pattern = Regex::new(&pattern).map_err(|e|
            AtpError::new(AtpErrorCode::BytecodeParsingError(e.to_string().into()), "", "")
        )?;
        Ok(Sslt {
            pattern,
            index,
        })
    }
}

impl Default for Sslt {
    fn default() -> Self {
        Sslt {
            pattern: Regex::new("").unwrap(),
            index: 0,
        }
    }
}

impl TokenMethods for Sslt {
    fn get_string_repr(&self) -> &'static str {
        "sslt"
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let item = self.pattern
            .split(input)
            .nth(self.index)
            .ok_or_else(|| {
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        "Index does not exist in the splitted vec".into()
                    ),
                    self.to_atp_line(),
                    input.to_string()
                )
            })?;

        Ok(item.to_string())
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        check_vec_len(&line, 3)?;
        if line[0] == "sslt" {
            self.pattern = Regex::new(&line[1]).map_err(|_|
                AtpError::new(
                    AtpErrorCode::TextParsingError("Failed to create regex".into()),
                    "sslt",
                    line[1].to_string()
                )
            )?;
            self.index = string_to_usize(&line[2])?;
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid parser for this token".into()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("sslt {} {};\n", self.pattern, self.index).into()
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x1a
    }
    #[cfg(feature = "bytecode")]
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        if instruction.len() != 2 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );
        }

        self.index = parse_args!(instruction, 0, Usize, "Index should be of type Usize");

        let pattern_payload = parse_args!(
            instruction,
            1,
            String,
            "Pattern should be of string type"
        );

        self.pattern = Regex::new(&pattern_payload.clone()).map_err(|_|
            AtpError::new(
                AtpErrorCode::TextParsingError("Failed to create regex".into()),
                "sslt",
                pattern_payload.clone()
            )
        )?;

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::Usize(self.index),
            AtpParamTypes::String(self.pattern.to_string()),
        ]);
        result
    }
}

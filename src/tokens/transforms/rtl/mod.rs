#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;
use crate::{ tokens::TokenMethods, utils::transforms::string_to_usize };

use crate::utils::errors::{ AtpError, AtpErrorCode };

/// RTL - Rotate Left
///
/// Rotates `input` to the left `n` times
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::rtl::Rtl};
///
/// let token = Rtl::params(3);
///
/// assert_eq!(token.transform("banana"),Ok("anaban".to_string()));
///
/// ```
#[derive(Clone, Default)]
pub struct Rtl {
    pub times: usize,
}

impl Rtl {
    pub fn params(times: usize) -> Rtl {
        Rtl {
            times,
        }
    }
}

impl TokenMethods for Rtl {
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        if input.is_empty() {
            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidParameters("Input is empty".into()),
                    self.to_atp_line(),
                    "\" \""
                )
            );
        }

        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();
        let times = self.times % len;

        Ok(
            chars[times..]
                .iter()
                .chain(&chars[..times])
                .collect()
        )
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("rtl {};\n", self.times).into()
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "rtl" {
            self.times = string_to_usize(&line[1])?;
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

    fn get_string_repr(&self) -> &'static str {
        "rtl"
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x0e
    }
    #[cfg(feature = "bytecode")]
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        if instruction.len() != 1 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );
        }

        self.times = parse_args!(instruction, 0, Usize, "Index should be of usize type");

        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [AtpParamTypes::Usize(self.times)]);
        result
    }
}

#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::tokens::TokenMethods;

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;
use crate::utils::errors::{ AtpError, AtpErrorCode };

/// SPLC - Split Characters
///
/// Split `input` characters in a result whose chars are separed by spaces
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::splc::Splc};
///
/// let token = Splc::default();
///
/// assert_eq!(token.transform("banana"), Ok("b a n a n a".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Splc {}

impl TokenMethods for Splc {
    fn get_string_repr(&self) -> &'static str {
        "splc"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        "splc;\n".into()
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "splc" {
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
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(
            input
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x23
    }
    #[cfg(feature = "bytecode")]
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        if instruction.len() == 0 {
            return Ok(());
        } else {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );
        }
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

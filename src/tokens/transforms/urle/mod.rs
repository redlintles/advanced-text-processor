#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::{ utils::params::AtpParamTypes };

/// URLE - URL Encode
///
/// Encodes `input` to the URL Encoding Format
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::urle::Urle};
///
/// let token = Urle::default();
///
/// assert_eq!(token.transform("banana laranja"), Ok("banana%20laranja".to_string()));
/// ```
///
#[derive(Copy, Clone, Default)]
pub struct Urle {}

impl TokenMethods for Urle {
    fn get_string_repr(&self) -> &'static str {
        "urle"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "urle;\n".into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(urlencoding::encode(input).to_string())
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "urle" {
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
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x20
    }
    #[cfg(feature = "bytecode")]
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        if instruction.len() == 0 {
            return Ok(());
        } else {
            Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            )
        }
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

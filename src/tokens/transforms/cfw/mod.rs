#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::capitalize },
};

#[cfg(feature = "bytecode")]
use crate::{ utils::params::AtpParamTypes };
/// Token `Cfw` — Capitalize First Word
///
/// Capitalizes the first word of `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::cfw::Cfw};
///
/// let token = Cfw::default();
/// assert_eq!(token.transform("foo bar"), Ok("Foo bar".to_string()));
/// ```
#[derive(Copy, Clone, Default)]
pub struct Cfw {}

impl TokenMethods for Cfw {
    fn get_string_repr(&self) -> &'static str {
        "cfw"
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "cfw" {
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
        Ok(capitalize(input))
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "cfw;\n".into()
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x18
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

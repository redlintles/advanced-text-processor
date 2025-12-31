#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::errors::{AtpError, AtpErrorCode},
};

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;

/// TLS - Trim left sides
///
/// Trim the right side of `input`, removing all spaces from the end
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::trs::Trs};
///
/// let token = Trs::default();
///
/// assert_eq!(token.transform("   banana   "), Ok("   banana".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Trs {}

impl TokenMethods for Trs {
    fn to_atp_line(&self) -> Cow<'static, str> {
        "trs;\n".into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(String::from(input.trim_end()))
    }

    fn get_string_repr(&self) -> &'static str {
        "trs"
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x07
    }
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        if instruction.len() == 0 {
            return Ok(());
        } else {
            Err(AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                "",
                "",
            ))
        }
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;

use crate::{
    tokens::TokenMethods,
    utils::errors::{AtpError, AtpErrorCode},
};

/// Rev - Reverse
///
/// Reverses `input` character order
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::rev::Rev};
///
/// let token = Rev::default();
/// assert_eq!(token.transform("foobar"), Ok("raboof".to_string()));
/// ``````
#[derive(Clone, Default, Copy)]
pub struct Rev {}

impl TokenMethods for Rev {
    fn get_string_repr(&self) -> &'static str {
        "rev"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        "rev;\n".into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.chars().rev().collect())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x22
    }
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        if instruction.len() == 0 {
            return Ok(());
        } else {
            return Err(AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                "",
                "",
            ));
        }
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

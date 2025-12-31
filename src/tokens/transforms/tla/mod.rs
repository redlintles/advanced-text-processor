#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::errors::{AtpError, AtpErrorCode},
};

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;
/// TLA - To Lowercase All
///
/// Lowercases every character from `input`
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::tla::Tla};
///
/// let token = Tla::default();
///
/// assert_eq!(token.transform("BANANA"), Ok("banana".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Tla {}

impl TokenMethods for Tla {
    fn get_string_repr(&self) -> &'static str {
        "tla"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "tla;\n".into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.to_lowercase())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x13
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

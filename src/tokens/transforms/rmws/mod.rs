#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::errors::{AtpError, AtpErrorCode},
};

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;

/// RMWS - Remove Whitespace
///
/// Removes all whitespaces in `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::rmws::Rmws};
///
/// let token = Rmws::default();
///
/// assert_eq!(token.transform("banana laranja cheia de canja"), Ok("bananalaranjacheiadecanja".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Rmws {}

impl TokenMethods for Rmws {
    fn get_string_repr(&self) -> &'static str {
        "rmws"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        "rmws;\n".into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.split_whitespace().collect::<Vec<_>>().join(""))
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x31
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

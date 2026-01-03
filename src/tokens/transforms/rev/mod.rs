#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::utils::params::AtpParamTypes;

use crate::utils::validations::check_vec_len;
use crate::{ tokens::TokenMethods, utils::errors::{ AtpError } };

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
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "rev", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x22
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

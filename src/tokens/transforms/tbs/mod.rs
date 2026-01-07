#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{ tokens::InstructionMethods, utils::{ errors::{ AtpError }, validations::check_vec_len } };

use crate::utils::params::AtpParamTypes;
/// TBS - Trim both sides
///
/// Trim Both Sides of `input`, removing all spaces from both the beginning and the end
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::tbs::Tbs};
///
/// let token = Tbs::default();
///
/// assert_eq!(token.transform("   banana   "), Ok("banana".to_string()));
/// ```
///
#[derive(Clone, Default)]
pub struct Tbs {}

impl InstructionMethods for Tbs {
    fn to_atp_line(&self) -> Cow<'static, str> {
        "tbs;\n".into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(String::from(input.trim()))
    }

    fn get_string_repr(&self) -> &'static str {
        "tbs"
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "tbs", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x05
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

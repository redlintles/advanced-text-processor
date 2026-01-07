#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{ tokens::InstructionMethods, utils::{ errors::{ AtpError }, validations::check_vec_len } };

use crate::utils::params::AtpParamTypes;

/// RMWS - Remove Whitespace
///
/// Removes all whitespaces in `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::rmws::Rmws};
///
/// let token = Rmws::default();
///
/// assert_eq!(token.transform("banana laranja cheia de canja"), Ok("bananalaranjacheiadecanja".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Rmws {}

impl InstructionMethods for Rmws {
    fn get_string_repr(&self) -> &'static str {
        "rmws"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        "rmws;\n".into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.split_whitespace().collect::<Vec<_>>().join(""))
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "rmws", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x31
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

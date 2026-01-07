#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{ tokens::InstructionMethods, utils::{ errors::{ AtpError }, validations::check_vec_len } };

use crate::utils::params::AtpParamTypes;

/// URLE - URL Encode
///
/// Encodes `input` to the URL Encoding Format
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::urle::Urle};
///
/// let token = Urle::default();
///
/// assert_eq!(token.transform("banana laranja"), Ok("banana%20laranja".to_string()));
/// ```
///
#[derive(Copy, Clone, Default)]
pub struct Urle {}

impl InstructionMethods for Urle {
    fn get_string_repr(&self) -> &'static str {
        "urle"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "urle;\n".into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(urlencoding::encode(input).to_string())
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "urle", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x20
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

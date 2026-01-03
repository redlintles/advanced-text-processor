#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::{ errors::{ AtpError }, validations::check_vec_len } };

use crate::utils::params::AtpParamTypes;

#[derive(Clone, Copy, Default)]
pub struct Tua {}

impl TokenMethods for Tua {
    fn get_string_repr(&self) -> &'static str {
        "tua"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "tua;\n".into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.to_uppercase())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x12
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "tua", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

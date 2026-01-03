#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{ errors::{ AtpError }, transforms::capitalize, validations::check_vec_len },
};

use crate::utils::params::AtpParamTypes;
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
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(capitalize(input))
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "cfw;\n".into()
    }

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::utils::params::AtpParamTypesJoin;

        check_vec_len(&params, 0, "cfw", params.join(""))?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x18
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

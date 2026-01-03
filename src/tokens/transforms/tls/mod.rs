#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::tokens::TokenMethods;

use crate::utils::errors::{ AtpError };

use crate::utils::params::AtpParamTypes;
use crate::utils::validations::check_vec_len;

/// TLS - Trim left sides
///
/// Trim the left Side of `input`, removing all spaces from the beginning
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::tls::Tls};
///
/// let token = Tls::default();
///
/// assert_eq!(token.transform("   banana   "), Ok("banana   ".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Tls {}

impl TokenMethods for Tls {
    fn to_atp_line(&self) -> Cow<'static, str> {
        "tls;\n".into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(String::from(input.trim_start()))
    }

    fn get_string_repr(&self) -> &'static str {
        "tls"
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "tls", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x06
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

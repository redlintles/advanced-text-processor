#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::tokens::TokenMethods;

use crate::utils::errors::{ AtpError };
use crate::utils::params::AtpParamTypes;
use crate::utils::validations::check_vec_len;

/// SPLC - Split Characters
///
/// Split `input` characters in a result whose chars are separed by spaces
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::splc::Splc};
///
/// let token = Splc::default();
///
/// assert_eq!(token.transform("banana"), Ok("b a n a n a".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Splc {}

impl TokenMethods for Splc {
    fn get_string_repr(&self) -> &'static str {
        "splc"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        "splc;\n".into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(
            input
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "rmws", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x23
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{
        errors::{AtpError, AtpErrorCode},
        transforms::capitalize,
        validations::check_vec_len,
    },
};

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;
/// Token `Clw` — Capitalize Last Word
///
/// Capitalizes the last word of `input`
///
/// This is achieved by splitting the input by whitespace, reversing the resulting vector,
/// capitalizing the first word, reversing it back, and rejoining into a single string.
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::clw::Clw};
///
/// let token = Clw::default();
/// assert_eq!(token.transform("foo bar"), Ok("foo Bar".to_string()));
/// ```
#[derive(Copy, Clone, Default)]
pub struct Clw {}

impl TokenMethods for Clw {
    fn get_string_repr(&self) -> &'static str {
        "clw"
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let mut v: Vec<String> = input
            .split(' ')
            .rev()
            .enumerate()
            .map(|(i, c)| if i == 0 { capitalize(c) } else { c.to_string() })
            .collect::<Vec<_>>();

        v.reverse();
        Ok(v.join(" "))
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "clw;\n".into()
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x19
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::utils::params::AtpParamTypesJoin;

        check_vec_len(&params, 1, "atb", params.join(""));
        if params.len() == 0 {
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

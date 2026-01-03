#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, validations::check_vec_len },
};

use crate::utils::params::AtpParamTypes;
/// URLD - URL Decode
///
/// Decodes `input` from the URL Encoding Format
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::urld::Urld};
///
/// let token = Urld::default();
///
/// assert_eq!(token.transform("banana%20laranja"), Ok("banana laranja".to_string()));
/// ```
///

#[derive(Copy, Clone, Default)]
pub struct Urld {}

impl TokenMethods for Urld {
    fn get_string_repr(&self) -> &'static str {
        "urld"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "urld;\n".into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(
            urlencoding
                ::decode(input)
                .map_err(|_| {
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed parsing URL string".into()),
                        "urld",
                        input.to_string()
                    )
                })?
                .to_string()
        )
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x21
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "urld", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

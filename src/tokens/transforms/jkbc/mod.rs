#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::{ errors::{ AtpError }, validations::check_vec_len } };

use crate::utils::params::AtpParamTypes;

/// JKBC - Join to Kebab Case
///
/// If `input` is a string whose words are separated by spaces, join `input` as a kebab-case lowercased string
/// For conversion between the screaming and normal versions of this case style, use the tokens TLA(To Lowercase All) and TUA(To Uppercase All) together with this one.
///
/// See Also:
///
/// - [`Tua` - To Uppercase All](crate::tokens::transforms::tua)
/// - [`Tla` - To Lowercase All](crate::tokens::transforms::tla)
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::jkbc::Jkbc};
///
/// let token = Jkbc::default();
///
/// assert_eq!(token.transform("banana laranja cheia de canja"), Ok("banana-laranja-cheia-de-canja".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Jkbc {}

impl TokenMethods for Jkbc {
    fn get_string_repr(&self) -> &'static str {
        "jkbc"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "jkbc;\n".into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.split_whitespace().collect::<Vec<_>>().join("-").to_lowercase())
    }

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "jkbc", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x2b
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

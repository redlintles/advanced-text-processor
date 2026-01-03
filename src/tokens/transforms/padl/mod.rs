#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{ errors::{ AtpError }, transforms::extend_string, validations::check_vec_len },
};

use crate::parse_args;

use crate::utils::params::AtpParamTypes;

/// PADL - Pad Left
///
/// Repeats `text` characters until `max_len` is reached, and then insert the result at the beggining of `input`
///
/// See Also:
///
/// - [`Padr` - Pad Right](crate::tokens::transforms::padr)
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::padl::Padl};
///
/// let token = Padl::params("xy", 7);
///
/// assert_eq!(token.transform("banana"), Ok("xbanana".to_string()));
/// ```
#[derive(Clone, Default)]
pub struct Padl {
    pub text: String,
    pub max_len: usize,
}

impl Padl {
    pub fn params(text: &str, max_len: usize) -> Self {
        Padl {
            text: text.to_string(),
            max_len,
        }
    }
}

impl TokenMethods for Padl {
    fn get_string_repr(&self) -> &'static str {
        "padl"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("padl {} {};\n", self.text, self.max_len).into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let character_count = input.chars().count();

        if character_count >= self.max_len {
            return Ok(input.to_string());
        }
        let ml = self.max_len - character_count;
        let s = extend_string(&self.text, ml);

        Ok(format!("{}{}", s, input))
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 2, "padl", "")?;

        self.text = parse_args!(params, 0, String, "Text_to_insert should be of String type");
        self.max_len = parse_args!(params, 1, Usize, "Index should be of usize type");

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x2f
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::String(self.text.clone()),
            AtpParamTypes::Usize(self.max_len),
        ]);
        result
    }
}

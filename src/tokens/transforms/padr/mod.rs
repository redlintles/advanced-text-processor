#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::extend_string },
};

use crate::utils::params::AtpParamTypes;
/// PADR - Pad Right
///
/// Repeats `text` characters until `max_len` is reached, and then insert the result at the end of `input`
///
/// See Also:
///
/// - [`Padr` - Pad Left](crate::tokens::transforms::padr)
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::padr::Padr};
///
/// let token = Padr::params("xy", 7);
///
/// assert_eq!(token.transform("banana"), Ok("bananax".to_string()));
/// ```
#[derive(Clone, Default)]
pub struct Padr {
    pub text: String,
    pub max_len: usize,
}

impl Padr {
    pub fn params(text: &str, max_len: usize) -> Self {
        Padr {
            text: text.to_string(),
            max_len,
        }
    }
}

impl TokenMethods for Padr {
    fn get_string_repr(&self) -> &'static str {
        "padr"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("padr {} {};\n", self.text, self.max_len).into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let character_count = input.chars().count();

        if character_count >= self.max_len {
            return Ok(input.to_string());
        }
        let ml = self.max_len - character_count;
        let s = extend_string(&self.text, ml);

        Ok(format!("{}{}", input, s))
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x30
    }
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        if instruction.len() != 2 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );
        }

        self.text = parse_args!(instruction, 0, String, "Text_to_insert should be of String type");
        self.max_len = parse_args!(instruction, 1, Usize, "Index should be of usize type");

        return Ok(());
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

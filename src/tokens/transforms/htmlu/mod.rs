#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use html_escape::decode_html_entities;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::{ utils::params::AtpParamTypes };

/// HTMLU - HTML Unescape
///
/// Unescapes Special HTML Entities in `input` to their corresponding characters
/// Used when some HTML text is gonna be processed as a normal string
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::htmlu::Htmlu};
///
/// let token = Htmlu::default();
///
/// assert_eq!(token.transform("&lt;div&gt;banana&lt;/div&gt;"), Ok("<div>banana</div>".to_string()));
/// ```
#[derive(Copy, Clone, Default)]
pub struct Htmlu {}

impl TokenMethods for Htmlu {
    fn get_string_repr(&self) -> &'static str {
        "htmlu"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "htmlu;\n".into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(decode_html_entities(input).to_string())
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "htmlu" {
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid parser for this token".into()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x25
    }
    #[cfg(feature = "bytecode")]
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        if instruction.len() == 0 {
            return Ok(());
        } else {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );
        }
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

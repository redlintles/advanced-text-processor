#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use html_escape::encode_text;

use crate::{
    tokens::TokenMethods,
    utils::errors::{AtpError, AtpErrorCode},
};

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;

/// HTMLE - HTML Escape
///
/// Escapes Special HTML Characters in `input` to HTML Entities
/// So they can be rendered correctly later
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::htmle::Htmle};
///
/// let token = Htmle::default();
///
/// assert_eq!(token.transform("<div>banana</div>"), Ok("&lt;div&gt;banana&lt;/div&gt;".to_string()));
/// ```

#[derive(Copy, Clone, Default)]
pub struct Htmle {}

impl TokenMethods for Htmle {
    fn get_string_repr(&self) -> &'static str {
        "htmle"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "htmle;\n".into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(encode_text(input).to_string())
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x24
    }
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        if instruction.len() == 0 {
            return Ok(());
        } else {
            return Err(AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                "",
                "",
            ));
        }
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

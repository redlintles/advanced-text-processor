#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use html_escape::encode_text;

use crate::{ tokens::InstructionMethods, utils::{ errors::{ AtpError }, validations::check_vec_len } };

use crate::utils::params::AtpParamTypes;

/// HTMLE - HTML Escape
///
/// Escapes Special HTML Characters in `input` to HTML Entities
/// So they can be rendered correctly later
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::htmle::Htmle};
///
/// let token = Htmle::default();
///
/// assert_eq!(token.transform("<div>banana</div>"), Ok("&lt;div&gt;banana&lt;/div&gt;".to_string()));
/// ```

#[derive(Copy, Clone, Default)]
pub struct Htmle {}

impl InstructionMethods for Htmle {
    fn get_string_repr(&self) -> &'static str {
        "htmle"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "htmle;\n".into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(encode_text(input).to_string())
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "dlf", "")?;
        Ok(())
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x24
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

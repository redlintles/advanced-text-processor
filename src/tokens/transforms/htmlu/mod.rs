#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use html_escape::decode_html_entities;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{ errors::AtpError, validations::check_vec_len },
};

use crate::utils::params::AtpParamTypes;

/// HTMLU - HTML Unescape
///
/// Unescapes Special HTML Entities in `input` to their corresponding characters
/// Used when some HTML text is gonna be processed as a normal string
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::htmlu::Htmlu};
///
/// let token = Htmlu::default();
///
/// assert_eq!(token.transform("&lt;div&gt;banana&lt;/div&gt;"), Ok("<div>banana</div>".to_string()));
/// ```
#[derive(Copy, Clone, Default)]
pub struct Htmlu {}

impl InstructionMethods for Htmlu {
    fn get_string_repr(&self) -> &'static str {
        "htmlu"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "htmlu;\n".into()
    }
    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        Ok(decode_html_entities(input).to_string())
    }

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "dlf", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x25
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

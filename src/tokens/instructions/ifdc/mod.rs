#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

#[cfg(feature = "bytecode")]
use crate::to_bytecode;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::{ InstructionMethods, transforms::dlf::Dlf },
};

use crate::utils::errors::{ AtpError };

use crate::utils::params::AtpParamTypes;

/// Ifdc - If Do Contains
///
/// if `input` contains `text`, the `inner` token will be executed, otherwise `input` is returned with no changes
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, instructions::ifdc::Ifdc};
///
/// let token = Ifdc::params("xy", "atb laranja;");
///
/// assert_eq!(token.transform("larryxy"), Ok("laranjalarryxy".to_string())); // Adds laranja to the beginning
/// assert_eq!(token.transform("banana"), Ok("banana".to_string())); // Does nothing
///
/// ```
#[derive(Clone)]
pub struct Ifdc {
    text: String,
    inner: Box<dyn InstructionMethods>,
}

impl Default for Ifdc {
    fn default() -> Self {
        Ifdc {
            text: "teste".to_string(),
            inner: Box::new(Dlf::default()),
        }
    }
}

impl Ifdc {
    pub fn params(text: &str, inner: Box<dyn InstructionMethods>) -> Self {
        Ifdc {
            text: text.to_string(),
            inner,
        }
    }
}

impl InstructionMethods for Ifdc {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("ifdc {} do {}", self.text, self.inner.to_atp_line()).into()
    }

    fn get_string_repr(&self) -> &'static str {
        "ifdc"
    }

    fn transform(&self, input: &str, c: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        if input.contains(&self.text) {
            return Ok(self.inner.transform(input, &mut *c)?);
        }

        Ok(input.to_string())
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x33
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::{ parse_args, utils::validations::check_vec_len };

        use crate::utils::params::AtpParamTypesJoin;

        check_vec_len(&params, 2, "ifdc", params.join(""))?;

        self.text = parse_args!(params, 0, String, "");

        self.inner = parse_args!(params, 1, Token, "");

        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        let result = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::String(self.text.clone()),
            AtpParamTypes::Token(self.inner.clone()),
        ]);

        result
    }
}

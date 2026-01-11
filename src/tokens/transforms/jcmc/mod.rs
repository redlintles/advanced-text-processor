#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{ errors::AtpError, transforms::capitalize, validations::check_vec_len },
};

use crate::utils::params::AtpParamTypes;

/// JCMC - Join to Camel Case
///
/// If `input` is a string whose words are separated by spaces, join `input` as a camelCase string
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::jcmc::Jcmc};
///
/// let token = Jcmc::default();
///
/// assert_eq!(token.transform("banana laranja cheia de canja"), Ok("bananaLaranjaCheiaDeCanja".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Jcmc {}

impl InstructionMethods for Jcmc {
    fn get_string_repr(&self) -> &'static str {
        "jcmc"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "jcmc;\n".into()
    }

    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        let v = input.split_whitespace().collect::<Vec<_>>();

        let processed = v
            .iter()
            .enumerate()
            .map(|(i, w)| if i >= 1 { capitalize(w) } else { w.to_string() })
            .collect::<Vec<_>>()
            .join("");

        Ok(processed)
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x2d
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "jcmc", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

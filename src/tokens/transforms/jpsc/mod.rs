#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{ errors::AtpError, transforms::capitalize, validations::check_vec_len },
};

use crate::utils::params::AtpParamTypes;

/// JPSC - Join to PascalCase
///
/// If `input` is a string whose words are separated by spaces, join `input` as a PascalCase string
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::jpsc::Jpsc};
///
/// let token = Jpsc::default();
///
/// assert_eq!(token.transform("banana laranja cheia de canja"), Ok("BananaLaranjaCheiaDeCanja".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Jpsc {}

impl InstructionMethods for Jpsc {
    fn get_string_repr(&self) -> &'static str {
        "jpsc"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "jpsc;\n".into()
    }

    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        let v = input.split_whitespace().collect::<Vec<_>>();

        let processed = v
            .iter()
            .map(|w| capitalize(w))
            .collect::<Vec<_>>()
            .join("");

        Ok(processed)
    }

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "jpsc", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x2e
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

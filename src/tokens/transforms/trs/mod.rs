#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{ errors::AtpError, validations::check_vec_len },
};

use crate::utils::params::AtpParamTypes;

/// TLS - Trim left sides
///
/// Trim the right side of `input`, removing all spaces from the end
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::trs::Trs};
///
/// let token = Trs::default();
///
/// assert_eq!(token.transform("   banana   "), Ok("   banana".to_string()));
/// ```
///
#[derive(Clone, Default)]
pub struct Trs {
    params: Vec<AtpParamTypes>,
}

impl InstructionMethods for Trs {
    fn get_params(&self) -> &Vec<AtpParamTypes> {
        &self.params
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        "trs;\n".into()
    }

    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        Ok(String::from(input.trim_end()))
    }

    fn get_string_repr(&self) -> &'static str {
        "trs"
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x07
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "trs", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

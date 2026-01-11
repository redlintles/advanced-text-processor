#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{ errors::AtpError, validations::check_vec_len },
};

use crate::utils::params::AtpParamTypes;

/// DLL - Delete Last
///
/// Deletes the last character of `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::dll::Dll};
///
/// let token = Dll::default();
///
/// assert_eq!(token.transform("banana"), Ok("banan".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Dll {}

impl InstructionMethods for Dll {
    fn to_atp_line(&self) -> Cow<'static, str> {
        "dll;\n".into()
    }

    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        let mut s = String::from(input);

        if let Some((x, _)) = s.char_indices().next_back() {
            s.drain(x..);
        }

        Ok(s)
    }

    fn get_string_repr(&self) -> &'static str {
        "dll"
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "dll", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x04
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

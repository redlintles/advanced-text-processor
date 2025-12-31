#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::errors::{AtpError, AtpErrorCode},
};

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;

/// DLL - Delete Last
///
/// Deletes the last character of `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::dll::Dll};
///
/// let token = Dll::default();
///
/// assert_eq!(token.transform("banana"), Ok("banan".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Dll {}

impl TokenMethods for Dll {
    fn to_atp_line(&self) -> Cow<'static, str> {
        "dll;\n".into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let mut s = String::from(input);

        if let Some((x, _)) = s.char_indices().next_back() {
            s.drain(x..);
        }

        Ok(s)
    }

    fn get_string_repr(&self) -> &'static str {
        "dll"
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x04
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

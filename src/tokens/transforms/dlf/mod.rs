#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

use crate::utils::params::AtpParamTypes;
/// DLF - Delete First
///
/// Deletes the first character of `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::dlf::Dlf};
///
/// let token = Dlf::default();
///
/// assert_eq!(token.transform("banana"), Ok("anana".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Dlf {}

impl TokenMethods for Dlf {
    fn to_atp_line(&self) -> Cow<'static, str> {
        "dlf;\n".into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        // Se a string é vazia, não há o que deletar.
        if input.is_empty() {
            return Ok(String::new());
        }

        // Encontra o byte-index do início do 2º caractere (se existir).
        // Se não existir, a string tem 1 char só => resultado é vazio.
        let cut = input
            .char_indices()
            .nth(1)
            .map(|(byte_idx, _)| byte_idx)
            .unwrap_or(input.len());

        Ok(input[cut..].to_string())
    }

    fn get_string_repr(&self) -> &'static str {
        "dlf"
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x03
    }
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

#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{ tokens::TokenMethods };

#[cfg(feature = "bytecode")]
use crate::{ utils::params::AtpParamTypes };

use crate::utils::errors::{ AtpError, AtpErrorCode };

/// Jsone - Json Escape
///
/// Escapes JSON Special Characters in `input` with serde_json::to_string
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::jsone::Jsone};
///
///
/// let token = Jsone::default();
/// let expected_output = "\"{banana: '10'}\"".to_string();
///
/// assert_eq!(token.transform("{banana: '10'}"), Ok(expected_output));
/// ```

#[derive(Clone, Copy, Default)]
pub struct Jsone {}

impl TokenMethods for Jsone {
    fn get_string_repr(&self) -> &'static str {
        "jsone"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        "jsone;\n".into()
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "jsone" {
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid parser for this token".into()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(
            serde_json
                ::to_string(input)
                .map_err(|_|
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed to serialize to JSON".into()),
                        "serde_json::to_string".to_string(),
                        input.to_string()
                    )
                )?
        )
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x26
    }
    #[cfg(feature = "bytecode")]
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
        let mut result = Vec::with_capacity(13);
        // Tamanho total da instrução
        result.extend_from_slice(&(0x0d as u64).to_be_bytes());
        // Código da instrução
        result.extend_from_slice(&self.get_opcode().to_be_bytes());
        // Número de parâmetros
        result.push(0);

        result
    }
}

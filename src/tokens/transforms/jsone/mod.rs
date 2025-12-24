#[cfg(feature="test_access")]
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

#[cfg(feature = "test_access")]
#[cfg(test)]
mod jsone_tests {
    use crate::{ tokens::{ transforms::jsone::Jsone, TokenMethods } };

    #[test]
    fn test_json_escape() {
        let mut token = Jsone::default();

        let expected_output = "\"{banana: '10'}\"".to_string();

        assert_eq!(
            token.transform("{banana: '10'}"),
            Ok(expected_output),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "jsone;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "jsone".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["jsone".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_json_escape_bytecode() {
        use crate::{ utils::params::AtpParamTypes };

        let mut token = Jsone::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x26, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            vec![
                // Instruction total size
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x0d,
                // Instruction Type
                0x00,
                0x00,
                0x00,
                0x26,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

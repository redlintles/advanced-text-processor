use std::borrow::Cow;

use crate::{ tokens::TokenMethods };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeTokenMethods };

use crate::utils::errors::{ AtpError, AtpErrorCode };

/// Jsonu - Json Unescape
///
/// Unescapes JSON Special Characters in `input` with serde_json::from_str
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::jsonu::Jsonu};
///
///
/// let token = Jsonu::default();
///
/// let expected_output = "{banana: '10'}".to_string();
///
/// assert_eq!(token.parse("\"{banana: '10'}\""), Ok(expected_output));
/// ```

#[derive(Clone, Copy, Default)]
pub struct Jsonu {}

impl TokenMethods for Jsonu {
    fn get_string_repr(&self) -> &'static str {
        "jsonu"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        "jsonu;\n".into()
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "jsonu" {
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

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(
            serde_json
                ::from_str::<String>(input)
                .map_err(|_|
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed to deserialize to JSON".into()),
                        "serde_json::from_str",
                        input.to_string()
                    )
                )?
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Jsonu {
    fn get_opcode(&self) -> u8 {
        0x27
    }

    fn token_from_bytecode_instruction(&mut self, instruction: Vec<u8>) -> Result<(), AtpError> {
        if instruction[0] == Jsonu::default().get_opcode() {
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                instruction[0].to_string(),
                instruction
                    .iter()
                    .map(|b| b.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        )
    }
    fn token_to_bytecode_instruction(&self) -> Vec<u8> {
        vec![Jsonu::default().get_opcode(), 0]
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod jsonu_tests {
    use crate::{ tokens::{ transforms::jsonu::Jsonu, TokenMethods } };

    #[test]
    fn test_json_unescape() {
        let mut token = Jsonu::default();

        let expected_output = "{banana: '10'}".to_string();

        assert_eq!(
            token.parse("\"{banana: '10'}\""),
            Ok(expected_output),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "jsonu;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "jsonu".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["jsonu".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_json_unescape_bytecode() {
        use crate::bytecode::{ BytecodeTokenMethods };

        let mut token = Jsonu::default();

        let instruction = vec![0x27, 0];

        assert_eq!(token.get_opcode(), 0x27, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.token_from_bytecode_instruction(instruction.clone()),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.token_to_bytecode_instruction(),
            instruction,
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

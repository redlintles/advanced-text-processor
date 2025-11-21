use std::borrow::Cow;

use crate::{ tokens::TokenMethods };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

use crate::utils::errors::{ AtpError, AtpErrorCode };

/// Jsone - Json Escape
///
/// Escapes JSON Special Characters in `input` with serde_json::to_string
///
/// # Example:
///
/// ```rust
/// use atp_project::tokens::{TokenMethods, transforms::jsone::Jsone};
///
///
/// let token = Jsone::default();
/// let expected_output = "\"{banana: '10'}\"".to_string();
///
/// assert_eq!(token.parse("{banana: '10'}"), Ok(expected_output));
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

    fn parse(&self, input: &str) -> Result<String, AtpError> {
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
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Jsone {
    fn get_opcode(&self) -> u8 {
        0x26
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Jsone::default().get_opcode() {
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("".into()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> crate::bytecode::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Jsone::default().get_opcode(),
            operands: [].to_vec(),
        }
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
            token.parse("{banana: '10'}"),
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
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Jsone::default();

        let instruction = BytecodeInstruction {
            op_code: 0x26,
            operands: [].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x26, "get_opcode does not disrepect ATP token mapping");

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

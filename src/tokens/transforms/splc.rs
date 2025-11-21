use std::borrow::Cow;

use crate::tokens::TokenMethods;

use crate::utils::errors::{ AtpError, AtpErrorCode };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeTokenMethods, BytecodeInstruction };

/// SPLC - Split Characters
///
/// Split `input` characters in a result whose chars are separed by spaces
///
/// # Example
///
/// ```rust
/// use atp_project::tokens::{TokenMethods, transforms::splc::Splc};
///
/// let token = Splc::default();
///
/// assert_eq!(token.parse("banana"), Ok("b a n a n a".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Splc {}

impl TokenMethods for Splc {
    fn get_string_repr(&self) -> &'static str {
        "splc"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        "splc;\n".into()
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "splc" {
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
            input
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Splc {
    fn get_opcode(&self) -> u8 {
        0x23
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Splc::default().get_opcode() {
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
            op_code: Splc::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod splc_tests {
    use crate::tokens::{ TokenMethods, transforms::splc::Splc };
    #[test]
    fn split_characters_tests() {
        let mut token = Splc::default();
        assert_eq!(
            token.parse("banana"),
            Ok("b a n a n a".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "splc;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "splc".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["splc".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
        assert_eq!(
            token.parse("banana"),
            Ok("b a n a n a".to_string()),
            "It works correctly after re-parsing"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn split_characters_bytecode_tests() {
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Splc::default();

        let instruction = BytecodeInstruction {
            op_code: 0x23,
            operands: [].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x23, "get_opcode does not disrepect ATP token mapping");

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

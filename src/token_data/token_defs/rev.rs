#[cfg(feature = "bytecode")]
use crate::{ bytecode_parser::{ BytecodeTokenMethods, BytecodeInstruction } };

use crate::{ utils::errors::{ AtpError, AtpErrorCode }, token_data::TokenMethods };

/// Rev - Reverse
///
/// Reverses `input` character order
///
/// # Example:
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::rev::Rev};
///
/// let token = Rev::default();
/// assert_eq!(token.parse("foobar"), Ok("raboof".to_string()));
/// ``````
#[derive(Clone, Default, Copy)]
pub struct Rev {}

impl TokenMethods for Rev {
    fn get_string_repr(&self) -> String {
        "rev".to_string()
    }
    fn token_to_atp_line(&self) -> String {
        "rev;\n".to_string()
    }

    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "rev" {
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid parser for this token".to_string()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.chars().rev().collect())
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Rev {
    fn get_opcode(&self) -> u8 {
        0x22
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), crate::utils::errors::AtpError> {
        if instruction.op_code == Rev::default().get_opcode() {
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("".to_string()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> crate::bytecode_parser::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Rev::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod rev_tests {
    use crate::{ token_data::{ token_defs::rev::Rev, TokenMethods } };

    #[test]
    fn test_capitalize_last_word() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut token = Rev::default();

        assert_eq!(
            token.parse(&random_text),
            Ok(
                random_text
                    .chars()
                    .rev()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            ),
            "It supports random inputs"
        );
        assert_eq!(token.parse("banana"), Ok("ananab".to_string()), "It supports expected inputs");
        assert_eq!(
            token.token_to_atp_line(),
            "rev;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "rev".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.token_from_vec_params(["rev".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_capitalize_last_word_bytecode() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Rev::default();

        let instruction = BytecodeInstruction {
            op_code: 0x22,
            operands: [].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x22, "get_opcode does not disrepect ATP token mapping");

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

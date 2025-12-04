use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeTokenMethods };
/// TLA - To Lowercase All
///
/// Lowercases every character from `input`
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::tla::Tla};
///
/// let token = Tla::default();
///
/// assert_eq!(token.parse("BANANA"), Ok("banana".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Tla {}

impl TokenMethods for Tla {
    fn get_string_repr(&self) -> &'static str {
        "tla"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "tla;\n".into()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.to_lowercase())
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "tla" {
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
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Tla {
    fn get_opcode(&self) -> u8 {
        0x13
    }

    fn token_from_bytecode_instruction(&mut self, instruction: Vec<u8>) -> Result<(), AtpError> {
        if instruction[0] != Tla::default().get_opcode() {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Bytecode Not Found".into()),
                    instruction[0].to_string(),
                    instruction
                        .iter()
                        .map(|b| b.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            );
        }

        if instruction[1] != 0 {
            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidArgumentNumber("Tua has no arguments".into()),
                    instruction[1].to_string(),
                    instruction
                        .iter()
                        .map(|b| b.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            );
        }

        Ok(())
    }

    fn token_to_bytecode_instruction(&self) -> Vec<u8> {
        vec![Tla::default().get_opcode(), 0]
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod tla_tests {
    use crate::tokens::{ transforms::tla::Tla, TokenMethods };
    #[test]
    fn test_to_lowercase_all() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut token = Tla::default();

        assert_eq!(token.parse("BANANA"), Ok("banana".to_string()), "It supports expected inputs");
        assert_eq!(
            token.parse(&random_text),
            Ok(random_text.to_lowercase()),
            "It supports random inputs"
        );

        assert_eq!(
            token.to_atp_line(),
            "tla;\n".to_string(),
            "Conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "tla".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["tla".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_to_lowercase_all_bytecode() {
        use crate::bytecode::{ BytecodeTokenMethods };

        let mut token = Tla::default();

        let instruction = vec![0x13, 0];

        assert_eq!(token.get_opcode(), 0x13, "get_opcode does not disrepect ATP token mapping");

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

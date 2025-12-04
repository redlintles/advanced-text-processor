use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeTokenMethods };

/// TLS - Trim left sides
///
/// Trim the right side of `input`, removing all spaces from the end
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::trs::Trs};
///
/// let token = Trs::default();
///
/// assert_eq!(token.parse("   banana   "), Ok("   banana".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Trs {}

impl TokenMethods for Trs {
    fn to_atp_line(&self) -> Cow<'static, str> {
        "trs;\n".into()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(String::from(input.trim_end()))
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "trs;"

        if line[0] == "trs" {
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

    fn get_string_repr(&self) -> &'static str {
        "trs"
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Trs {
    fn get_opcode(&self) -> u8 {
        0x07
    }

    fn token_from_bytecode_instruction(&mut self, instruction: Vec<u8>) -> Result<(), AtpError> {
        if instruction[0] == Trs::default().get_opcode() {
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
        vec![Trs::default().get_opcode(), 0]
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod trs_tests {
    #[test]
    fn test_trim_right_side() {
        use crate::tokens::{ transforms::trs::Trs, TokenMethods };
        use rand::Rng;
        let mut token = Trs::default();

        let mut rng = rand::rng();

        let random_number: usize = rng.random_range(0..100);
        let spaces = " ".repeat(random_number);
        let mut text = String::from("banana");

        text.push_str(&spaces);

        assert_eq!(
            token.parse("banana     "),
            Ok("banana".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(token.parse(&text), Ok("banana".to_string()), "It supports random inputs");
        assert_eq!(
            token.to_atp_line(),
            "trs;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "trs".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["trs".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_bytecode_trim_right_side() {
        use crate::tokens::{ transforms::trs::Trs };
        use crate::bytecode::{ BytecodeTokenMethods };

        let mut token = Trs::default();

        let instruction = vec![0x07, 0];
        assert_eq!(token.get_opcode(), 0x07, "get_opcode does not disrepect ATP token mapping");

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

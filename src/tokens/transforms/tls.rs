use std::borrow::Cow;

use crate::tokens::TokenMethods;

use crate::utils::errors::{ AtpError, AtpErrorCode };

#[cfg(feature = "bytecode")]
use crate::{ bytecode::{ BytecodeTokenMethods } };
/// TLS - Trim left sides
///
/// Trim the left Side of `input`, removing all spaces from the beginning
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::tls::Tls};
///
/// let token = Tls::default();
///
/// assert_eq!(token.parse("   banana   "), Ok("banana   ".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Tls {}

impl TokenMethods for Tls {
    fn to_atp_line(&self) -> Cow<'static, str> {
        "tls;\n".into()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(String::from(input.trim_start()))
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "tls;"

        if line[0] == "tls" {
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
        "tls"
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Tls {
    fn get_opcode(&self) -> u8 {
        0x06
    }

    fn token_from_bytecode_instruction(&mut self, instruction: Vec<u8>) -> Result<(), AtpError> {
        if instruction[0] == Tls::default().get_opcode() {
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
        vec![Tls::default().get_opcode(), 0]
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod tls_tests {
    #[test]
    fn test_trim_left_side() {
        use crate::tokens::{ transforms::tls::Tls, TokenMethods };
        use rand::Rng;
        let mut token = Tls::default();

        let mut rng = rand::rng();

        let random_number: usize = rng.random_range(0..100);
        let spaces = " ".repeat(random_number);
        let mut text = String::from("banana");

        text = format!("{}{}", spaces, text);

        assert_eq!(
            token.parse("     banana"),
            Ok("banana".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(token.parse(&text), Ok("banana".to_string()));
        assert_eq!(token.to_atp_line(), "tls;\n".to_string(), "It supports random inputs");
        assert_eq!(token.get_string_repr(), "tls".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["tls".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_bytecode_trim_left_side() {
        use crate::tokens::{ transforms::tls::Tls };
        use crate::bytecode::{ BytecodeTokenMethods };

        let mut token = Tls::default();

        let instruction = vec![0x06, 0];

        assert_eq!(token.get_opcode(), 0x06, "get_opcode does not disrepect ATP token mapping");

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

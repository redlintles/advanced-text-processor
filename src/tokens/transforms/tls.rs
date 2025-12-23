use std::borrow::Cow;

use crate::tokens::TokenMethods;

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;
use crate::utils::errors::{ AtpError, AtpErrorCode };

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
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x06
    }
    #[cfg(feature = "bytecode")]
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        if instruction.len() == 0 {
            return Ok(());
        } else {
            Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            )
        }
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // tamanho total = opcode (4) + param_count (1) + header (8)
        let instruction_size: u64 = 13;

        result.extend_from_slice(&instruction_size.to_be_bytes());

        let opcode: u32 = self.get_opcode() as u32;
        result.extend_from_slice(&opcode.to_be_bytes());

        result.push(0); // número de parâmetros

        result
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
        use crate::tokens::TokenMethods;
        use crate::tokens::{ transforms::tls::Tls };
        use crate::utils::params::AtpParamTypes;

        let mut token = Tls::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x06, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            vec![
                // Instruction Total Size
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
                0x06,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

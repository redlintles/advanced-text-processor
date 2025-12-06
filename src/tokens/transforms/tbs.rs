use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::{ bytecode::BytecodeTokenMethods, utils::bytecode_utils::AtpParamTypes };
/// TBS - Trim both sides
///
/// Trim Both Sides of `input`, removing all spaces from both the beginning and the end
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::tbs::Tbs};
///
/// let token = Tbs::default();
///
/// assert_eq!(token.parse("   banana   "), Ok("banana".to_string()));
/// ```
///
#[derive(Clone, Default)]
pub struct Tbs {}

impl TokenMethods for Tbs {
    fn to_atp_line(&self) -> Cow<'static, str> {
        "tbs;\n".into()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(String::from(input.trim()))
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "tbs;"

        if line[0] == "tbs" {
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
        "tbs"
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Tbs {
    fn get_opcode(&self) -> u8 {
        0x05
    }

    fn from_params(&mut self, instruction: Vec<AtpParamTypes>) -> Result<(), AtpError> {
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
mod tbs_tests {
    #[test]
    fn test_trim_both_sides() {
        use crate::tokens::{ transforms::tbs::Tbs, TokenMethods };
        use rand::Rng;
        let mut token = Tbs::default();

        let mut rng = rand::rng();

        let random_number_1: usize = rng.random_range(0..100);
        let random_number_2: usize = rng.random_range(0..100);
        let spaces_start = " ".repeat(random_number_1);
        let spaces_end = " ".repeat(random_number_2);
        let mut text = String::from("banana");

        text = format!("{}{}{}", spaces_start, text, spaces_end);

        assert_eq!(
            token.parse("     banana  "),
            Ok("banana".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(token.parse(&text), Ok("banana".to_string()), "It supports random inputs");
        assert_eq!(
            token.to_atp_line(),
            "tbs;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "tbs".to_string(), "get_string_repr works correctly");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["tbs".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_bytecode_trim_both_sides() {
        use crate::tokens::{ transforms::tbs::Tbs };
        use crate::bytecode::{ BytecodeTokenMethods };
        use crate::utils::bytecode_utils::AtpParamTypes;

        let mut token = Tbs::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x05, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            vec![
                // tamanho total da instrução (8 bytes)
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x0d,
                // opcode (4 bytes)
                0x00,
                0x00,
                0x00,
                0x05,
                // número de parâmetros
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

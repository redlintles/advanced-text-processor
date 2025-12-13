use std::borrow::Cow;

#[cfg(feature = "bytecode")]
use crate::{ bytecode::{ BytecodeTokenMethods }, utils::bytecode_utils::AtpParamTypes };

use crate::{ utils::errors::{ AtpError, AtpErrorCode }, tokens::TokenMethods };

/// Rev - Reverse
///
/// Reverses `input` character order
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::rev::Rev};
///
/// let token = Rev::default();
/// assert_eq!(token.parse("foobar"), Ok("raboof".to_string()));
/// ``````
#[derive(Clone, Default, Copy)]
pub struct Rev {}

impl TokenMethods for Rev {
    fn get_string_repr(&self) -> &'static str {
        "rev"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        "rev;\n".into()
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "rev" {
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
        Ok(input.chars().rev().collect())
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Rev {
    fn get_opcode(&self) -> u32 {
        0x22
    }

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
mod rev_tests {
    use crate::{ tokens::{ transforms::rev::Rev, TokenMethods } };

    #[test]
    fn test_reverse() {
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
            token.to_atp_line(),
            "rev;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "rev".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["rev".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_reverse_bytecode() {
        use crate::{ bytecode::BytecodeTokenMethods, utils::bytecode_utils::AtpParamTypes };

        let mut token = Rev::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x22, "get_opcode does not disrepect ATP token mapping");

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
                0x22,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

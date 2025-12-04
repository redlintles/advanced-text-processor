use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::capitalize },
};

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeTokenMethods };
/// Token `Clw` — Capitalize Last Word
///
/// Capitalizes the last word of `input`
///
/// This is achieved by splitting the input by whitespace, reversing the resulting vector,
/// capitalizing the first word, reversing it back, and rejoining into a single string.
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::clw::Clw};
///
/// let token = Clw::default();
/// assert_eq!(token.parse("foo bar"), Ok("foo Bar".to_string()));
/// ```
#[derive(Copy, Clone, Default)]
pub struct Clw {}

impl TokenMethods for Clw {
    fn get_string_repr(&self) -> &'static str {
        "clw"
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "clw" {
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
        let mut v: Vec<String> = input
            .split(' ')
            .rev()
            .enumerate()

            .map(|(i, c)| {
                if i == 0 { capitalize(c) } else { c.to_string() }
            })
            .collect::<Vec<_>>();

        v.reverse();
        Ok(v.join(" "))
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "clw;\n".into()
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Clw {
    fn get_opcode(&self) -> u8 {
        0x19
    }

    fn token_from_bytecode_instruction(&mut self, instruction: Vec<u8>) -> Result<(), AtpError> {
        if instruction[0] == Clw::default().get_opcode() {
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
        vec![Clw::default().get_opcode(), 0]
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod clw_tests {
    use crate::{ tokens::{ transforms::clw::Clw, TokenMethods }, utils::transforms::capitalize };

    #[test]
    fn test_capitalize_last_word() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut token = Clw::default();

        assert_eq!(
            token.parse(&random_text),
            Ok(
                random_text
                    .split_whitespace()
                    .rev()
                    .enumerate()
                    .map(|(i, w)| if i == 0 { capitalize(w) } else { w.to_string() })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            "It supports random inputs"
        );
        assert_eq!(
            token.parse("banana bananosa"),
            Ok("banana Bananosa".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "clw;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "clw".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["clw".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_capitalize_last_word_bytecode() {
        use crate::bytecode::{ BytecodeTokenMethods };

        let mut token = Clw::default();

        let instruction = vec![0x19, 0];

        assert_eq!(token.get_opcode(), 0x19, "get_opcode does not disrepect ATP token mapping");

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

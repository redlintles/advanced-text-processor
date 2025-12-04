use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeTokenMethods };

/// DLL - Delete Last
///
/// Deletes the last character of `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::dll::Dll};
///
/// let token = Dll::default();
///
/// assert_eq!(token.parse("banana"), Ok("banan".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Dll {}

impl TokenMethods for Dll {
    fn to_atp_line(&self) -> Cow<'static, str> {
        "dll;\n".into()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let mut s = String::from(input);

        if let Some((x, _)) = s.char_indices().next_back() {
            s.drain(x..);
        }

        Ok(s)
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "dll;"

        if line[0] == "dll" {
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
        "dll"
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Dll {
    fn get_opcode(&self) -> u8 {
        0x04
    }

    fn token_from_bytecode_instruction(&mut self, instruction: Vec<u8>) -> Result<(), AtpError> {
        if instruction[0] == Dll::default().get_opcode() {
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
        vec![Dll::default().get_opcode(), 0]
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod dll_tests {
    use crate::tokens::{ transforms::dll::Dll, TokenMethods };

    #[test]
    fn test_delete_last() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut expected_output = random_text.clone();

        if let Some((x, _)) = expected_output.char_indices().next_back() {
            expected_output.drain(x..);
        }

        let mut token = Dll::default();

        assert_eq!(
            token.parse(&random_text),
            Ok(expected_output.to_string()),
            "It supports random inputs"
        );
        assert_eq!(token.parse("banana"), Ok("banan".to_string()), "It supports expected inputs");
        assert_eq!(
            token.to_atp_line(),
            "dll;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "dll".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["dll".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_delete_last_bytecode() {
        use crate::bytecode::{ BytecodeTokenMethods };

        let mut token = Dll::default();

        let instruction = vec![0x04, 0];

        assert_eq!(token.get_opcode(), 0x04, "get_opcode does not disrepect ATP token mapping");

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

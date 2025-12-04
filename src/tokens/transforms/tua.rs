use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeTokenMethods };

#[derive(Clone, Copy, Default)]
pub struct Tua {}

impl TokenMethods for Tua {
    fn get_string_repr(&self) -> &'static str {
        "tua"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "tua;\n".into()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.to_uppercase())
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "tua" {
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
impl BytecodeTokenMethods for Tua {
    fn get_opcode(&self) -> u8 {
        0x12
    }

    fn token_from_bytecode_instruction(&mut self, instruction: Vec<u8>) -> Result<(), AtpError> {
        if instruction[0] != Tua::default().get_opcode() {
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
        vec![Tua::default().get_opcode(), 0]
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod tua_tests {
    use crate::tokens::{ transforms::tua::Tua, TokenMethods };
    #[test]
    fn test_to_uppercase_all() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut token = Tua::default();

        assert_eq!(token.parse("banana"), Ok("BANANA".to_string()), "It supports expected inputs");
        assert_eq!(
            token.parse(&random_text),
            Ok(random_text.to_uppercase()),
            "It supports random inputs"
        );

        assert_eq!(token.to_atp_line(), "tua;\n".to_string());
        assert_eq!(token.get_string_repr(), "tua".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["tua".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_to_uppercase_all_bytecode() {
        use crate::bytecode::{ BytecodeTokenMethods };

        let mut token = Tua::default();

        let instruction = vec![0x12, 0];

        assert_eq!(token.get_opcode(), 0x12, "get_opcode does not disrepect ATP token mapping");

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

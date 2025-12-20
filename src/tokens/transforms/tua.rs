use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::{ bytecode::BytecodeTokenMethods, utils::params::AtpParamTypes };

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
    fn get_opcode(&self) -> u32 {
        0x12
    }

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
        use crate::{ bytecode::BytecodeTokenMethods, utils::params::AtpParamTypes };

        let mut token = Tua::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x12, "get_opcode does not disrepect ATP token mapping");

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
                0x12,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

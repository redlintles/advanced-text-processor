use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::capitalize },
};

#[cfg(feature = "bytecode")]
use crate::{ bytecode::BytecodeTokenMethods, utils::bytecode_utils::AtpParamTypes };
/// Token `Cfw` — Capitalize First Word
///
/// Capitalizes the first word of `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::cfw::Cfw};
///
/// let token = Cfw::default();
/// assert_eq!(token.parse("foo bar"), Ok("Foo bar".to_string()));
/// ```
#[derive(Copy, Clone, Default)]
pub struct Cfw {}

impl TokenMethods for Cfw {
    fn get_string_repr(&self) -> &'static str {
        "cfw"
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "cfw" {
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
        Ok(capitalize(input))
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "cfw;\n".into()
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Cfw {
    fn get_opcode(&self) -> u32 {
        0x18
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
mod cfw_tests {
    use crate::{ tokens::{ transforms::cfw::Cfw, TokenMethods }, utils::transforms::capitalize };

    #[test]
    fn test_capitalize_first_word() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut token = Cfw::default();

        assert_eq!(
            token.parse(&random_text),
            Ok(capitalize(&random_text)),
            "It supports random inputs"
        );
        assert_eq!(token.parse("banana"), Ok("Banana".to_string()), "It supports expected inputs");
        assert_eq!(
            token.to_atp_line(),
            "cfw;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "cfw".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["cfw".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_capitalize_first_word_bytecode() {
        use crate::{ bytecode::BytecodeTokenMethods, utils::bytecode_utils::AtpParamTypes };

        let mut token = Cfw::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x18, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(instruction),
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
                0x18,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

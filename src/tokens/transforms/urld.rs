use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::{ bytecode::BytecodeTokenMethods, utils::bytecode_utils::AtpParamTypes };
/// URLD - URL Decode
///
/// Decodes `input` from the URL Encoding Format
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::urld::Urld};
///
/// let token = Urld::default();
///
/// assert_eq!(token.parse("banana%20laranja"), Ok("banana laranja".to_string()));
/// ```
///

#[derive(Copy, Clone, Default)]
pub struct Urld {}

impl TokenMethods for Urld {
    fn get_string_repr(&self) -> &'static str {
        "urld"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "urld;\n".into()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(
            urlencoding
                ::decode(input)
                .map_err(|_|
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed parsing URL string".into()),
                        "urld",
                        input.to_string()
                    )
                )?
                .to_string()
        )
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "urld" {
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
impl BytecodeTokenMethods for Urld {
    fn get_opcode(&self) -> u32 {
        0x21
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
mod urld_tests {
    use crate::tokens::{ transforms::urld::Urld, TokenMethods };
    #[test]
    fn test_url_decode() {
        let mut token = Urld::default();

        assert_eq!(
            token.parse("banana%20laranja"),
            Ok("banana laranja".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "urld;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "urld".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["urld".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_bytecode_url_decode() {
        use crate::tokens::{ transforms::urld::Urld };
        use crate::bytecode::{ BytecodeTokenMethods };
        use crate::utils::bytecode_utils::AtpParamTypes;

        let mut token = Urld::default();

        let instruction: Vec<AtpParamTypes> = vec![];
        assert_eq!(token.get_opcode(), 0x21, "get_opcode does not disrepect ATP token mapping");

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
                0x21,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

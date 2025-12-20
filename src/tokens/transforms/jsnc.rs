use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::{ bytecode::BytecodeTokenMethods, utils::params::AtpParamTypes };

/// JSNC - Join to Snake Case
///
/// If `input` is a string whose words are separated by spaces, join `input` as a snake_case lowercased string
/// For conversion between the screaming and normal versions of this case style, use the tokens TLA(To Lowercase All) and TUA(To Uppercase All) together with this one.
///
/// See Also:
///
/// - [`Tua` - To Uppercase All](crate::tokens::transforms::tua)
/// - [`Tla` - To Lowercase All](crate::tokens::transforms::tla)
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::jsnc::Jsnc};
///
/// let token = Jsnc::default();
///
/// assert_eq!(token.parse("banana laranja cheia de canja"), Ok("banana_laranja_cheia_de_canja".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Jsnc {}

impl TokenMethods for Jsnc {
    fn get_string_repr(&self) -> &'static str {
        "jsnc"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "jsnc;\n".into()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.split_whitespace().collect::<Vec<_>>().join("_").to_lowercase())
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "jsnc" {
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid Parser for this token".into()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Jsnc {
    fn get_opcode(&self) -> u32 {
        0x2c
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
mod jsnc_tests {
    use crate::tokens::{ TokenMethods, transforms::jsnc::Jsnc };
    #[test]
    fn join_to_snake_case_tests() {
        let mut token = Jsnc::default();
        assert_eq!(
            token.parse("banana laranja cheia de canja"),
            Ok("banana_laranja_cheia_de_canja".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "jsnc;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "jsnc".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["jsnc".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn join_to_snake_case_bytecode_tests() {
        use crate::{ bytecode::BytecodeTokenMethods, utils::params::AtpParamTypes };

        let mut token = Jsnc::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x2c, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            vec![
                // Instruction total size
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
                0x2c,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

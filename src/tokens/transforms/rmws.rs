use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::{ bytecode::BytecodeTokenMethods, utils::bytecode_utils::AtpParamTypes };

/// RMWS - Remove Whitespace
///
/// Removes all whitespaces in `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::rmws::Rmws};
///
/// let token = Rmws::default();
///
/// assert_eq!(token.parse("banana laranja cheia de canja"), Ok("bananalaranjacheiadecanja".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Rmws {}

impl TokenMethods for Rmws {
    fn get_string_repr(&self) -> &'static str {
        "rmws"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        "rmws;\n".into()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.split_whitespace().collect::<Vec<_>>().join(""))
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "rmws" {
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
impl BytecodeTokenMethods for Rmws {
    fn get_opcode(&self) -> u32 {
        0x31
    }

    fn from_params(&mut self, instruction: Vec<AtpParamTypes>) -> Result<(), AtpError> {
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
        let opcode = self.get_opcode();

        let size: u16 = 13; // tamanho total, incluindo header
        let size_bytes = size.to_be_bytes(); // big-endian

        let mut v = Vec::with_capacity(13);
        v.push(opcode);
        v.extend_from_slice(&size_bytes);
        v.extend_from_slice(&[0u8; 10]); // payload vazio/padding

        v
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod rmws_tests {
    use crate::tokens::{ TokenMethods, transforms::rmws::Rmws };
    #[test]
    fn remove_whitespace_tests() {
        let mut token = Rmws::default();

        assert_eq!(
            token.parse("banana laranja cheia de canja"),
            Ok("bananalaranjacheiadecanja".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "rmws;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "rmws".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["rmws".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn remove_whitespace_bytecode_tests() {
        use crate::{ bytecode::BytecodeTokenMethods, utils::bytecode_utils::AtpParamTypes };

        let mut token = Rmws::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x31, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(instruction),
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
                0x31,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

use std::borrow::Cow;

use html_escape::decode_html_entities;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::{ bytecode::BytecodeTokenMethods, utils::bytecode_utils::AtpParamTypes };

/// HTMLU - HTML Unescape
///
/// Unescapes Special HTML Entities in `input` to their corresponding characters
/// Used when some HTML text is gonna be processed as a normal string
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::htmlu::Htmlu};
///
/// let token = Htmlu::default();
///
/// assert_eq!(token.parse("&lt;div&gt;banana&lt;/div&gt;"), Ok("<div>banana</div>".to_string()));
/// ```
#[derive(Copy, Clone, Default)]
pub struct Htmlu {}

impl TokenMethods for Htmlu {
    fn get_string_repr(&self) -> &'static str {
        "htmlu"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "htmlu;\n".into()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(decode_html_entities(input).to_string())
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "htmlu" {
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
impl BytecodeTokenMethods for Htmlu {
    fn get_opcode(&self) -> u32 {
        0x25
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
mod htmlu_tests {
    use crate::tokens::{ TokenMethods, transforms::htmlu::Htmlu };
    #[test]
    fn html_unescape_test() {
        let mut token = Htmlu::default();

        assert_eq!(
            token.parse("&lt;div&gt;banana&lt;/div&gt;"),
            Ok("<div>banana</div>".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "htmlu;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "htmlu".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["htmlu".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn html_unescape_bytecode_test() {
        use crate::{ bytecode::BytecodeTokenMethods, utils::bytecode_utils::AtpParamTypes };

        let mut token = Htmlu::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x25, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x25, 0x00],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

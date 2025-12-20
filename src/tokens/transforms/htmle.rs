use std::borrow::Cow;

use html_escape::encode_text;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::{ utils::params::AtpParamTypes };

/// HTMLE - HTML Escape
///
/// Escapes Special HTML Characters in `input` to HTML Entities
/// So they can be rendered correctly later
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::htmle::Htmle};
///
/// let token = Htmle::default();
///
/// assert_eq!(token.parse("<div>banana</div>"), Ok("&lt;div&gt;banana&lt;/div&gt;".to_string()));
/// ```

#[derive(Copy, Clone, Default)]
pub struct Htmle {}

impl TokenMethods for Htmle {
    fn get_string_repr(&self) -> &'static str {
        "htmle"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "htmle;\n".into()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(encode_text(input).to_string())
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "htmle" {
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
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x24
    }
    #[cfg(feature = "bytecode")]
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
    #[cfg(feature = "bytecode")]
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
mod htmle_tests {
    use crate::tokens::{ TokenMethods, transforms::htmle::Htmle };
    #[test]
    fn html_escape_test() {
        let mut token = Htmle::default();

        assert_eq!(
            token.parse("<div>banana</div>"),
            Ok("&lt;div&gt;banana&lt;/div&gt;".to_string()),
            "It supports expected inputs!"
        );
        assert_eq!(
            token.to_atp_line(),
            "htmle;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "htmle".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["htmle".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn html_escape_bytecode_test() {
        use crate::{ utils::params::AtpParamTypes };

        let mut token = Htmle::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x24, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            vec![
                // Token total size
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x0d,
                // Token Type
                0x00,
                0x00,
                0x00,
                0x24,
                // Token Params
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

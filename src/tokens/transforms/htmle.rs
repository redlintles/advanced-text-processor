use std::borrow::Cow;

use html_escape::encode_text;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeTokenMethods, BytecodeInstruction };

/// HTMLE - HTML Escape
///
/// Escapes Special HTML Characters in `input` to HTML Entities
/// So they can be rendered correctly later
///
/// # Example
///
/// ```rust
/// use atp_project::tokens::{TokenMethods, transforms::htmle::Htmle};
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
        Cow::Borrowed("htmle;\n")
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
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Htmle {
    fn get_opcode(&self) -> u8 {
        0x24
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Htmle::default().get_opcode() {
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("".into()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> crate::bytecode::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Htmle::default().get_opcode(),
            operands: [].to_vec(),
        }
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
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Htmle::default();

        let instruction = BytecodeInstruction {
            op_code: 0x24,
            operands: [].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x24, "get_opcode does not disrepect ATP token mapping");

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

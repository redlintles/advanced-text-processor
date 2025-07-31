use html_escape::decode_html_entities;

use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeTokenMethods, BytecodeInstruction };

/// HTMLU - HTML Unescape
///
/// Unescapes Special HTML Entities in `input` to their corresponding characters
/// Used when some HTML text is gonna be processed as a normal string
///
/// # Example
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::htmlu::Htmlu};
///
/// let token = Htmlu::default();
///
/// assert_eq!(token.parse("&lt;div&gt;banana&lt;/div&gt;"), Ok("<div>banana</div>".to_string()));
/// ```
#[derive(Copy, Clone, Default)]
pub struct Htmlu {}

impl TokenMethods for Htmlu {
    fn get_string_repr(&self) -> String {
        "htmlu".to_string()
    }

    fn to_atp_line(&self) -> String {
        "htmlu;\n".to_string()
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
                AtpErrorCode::TokenNotFound("Invalid parser for this token".to_string()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Htmlu {
    fn get_opcode(&self) -> u8 {
        0x25
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Htmlu::default().get_opcode() {
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("".to_string()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> crate::bytecode_parser::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Htmlu::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod htmlu_tests {
    use crate::token_data::{ TokenMethods, transforms::htmlu::Htmlu };
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
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Htmlu::default();

        let instruction = BytecodeInstruction {
            op_code: 0x25,
            operands: [].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x25, "get_opcode does not disrepect ATP token mapping");

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

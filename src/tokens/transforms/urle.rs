use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeTokenMethods, BytecodeInstruction };

/// URLE - URL Encode
///
/// Encodes `input` to the URL Encoding Format
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::urle::Urle};
///
/// let token = Urle::default();
///
/// assert_eq!(token.parse("banana laranja"), Ok("banana%20laranja".to_string()));
/// ```
///
#[derive(Copy, Clone, Default)]
pub struct Urle {}

impl TokenMethods for Urle {
    fn get_string_repr(&self) -> &'static str {
        "urle"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "urle;\n".into()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(urlencoding::encode(input).to_string())
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "urle" {
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
impl BytecodeTokenMethods for Urle {
    fn get_opcode(&self) -> u8 {
        0x20
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Urle::default().get_opcode() {
            return Ok(());
        }
        let code = instruction.op_code.to_string();
        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("".into()),
                code,
                instruction.operands.join(" ")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> crate::bytecode::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Urle::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod urle_tests {
    use crate::tokens::{ transforms::urle::Urle, TokenMethods };
    #[test]
    fn test_url_encode() {
        let mut token = Urle::default();

        assert_eq!(
            token.parse("banana laranja"),
            Ok("banana%20laranja".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "urle;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "urle".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["urle".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_bytecode_url_encode() {
        use crate::tokens::{ transforms::urle::Urle };
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Urle::default();

        let instruction = BytecodeInstruction {
            op_code: 0x20,
            operands: [].to_vec(),
        };
        assert_eq!(token.get_opcode(), 0x20, "get_opcode does not disrepect ATP token mapping");

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

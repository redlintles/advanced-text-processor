use crate::token_data::TokenMethods;

use crate::utils::errors::{ AtpError, AtpErrorCode };

#[cfg(feature = "bytecode")]
use crate::{ bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods } };
// Trim left side
#[derive(Clone, Copy, Default)]
pub struct Tls {}

impl TokenMethods for Tls {
    fn token_to_atp_line(&self) -> String {
        "tls;\n".to_string()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(String::from(input.trim_start()))
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "tls;"

        if line[0] == "tls" {
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
    fn get_string_repr(&self) -> String {
        "tls".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Tls {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        use crate::utils::errors::AtpErrorCode;

        if instruction.op_code == Tls::default().get_opcode() {
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

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Tls::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x06
    }
}

#[cfg(test)]
mod tls_tests {
    #[test]
    fn test_trim_left_side() {
        use crate::token_data::{ token_defs::tls::Tls, TokenMethods };
        let mut token = Tls::default();

        assert_eq!(token.parse("   banana"), Ok("banana".to_string()));
        assert_eq!(token.token_to_atp_line(), "tls;\n".to_string());
        assert_eq!(token.get_string_repr(), "tls".to_string());
        assert!(matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)));
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_bytecode_trim_left_side() {
        use crate::token_data::{ token_defs::tls::Tls };
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Tls::default();

        let instruction = BytecodeInstruction {
            op_code: 0x06,
            operands: [].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x06);

        assert_eq!(token.token_from_bytecode_instruction(instruction.clone()), Ok(()));

        assert_eq!(token.token_to_bytecode_instruction(), instruction);
    }
}

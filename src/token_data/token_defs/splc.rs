use crate::token_data::TokenMethods;

use crate::utils::errors::{ AtpError, AtpErrorCode };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeTokenMethods, BytecodeInstruction };

#[derive(Clone, Copy, Default)]
pub struct Splc {}

impl TokenMethods for Splc {
    fn get_string_repr(&self) -> String {
        "splc".to_string()
    }
    fn token_to_atp_line(&self) -> String {
        "splc;\n".to_string()
    }

    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "splc" {
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
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(
            input
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Splc {
    fn get_opcode(&self) -> u8 {
        0x23
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Splc::default().get_opcode() {
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
            op_code: Splc::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

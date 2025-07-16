#[cfg(feature = "bytecode")]
use crate::{ bytecode_parser::{ BytecodeTokenMethods, BytecodeInstruction } };

use crate::{ utils::errors::{ AtpError, AtpErrorCode }, token_data::TokenMethods };

#[derive(Clone, Default, Copy)]
pub struct Rev {}

impl TokenMethods for Rev {
    fn get_string_repr(&self) -> String {
        "rev".to_string()
    }
    fn token_to_atp_line(&self) -> String {
        "rev;\n".to_string()
    }

    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "rev" {
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
        Ok(input.chars().rev().collect())
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Rev {
    fn get_opcode(&self) -> u8 {
        0x22
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), crate::utils::errors::AtpError> {
        if instruction.op_code == Rev::default().get_opcode() {
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
            op_code: Rev::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

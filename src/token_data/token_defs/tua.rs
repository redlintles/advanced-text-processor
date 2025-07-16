use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

#[derive(Clone, Copy, Default)]
pub struct Tua {}

impl TokenMethods for Tua {
    fn get_string_repr(&self) -> String {
        "tua".to_string()
    }

    fn token_to_atp_line(&self) -> String {
        "tua;\n".to_string()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.to_uppercase())
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "tua" {
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

impl BytecodeTokenMethods for Tua {
    fn get_opcode(&self) -> u8 {
        0x12
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Tua::default().get_opcode() {
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
        BytecodeInstruction { op_code: Tua::default().get_opcode(), operands: [].to_vec() }
    }
}

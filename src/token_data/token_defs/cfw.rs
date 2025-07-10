use crate::{
    token_data::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::capitalize },
};

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeTokenMethods, BytecodeInstruction };

#[derive(Copy, Clone, Default)]
pub struct Cfw {}

impl TokenMethods for Cfw {
    fn get_string_repr(&self) -> String {
        "cfw".to_string()
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "cfw" {
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
        Ok(capitalize(input))
    }

    fn token_to_atp_line(&self) -> String {
        "cfw;\n".to_string()
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Cfw {
    fn get_opcode(&self) -> u8 {
        0x18
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Cfw::default().get_opcode() {
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
        BytecodeInstruction { op_code: Cfw::default().get_opcode(), operands: [].to_vec() }
    }
}

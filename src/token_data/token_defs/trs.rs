use crate::{ token_data::TokenMethods, utils::errors::AtpError };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

// Trim right side
#[derive(Clone, Copy, Default)]
pub struct Trs {}

impl TokenMethods for Trs {
    fn token_to_atp_line(&self) -> String {
        "trs;\n".to_string()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(String::from(input.trim_end()))
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "trs;"

        if line[0] == "trs" {
            return Ok(());
        }
        Err(
            AtpError::new(
                crate::utils::errors::AtpErrorCode::TokenNotFound(
                    "Invalid parser for this token".to_string()
                ),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }

    fn get_string_repr(&self) -> String {
        "trs".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Trs {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        use crate::utils::errors::AtpErrorCode;

        if instruction.op_code == Trs::default().get_opcode() {
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
            op_code: Trs::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x07
    }
}

use crate::{ token_data::TokenMethods };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

use crate::utils::errors::{ AtpError, AtpErrorCode };

#[derive(Clone, Copy, Default)]
pub struct Jsonu {}

impl TokenMethods for Jsonu {
    fn get_string_repr(&self) -> String {
        "jsonu".to_string()
    }
    fn token_to_atp_line(&self) -> String {
        "jsonu;\n".to_string()
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "jsonu" {
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
            serde_json
                ::from_str(input)
                .map_err(|_|
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed to deserialize to JSON".to_string()),
                        "serde_json::from_str".to_string(),
                        input.to_string()
                    )
                )?
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Jsonu {
    fn get_opcode(&self) -> u8 {
        0x27
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Jsonu::default().get_opcode() {
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
            op_code: Jsonu::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

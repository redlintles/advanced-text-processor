use crate::{ token_data::TokenMethods };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

use crate::utils::errors::{ AtpError, AtpErrorCode };

#[derive(Clone, Copy, Default)]
pub struct Jsone {}

impl TokenMethods for Jsone {
    fn get_string_repr(&self) -> String {
        "jsone".to_string()
    }
    fn token_to_atp_line(&self) -> String {
        "jsone;\n".to_string()
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "jsone" {
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

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(
            serde_json
                ::to_string(input)
                .map_err(|_|
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed to serialize to JSON".to_string()),
                        "serde_json::to_string".to_string(),
                        input.to_string()
                    )
                )?
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Jsone {
    fn get_opcode(&self) -> u8 {
        0x26
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Jsone::default().get_opcode() {
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
            op_code: Jsone::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

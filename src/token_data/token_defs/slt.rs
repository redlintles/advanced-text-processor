use crate::{ token_data::TokenMethods, utils::transforms::string_to_usize };

#[cfg(feature = "bytecode")]
use crate::{
    bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods },
    utils::errors::{ AtpError, AtpErrorCode },
};
#[derive(Clone, Default)]
pub struct Slt {
    pub start_index: usize,
    pub end_index: usize,
}

impl Slt {
    pub fn params(start_index: usize, end_index: usize) -> Self {
        Slt {
            start_index,
            end_index,
        }
    }
}

impl TokenMethods for Slt {
    fn get_string_repr(&self) -> String {
        "slt".to_string()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        if self.start_index <= self.end_index && self.end_index <= input.len() {
            if let Some(slice) = input.get(self.start_index..self.end_index) {
                return Ok(slice.to_string());
            }
            return Err(
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::TextParsingError(
                        "Failed slicing the desired input".to_string()
                    ),
                    self.token_to_atp_line(),
                    input.to_string()
                )
            );
        }
        Err(
            AtpError::new(
                crate::utils::errors::AtpErrorCode::IndexOutOfRange(
                    "Invalid bounds for the chunk to select".to_string()
                ),
                self.token_to_atp_line(),
                input.to_string()
            )
        )
    }

    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "slt" {
            self.start_index = string_to_usize(&line[1])?;
            self.end_index = string_to_usize(&line[2])?;
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

    fn token_to_atp_line(&self) -> String {
        format!("slt {} {};\n", self.start_index, self.end_index)
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Slt {
    fn get_opcode(&self) -> u8 {
        0x11
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Slt::default().get_opcode() {
            self.start_index = string_to_usize(&instruction.operands[1])?;
            self.end_index = string_to_usize(&instruction.operands[2])?;
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
            op_code: Slt::default().get_opcode(),
            operands: [self.start_index.to_string(), self.end_index.to_string()].to_vec(),
        }
    }
}

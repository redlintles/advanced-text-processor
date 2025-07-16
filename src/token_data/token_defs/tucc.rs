use crate::{
    token_data::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::string_to_usize },
};

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

#[derive(Clone, Copy, Default)]
pub struct Tucc {
    start_index: usize,
    end_index: usize,
}

impl Tucc {
    pub fn params(start_index: usize, end_index: usize) -> Self {
        Tucc {
            start_index,
            end_index,
        }
    }
}

impl TokenMethods for Tucc {
    fn get_string_repr(&self) -> String {
        "tucc".to_string()
    }

    fn token_to_atp_line(&self) -> String {
        format!("tucc {} {};\n", self.start_index, self.end_index)
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        if self.start_index > self.end_index {
            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidIndex(
                        "Start index must be smaller than end index".to_string()
                    ),
                    self.token_to_atp_line(),
                    input.to_string()
                )
            );
        }

        // Since the user will probably not know the length of the string in the middle of the processing
        // Better simply adjust end_index to input.len() if its bigger. instead of throwing an "hard to debug" error
        let mut end = self.end_index;
        let total = input.chars().count();

        if end > total {
            end = input.len();
        }
        let result: String = input
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i >= self.start_index && i < end {
                    c.to_uppercase().to_string()
                } else {
                    c.to_string()
                }
            })
            .collect();
        Ok(result)
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "tucc" {
            self.start_index = string_to_usize(&line[1])?;
            self.end_index = string_to_usize(&line[2])?;
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

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Tucc {
    fn get_opcode(&self) -> u8 {
        0x16
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Tucc::default().get_opcode() {
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

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Tucc::default().get_opcode(),
            operands: [self.start_index.to_string(), self.end_index.to_string()].to_vec(),
        }
    }
}

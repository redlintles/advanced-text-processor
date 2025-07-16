use crate::{ token_data::TokenMethods, utils::transforms::{ capitalize, string_to_usize } };
use crate::utils::errors::{ AtpError, AtpErrorCode };
#[cfg(feature = "bytecode")]
use crate::{ bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods } };
#[derive(Clone, Default)]
pub struct Ctr {
    pub start_index: usize,
    pub end_index: usize,
}

impl Ctr {
    pub fn params(start_index: usize, end_index: usize) -> Self {
        Ctr {
            start_index,
            end_index,
        }
    }
}

impl TokenMethods for Ctr {
    fn get_string_repr(&self) -> String {
        "ctr".to_string()
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
        let total = input.split_whitespace().count();

        if end > total {
            end = total;
        }

        let result = input
            .split_whitespace()
            .enumerate()
            .map(|(i, c)| {
                if (self.start_index..end).contains(&i) { capitalize(c) } else { c.to_string() }
            })
            .collect::<Vec<_>>()
            .join(" ");

        Ok(result)
    }

    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "ctr" {
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

    fn token_to_atp_line(&self) -> String {
        format!("ctr {} {};\n", self.start_index, self.end_index)
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Ctr {
    fn get_opcode(&self) -> u8 {
        0x1c
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Ctr::default().get_opcode() {
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
            op_code: Ctr::default().get_opcode(),
            operands: [self.start_index.to_string(), self.end_index.to_string()].to_vec(),
        }
    }
}

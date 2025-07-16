use crate::{ token_data::TokenMethods, utils::transforms::{ capitalize, string_to_usize } };
use crate::utils::errors::{ AtpError, AtpErrorCode };

#[cfg(feature = "bytecode")]
use crate::{ bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods } };
#[derive(Clone, Default)]
pub struct Ctc {
    pub start_index: usize,
    pub end_index: usize,
}

impl Ctc {
    pub fn params(start_index: usize, end_index: usize) -> Self {
        Ctc {
            start_index,
            end_index,
        }
    }
}

impl TokenMethods for Ctc {
    fn get_string_repr(&self) -> String {
        "ctc".to_string()
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

        let slice: String = (&input[self.start_index..end]).to_string();

        let capitalized_chunk = slice
            .split_whitespace()
            .map(|w| capitalize(w).to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let prefix: String = input.chars().take(self.start_index).collect();
        let suffix: String = input.chars().skip(end).collect();

        let result = format!("{}{}{}", prefix, capitalized_chunk, suffix);

        Ok(result)
    }

    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "ctc" {
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
        format!("ctc {} {};\n", self.start_index, self.end_index)
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Ctc {
    fn get_opcode(&self) -> u8 {
        0x1b
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Ctc::default().get_opcode() {
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
            op_code: Ctc::default().get_opcode(),
            operands: [self.start_index.to_string(), self.end_index.to_string()].to_vec(),
        }
    }
}

use crate::{
    token_data::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::string_to_usize },
};

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
#[derive(Clone, Default)]
pub struct Rpt {
    pub times: usize,
}

impl Rpt {
    pub fn params(times: usize) -> Self {
        Rpt { times }
    }
}

impl TokenMethods for Rpt {
    fn token_to_atp_line(&self) -> String {
        format!("rpt {};\n", self.times)
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.repeat(self.times))
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "rpt" {
            self.times = string_to_usize(&line[1])?;
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

    fn get_string_repr(&self) -> String {
        "rpt".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Rpt {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Rpt::default().get_opcode() {
            if !(instruction.operands[0].is_empty() || instruction.operands[1].is_empty()) {
                self.times = string_to_usize(&instruction.operands[1])?;
                return Ok(());
            }

            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidOperands(
                        "Invalid operands for this instruction".to_string()
                    ),
                    instruction.op_code.to_string(),
                    instruction.operands.join(" ")
                )
            );
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
            op_code: Rpt::default().get_opcode(),
            operands: [self.times.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x0d
    }
}

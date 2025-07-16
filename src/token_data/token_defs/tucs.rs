use crate::{
    token_data::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::string_to_usize },
};

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

#[derive(Clone, Copy, Default)]
pub struct Tucs {
    index: usize,
}

impl Tucs {
    pub fn params(index: usize) -> Self {
        Tucs {
            index,
        }
    }
}

impl TokenMethods for Tucs {
    fn get_string_repr(&self) -> String {
        "tucs".to_string()
    }

    fn token_to_atp_line(&self) -> String {
        format!("tucs {};\n", self.index)
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let result: String = input
            .char_indices()
            .map(|(i, c)| {
                if i == self.index { c.to_uppercase().to_string() } else { c.to_string() }
            })
            .collect();
        Ok(result)
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "tucs" {
            self.index = string_to_usize(&line[1])?;
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

impl BytecodeTokenMethods for Tucs {
    fn get_opcode(&self) -> u8 {
        0x14
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Tucs::default().get_opcode() {
            self.index = string_to_usize(&instruction.operands[1])?;
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
            op_code: Tucs::default().get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }
}

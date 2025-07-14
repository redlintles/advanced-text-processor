#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

use crate::{
    token_data::TokenMethods,
    utils::{ transforms::string_to_usize, errors::{ AtpError, AtpErrorCode } },
};

#[derive(Clone, Default)]
pub struct Ins {
    index: usize,
    text_to_insert: String,
}

impl Ins {
    pub fn params(index: usize, text_to_insert: &str) -> Self {
        Ins {
            index,
            text_to_insert: text_to_insert.to_string(),
        }
    }
}
impl TokenMethods for Ins {
    fn get_string_repr(&self) -> String {
        "ins".to_string()
    }
    fn token_to_atp_line(&self) -> String {
        "ins;\n".to_string()
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "ins" {
            self.index = string_to_usize(&line[1])?;
            self.text_to_insert = line[2].clone();
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("".to_string()),
                "ins".to_string(),
                line.join(" ")
            )
        )
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let before = &input[..self.index];
        let after = &input[self.index..];

        let result = format!("{}{}{}", before, self.text_to_insert, after);

        Ok(result)
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Ins {
    fn get_opcode(&self) -> u8 {
        0x28
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Ins::default().get_opcode() {
            self.index = string_to_usize(&instruction.operands[1])?;
            self.text_to_insert = instruction.operands[2].clone();
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
            op_code: Ins::default().get_opcode(),
            operands: [self.index.to_string(), self.text_to_insert.clone()].to_vec(),
        }
    }
}

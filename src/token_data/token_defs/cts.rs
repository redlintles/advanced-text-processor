use crate::{ token_data::TokenMethods, utils::transforms::{ capitalize, string_to_usize } };

#[cfg(feature = "bytecode")]
use crate::{
    bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods },
    utils::errors::{ AtpError, AtpErrorCode },
};
#[derive(Clone, Default)]
pub struct Cts {
    pub index: usize,
}

impl Cts {
    pub fn params(index: usize) -> Self {
        Cts {
            index,
        }
    }
}

impl TokenMethods for Cts {
    fn get_string_repr(&self) -> String {
        "cts".to_string()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let v = input.split_whitespace().collect::<Vec<_>>();

        let i = match self.index > v.len() {
            true => v.len() - 1,
            false => self.index,
        };

        Ok(
            v
                .iter()
                .enumerate()
                .map(|(index, word)| {
                    if index == i { capitalize(word) } else { word.to_string() }
                })
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "cts" {
            self.index = string_to_usize(&line[1])?;
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
        format!("cts {};\n", self.index)
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Cts {
    fn get_opcode(&self) -> u8 {
        0x11
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Cts::default().get_opcode() {
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

    fn token_to_bytecode_instruction(&self) -> crate::bytecode_parser::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Cts::default().get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }
}

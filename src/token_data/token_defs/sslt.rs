use regex::Regex;

use crate::{ token_data::TokenMethods, utils::transforms::string_to_usize };

#[cfg(feature = "bytecode")]
use crate::{
    bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods },
    utils::errors::{ AtpError, AtpErrorCode },
};
#[derive(Clone)]
pub struct Sslt {
    pub pattern: Regex,
    pub index: usize,
}

impl Sslt {
    pub fn params(pattern: &str, index: usize) -> Result<Self, String> {
        let pattern = Regex::new(&pattern).map_err(|x| x.to_string())?;
        Ok(Sslt {
            pattern,
            index,
        })
    }
}

impl Default for Sslt {
    fn default() -> Self {
        Sslt {
            pattern: Regex::new("").unwrap(),
            index: 0,
        }
    }
}

impl TokenMethods for Sslt {
    fn get_string_repr(&self) -> String {
        "sslt".to_string()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let s = self.pattern
            .split(input)

            .collect::<Vec<_>>();
        let i = match self.index >= s.len() {
            true => s.len() - 1,
            false => self.index,
        };

        let item = s
            .get(i)
            .ok_or_else(||
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange("Item not found".to_string()),
                    "sslt".to_string(),
                    input.to_string()
                )
            )?;
        Ok(item.to_string())
    }

    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "sslt" {
            self.pattern = Regex::new(&line[1]).map_err(|_|
                AtpError::new(
                    AtpErrorCode::TextParsingError("Failed to create regex".to_string()),
                    "sslt".to_string(),
                    String::from(&line[1])
                )
            )?;
            self.index = string_to_usize(&line[2])?;
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
        format!("sslt {} {};\n", self.pattern, self.index)
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Sslt {
    fn get_opcode(&self) -> u8 {
        0x1a
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Sslt::default().get_opcode() {
            self.pattern = Regex::new(&instruction.operands[1]).map_err(|_|
                AtpError::new(
                    AtpErrorCode::TextParsingError("Failed to create regex".to_string()),
                    "sslt".to_string(),
                    String::from(&instruction.operands[1])
                )
            )?;
            self.index = string_to_usize(&instruction.operands[2])?;
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
            op_code: Sslt::default().get_opcode(),
            operands: [self.pattern.to_string(), self.index.to_string()].to_vec(),
        }
    }
}

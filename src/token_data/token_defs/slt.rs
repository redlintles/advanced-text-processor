use crate::{ token_data::TokenMethods, utils::transforms::string_to_usize };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
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
    fn parse(&self, input: &str) -> String {
        if self.start_index <= self.end_index && self.end_index <= input.len() {
            if let Some(slice) = input.get(self.start_index..self.end_index) {
                return slice.to_string();
            } else {
                input.to_string();
            }
        }
        input.to_string()
    }

    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        if line[0] == "slt" {
            return Ok(());
        } else {
            return Err("".to_string());
        }
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
    ) -> Result<(), String> {
        if instruction.op_code == Slt::default().get_opcode() {
            self.start_index = string_to_usize(&instruction.operands[1])?;
            self.end_index = string_to_usize(&instruction.operands[2])?;
            return Ok(());
        } else {
            return Err("An Atp Bytecode Parsing error ocurred: Invalid token".to_string());
        }
    }

    fn token_to_bytecode_instruction(&self) -> crate::bytecode_parser::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Slt::default().get_opcode(),
            operands: [self.start_index.to_string(), self.end_index.to_string()].to_vec(),
        }
    }
}

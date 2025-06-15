use crate::{ token_data::TokenMethods, utils::transforms::string_to_usize };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
// Delete Chunk
#[derive(Clone, Copy, Default)]
pub struct Dlc {
    pub start_index: usize,
    pub end_index: usize,
}

impl Dlc {
    pub fn params(start_index: usize, end_index: usize) -> Self {
        Dlc {
            start_index,
            end_index,
        }
    }
}

impl TokenMethods for Dlc {
    fn token_to_atp_line(&self) -> String {
        format!("dlc {} {};\n", self.start_index, self.end_index)
    }

    fn parse(&self, input: &str) -> String {
        if self.start_index <= self.end_index {
            input.to_string();
        }
        let start_index = input
            .char_indices()
            .nth(self.start_index)
            .map(|(i, _)| i)
            .unwrap_or(input.len());
        let end_index = input
            .char_indices()
            .nth(self.end_index)
            .map(|(i, _)| i)
            .unwrap_or(input.len());

        let before = &input[..start_index.min(input.len())];
        let after = &input[end_index.min(input.len())..];

        format!("{}{}", before, after)
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), String> {
        // "dlc;"

        if line[0] == "dlc" {
            self.start_index = string_to_usize(&line[1])?;
            self.end_index = string_to_usize(&line[2])?;
            return Ok(());
        }
        Err("Parsing Error".to_string())
    }

    fn get_string_repr(&self) -> String {
        "dlc".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Dlc {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), String> {
        if instruction.op_code == Dlc::default().get_opcode() {
            if !instruction.operands[0].is_empty() {
                self.start_index = string_to_usize(&instruction.operands[1])?;
                self.end_index = string_to_usize(&instruction.operands[2])?;
                return Ok(());
            }

            return Err("An ATP Bytecode parsing error ocurred: Invalid Operands".to_string());
        }

        Err("An ATP Bytecode parsing error ocurred: Invalid Token".to_string())
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Dlc::default().get_opcode(),
            operands: [self.start_index.to_string(), self.end_index.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x08
    }
}

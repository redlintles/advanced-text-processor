use crate::{ token_data::TokenMethods, utils::errors::AtpError };

pub mod writer;
pub mod reader;
pub mod transformer;
pub mod parser;

pub struct BytecodeInstruction {
    pub op_code: u8,
    pub operands: Vec<String>,
}

impl BytecodeInstruction {
    pub fn to_bytecode_line(&self) -> String {
        let mut result = format!("{:#04x}", self.op_code as u8);
        for operand in self.operands.iter() {
            result = format!("{} {}", result, operand);
        }

        result.push_str("\n");

        result
    }
}

pub trait BytecodeTokenMethods: TokenMethods {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError>;
    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction;

    fn get_opcode(&self) -> u8;
}

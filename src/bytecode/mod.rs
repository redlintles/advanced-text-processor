use crate::{ tokens::TokenMethods, utils::errors::AtpError };

pub mod writer;
pub mod reader;
pub mod transformer;
pub mod parser;

pub trait BytecodeTokenMethods: TokenMethods {
    fn token_from_bytecode_instruction(&mut self, instruction: Vec<u8>) -> Result<(), AtpError>;
    fn token_to_bytecode_instruction(&self) -> Vec<u8>;

    fn get_opcode(&self) -> u8;
}

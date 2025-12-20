use crate::{ tokens::TokenMethods, utils::{ params::AtpParamTypes, errors::AtpError } };

pub mod writer;
pub mod reader;
pub mod transformer;
pub mod parser;

pub trait BytecodeTokenMethods: TokenMethods {
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError>;
    fn to_bytecode(&self) -> Vec<u8>;

    fn get_opcode(&self) -> u32;
}

use crate::{
    api::{ AtpBuilderMethods, AtpConditionalMethods },
    tokens::{ InstructionMethods, instructions::blk::{ Blk } },
    utils::{ errors::AtpError, params::AtpParamTypes },
};

pub struct BlockBuilder {
    block_name: &'static str,
    block_tokens: Vec<Box<dyn InstructionMethods>>,
}

impl BlockBuilder {
    pub fn new(block_name: &'static str) -> Self {
        BlockBuilder {
            block_name,
            block_tokens: Vec::new(),
        }
    }

    pub fn build(self) -> Vec<Box<dyn InstructionMethods>> {
        self.block_tokens
    }
}

impl AtpBuilderMethods for BlockBuilder {
    fn push_token(&mut self, t: Box<dyn InstructionMethods>) -> Result<(), AtpError> {
        let param_vec = vec![
            AtpParamTypes::String(self.block_name.to_string()),
            AtpParamTypes::Token(t)
        ];

        let mut new_block = Box::new(Blk::default());
        new_block.from_params(&param_vec)?;

        self.block_tokens.push(new_block);
        Ok(())
    }
}

impl AtpConditionalMethods for BlockBuilder {}

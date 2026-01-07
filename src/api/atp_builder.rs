use crate::{
    api::{ AtpBuilderMethods, AtpConditionalMethods },
    tokens::InstructionMethods,
    utils::errors::AtpError,
};

use super::atp_processor::{ AtpProcessor, AtpProcessorMethods };

pub struct AtpBuilder<'ap> {
    tokens: Vec<Box<dyn InstructionMethods>>,
    processor: &'ap mut AtpProcessor,
}

impl<'ap> AtpBuilder<'ap> {
    pub fn new(processor: &'ap mut AtpProcessor) -> AtpBuilder<'ap> {
        AtpBuilder { tokens: Vec::new(), processor }
    }

    pub fn build(&mut self) -> String {
        let id = self.processor.add_transform(self.tokens.clone());

        id
    }
}

impl<'ap> AtpBuilderMethods for AtpBuilder<'ap> {
    fn push_token(&mut self, t: Box<dyn InstructionMethods>) -> Result<(), AtpError> {
        self.tokens.push(t);
        Ok(())
    }
}

impl<'ap> AtpConditionalMethods for AtpBuilder<'ap> {}

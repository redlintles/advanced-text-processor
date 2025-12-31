use crate::{ builder::AtpBuilderMethods, tokens::TokenMethods };

use super::atp_processor::{ AtpProcessor, AtpProcessorMethods };

#[derive(Default, Clone)]
pub struct AtpBuilder {
    tokens: Vec<Box<dyn TokenMethods>>,
}

impl AtpBuilder {
    pub fn new() -> AtpBuilder {
        AtpBuilder { tokens: Vec::new() }
    }

    pub fn build(&self) -> (AtpProcessor, String) {
        let mut p = AtpProcessor::new();
        let id = p.add_transform(self.tokens.clone());

        (p, id)
    }
}

impl AtpBuilderMethods for AtpBuilder {
    fn push_token(&mut self, t: Box<dyn TokenMethods>) {
        self.tokens.push(t);
    }
}

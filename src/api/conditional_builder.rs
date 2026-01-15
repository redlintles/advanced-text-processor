use crate::{
    api::AtpBuilderMethods,
    globals::var::TokenWrapper,
    tokens::InstructionMethods,
    utils::{ errors::AtpError, params::AtpParamTypes },
};

pub struct ConditionalBuilderEach {
    token: Box<dyn InstructionMethods>,
    params: Vec<AtpParamTypes>,
    conditional_tokens: Vec<Box<dyn InstructionMethods>>,
}

impl ConditionalBuilderEach {
    pub fn new(token: Box<dyn InstructionMethods>, params: Vec<AtpParamTypes>) -> Self {
        ConditionalBuilderEach {
            token,
            params,
            conditional_tokens: Vec::new(),
        }
    }

    pub fn build(self) -> Vec<Box<dyn InstructionMethods>> {
        self.conditional_tokens
    }
}

// push_token funciona normalmente para incrementar conditional_tokens
impl AtpBuilderMethods for ConditionalBuilderEach {
    fn push_token(&mut self, t: impl Into<TokenWrapper>) -> Result<(), AtpError> {
        let mut new_token: Box<dyn InstructionMethods> = self.token.clone();

        let mut param_vec = self.params.clone();

        param_vec.push(t.into().into());

        new_token.from_params(&param_vec)?;

        self.conditional_tokens.push(new_token);

        Ok(())
    }
}

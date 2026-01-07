use crate::{
    api::AtpBuilderMethods,
    tokens::TokenMethods,
    utils::{ errors::AtpError, params::AtpParamTypes },
};

pub struct ConditionalBuilderEach {
    token: Box<dyn TokenMethods>,
    params: Vec<AtpParamTypes>,
    conditional_tokens: Vec<Box<dyn TokenMethods>>,
}

impl ConditionalBuilderEach {
    pub fn new(token: Box<dyn TokenMethods>, params: Vec<AtpParamTypes>) -> Self {
        ConditionalBuilderEach {
            token,
            params,
            conditional_tokens: Vec::new(),
        }
    }

    pub fn build(self) -> Vec<Box<dyn TokenMethods>> {
        self.conditional_tokens
    }
}

// push_token funciona normalmente para incrementar conditional_tokens
impl AtpBuilderMethods for ConditionalBuilderEach {
    fn push_token(&mut self, t: Box<dyn crate::tokens::TokenMethods>) -> Result<(), AtpError> {
        let mut new_token = self.token.clone();
        let token_param = AtpParamTypes::Token(t);

        let mut param_vec = self.params.clone();

        param_vec.push(token_param);

        new_token.from_params(&param_vec)?;

        self.conditional_tokens.push(new_token);

        Ok(())
    }
}

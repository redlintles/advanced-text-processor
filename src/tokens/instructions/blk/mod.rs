use crate::{
    context::execution_context::{ GlobalContextMethods, GlobalExecutionContext },
    globals::var::TokenWrapper,
    parse_args,
    to_bytecode,
    tokens::{ InstructionMethods },
    utils::{ params::AtpParamTypes, validations::check_vec_len },
};

#[cfg(feature = "test_access")]
pub mod test;
#[derive(Clone)]
pub struct Blk {
    block_name: String,
    inner: TokenWrapper,
    params: Vec<AtpParamTypes>,
}

impl Default for Blk {
    fn default() -> Self {
        Blk {
            block_name: "x".to_string(),
            inner: TokenWrapper::default(),
            params: vec![
                AtpParamTypes::String("x".to_string()),
                AtpParamTypes::Token(TokenWrapper::default())
            ],
        }
    }
}

impl InstructionMethods for Blk {
    fn get_params(&self) -> &Vec<AtpParamTypes> {
        return &self.params;
    }
    fn get_opcode(&self) -> u32 {
        0x34
    }
    fn get_string_repr(&self) -> &'static str {
        "blk".into()
    }

    fn to_atp_line(&self) -> std::borrow::Cow<'static, str> {
        format!("blk {} assoc {}", self.block_name, self.inner.to_atp_line()).into()
    }

    fn transform(
        &self,
        input: &str,
        context: &mut GlobalExecutionContext
    ) -> Result<String, crate::utils::errors::AtpError> {
        context.add_to_block(&self.block_name, self.inner.clone())?;
        return Ok(input.to_string());
    }

    fn from_params(
        &mut self,
        params: &Vec<crate::utils::params::AtpParamTypes>
    ) -> Result<(), crate::utils::errors::AtpError> {
        check_vec_len(&params, 2, "block assoc", "param parsing error, invalid vec len")?;

        self.block_name = parse_args!(params, 0, String, "Block name should be of string type");

        self.inner = parse_args!(params, 1, Token, "Block inner should be of token type");

        Ok(())
    }

    fn to_bytecode(&self) -> Vec<u8> {
        let result = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::String(self.block_name.to_string()),
            AtpParamTypes::Token(self.inner.clone()),
        ]);

        result
    }
}

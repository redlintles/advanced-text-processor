use crate::{
    context::execution_context::{ GlobalContextMethods, GlobalExecutionContext },
    parse_args,
    to_bytecode,
    tokens::{ InstructionMethods, transforms::dlf::Dlf },
    utils::{ params::AtpParamTypes, validations::check_vec_len },
};

#[cfg(feature = "test_access")]
pub mod test;
#[derive(Clone)]
pub struct Blk {
    block_name: String,
    inner: Box<dyn InstructionMethods>,
}

impl Default for Blk {
    fn default() -> Self {
        Blk {
            block_name: "x".to_string(),
            inner: Box::new(Dlf::default()),
        }
    }
}

impl InstructionMethods for Blk {
    fn get_opcode(&self) -> u32 {
        0x34
    }
    fn get_string_repr(&self) -> &'static str {
        "blk".into()
    }

    fn to_atp_line(&self) -> std::borrow::Cow<'static, str> {
        format!("blk {} assoc {};", self.block_name, self.inner.to_atp_line()).into()
    }

    fn transform(&self, input: &str) -> Result<String, crate::utils::errors::AtpError> {
        Ok(input.to_string())
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
    fn needs_context(&self) -> bool {
        true
    }

    fn transform_with_context(
        &self,
        input: &str,
        context: &mut GlobalExecutionContext
    ) -> Result<String, crate::utils::errors::AtpError> {
        context.add_to_block(&self.block_name, self.inner.clone())?;
        Ok(input.to_string())
    }
}

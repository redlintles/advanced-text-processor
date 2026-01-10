use crate::{
    context::execution_context::{ GlobalContextMethods, GlobalExecutionContext },
    parse_args,
    to_bytecode,
    tokens::{ InstructionMethods },
    utils::{ params::AtpParamTypes, validations::check_vec_len },
};

#[cfg(feature = "test_access")]
pub mod test;
#[derive(Clone)]
pub struct Cblk {
    block_name: String,
}

impl Default for Cblk {
    fn default() -> Self {
        Cblk {
            block_name: "x".to_string(),
        }
    }
}

impl InstructionMethods for Cblk {
    fn get_opcode(&self) -> u32 {
        0x35
    }
    fn get_string_repr(&self) -> &'static str {
        "cblk".into()
    }

    fn to_atp_line(&self) -> std::borrow::Cow<'static, str> {
        format!("cblk {};", self.block_name).into()
    }

    fn transform(&self, input: &str) -> Result<String, crate::utils::errors::AtpError> {
        Ok(input.to_string())
    }

    fn from_params(
        &mut self,
        params: &Vec<crate::utils::params::AtpParamTypes>
    ) -> Result<(), crate::utils::errors::AtpError> {
        check_vec_len(&params, 1, "call block", "param parsing error, invalid vec len")?;

        self.block_name = parse_args!(params, 0, String, "Block name should be of string type");

        Ok(())
    }

    fn to_bytecode(&self) -> Vec<u8> {
        let result = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::String(self.block_name.to_string()),
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
        let mut result = input.to_string();
        let tokens = context.get_block(&self.block_name)?;

        for token in tokens.iter() {
            result = token.transform(&result)?;
        }
        Ok(result)
    }
}

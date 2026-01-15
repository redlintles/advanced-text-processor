#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{ errors::AtpError, validations::check_vec_len },
};

use crate::utils::params::AtpParamTypes;

/// RPT - Repeat
///
/// Repeats `input` n `times`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::rpt::Rpt};
///
/// let token = Rpt::new(3);
///
/// assert_eq!(token.transform("banana"),Ok("bananabananabanana".to_string()));
///
/// ```
#[derive(Clone, Default)]
pub struct Rpt {
    pub times: usize,
    params: Vec<AtpParamTypes>,
}

impl Rpt {
    pub fn new(times: usize) -> Self {
        Rpt { times, params: vec![times.into()] }
    }
}

impl InstructionMethods for Rpt {
    fn get_params(&self) -> &Vec<AtpParamTypes> {
        &self.params
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("rpt {};\n", self.times).into()
    }

    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        Ok(input.repeat(self.times))
    }

    fn get_string_repr(&self) -> &'static str {
        "rpt"
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 1, "rpt", "")?;

        self.times = parse_args!(params, 0, Usize, "Index should be of usize type");

        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x0d
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [AtpParamTypes::Usize(self.times)]);
        result
    }
}

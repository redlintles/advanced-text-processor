#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{ errors::AtpError, validations::{ check_index_against_input, check_vec_len } },
};

use crate::utils::params::AtpParamTypes;

/// TUCS - To Uppercase Single
///
/// Uppercases a single character in `input` identified by `index`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::tucs::Tucs};
///
/// let token = Tucs::params(1);
///
/// assert_eq!(token.transform("banana"), Ok("bAnana".to_string()));
///
/// ```

#[derive(Clone, Copy, Default)]
pub struct Tucs {
    index: usize,
}

impl Tucs {
    pub fn params(index: usize) -> Self {
        Tucs { index }
    }
}

impl InstructionMethods for Tucs {
    fn get_string_repr(&self) -> &'static str {
        "tucs"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("tucs {};\n", self.index).into()
    }
    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        check_index_against_input(self.index, input)?;
        let result: String = input
            .char_indices()
            .map(|(i, c)| {
                if i == self.index { c.to_uppercase().to_string() } else { c.to_string() }
            })
            .collect();
        Ok(result)
    }

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 1, "tucs", "")?;

        self.index = parse_args!(params, 0, Usize, "Index should be of usize type");
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x14
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [AtpParamTypes::Usize(self.index)]);
        result
    }
}

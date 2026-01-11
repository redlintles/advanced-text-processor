#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::context::execution_context::GlobalExecutionContext;
use crate::utils::params::AtpParamTypes;
use crate::utils::validations::check_vec_len;
use crate::{ tokens::InstructionMethods, utils::validations::check_chunk_bound_indexes };

use crate::utils::errors::{ AtpError };

/// Slt - Select
///
/// Selects a subslice of `input` delimited by `start_index` and `end_index`(inclusive) discarding the rest in the process
/// If end_index is bigger than the length of the string, the subslice will include up to the last character of `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods,transforms::slt::Slt};
///
/// let token = Slt::params(1,9999).unwrap();
///
/// assert_eq!(token.transform("banàna"), Ok("anàn".to_string()));
///
///
/// ```
#[derive(Clone, Default)]
pub struct Slt {
    pub start_index: usize,
    pub end_index: usize,
}

impl Slt {
    pub fn params(start_index: usize, end_index: usize) -> Result<Self, AtpError> {
        check_chunk_bound_indexes(start_index, end_index, None)?;
        Ok(Slt {
            start_index,
            end_index,
        })
    }
}

impl InstructionMethods for Slt {
    fn get_string_repr(&self) -> &'static str {
        "slt"
    }
    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        let len = input.chars().count();
        let mut end = self.end_index;

        if end > len {
            end = len - 1;
        }

        check_chunk_bound_indexes(self.start_index, end, Some(input))?;

        let start_byte = input
            .char_indices()
            .nth(self.start_index)
            .map(|(i, _)| i)
            .unwrap_or(0);

        // Fim EXCLUSIVO: byte do (end_index + 1)º char, ou input.len() se passar do fim
        let end_byte_exclusive = input
            .char_indices()
            .nth(end.saturating_add(1))
            .map(|(i, _)| i)
            .unwrap_or(input.len());

        Ok(input[start_byte..end_byte_exclusive].to_string())
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("slt {} {};\n", self.start_index, self.end_index).into()
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x11
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 2, "slt", "")?;

        self.start_index = parse_args!(params, 0, Usize, "Index should be of usize type");
        self.end_index = parse_args!(params, 1, Usize, "Index should be of usize type");

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::Usize(self.start_index),
            AtpParamTypes::Usize(self.end_index),
        ]);
        result
    }
}

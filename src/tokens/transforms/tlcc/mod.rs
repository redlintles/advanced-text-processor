#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::InstructionMethods,
    utils::{ errors::{ AtpError }, validations::{ check_chunk_bound_indexes, check_vec_len } },
};

use crate::utils::params::AtpParamTypes;

/// TLCC - To Lowercase Chunk
///
/// Lowercases every character from a chunk delimited by `start_index` and `end_index`(inclusive) in `input`
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::tlcc::Tlcc};
///
/// let token = Tlcc::params(1,4).unwrap();
///
/// assert_eq!(token.transform("BANANA"), Ok("BananA".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Tlcc {
    start_index: usize,
    end_index: usize,
}

impl Tlcc {
    pub fn params(start_index: usize, end_index: usize) -> Result<Self, AtpError> {
        check_chunk_bound_indexes(start_index, end_index, None)?;
        Ok(Tlcc {
            start_index,
            end_index,
        })
    }
}

impl InstructionMethods for Tlcc {
    fn get_string_repr(&self) -> &'static str {
        "tlcc"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("tlcc {} {};\n", self.start_index, self.end_index).into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        check_chunk_bound_indexes(self.start_index, self.end_index, Some(input))?;

        let total_chars = input.chars().count();
        let last_char_index = total_chars.saturating_sub(1);

        let end = if self.end_index > last_char_index { last_char_index } else { self.end_index };

        let result: String = input
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i >= self.start_index && i <= end {
                    c.to_lowercase().to_string()
                } else {
                    c.to_string()
                }
            })
            .collect();

        Ok(result)
    }

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 2, "tlcc", "")?;

        self.start_index = parse_args!(params, 0, Usize, "Index should be of usize type");
        self.end_index = parse_args!(params, 1, Usize, "Index should be of usize type");

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x17
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

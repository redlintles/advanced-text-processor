#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::InstructionMethods,
    utils::{ errors::{ AtpError }, validations::{ check_chunk_bound_indexes, check_vec_len } },
};

use crate::utils::params::AtpParamTypes;
/// TUCC - To uppercase Chunk
///
/// Lowercases every character from a chunk delimited by `start_index` and `end_index`(inclusive) in `input`
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::tucc::Tucc};
///
/// let token = Tucc::params(1,4).unwrap();
///
/// assert_eq!(token.transform("banana"), Ok("bANANa".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Tucc {
    start_index: usize,
    end_index: usize,
}

impl Tucc {
    pub fn params(start_index: usize, end_index: usize) -> Result<Self, AtpError> {
        check_chunk_bound_indexes(start_index, end_index, None)?;
        Ok(Tucc {
            start_index,
            end_index,
        })
    }
}

impl InstructionMethods for Tucc {
    fn get_string_repr(&self) -> &'static str {
        "tucc"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("tucc {} {};\n", self.start_index, self.end_index).into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        check_chunk_bound_indexes(self.start_index, self.end_index, Some(input))?;

        // Since the user will probably not know the length of the string in the middle of the processing
        // Better simply adjust end_index to input.len() if its bigger. instead of throwing an "hard to debug" error
        let mut end = self.end_index;
        let total = input.chars().count();

        if end > total {
            end = input.len();
        }
        let result: String = input
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i >= self.start_index && i <= end {
                    c.to_uppercase().to_string()
                } else {
                    c.to_string()
                }
            })
            .collect();
        Ok(result)
    }

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 2, "tucc", "")?;

        self.start_index = parse_args!(params, 0, Usize, "Index should be of usize type");
        self.end_index = parse_args!(params, 1, Usize, "Index should be of usize type");

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x16
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

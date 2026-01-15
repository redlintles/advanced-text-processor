#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::context::execution_context::GlobalExecutionContext;
use crate::utils::errors::AtpError;
use crate::utils::validations::check_vec_len;
use crate::{
    tokens::InstructionMethods,
    utils::transforms::{ capitalize },
    utils::validations::check_chunk_bound_indexes,
};

use crate::utils::{ params::AtpParamTypes };
/// Token `Ctc` — Capitalize Chunk
///
/// Capitalizes every word in a character slice of the input, defined by `start_index` and `end_index` (inclusive).
///
/// The range is applied directly to the character indices of the original string. The extracted chunk is then split
/// into words (using `split_whitespace()`), capitalized individually, and finally reinserted into the original string.
///
/// - If `start_index` is out of bounds for the number of characters in the input, an `AtpError` is returned.
/// - If `end_index` exceeds the input's length, it will be clamped to the input's character count.
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::ctc::Ctc};
///
/// let token = Ctc::new(1, 5).unwrap();
/// assert_eq!(token.transform("bananabananosa"), Ok("bAnanabananosa".to_string()));
/// ```
#[derive(Clone, Default)]
pub struct Ctc {
    pub start_index: usize,
    pub end_index: usize,
    params: Vec<AtpParamTypes>,
}

impl Ctc {
    pub fn new(start_index: usize, end_index: usize) -> Result<Self, AtpError> {
        check_chunk_bound_indexes(start_index, end_index, None)?;
        Ok(Ctc {
            start_index,
            end_index,
            params: vec![start_index.into(), end_index.into()],
        })
    }
}

impl InstructionMethods for Ctc {
    fn get_params(&self) -> &Vec<AtpParamTypes> {
        &self.params
    }
    fn get_string_repr(&self) -> &'static str {
        "ctc"
    }
    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        let len = input.chars().count();

        let mut end = self.end_index;

        if end > len {
            end = len - 1;
        }

        check_chunk_bound_indexes(self.start_index, end, Some(input))?;

        // clamp to avoid overflow

        // Convert char indices to byte indices
        let start_byte = input
            .char_indices()
            .nth(self.start_index)
            .map(|(byte_idx, _)| byte_idx)
            .unwrap(); // safe: start_index < total_chars

        let end_byte = if end == len {
            input.len() // go to the end
        } else {
            input
                .char_indices()
                .nth(end)
                .map(|(byte_idx, _)| byte_idx)
                .unwrap()
        };

        // Extract slice safely
        let slice = &input[start_byte..end_byte];

        // Capitalize all words in the slice
        let capitalized_chunk = slice
            .split_whitespace()
            .map(|w| capitalize(w))
            .collect::<Vec<_>>()
            .join(" ");

        // Rebuild final string
        let prefix = &input[..start_byte];
        let suffix = &input[end_byte..];

        let result = format!("{}{}{}", prefix, capitalized_chunk, suffix);

        Ok(result)
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("ctc {} {};\n", self.start_index, self.end_index).into()
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;
        use crate::utils::params::AtpParamTypesJoin;

        check_vec_len(&params, 2, "ctc", params.join(""))?;

        self.start_index = parse_args!(params, 0, Usize, "Index should be of usize type");
        self.end_index = parse_args!(params, 1, Usize, "Index should be of usize type");

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x1b
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

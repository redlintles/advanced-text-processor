#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::transforms::{ capitalize, string_to_usize },
    utils::validations::check_chunk_bound_indexes,
};
use crate::utils::errors::{ AtpError };

#[cfg(feature = "bytecode")]
use crate::{ utils::{ params::AtpParamTypes, errors::AtpErrorCode } };
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
/// use atp::tokens::{TokenMethods, transforms::ctc::Ctc};
///
/// let token = Ctc::params(1, 5).unwrap();
/// assert_eq!(token.transform("bananabananosa"), Ok("bAnanabananosa".to_string()));
/// ```
#[derive(Clone, Default)]
pub struct Ctc {
    pub start_index: usize,
    pub end_index: usize,
}

impl Ctc {
    pub fn params(start_index: usize, end_index: usize) -> Result<Self, AtpError> {
        check_chunk_bound_indexes(start_index, end_index, None)?;
        Ok(Ctc {
            start_index,
            end_index,
        })
    }
}

impl TokenMethods for Ctc {
    fn get_string_repr(&self) -> &'static str {
        "ctc"
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let total_chars = input.chars().count();

        check_chunk_bound_indexes(self.start_index, self.end_index, Some(input))?;

        let end_index = self.end_index.min(total_chars); // clamp to avoid overflow

        // Convert char indices to byte indices
        let start_byte = input
            .char_indices()
            .nth(self.start_index)
            .map(|(byte_idx, _)| byte_idx)
            .unwrap(); // safe: start_index < total_chars

        let end_byte = if end_index == total_chars {
            input.len() // go to the end
        } else {
            input
                .char_indices()
                .nth(end_index)
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

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "ctc" {
            let start_index = string_to_usize(&line[1])?;
            let end_index = string_to_usize(&line[2])?;

            check_chunk_bound_indexes(start_index, end_index, None)?;
            self.start_index = start_index;
            self.end_index = end_index;
            return Ok(());
        }
        Err(
            AtpError::new(
                crate::utils::errors::AtpErrorCode::TokenNotFound(
                    "Invalid parser for this token".into()
                ),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("ctc {} {};\n", self.start_index, self.end_index).into()
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x1b
    }
    #[cfg(feature = "bytecode")]
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        if instruction.len() != 2 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );
        }

        self.start_index = parse_args!(instruction, 0, Usize, "Index should be of usize type");
        self.end_index = parse_args!(instruction, 1, Usize, "Index should be of usize type");

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        let mut result = Vec::new();

        let instruction_type: u32 = self.get_opcode() as u32;

        let first_param_type: u32 = 0x02;
        let first_param_payload = (self.start_index as u32).to_be_bytes();
        let first_param_payload_size: u32 = first_param_payload.len() as u32;

        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

        let second_param_type: u32 = 0x02;
        let second_param_payload = (self.end_index as u32).to_be_bytes();
        let second_param_payload_size: u32 = second_param_payload.len() as u32;

        let second_param_total_size: u64 = 4 + 4 + (second_param_payload_size as u64);

        let instruction_total_size: u64 =
            8 + 4 + 1 + first_param_total_size + second_param_total_size;

        // Instruction Total Size
        result.extend_from_slice(&instruction_total_size.to_be_bytes());
        // Instruction Type
        result.extend_from_slice(&instruction_type.to_be_bytes());
        // Param Count
        result.push(2);
        // First Param Total Size
        result.extend_from_slice(&first_param_total_size.to_be_bytes());
        // First Param Type
        result.extend_from_slice(&first_param_type.to_be_bytes());
        // First Param Payload Size
        result.extend_from_slice(&first_param_payload_size.to_be_bytes());
        // First Param Payload
        result.extend_from_slice(&first_param_payload);

        // Second Param Total Size
        result.extend_from_slice(&second_param_total_size.to_be_bytes());
        // Second Param Type
        result.extend_from_slice(&second_param_type.to_be_bytes());
        // Second Param Payload Size
        result.extend_from_slice(&second_param_payload_size.to_be_bytes());
        // Second Param Payload
        result.extend_from_slice(&second_param_payload);

        result
    }
}

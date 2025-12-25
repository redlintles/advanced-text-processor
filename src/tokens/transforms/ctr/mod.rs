#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::transforms::{ capitalize, string_to_usize },
    utils::validations::check_chunk_bound_indexes,
};
use crate::utils::errors::{ AtpError, AtpErrorCode };
#[cfg(feature = "bytecode")]
use crate::{ utils::params::AtpParamTypes };

/// Token `Ctr` — Capitalize Range
///
/// Capitalizes a range of words delimited by `start_index` and `end_index`(inclusive)
///
/// Words are defined as sequences of characters separated by whitespace,
/// following the behavior of `input.split_whitespace()`.
///
/// If `start_index` is out of bounds for the number of words in the `input``, an `AtpError` is returned.
/// If `end_index` is out of bound for the number of words in the input, it's clamped up to the number of words in `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods,transforms::ctr::Ctr};
/// let token = Ctr::params(1,5).unwrap();
/// assert_eq!(token.transform("foo bar mar"), Ok("foo Bar Mar".to_string()));
/// ```
///
#[derive(Clone, Default)]
pub struct Ctr {
    pub start_index: usize,
    pub end_index: usize,
}

impl Ctr {
    pub fn params(start_index: usize, end_index: usize) -> Result<Self, AtpError> {
        check_chunk_bound_indexes(start_index, end_index, None)?;
        Ok(Ctr {
            start_index,
            end_index,
        })
    }
}

impl TokenMethods for Ctr {
    fn get_string_repr(&self) -> &'static str {
        "ctr"
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        check_chunk_bound_indexes(self.start_index, self.end_index, Some(input))?;
        // Since the user will probably not know the length of the string in the middle of the processing
        // Better simply adjust end_index to input.len() if its bigger. instead of throwing an "hard to debug" error

        let mut end = self.end_index;
        let total = input.split_whitespace().count();
        if end > total {
            end = total;
        }

        let result = input
            .split_whitespace()
            .enumerate()
            .map(|(i, c)| {
                if (self.start_index..=end).contains(&i) { capitalize(c) } else { c.to_string() }
            })
            .collect::<Vec<_>>()
            .join(" ");

        Ok(result)
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "ctr" {
            let start_index = string_to_usize(&line[1])?;
            let end_index = string_to_usize(&line[2])?;
            check_chunk_bound_indexes(start_index, end_index, None)?;

            self.start_index = start_index;
            self.end_index = end_index;

            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid parser for this token".into()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("ctr {} {};\n", self.start_index, self.end_index).into()
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x1c
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
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::Usize(self.start_index),
            AtpParamTypes::Usize(self.end_index),
        ]);
        result
    }
}

#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::utils::validations::check_vec_len;
use crate::{ tokens::InstructionMethods, utils::validations::check_chunk_bound_indexes };

use crate::utils::errors::{ AtpError, AtpErrorCode };

use crate::utils::params::AtpParamTypes;
/// Dlc - Delete Chunk
///
/// Deletes an specific subslice of `input` delimited by `start_index` and `end_index`(inclusive)
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::dlc::Dlc};
///
/// let token = Dlc::params(1,5).unwrap();
///
/// assert_eq!(token.transform("bananalaranjacheiadecanja"), Ok("blaranjacheiadecanja".to_string()))
///
/// ```
#[derive(Clone, Copy, Default)]
pub struct Dlc {
    pub start_index: usize,
    pub end_index: usize,
}

impl Dlc {
    pub fn params(start_index: usize, end_index: usize) -> Result<Self, AtpError> {
        check_chunk_bound_indexes(start_index, end_index, None)?;
        Ok(Dlc {
            start_index,
            end_index,
        })
    }
}

impl InstructionMethods for Dlc {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("dlc {} {};\n", self.start_index, self.end_index).into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let len = input.chars().count();

        // opcional: se string vazia, deletar "tudo" vira vazio
        if len == 0 {
            return Ok("".to_string());
        }

        let mut end = self.end_index;

        // ✅ clamp correto (inclusive range)
        if end >= len {
            end = len - 1;
        }

        check_chunk_bound_indexes(self.start_index, end, Some(input))?;

        let start_index = input
            .char_indices()
            .nth(self.start_index)
            .map(|(i, _)| i)
            .ok_or_else(|| {
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        format!(
                            "Invalid Index for this specific input, supported indexes 0-{}, entered index {}",
                            input.chars().count().saturating_sub(1),
                            self.start_index
                        ).into()
                    ),
                    self.to_atp_line(),
                    input.to_string()
                )
            })?;

        // ✅ agora end é garantidamente <= last_index, então end+1 pode ser == len
        let end_index = input
            .char_indices()
            .nth(end + 1)
            .map(|(i, _)| i)
            .unwrap_or_else(|| input.len()); // se end é o último char, after começa no fim

        let before = &input[..start_index.min(input.len())];
        let after = &input[end_index.min(input.len())..];

        Ok(format!("{}{}", before, after))
    }

    fn get_string_repr(&self) -> &'static str {
        "dlc"
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 2, "dlc", "")?;

        self.start_index = parse_args!(params, 0, Usize, "Index should be of usize type");
        self.end_index = parse_args!(params, 1, Usize, "Index should be of usize type");

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x08
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

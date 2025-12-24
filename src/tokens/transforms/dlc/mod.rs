#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::transforms::string_to_usize,
    utils::validations::check_chunk_bound_indexes,
};

#[cfg(feature = "bytecode")]
use crate::{ utils::params::AtpParamTypes };
use crate::utils::errors::{ AtpError, AtpErrorCode };
/// Dlc - Delete Chunk
///
/// Deletes an specific subslice of `input` delimited by `start_index` and `end_index`(inclusive)
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::dlc::Dlc};
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

impl TokenMethods for Dlc {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("dlc {} {};\n", self.start_index, self.end_index).into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        check_chunk_bound_indexes(self.start_index, self.end_index, Some(input))?;
        let start_index = input
            .char_indices()
            .nth(self.start_index)
            .map(|(i, _)| i)
            .ok_or_else(||
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        format!(
                            "Invalid Index for this specific input, supported indexes 0-{}, entered index {}",
                            input.char_indices().count().saturating_sub(1),
                            self.start_index
                        ).into()
                    ),
                    self.to_atp_line(),
                    input.to_string()
                )
            )?;
        let end_index = input
            .char_indices()
            .nth(self.end_index + 1)
            .map(|(i, _)| i)
            .ok_or_else(||
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        format!(
                            "Invalid Index for this specific input, supported indexes 0-{}, entered index {}",
                            input.chars().count().saturating_sub(1),
                            self.end_index
                        ).into()
                    ),
                    self.to_atp_line(),
                    input.to_string()
                )
            )?;

        let before = &input[..start_index.min(input.len())];
        let after = &input[end_index.min(input.len())..];

        Ok(format!("{}{}", before, after))
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "dlc;"

        if line[0] == "dlc" {
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

    fn get_string_repr(&self) -> &'static str {
        "dlc"
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x08
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

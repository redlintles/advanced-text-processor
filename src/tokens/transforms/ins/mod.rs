#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

#[cfg(feature = "bytecode")]
use crate::{ utils::params::AtpParamTypes };

use crate::{
    tokens::TokenMethods,
    utils::{ transforms::string_to_usize, errors::{ AtpError, AtpErrorCode } },
};
/// Ins - Insert
///
/// Inserts `text` after `index` position in `input`
///
/// If index does not exists in current string, `AtpError` is returned
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::ins::Ins};
///
/// let token = Ins::params(2,"laranja");
///
/// assert_eq!(token.transform("banana"), Ok("banlaranjaana".to_string()));
/// ```
#[derive(Clone, Default)]
pub struct Ins {
    index: usize,
    text_to_insert: String,
}

impl Ins {
    pub fn params(index: usize, text_to_insert: &str) -> Self {
        Ins {
            index,
            text_to_insert: text_to_insert.to_string(),
        }
    }
}
impl TokenMethods for Ins {
    fn get_string_repr(&self) -> &'static str {
        "ins"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("ins {} {};\n", self.index, self.text_to_insert).into()
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "ins" {
            self.index = string_to_usize(&line[1])?;
            self.text_to_insert = line[2].clone();
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("".into()),
                "ins".to_string(),
                line.join(" ")
            )
        )
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        if self.index > input.chars().count() {
            return Err(
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        format!(
                            "Index does not exist in current string, for the input {}, only indexes between 0-{} are allowed",
                            input,
                            input.len().saturating_sub(1)
                        ).into()
                    ),
                    self.to_atp_line(),
                    input.to_string()
                )
            );
        }
        let byte_index = input
            .char_indices()
            .nth(self.index + 1)
            .map(|(i, _)| i)
            .unwrap_or(input.len());

        let (before, after) = input.split_at(byte_index);

        let result = format!("{}{}{}", before, self.text_to_insert, after);

        Ok(result)
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x28
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

        self.index = parse_args!(instruction, 0, Usize, "Index should be of usize type");
        self.text_to_insert = parse_args!(
            instruction,
            1,
            String,
            "Text_to_insert should be of String type"
        );

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        let mut result = Vec::new();

        let instruction_type: u32 = self.get_opcode() as u32;

        let first_param_type: u32 = 0x02;
        let first_param_payload = (self.index as u32).to_be_bytes();
        let first_param_payload_size: u32 = first_param_payload.len() as u32;

        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

        let second_param_type: u32 = 0x02;
        let second_param_payload = self.text_to_insert.as_bytes();
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

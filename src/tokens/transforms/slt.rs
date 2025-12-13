use std::borrow::Cow;

#[cfg(feature = "bytecode")]
use crate::utils::bytecode_utils::AtpParamTypes;
use crate::{
    tokens::TokenMethods,
    utils::transforms::string_to_usize,
    utils::validations::check_chunk_bound_indexes,
};

use crate::utils::errors::{ AtpError, AtpErrorCode };
#[cfg(feature = "bytecode")]
use crate::{ bytecode::{ BytecodeTokenMethods } };

/// Slt - Select
///
/// Selects a subslice of `input` delimited by `start_index` and `end_index`(inclusive) discarding the rest in the process
/// If end_index is bigger than the length of the string, the subslice will include up to the last character of `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods,transforms::slt::Slt};
///
/// let token = Slt::params(1,9999).unwrap();
///
/// assert_eq!(token.parse("banàna"), Ok("anàn".to_string()));
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

impl TokenMethods for Slt {
    fn get_string_repr(&self) -> &'static str {
        "slt"
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let total_chars = input.chars().count();
        let last_char_index = total_chars.saturating_sub(1);

        check_chunk_bound_indexes(self.start_index, self.end_index, Some(input))?;

        let subslice_start = input
            .char_indices()
            .nth(self.start_index)
            .map(|(i, _)| i)
            .unwrap_or(0);

        let subslice_end = if self.end_index > last_char_index {
            last_char_index
        } else {
            input
                .char_indices()
                .nth(self.end_index)
                .map(|(i, _)| i)
                .unwrap_or(input.len())
        };

        Ok(input[subslice_start..=subslice_end].to_string())
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "slt" {
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
        format!("slt {} {};\n", self.start_index, self.end_index).into()
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Slt {
    fn get_opcode(&self) -> u32 {
        0x11
    }

    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        if instruction.len() != 2 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );
        }

        match &instruction[0] {
            AtpParamTypes::Usize(payload) => {
                self.start_index = payload.clone();
            }
            _ => {
                return Err(
                    AtpError::new(
                        AtpErrorCode::InvalidParameters(
                            "This token takes a single usize as argument".into()
                        ),
                        "",
                        ""
                    )
                );
            }
        }
        match &instruction[1] {
            AtpParamTypes::Usize(payload) => {
                self.end_index = payload.clone();
            }
            _ => {
                return Err(
                    AtpError::new(
                        AtpErrorCode::InvalidParameters(
                            "This token takes a single usize as argument".into()
                        ),
                        "",
                        ""
                    )
                );
            }
        }

        return Ok(());
    }

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

#[cfg(feature = "test_access")]
#[cfg(test)]
mod slt_tests {
    use crate::tokens::{ TokenMethods, transforms::slt::Slt };

    #[test]
    fn select() {
        let mut token = Slt::params(1, 5).unwrap();
        assert!(
            matches!(Slt::params(5, 1), Err(_)),
            "it throws an error if start_index is bigger than end_index"
        );

        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if start_index does not exists in input"
        );

        assert_eq!(
            token.parse("banànalaranjacheiadecanja"),
            Ok("anàna".to_string()),
            "It works with expected inputs"
        );

        assert_eq!(
            token.to_atp_line(),
            "slt 1 5;\n".to_string(),
            "conversion to atp_line works correctly"
        );

        assert_eq!(token.get_string_repr(), "slt".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["slt".to_string(), (5).to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["slt".to_string(), (1).to_string(), (5).to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn select_bytecode() {
        use crate::{ bytecode::BytecodeTokenMethods, utils::bytecode_utils::AtpParamTypes };

        let mut token = Slt::params(1, 3).unwrap();

        let instruction: Vec<AtpParamTypes> = vec![AtpParamTypes::Usize(3)];

        assert_eq!(token.get_opcode(), 0x11, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        let first_param_type: u32 = 0x02;
        let first_param_payload = vec![0x01];
        let first_param_payload_size = first_param_payload.len() as u32;
        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

        let second_param_type: u32 = 0x02;
        let second_param_payload = vec![0x03];
        let second_param_payload_size = second_param_payload.len() as u32;
        let second_param_total_size: u64 = 4 + 4 + (second_param_payload_size as u64);

        let instruction_type: u32 = 0x11;
        let param_count: u8 = 0x02;

        let instruction_total_size: u64 =
            8 + 4 + 1 + first_param_total_size + second_param_total_size;

        let mut expected_output: Vec<u8> = vec![];

        expected_output.extend_from_slice(&instruction_total_size.to_be_bytes());

        expected_output.extend_from_slice(&instruction_type.to_be_bytes());

        expected_output.push(param_count);

        expected_output.extend_from_slice(&first_param_total_size.to_be_bytes());
        expected_output.extend_from_slice(&first_param_type.to_be_bytes());
        expected_output.extend_from_slice(&first_param_payload_size.to_be_bytes());
        expected_output.extend_from_slice(&first_param_payload);

        expected_output.extend_from_slice(&second_param_total_size.to_be_bytes());
        expected_output.extend_from_slice(&second_param_type.to_be_bytes());
        expected_output.extend_from_slice(&second_param_payload_size.to_be_bytes());
        expected_output.extend_from_slice(&second_param_payload);

        assert_eq!(
            token.to_bytecode(),
            expected_output,
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

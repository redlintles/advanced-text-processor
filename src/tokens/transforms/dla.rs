use std::borrow::Cow;

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;
use crate::utils::validations::check_index_against_input;
use crate::{ tokens::TokenMethods, utils::transforms::string_to_usize };

use crate::utils::errors::{ AtpError, AtpErrorCode };

/// Dla - Delete After
/// Delete all characters after `index` in the specified `input`
///
/// It will throw an `AtpError` if index does not exists in the current `input`
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::dla::Dla};
///
/// let token = Dla::params(3);
///
/// assert_eq!(token.parse("banana laranja vermelha azul"), Ok("bana".to_string()))
///
/// ```
#[derive(Clone, Copy, Default)]
pub struct Dla {
    pub index: usize,
}

impl Dla {
    pub fn params(index: usize) -> Self {
        Dla {
            index,
        }
    }
}

impl TokenMethods for Dla {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("dla {};\n", self.index).into()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        check_index_against_input(self.index, input)?;

        let mut s = String::from(input);
        if
            let Some(byte_index) = s
                .char_indices()
                .nth(self.index + 1)
                .map(|(i, _)| i)
        {
            s.drain(byte_index..);
            return Ok(s);
        }
        Err(
            AtpError::new(
                AtpErrorCode::IndexOutOfRange(
                    "Index is out of range for the desired string".into()
                ),
                self.to_atp_line(),
                input.to_string()
            )
        )
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "dla;"

        if line[0] == "dla" {
            self.index = string_to_usize(&line[1])?;
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
        "dla"
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x09
    }
    #[cfg(feature = "bytecode")]
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        if instruction.len() != 1 {
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
                self.index = payload.clone();
                return Ok(());
            }
            _ => {
                Err(
                    AtpError::new(
                        AtpErrorCode::InvalidParameters(
                            "This token takes a single usize as argument".into()
                        ),
                        "",
                        ""
                    )
                )
            }
        }
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        let mut result = Vec::new();

        let instruction_type: u32 = self.get_opcode() as u32;

        let first_param_type: u32 = 0x02;
        let first_param_payload = (self.index as u32).to_be_bytes();
        let first_param_payload_size: u32 = first_param_payload.len() as u32;

        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);
        let instruction_total_size: u64 = 4 + 1 + first_param_total_size;

        // Instruction Total Size
        result.extend_from_slice(&instruction_total_size.to_be_bytes());
        // Instruction Type
        result.extend_from_slice(&instruction_type.to_be_bytes());
        // Param Count
        result.push(1);
        // First Param Total Size
        result.extend_from_slice(&first_param_total_size.to_be_bytes());
        // First Param Type
        result.extend_from_slice(&first_param_type.to_be_bytes());
        // First Param Payload Size
        result.extend_from_slice(&first_param_payload_size.to_be_bytes());
        // First Param Payload
        result.extend_from_slice(&first_param_payload);

        result
    }
}
#[cfg(feature = "test_access")]
#[cfg(test)]
mod dla_tests {
    use crate::{ tokens::{ TokenMethods, transforms::dla::Dla }, utils::params::AtpParamTypes };
    #[test]
    fn delete_after_test() {
        let mut token = Dla::params(3);

        assert_eq!(
            token.parse("banana laranja vermelha azul"),
            Ok("bana".to_string()),
            "It works correctly with expected inputs"
        );
        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if the string does not have the current token index"
        );

        assert_eq!(
            token.to_atp_line(),
            "dla 3;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "dla".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["dla".to_string(), (3).to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }
    #[test]
    fn delete_after_bytecode() {
        let mut token = Dla::params(3);

        let instruction: Vec<AtpParamTypes> = vec![AtpParamTypes::Usize(3)];

        assert_eq!(token.get_opcode(), 0x09, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        let first_param_type: u32 = 0x02;
        let first_param_payload = vec![0x03];
        let first_param_payload_size = first_param_payload.len() as u32;
        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

        let instruction_type: u32 = 0x09;
        let param_count: u8 = 0x01;

        let instruction_total_size: u64 = 4 + 1 + first_param_total_size;

        let mut expected_output: Vec<u8> = vec![];

        expected_output.extend_from_slice(&instruction_total_size.to_be_bytes());
        expected_output.extend_from_slice(&instruction_type.to_be_bytes());
        expected_output.push(param_count);
        expected_output.extend_from_slice(&first_param_total_size.to_be_bytes());
        expected_output.extend_from_slice(&first_param_type.to_be_bytes());
        expected_output.extend_from_slice(&first_param_payload_size.to_be_bytes());
        expected_output.extend_from_slice(&first_param_payload);
        assert_eq!(
            token.to_bytecode(),
            expected_output,
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

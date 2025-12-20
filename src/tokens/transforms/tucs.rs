use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{
        errors::{ AtpError, AtpErrorCode },
        transforms::string_to_usize,
        validations::{ check_index_against_input, check_vec_len },
    },
};

#[cfg(feature = "bytecode")]
use crate::{ bytecode::BytecodeTokenMethods, utils::params::AtpParamTypes };

/// TUCS - To Uppercase Single
///
/// Uppercases a single character in `input` identified by `index`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::tucs::Tucs};
///
/// let token = Tucs::params(1);
///
/// assert_eq!(token.parse("banana"), Ok("bAnana".to_string()));
///
/// ```

#[derive(Clone, Copy, Default)]
pub struct Tucs {
    index: usize,
}

impl Tucs {
    pub fn params(index: usize) -> Self {
        Tucs {
            index,
        }
    }
}

impl TokenMethods for Tucs {
    fn get_string_repr(&self) -> &'static str {
        "tucs"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("tucs {};\n", self.index).into()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        check_index_against_input(self.index, input)?;
        let result: String = input
            .char_indices()
            .map(|(i, c)| {
                if i == self.index { c.to_uppercase().to_string() } else { c.to_string() }
            })
            .collect();
        Ok(result)
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        check_vec_len(&line, 2)?;
        if line[0] == "tucs" {
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
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Tucs {
    fn get_opcode(&self) -> u32 {
        0x14
    }

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
mod tucs_tests {
    use crate::tokens::{ TokenMethods, transforms::tucs::Tucs };
    #[test]
    fn to_uppercase_single_tests() {
        let mut token = Tucs::params(1);

        assert_eq!(token.parse("banana"), Ok("bAnana".to_string()));

        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if start_index does not exists in input"
        );

        assert_eq!(
            token.to_atp_line(),
            "tucs 1;\n".to_string(),
            "conversion to atp_line works correctly"
        );

        assert_eq!(
            token.get_string_repr(),
            "tucs".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(["tucs".to_string(), "banana".to_string()].to_vec()),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
        assert!(
            matches!(token.from_vec_params(["tucs".to_string(), (1).to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn to_uppercase_single_bytecode_tests() {
        use crate::{ bytecode::BytecodeTokenMethods, utils::params::AtpParamTypes };

        let mut token = Tucs::params(3);

        let instruction: Vec<AtpParamTypes> = vec![AtpParamTypes::Usize(3)];

        assert_eq!(token.get_opcode(), 0x14, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        let first_param_type: u32 = 0x02;
        let first_param_payload = vec![0x03];
        let first_param_payload_size = first_param_payload.len() as u32;
        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

        let instruction_type: u32 = 0x14;
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

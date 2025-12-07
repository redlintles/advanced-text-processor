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
use crate::{ bytecode::BytecodeTokenMethods, utils::bytecode_utils::AtpParamTypes };

/// TLCW - To Lowercase Word
///
/// Lowercase a single word of string
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::tlcw::Tlcw};
///
/// let token = Tlcw::params(1);
///
/// assert_eq!(token.parse("BANANA LARANJA CHEIA DE CANJA"), Ok("BANANA laranja CHEIA DE CANJA".to_string()));
///
/// ```
#[derive(Clone, Default, Copy)]
pub struct Tlcw {
    index: usize,
}

impl Tlcw {
    pub fn params(index: usize) -> Self {
        Tlcw { index }
    }
}
impl TokenMethods for Tlcw {
    fn get_string_repr(&self) -> &'static str {
        "tlcw"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("tlcw {};\n", self.index).into()
    }

    fn parse(&self, input: &str) -> Result<String, crate::utils::errors::AtpError> {
        check_index_against_input(self.index, input)?;
        Ok(
            input
                .split_whitespace()
                .enumerate()
                .map(|(i, w)| {
                    if i == self.index { w.to_lowercase() } else { w.to_string() }
                })
                .collect::<Vec<_>>()
                .join(" ")
                .to_string()
        )
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), crate::utils::errors::AtpError> {
        check_vec_len(&line, 2)?;
        if line[0] == "tlcw" {
            self.index = string_to_usize(&line[1])?;
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid Parser for this token".into()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Tlcw {
    fn get_opcode(&self) -> u8 {
        0x29
    }

    fn from_params(&mut self, instruction: Vec<AtpParamTypes>) -> Result<(), AtpError> {
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

        let first_param_total_size: u64 = 8 + 4 + 4 + (first_param_payload_size as u64);
        let instruction_total_size: u64 = 8 + 4 + 1 + first_param_total_size;

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
mod tlcw_tests {
    use crate::tokens::{ TokenMethods, transforms::tlcw::Tlcw };
    #[test]
    fn to_lowercase_word_tests() {
        let mut token = Tlcw::params(1);

        assert_eq!(
            token.parse("BANANA LARANJA CHEIA DE CANJA"),
            Ok("BANANA laranja CHEIA DE CANJA".to_string())
        );

        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if start_index does not exists in input"
        );

        assert_eq!(
            token.to_atp_line(),
            "tlcw 1;\n".to_string(),
            "conversion to atp_line works correctly"
        );

        assert_eq!(
            token.get_string_repr(),
            "tlcw".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(["tlcw".to_string(), "banana".to_string()].to_vec()),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
        assert!(
            matches!(token.from_vec_params(["tlcw".to_string(), (1).to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn to_lowercase_word_bytecode_tests() {
        use crate::{ bytecode::BytecodeTokenMethods, utils::bytecode_utils::AtpParamTypes };

        let mut token = Tlcw::params(3);

        let instruction: Vec<AtpParamTypes> = vec![AtpParamTypes::Usize(3)];

        assert_eq!(token.get_opcode(), 0x29, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        let first_param_type: u32 = 0x02;
        let first_param_payload = vec![0x03];
        let first_param_payload_size = first_param_payload.len() as u32;
        let first_param_total_size: u64 = 8 + 4 + 4 + (first_param_payload_size as u64);

        let instruction_type: u32 = 0x29;
        let param_count: u8 = 0x01;

        let instruction_total_size: u64 = 8 + 4 + 1 + first_param_total_size;

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

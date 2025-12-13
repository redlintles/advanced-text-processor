use std::borrow::Cow;

#[cfg(feature = "bytecode")]
use crate::utils::bytecode_utils::AtpParamTypes;
use crate::utils::validations::{ check_index_against_input, check_vec_len };
use crate::{ tokens::TokenMethods, utils::transforms::string_to_usize };
use crate::utils::errors::{ AtpError, AtpErrorCode };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeTokenMethods };
/// Dlb - Delete Before
/// Delete all characters before `index` in the specified `input`
///
/// It will throw an `AtpError` if index does not exists in the current `input`
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::dlb::Dlb};
///
/// let token = Dlb::params(3);
///
/// assert_eq!(token.parse("banana laranja vermelha azul"), Ok("ana laranja vermelha azul".to_string()))
///
/// ```
#[derive(Clone, Copy, Default)]
pub struct Dlb {
    pub index: usize,
}

impl Dlb {
    pub fn params(index: usize) -> Self {
        Dlb {
            index,
        }
    }
}

impl TokenMethods for Dlb {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("dlb {};\n", self.index).into()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let mut s = String::from(input);

        check_index_against_input(self.index, input)?;

        if
            let Some(byte_index) = s
                .char_indices()
                .nth(self.index)
                .map(|(i, _)| i)
        {
            s.drain(0..byte_index);
            return Ok(s);
        }

        Err(
            AtpError::new(
                AtpErrorCode::IndexOutOfRange(
                    format!(
                        "Supported indexes 0-{}, entered index {}",
                        input.chars().count().saturating_sub(1),
                        self.index
                    ).into()
                ),
                self.to_atp_line(),
                input.to_string()
            )
        )
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "dlb;"

        check_vec_len(&line, 2)?;

        if line[0] == "dlb" {
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
        "dlb"
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Dlb {
    fn get_opcode(&self) -> u32 {
        0x0a
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
mod dlb_tests {
    use crate::{
        tokens::{ TokenMethods, transforms::dlb::Dlb },
        utils::bytecode_utils::AtpParamTypes,
    };
    #[test]
    fn delete_before_test() {
        let mut token = Dlb::params(3);

        assert_eq!(
            token.parse("banana laranja vermelha azul"),
            Ok("ana laranja vermelha azul".to_string()),
            "It works correctly with expected inputs"
        );
        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if the string does not have the current token index"
        );

        assert_eq!(
            token.to_atp_line(),
            "dlb 3;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "dlb".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["dlb".to_string(), (3).to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }
    #[test]
    fn delete_before_bytecode() {
        use crate::bytecode::{ BytecodeTokenMethods };

        let mut token = Dlb::params(3);

        let instruction: Vec<AtpParamTypes> = vec![AtpParamTypes::Usize(3)];

        assert_eq!(token.get_opcode(), 0x0a, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        let first_param_type: u32 = 0x02;
        let first_param_payload = vec![0x03];
        let first_param_payload_size = first_param_payload.len() as u32;
        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

        let instruction_type: u32 = 0x0a;
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

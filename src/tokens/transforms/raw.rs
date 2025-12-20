use std::borrow::Cow;

use regex::Regex;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::{ bytecode::{ BytecodeTokenMethods }, utils::params::AtpParamTypes };
/// RAW - Replace All With
///
/// Replace all ocurrences of `pattern` in `input` with `text_to_replace`
///
/// See Also:
///
/// - [`RCW` - Replace Count With](crate::tokens::transforms::rcw)
/// - [`RFW` - Replace First With](crate::tokens::transforms::rfw)
/// - [`RLW` - Replace Last With](crate::tokens::transforms::rlw)
/// - [`RNW` - Replace Nth With](crate::tokens::transforms::rnw)
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::raw::Raw};
///
/// let token = Raw::params(&"a", "b").unwrap();
///
/// assert_eq!(token.parse("aaaaa"), Ok("bbbbb".to_string()));
/// ```
///
#[derive(Clone)]
pub struct Raw {
    pub pattern: Regex,
    pub text_to_replace: String,
}

impl Raw {
    pub fn params(pattern: &str, text_to_replace: &str) -> Result<Self, String> {
        let pattern = Regex::new(&pattern).map_err(|x| x.to_string())?;
        Ok(Raw {
            text_to_replace: text_to_replace.to_string(),
            pattern,
        })
    }
}

impl Default for Raw {
    fn default() -> Self {
        Raw {
            pattern: Regex::new("").unwrap(),
            text_to_replace: "_".to_string(),
        }
    }
}

impl TokenMethods for Raw {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("raw {} {};\n", self.pattern, self.text_to_replace).into()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(self.pattern.replace_all(input, &self.text_to_replace).to_string())
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "raw;"

        if line[0] == "raw" {
            self.pattern = Regex::new(&line[1]).map_err(|_|
                AtpError::new(
                    AtpErrorCode::TextParsingError("Failed creating regex".into()),
                    line[0].to_string(),
                    line.join(" ")
                )
            )?;
            self.text_to_replace = line[2].clone();
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
        "raw"
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Raw {
    fn get_opcode(&self) -> u32 {
        0x0b
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
            AtpParamTypes::String(payload) => {
                self.pattern = Regex::new(&payload.clone()).map_err(|_|
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed to create regex".into()),
                        "sslt",
                        payload.clone()
                    )
                )?;
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
            AtpParamTypes::String(payload) => {
                self.text_to_replace = payload.to_string();
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
        let first_param_payload = self.pattern.as_str().as_bytes();
        let first_param_payload_size: u32 = first_param_payload.len() as u32;

        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

        let second_param_type: u32 = 0x02;
        let second_param_payload = self.text_to_replace.as_bytes();
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

#[cfg(test)]
#[cfg(feature = "test_access")]
mod raw_tests {
    use crate::tokens::{ TokenMethods, transforms::raw::Raw };
    #[test]
    fn replace_all_with_tests() {
        let mut token = Raw::params("a", "b").unwrap();
        assert_eq!(token.parse("aaaaa"), Ok("bbbbb".to_string()), "It supports expected inputs");

        assert_eq!(
            token.to_atp_line(),
            "raw a b;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "raw".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["raw".to_string(), "a".to_string(), "b".to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn replace_all_with_bytecode_tests() {
        use crate::{ bytecode::BytecodeTokenMethods, utils::params::AtpParamTypes };

        let mut token = Raw::params("banana", "laranja").unwrap();

        let instruction: Vec<AtpParamTypes> = vec![
            AtpParamTypes::Usize(3),
            AtpParamTypes::String("Banana".to_string())
        ];

        assert_eq!(token.get_opcode(), 0x0b, "get_opcode does not disrepect ATP token mapping");

        let first_param_type: u32 = *&instruction[0].get_param_type_code();
        let first_param_payload = "banana".as_bytes();
        let first_param_payload_size = first_param_payload.len() as u32;
        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

        let second_param_type: u32 = *&instruction[1].get_param_type_code();
        let second_param_payload = "laranja".as_bytes();
        let second_param_payload_size = second_param_payload.len() as u32;
        let second_param_total_size: u64 = 4 + 4 + (second_param_payload_size as u64);

        let instruction_type: u32 = 0x0b;
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
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            expected_output,
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

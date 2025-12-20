use std::borrow::Cow;

use regex::Regex;

use crate::{
    tokens::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::string_to_usize },
};

#[cfg(feature = "bytecode")]
use crate::{ bytecode::BytecodeTokenMethods, utils::params::AtpParamTypes };
/// RLW - Replace Last With
///
/// Replace the `nth`` ocurrency of `pattern` in `input` with `text_to_replace`
///
/// See Also:
///
/// - [`RAW` - Replace All With](crate::tokens::transforms::raw)
/// - [`RCW` - Replace Count With](crate::tokens::transforms::rcw)
/// - [`RFW` - Replace First With](crate::tokens::transforms::rfw)
/// - [`RLW` - Replace Last With](crate::tokens::transforms::rlw)
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::rnw::Rnw};
///
/// let token = Rnw::params(&"a", "b", 2).unwrap();
///
/// assert_eq!(token.parse("aaaaa"), Ok("aabaa".to_string()));
/// ```
///
#[derive(Clone)]
pub struct Rnw {
    pub pattern: Regex,
    pub text_to_replace: String,
    pub index: usize,
}

impl Rnw {
    pub fn params(pattern: &str, text_to_replace: &str, index: usize) -> Result<Self, String> {
        let pattern = Regex::new(&pattern).map_err(|x| x.to_string())?;
        Ok(Rnw {
            text_to_replace: text_to_replace.to_string(),
            pattern,
            index,
        })
    }
}

impl Default for Rnw {
    fn default() -> Self {
        Rnw {
            pattern: Regex::new("").unwrap(),
            text_to_replace: "_".to_string(),
            index: 0,
        }
    }
}

impl TokenMethods for Rnw {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("rnw {} {} {};\n", self.pattern, self.text_to_replace, self.index).into()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let mut count = 0;

        let mut idx = None;

        for m in self.pattern.find_iter(input) {
            if count == self.index {
                idx = Some((m.start(), m.end()));
                break;
            }
            count += 1;
        }

        if let Some((start, end)) = idx {
            let mut result = String::with_capacity(
                input.len() - (end - start) + self.text_to_replace.len()
            );
            result.push_str(&input[..start]);
            result.push_str(&self.text_to_replace);
            result.push_str(&input[end..]);
            return Ok(result);
        }
        Ok(input.to_string())
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "rnw;"

        if line[0] == "rnw" {
            self.pattern = Regex::new(&line[1]).map_err(|_|
                AtpError::new(
                    AtpErrorCode::TextParsingError("Failed creating regex".into()),
                    line[0].to_string(),
                    line.join(" ")
                )
            )?;
            self.text_to_replace = line[2].clone();
            self.index = string_to_usize(&line[3])?;
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
        "rnw"
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Rnw {
    fn get_opcode(&self) -> u32 {
        0x1f
    }

    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        if instruction.len() != 3 {
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
                            "This token takes a single String as argument".into()
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
                            "This token takes a single String as argument".into()
                        ),
                        "",
                        ""
                    )
                );
            }
        }
        match &instruction[2] {
            AtpParamTypes::Usize(payload) => {
                self.index = payload.clone();
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

        let first_param_type: u32 = 0x01;
        let first_param_payload = self.pattern.as_str().as_bytes();
        let first_param_payload_size: u32 = first_param_payload.len() as u32;

        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

        let second_param_type: u32 = 0x01;
        let second_param_payload = self.text_to_replace.as_bytes();
        let second_param_payload_size: u32 = second_param_payload.len() as u32;

        let second_param_total_size: u64 = 4 + 4 + (second_param_payload_size as u64);

        let third_param_type: u32 = 0x02;
        let third_param_payload = (self.index as u32).to_be_bytes();
        let third_param_payload_size: u32 = third_param_payload.len() as u32;

        let third_param_total_size: u64 = 4 + 4 + (third_param_payload_size as u64);

        let instruction_total_size: u64 =
            8 + 4 + 1 + first_param_total_size + second_param_total_size + third_param_total_size;

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

        // Third Param Total Size
        result.extend_from_slice(&third_param_total_size.to_be_bytes());
        // Third Param Type
        result.extend_from_slice(&third_param_type.to_be_bytes());
        // Third Param Payload Size
        result.extend_from_slice(&third_param_payload_size.to_be_bytes());
        // Third Param Payload
        result.extend_from_slice(&third_param_payload);

        result
    }
}

#[cfg(test)]
#[cfg(feature = "test_access")]
mod rnw_tests {
    use crate::tokens::{ TokenMethods, transforms::rnw::Rnw };
    #[test]
    fn replace_nth_with_tests() {
        let mut token = Rnw::params("a", "b", 2).unwrap();
        assert_eq!(token.parse("aaaaa"), Ok("aabaa".to_string()), "It supports expected inputs");

        assert_eq!(
            token.to_atp_line(),
            "rnw a b 2;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "rnw".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["rnw".to_string(), "a".to_string(), "b".to_string(), (2).to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn replace_count_with_bytecode_tests() {
        use crate::{ bytecode::BytecodeTokenMethods, utils::params::AtpParamTypes };

        let mut token = Rnw::params("banana", "laranja", 3).unwrap();

        let instruction: Vec<AtpParamTypes> = vec![
            AtpParamTypes::String("banana".to_string()),
            AtpParamTypes::String("Laranja".to_string()),
            AtpParamTypes::Usize(3)
        ];

        assert_eq!(token.get_opcode(), 0x0b, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        let first_param_type: u32 = (&instruction[0]).get_param_type_code();
        let first_param_payload = "banana".as_bytes();
        let first_param_payload_size = first_param_payload.len() as u32;
        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

        let second_param_type: u32 = (&instruction[1]).get_param_type_code();
        let second_param_payload = "laranja".as_bytes();
        let second_param_payload_size = second_param_payload.len() as u32;
        let second_param_total_size: u64 = 4 + 4 + (second_param_payload_size as u64);

        let third_param_type: u32 = (&instruction[2]).get_param_type_code();
        let third_param_payload = (3 as usize).to_be_bytes();
        let third_param_payload_size = third_param_payload.len() as u32;
        let third_param_total_size: u64 = 4 + 4 + (third_param_payload_size as u64);

        let instruction_type: u32 = 0x0b;
        let param_count: u8 = 0x02;

        let instruction_total_size: u64 =
            8 + 4 + 1 + first_param_total_size + second_param_total_size + third_param_total_size;

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

        expected_output.extend_from_slice(&third_param_total_size.to_be_bytes());
        expected_output.extend_from_slice(&third_param_type.to_be_bytes());
        expected_output.extend_from_slice(&third_param_payload_size.to_be_bytes());
        expected_output.extend_from_slice(&third_param_payload);

        assert_eq!(
            token.to_bytecode(),
            expected_output,
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

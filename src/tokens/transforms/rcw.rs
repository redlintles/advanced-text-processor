use std::borrow::Cow;

use regex::Regex;

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;
use crate::{ tokens::TokenMethods, utils::transforms::string_to_usize };
use crate::utils::errors::{ AtpError, AtpErrorCode };

/// RCW - Replace Count With
///
/// Replace `count` ocurrences of `pattern` in `input` with `text_to_replace`
///
/// See Also:
///
/// - [`RAW` - Replace All With](crate::tokens::transforms::rcw)
/// - [`RFW` - Replace First With](crate::tokens::transforms::rfw)
/// - [`RLW` - Replace Last With](crate::tokens::transforms::rlw)
/// - [`RNW` - Replace Nth With](crate::tokens::transforms::rnw)
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::rcw::Rcw};
///
/// let token = Rcw::params(&"a", "b", 3).unwrap();
///
/// assert_eq!(token.parse("aaaaa"), Ok("bbbaa".to_string()));
/// ```
///
#[derive(Clone)]
pub struct Rcw {
    pub pattern: Regex,
    pub count: usize,
    pub text_to_replace: String,
}

impl Rcw {
    pub fn params(pattern: &str, text_to_replace: &str, count: usize) -> Result<Self, String> {
        let pattern = Regex::new(&pattern).map_err(|x| x.to_string())?;
        Ok(Rcw {
            text_to_replace: text_to_replace.to_string(),
            pattern,
            count,
        })
    }
}

impl Default for Rcw {
    fn default() -> Self {
        Rcw {
            pattern: Regex::new("").unwrap(),
            text_to_replace: "_".to_string(),
            count: 0 as usize,
        }
    }
}

impl TokenMethods for Rcw {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("rcw {} {} {};\n", self.pattern, self.text_to_replace, self.count).into()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(self.pattern.replacen(input, self.count, &self.text_to_replace).to_string())
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "rcw;"

        if line[0] == "rcw" {
            self.pattern = Regex::new(&line[1]).map_err(|_|
                AtpError::new(
                    AtpErrorCode::TextParsingError("Failed creating regex".into()),
                    line[0].to_string(),
                    line.join(" ")
                )
            )?;
            self.text_to_replace = line[2].clone();
            self.count = string_to_usize(&line[3])?;
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
        "rcw"
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x10
    }
    #[cfg(feature = "bytecode")]
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        if instruction.len() != 3 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );
        }

        let pattern_payload = parse_args!(
            instruction,
            0,
            String,
            "Pattern should be of string type"
        );

        self.pattern = Regex::new(&pattern_payload.clone()).map_err(|_|
            AtpError::new(
                AtpErrorCode::TextParsingError("Failed to create regex".into()),
                "sslt",
                pattern_payload.clone()
            )
        )?;

        self.text_to_replace = parse_args!(
            instruction,
            1,
            String,
            "Text_to_replace should be of type String"
        );

        self.count = parse_args!(instruction, 2, Usize, "Index should be of type Usize");

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
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
        let third_param_payload = (self.count as u32).to_be_bytes();
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
mod rcw_tests {
    use crate::tokens::{ TokenMethods, transforms::rcw::Rcw };
    #[test]
    fn replace_count_with_tests() {
        let mut token = Rcw::params("a", "b", 3).unwrap();
        assert_eq!(token.parse("aaaaa"), Ok("bbbaa".to_string()), "It supports expected inputs");

        assert_eq!(
            token.to_atp_line(),
            "rcw a b 3;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "rcw".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["rcw".to_string(), "a".to_string(), "b".to_string(), (3).to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn replace_count_with_bytecode_tests() {
        use crate::{ utils::params::AtpParamTypes };

        let mut token = Rcw::params("banana", "laranja", 3).unwrap();

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

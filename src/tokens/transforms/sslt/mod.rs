#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use regex::Regex;

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;
use crate::utils::validations::check_vec_len;
use crate::{ tokens::TokenMethods, utils::transforms::string_to_usize };

use crate::utils::errors::{ AtpError, AtpErrorCode };

/// SSLT - Split Select
///
/// Splits `input` by `pattern and return `index` of the resulting vec,
/// *discarding* the rest of the text in the process.
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::sslt::Sslt};
///
/// let token = Sslt::params("_", 1).unwrap();
///
/// assert_eq!(token.transform("foobar_foo_bar_bar_foo_barfoo"), Ok("foo".to_string()));
///
/// ```
#[derive(Clone)]
pub struct Sslt {
    pub pattern: Regex,
    pub index: usize,
}

impl Sslt {
    pub fn params(pattern: &str, index: usize) -> Result<Self, AtpError> {
        let pattern = Regex::new(&pattern).map_err(|e|
            AtpError::new(AtpErrorCode::BytecodeParsingError(e.to_string().into()), "", "")
        )?;
        Ok(Sslt {
            pattern,
            index,
        })
    }
}

impl Default for Sslt {
    fn default() -> Self {
        Sslt {
            pattern: Regex::new("").unwrap(),
            index: 0,
        }
    }
}

impl TokenMethods for Sslt {
    fn get_string_repr(&self) -> &'static str {
        "sslt"
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let s = self.pattern
            .split(input)

            .collect::<Vec<_>>();

        if !(0..s.len()).contains(&self.index) {
            return Err(
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        "Index does not exist in the splitted vec".into()
                    ),
                    self.to_atp_line(),
                    input.to_string()
                )
            );
        }
        let i = match self.index >= s.len() {
            true => s.len() - 1,
            false => self.index,
        };

        let item = s
            .get(i)
            .ok_or_else(||
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange("Item not found".into()),
                    "sslt".to_string(),
                    input.to_string()
                )
            )?;
        Ok(item.to_string())
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        check_vec_len(&line, 3)?;
        if line[0] == "sslt" {
            self.pattern = Regex::new(&line[1]).map_err(|_|
                AtpError::new(
                    AtpErrorCode::TextParsingError("Failed to create regex".into()),
                    "sslt",
                    line[1].to_string()
                )
            )?;
            self.index = string_to_usize(&line[2])?;
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
        format!("sslt {} {};\n", self.pattern, self.index).into()
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x1a
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

        self.index = parse_args!(instruction, 0, Usize, "Index should be of type Usize");

        let pattern_payload = parse_args!(
            instruction,
            1,
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
        let second_param_payload = self.pattern.as_str().as_bytes();
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

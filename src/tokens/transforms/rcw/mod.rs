#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use regex::Regex;

use crate::context::execution_context::GlobalExecutionContext;
use crate::utils::errors::{ AtpError, AtpErrorCode };

use crate::utils::params::AtpParamTypes;
use crate::utils::validations::check_vec_len;
use crate::{ tokens::InstructionMethods };

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
/// use atp::tokens::{InstructionMethods, transforms::rcw::Rcw};
///
/// let token = Rcw::params(&"a", "b", 3).unwrap();
///
/// assert_eq!(token.transform("aaaaa"), Ok("bbbaa".to_string()));
/// ```
///
#[derive(Clone, Debug)]
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

impl InstructionMethods for Rcw {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("rcw {} {} {};\n", self.pattern, self.text_to_replace, self.count).into()
    }

    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        if self.count == 0 {
            return Ok(input.to_string());
        }
        Ok(self.pattern.replacen(input, self.count, &self.text_to_replace).to_string())
    }

    fn get_string_repr(&self) -> &'static str {
        "rcw"
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 3, "rcw", "")?;

        let pattern_payload = parse_args!(params, 0, String, "Pattern should be of string type");

        self.pattern = Regex::new(&pattern_payload.clone()).map_err(|_| {
            AtpError::new(
                AtpErrorCode::TextParsingError("Failed to create regex".into()),
                "sslt",
                pattern_payload.clone()
            )
        })?;

        self.text_to_replace = parse_args!(
            params,
            1,
            String,
            "Text_to_replace should be of type String"
        );

        self.count = parse_args!(params, 2, Usize, "Index should be of type Usize");

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x10
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::String(self.pattern.to_string()),
            AtpParamTypes::String(self.text_to_replace.clone()),
            AtpParamTypes::Usize(self.count),
        ]);
        result
    }
}

#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use regex::Regex;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

use crate::utils::params::AtpParamTypes;
/// RLW - Replace Last With
///
/// Replace the last ocurrency of `pattern` in `input` with `text_to_replace`
///
/// See Also:
///
/// - [`RAW` - Replace All With](crate::tokens::transforms::raw)
/// - [`RCW` - Replace First With](crate::tokens::transforms::rcw)
/// - [`RFW` - Replace Last With](crate::tokens::transforms::rfw)
/// - [`RNW` - Replace Nth With](crate::tokens::transforms::rnw)
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::rlw::Rlw};
///
/// let token = Rlw::params(&"a", "b").unwrap();
///
/// assert_eq!(token.transform("aaaaa"), Ok("aaaab".to_string()));
/// ```
///
#[derive(Clone, Debug)]
pub struct Rlw {
    pub pattern: Regex,
    pub text_to_replace: String,
}

impl Rlw {
    pub fn params(pattern: &str, text_to_replace: &str) -> Result<Self, String> {
        let pattern = Regex::new(&pattern).map_err(|x| x.to_string())?;
        Ok(Rlw {
            text_to_replace: text_to_replace.to_string(),
            pattern,
        })
    }
}

impl Default for Rlw {
    fn default() -> Self {
        Rlw {
            pattern: Regex::new("").unwrap(),
            text_to_replace: "_".to_string(),
        }
    }
}

impl TokenMethods for Rlw {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("rlw {} {};\n", self.pattern, self.text_to_replace).into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let caps: Vec<_> = self.pattern.find_iter(input).collect();

        if let Some(m) = caps.last() {
            let (start, end) = (m.start(), m.end());

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

    fn get_string_repr(&self) -> &'static str {
        "rlw"
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x1e
    }
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

        let pattern_payload = parse_args!(
            instruction,
            0,
            String,
            "Pattern should be of string type"
        );

        self.pattern = Regex::new(&pattern_payload.clone()).map_err(|_| {
            AtpError::new(
                AtpErrorCode::TextParsingError("Failed to create regex".into()),
                "sslt",
                pattern_payload.clone()
            )
        })?;

        self.text_to_replace = parse_args!(
            instruction,
            1,
            String,
            "Text_to_replace should be of type String"
        );

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::String(self.pattern.to_string()),
            AtpParamTypes::String(self.text_to_replace.clone()),
        ]);
        result
    }
}

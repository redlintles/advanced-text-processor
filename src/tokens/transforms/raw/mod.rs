#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use regex::Regex;

use crate::{
    tokens::TokenMethods,
    utils::errors::{AtpError, AtpErrorCode},
};

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;
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
/// assert_eq!(token.transform("aaaaa"), Ok("bbbbb".to_string()));
/// ```
///
#[derive(Clone, Debug)]
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

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(self
            .pattern
            .replace_all(input, &self.text_to_replace)
            .to_string())
    }

    fn get_string_repr(&self) -> &'static str {
        "raw"
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x0b
    }
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        if instruction.len() != 2 {
            return Err(AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                "",
                "",
            ));
        }

        let pattern_payload =
            parse_args!(instruction, 0, String, "Pattern should be of string type");

        self.pattern = Regex::new(&pattern_payload.clone()).map_err(|_| {
            AtpError::new(
                AtpErrorCode::TextParsingError("Failed to create regex".into()),
                "sslt",
                pattern_payload.clone(),
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
        let result: Vec<u8> = to_bytecode!(
            self.get_opcode(),
            [
                AtpParamTypes::String(self.pattern.to_string()),
                AtpParamTypes::String(self.text_to_replace.clone()),
            ]
        );
        result
    }
}

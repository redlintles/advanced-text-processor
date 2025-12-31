#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::parse_args;

use regex::Regex;

use crate::{
    tokens::TokenMethods,
    utils::{
        errors::{AtpError, AtpErrorCode},
        transforms::string_to_usize,
    },
};

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;
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
/// assert_eq!(token.transform("aaaaa"), Ok("aabaa".to_string()));
/// ```
///
#[derive(Clone, Debug)]
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
        format!(
            "rnw {} {} {};\n",
            self.pattern, self.text_to_replace, self.index
        )
        .into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
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
            let mut result =
                String::with_capacity(input.len() - (end - start) + self.text_to_replace.len());
            result.push_str(&input[..start]);
            result.push_str(&self.text_to_replace);
            result.push_str(&input[end..]);
            return Ok(result);
        }
        Ok(input.to_string())
    }
    fn get_string_repr(&self) -> &'static str {
        "rnw"
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x1f
    }
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        if instruction.len() != 3 {
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

        self.index = parse_args!(instruction, 2, Usize, "Index should be of type Usize");

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
                AtpParamTypes::Usize(self.index),
            ]
        );
        result
    }
}

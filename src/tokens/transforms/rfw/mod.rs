#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use regex::Regex;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, validations::check_vec_len },
};

use crate::utils::params::AtpParamTypes;
/// RFW - Replace First With
///
/// Replace the first ocurrency of `pattern` in `input` with `text_to_replace`
///
/// See Also:
///
/// - [`RAW` - Replace All With](crate::tokens::transforms::raw)
/// - [`RCW` - Replace First With](crate::tokens::transforms::rcw)
/// - [`RLW` - Replace Last With](crate::tokens::transforms::rlw)
/// - [`RNW` - Replace Nth With](crate::tokens::transforms::rnw)
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::rfw::Rfw};
///
/// let token = Rfw::params(&"a", "b").unwrap();
///
/// assert_eq!(token.transform("aaaaa"), Ok("baaaa".to_string()));
/// ```
///
#[derive(Clone, Debug)]
pub struct Rfw {
    pub pattern: Regex,
    pub text_to_replace: String,
}

impl Rfw {
    pub fn params(pattern: &str, text_to_replace: &str) -> Result<Self, String> {
        let pattern = Regex::new(&pattern).map_err(|x| x.to_string())?;
        Ok(Rfw {
            text_to_replace: text_to_replace.to_string(),
            pattern,
        })
    }
}

impl Default for Rfw {
    fn default() -> Self {
        Rfw {
            pattern: Regex::new("").unwrap(),
            text_to_replace: "_".to_string(),
        }
    }
}

impl InstructionMethods for Rfw {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("rfw {} {};\n", self.pattern, self.text_to_replace).into()
    }

    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        Ok(self.pattern.replace(input, &self.text_to_replace).to_string())
    }

    fn get_string_repr(&self) -> &'static str {
        "rfw"
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 2, "rfw", "")?;

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

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x0c
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

#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    parse_args,
    utils::validations::check_vec_len,
};

use regex::Regex;

use crate::{ tokens::InstructionMethods, utils::{ errors::{ AtpError, AtpErrorCode } } };

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
/// use atp::tokens::{InstructionMethods, transforms::rnw::Rnw};
///
/// let token = Rnw::new(&"a", "b", 2).unwrap();
///
/// assert_eq!(token.transform("aaaaa"), Ok("aabaa".to_string()));
/// ```
///
#[derive(Clone, Debug)]
pub struct Rnw {
    pub pattern: Regex,
    pub text_to_replace: String,
    pub index: usize,
    params: Vec<AtpParamTypes>,
}

impl Rnw {
    pub fn new(pattern: &str, text_to_replace: &str, index: usize) -> Result<Self, String> {
        let pattern = Regex::new(&pattern).map_err(|x| x.to_string())?;
        Ok(Rnw {
            text_to_replace: text_to_replace.to_string(),
            params: vec![
                text_to_replace.to_string().into(),
                pattern.to_string().into(),
                index.into()
            ],
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
            params: vec!["".to_string().into(), "_".to_string().into(), (0).into()],
        }
    }
}

impl InstructionMethods for Rnw {
    fn get_params(&self) -> &Vec<AtpParamTypes> {
        &self.params
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("rnw {} {} {};\n", self.pattern, self.text_to_replace, self.index).into()
    }

    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
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
    fn get_string_repr(&self) -> &'static str {
        "rnw"
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 3, "rnw", "")?;

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

        self.index = parse_args!(params, 2, Usize, "Index should be of type Usize");

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x1f
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::String(self.pattern.to_string()),
            AtpParamTypes::String(self.text_to_replace.clone()),
            AtpParamTypes::Usize(self.index),
        ]);
        result
    }
}

#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::tokens::InstructionMethods;

use crate::utils::params::AtpParamTypes;

use crate::utils::errors::{ AtpError, AtpErrorCode };
use crate::utils::validations::check_vec_len;

/// Jsonu - Json Unescape
///
/// Unescapes JSON Special Characters in `input` with serde_json::from_str
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::jsonu::Jsonu};
///
///
/// let token = Jsonu::default();
///
/// let expected_output = "{banana: '10'}".to_string();
///
/// assert_eq!(token.transform("\"{banana: '10'}\""), Ok(expected_output));
/// ```

#[derive(Clone, Copy, Default)]
pub struct Jsonu {}

impl InstructionMethods for Jsonu {
    fn get_string_repr(&self) -> &'static str {
        "jsonu"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        "jsonu;\n".into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        Ok(
            serde_json
                ::from_str::<String>(input)
                .map_err(|_| {
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed to deserialize to JSON".into()),
                        "serde_json::from_str",
                        input.to_string()
                    )
                })?
        )
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "jcmc", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x27
    }

    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{
        errors::{AtpError, AtpErrorCode},
        validations::check_vec_len,
    },
};

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;

/// Token `Atb` — Add to Beginning
///
/// Adds `text` to the beginning of `input`
///
/// - See Also
///
/// - [ATE - Add to End](crate::tokens::transforms::ate)
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::atb::Atb};
///
/// let token = Atb::params("foo");
/// assert_eq!(token.transform(" bar"), Ok("foo bar".to_string()));
/// ```
#[derive(Clone, Default)]
pub struct Atb {
    pub text: String,
}

impl Atb {
    pub fn params(text: &str) -> Self {
        Atb {
            text: text.to_string(),
        }
    }
}

impl TokenMethods for Atb {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("atb {};\n", self.text).into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let mut s = String::from(&self.text);
        s.push_str(input);
        Ok(s)
    }

    fn get_string_repr(&self) -> &'static str {
        "atb"
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x01
    }

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;
        use crate::utils::params::AtpParamTypesJoin;

        check_vec_len(&params, 1, "atb", params.join(""));

        if params.len() != 1 {
            return Err(AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                "",
                "",
            ));
        }

        self.text = parse_args!(params, 0, String, "Text should be of string type");

        Ok(())
    }

    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(
            self.get_opcode(),
            [AtpParamTypes::String(self.text.clone()),]
        );
        result
    }
}

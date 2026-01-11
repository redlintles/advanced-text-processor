#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::context::execution_context::GlobalExecutionContext;
use crate::utils::params::AtpParamTypes;
use crate::utils::validations::check_vec_len;
use crate::{ tokens::InstructionMethods };

use crate::utils::errors::{ AtpError, AtpErrorCode };

/// RTL - Rotate Left
///
/// Rotates `input` to the left `n` times
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::rtl::Rtl};
///
/// let token = Rtl::params(3);
///
/// assert_eq!(token.transform("banana"),Ok("anaban".to_string()));
///
/// ```
#[derive(Clone, Default)]
pub struct Rtl {
    pub times: usize,
}

impl Rtl {
    pub fn params(times: usize) -> Rtl {
        Rtl { times }
    }
}

impl InstructionMethods for Rtl {
    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        if input.is_empty() {
            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidParameters("Input is empty".into()),
                    self.to_atp_line(),
                    "\" \""
                )
            );
        }

        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();
        let times = self.times % len;

        Ok(
            chars[times..]
                .iter()
                .chain(&chars[..times])
                .collect()
        )
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("rtl {};\n", self.times).into()
    }

    fn get_string_repr(&self) -> &'static str {
        "rtl"
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 1, "rtl", "")?;

        self.times = parse_args!(params, 0, Usize, "Index should be of usize type");

        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x0e
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [AtpParamTypes::Usize(self.times)]);
        result
    }
}

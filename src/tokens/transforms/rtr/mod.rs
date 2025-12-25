#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::string_to_usize },
};

#[cfg(feature = "bytecode")]
use crate::{ utils::params::AtpParamTypes };
/// RTR - Rotate Right
///
/// Rotates `input` to the right `n` times
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::rtr::Rtr};
///
/// let token = Rtr::params(2);
///
/// assert_eq!(token.transform("banana"),Ok("nabana".to_string()));
///
/// ```
#[derive(Clone, Default)]
pub struct Rtr {
    pub times: usize,
}

impl Rtr {
    pub fn params(times: usize) -> Rtr {
        Rtr {
            times,
        }
    }
}

impl TokenMethods for Rtr {
    fn transform(&self, input: &str) -> Result<String, AtpError> {
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
            chars[len - times..]
                .iter()
                .chain(&chars[..len - times])
                .collect()
        )
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("rtr {};\n", self.times).into()
    }
    fn get_string_repr(&self) -> &'static str {
        "rtr"
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "rtr" {
            self.times = string_to_usize(&line[1])?;
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
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x0f
    }
    #[cfg(feature = "bytecode")]
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        if instruction.len() != 1 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );
        }

        self.times = parse_args!(instruction, 0, Usize, "Index should be of usize type");

        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [AtpParamTypes::Usize(self.times)]);
        result
    }
}

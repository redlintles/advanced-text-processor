#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::utils::params::AtpParamTypes;
use crate::utils::validations::check_index_against_input;
use crate::{ tokens::TokenMethods };

use crate::utils::errors::{ AtpError, AtpErrorCode };

/// Dla - Delete After
/// Delete all characters after `index` in the specified `input`
///
/// It will throw an `AtpError` if index does not exists in the current `input`
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::dla::Dla};
///
/// let token = Dla::params(3);
///
/// assert_eq!(token.transform("banana laranja vermelha azul"), Ok("bana".to_string()))
///
/// ```
#[derive(Clone, Copy, Default)]
pub struct Dla {
    pub index: usize,
}

impl Dla {
    pub fn params(index: usize) -> Self {
        Dla { index }
    }
}

impl TokenMethods for Dla {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("dla {};\n", self.index).into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        check_index_against_input(self.index, input)?;

        let mut s = String::from(input);
        if
            let Some(byte_index) = s
                .char_indices()
                .nth(self.index + 1)
                .map(|(i, _)| i)
        {
            s.drain(byte_index..);
            return Ok(s);
        }
        Err(
            AtpError::new(
                AtpErrorCode::IndexOutOfRange(
                    "Index is out of range for the desired string".into()
                ),
                self.to_atp_line(),
                input.to_string()
            )
        )
    }

    fn get_string_repr(&self) -> &'static str {
        "dla"
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x09
    }
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

        self.index = parse_args!(instruction, 0, Usize, "Index should be of usize type");
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [AtpParamTypes::Usize(self.index)]);
        result
    }
}

#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::utils::errors::{ AtpError, AtpErrorCode };

use crate::utils::params::AtpParamTypes;
use crate::utils::validations::{ check_index_against_input };
use crate::{ tokens::TokenMethods };

/// Dlb - Delete Before
/// Delete all characters before `index` in the specified `input`
///
/// It will throw an `AtpError` if index does not exists in the current `input`
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::dlb::Dlb};
///
/// let token = Dlb::params(3);
///
/// assert_eq!(token.transform("banana laranja vermelha azul"), Ok("ana laranja vermelha azul".to_string()))
///
/// ```
#[derive(Clone, Copy, Default)]
pub struct Dlb {
    pub index: usize,
}

impl Dlb {
    pub fn params(index: usize) -> Self {
        Dlb { index }
    }
}

impl TokenMethods for Dlb {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("dlb {};\n", self.index).into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let mut s = String::from(input);

        check_index_against_input(self.index, input)?;

        if
            let Some(byte_index) = s
                .char_indices()
                .nth(self.index)
                .map(|(i, _)| i)
        {
            s.drain(0..byte_index);
            return Ok(s);
        }

        Err(
            AtpError::new(
                AtpErrorCode::IndexOutOfRange(
                    format!(
                        "Supported indexes 0-{}, entered index {}",
                        input.chars().count().saturating_sub(1),
                        self.index
                    ).into()
                ),
                self.to_atp_line(),
                input.to_string()
            )
        )
    }
    fn get_string_repr(&self) -> &'static str {
        "dlb"
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x0a
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

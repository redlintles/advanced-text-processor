#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::context::execution_context::GlobalExecutionContext;
use crate::utils::errors::{ AtpError, AtpErrorCode };

use crate::utils::params::AtpParamTypes;
use crate::utils::validations::{ check_index_against_input, check_vec_len };
use crate::{ tokens::InstructionMethods };

/// Dlb - Delete Before
/// Delete all characters before `index` in the specified `input`
///
/// It will throw an `AtpError` if index does not exists in the current `input`
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::dlb::Dlb};
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

impl InstructionMethods for Dlb {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("dlb {};\n", self.index).into()
    }

    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
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
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 1, "dlb", "")?;

        self.index = parse_args!(params, 0, Usize, "Index should be of usize type");
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x0a
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [AtpParamTypes::Usize(self.index)]);
        result
    }
}

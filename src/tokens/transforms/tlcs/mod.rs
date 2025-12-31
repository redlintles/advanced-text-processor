#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{
        errors::{AtpError, AtpErrorCode},
        transforms::string_to_usize,
        validations::{check_index_against_input, check_vec_len},
    },
};

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;

/// TLCS - To Lowercase Single
///
/// Lowercases a single character in `input` identified by `index`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::tlcs::Tlcs};
///
/// let token = Tlcs::params(1);
///
/// assert_eq!(token.transform("BANANA"), Ok("BaNANA".to_string()));
///
/// ```

#[derive(Clone, Copy, Default)]
pub struct Tlcs {
    index: usize,
}

impl Tlcs {
    pub fn params(index: usize) -> Self {
        Tlcs { index }
    }
}

impl TokenMethods for Tlcs {
    fn get_string_repr(&self) -> &'static str {
        "tlcs"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("tlcs {};\n", self.index).into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        check_index_against_input(self.index, input)?;

        let result: String = input
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i == self.index {
                    c.to_lowercase().to_string()
                } else {
                    c.to_string()
                }
            })
            .collect();

        Ok(result)
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x15
    }
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        if instruction.len() != 1 {
            return Err(AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                "",
                "",
            ));
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

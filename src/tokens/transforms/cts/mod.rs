#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{ transforms::capitalize, validations::{ check_index_against_input, check_vec_len } },
};

use crate::utils::errors::{ AtpError };

use crate::utils::params::AtpParamTypes;

/// Token `Cts` — Capitalize Single
///
/// Capitalizes a single word at the given index `i` within the input string.
///
/// Words are defined as sequences of characters separated by whitespace,
/// following the behavior of `input.split_whitespace()`.
///
/// If `i` is out of bounds for the number of words in the input, an `AtpError` is returned.
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::cts::Cts};
/// let token = Cts::params(1);
/// assert_eq!(token.transform("foo bar"), Ok("foo Bar".to_string()));
/// ```

#[derive(Clone, Default)]
pub struct Cts {
    pub index: usize,
}

impl Cts {
    pub fn params(index: usize) -> Self {
        Cts { index }
    }
}

impl InstructionMethods for Cts {
    fn get_string_repr(&self) -> &'static str {
        "cts"
    }
    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        check_index_against_input(self.index, input)?;
        let v = input.split_whitespace().collect::<Vec<_>>();

        Ok(
            v
                .iter()
                .enumerate()
                .map(|(index, word)| {
                    if index == self.index { capitalize(word) } else { word.to_string() }
                })
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("cts {};\n", self.index).into()
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 1, "cts", "")?;

        self.index = parse_args!(params, 0, Usize, "Index should be of usize type");

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x1d
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [AtpParamTypes::Usize(self.index)]);
        result
    }
}

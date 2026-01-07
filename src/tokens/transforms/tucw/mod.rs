#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::utils::params::AtpParamTypes;
use crate::{
    tokens::InstructionMethods,
    utils::{ errors::{ AtpError }, validations::{ check_index_against_words, check_vec_len } },
};
/// TUCW - To Uppercase Word
///
/// Uppercase a single word of string
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::tucw::Tucw};
///
/// let token = Tucw::params(1);
///
/// assert_eq!(token.transform("banana laranja cheia de canja"), Ok("banana LARANJA cheia de canja".to_string()));
///
/// ```
#[derive(Clone, Default, Copy)]
pub struct Tucw {
    index: usize,
}

impl Tucw {
    pub fn params(index: usize) -> Self {
        Tucw { index }
    }
}
impl InstructionMethods for Tucw {
    fn get_string_repr(&self) -> &'static str {
        "tucw"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("tucw {};\n", self.index).into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        check_index_against_words(self.index, input)?;
        Ok(
            input
                .split_whitespace()
                .enumerate()
                .map(|(i, w)| {
                    if i == self.index { w.to_uppercase() } else { w.to_string() }
                })
                .collect::<Vec<_>>()
                .join(" ")
                .to_string()
        )
    }

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;
        check_vec_len(&params, 1, "tucw", "")?;

        self.index = parse_args!(params, 0, Usize, "Index should be of usize type");
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x2a
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [AtpParamTypes::Usize(self.index)]);
        result
    }
}

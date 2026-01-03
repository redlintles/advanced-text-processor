#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::utils::params::AtpParamTypes;
use crate::utils::validations::check_vec_len;
use crate::{
    tokens::TokenMethods,
    utils::{ errors::{ AtpError }, validations::{ check_index_against_words } },
};

/// TLCW - To Lowercase Word
///
/// Lowercase a single word of string
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::tlcw::Tlcw};
///
/// let token = Tlcw::params(1);
///
/// assert_eq!(token.transform("BANANA LARANJA CHEIA DE CANJA"), Ok("BANANA laranja CHEIA DE CANJA".to_string()));
///
/// ```
#[derive(Clone, Default, Copy)]
pub struct Tlcw {
    index: usize,
}

impl Tlcw {
    pub fn params(index: usize) -> Self {
        Tlcw { index }
    }
}
impl TokenMethods for Tlcw {
    fn get_string_repr(&self) -> &'static str {
        "tlcw"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("tlcw {};\n", self.index).into()
    }

    fn transform(&self, input: &str) -> Result<String, crate::utils::errors::AtpError> {
        check_index_against_words(self.index, input)?;
        Ok(
            input
                .split_whitespace()
                .enumerate()
                .map(|(i, w)| {
                    if i == self.index { w.to_lowercase() } else { w.to_string() }
                })
                .collect::<Vec<_>>()
                .join(" ")
                .to_string()
        )
    }

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 1, "tlcw", "")?;

        self.index = parse_args!(params, 0, Usize, "Index should be of usize type");
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x29
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [AtpParamTypes::Usize(self.index)]);
        result
    }
}

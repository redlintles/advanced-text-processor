#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{
        errors::{ AtpError, AtpErrorCode },
        transforms::string_to_usize,
        validations::{ check_index_against_words, check_vec_len },
    },
};
#[cfg(feature = "bytecode")]
use crate::{ utils::params::AtpParamTypes };
/// TUCW - To Uppercase Word
///
/// Uppercase a single word of string
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::tucw::Tucw};
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
impl TokenMethods for Tucw {
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

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        check_vec_len(&line, 2)?;
        if line[0] == "tucw" {
            self.index = string_to_usize(&line[1])?;
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid Parser for this token".into()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x2a
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

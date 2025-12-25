#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{
        errors::{ AtpError, AtpErrorCode },
        transforms::string_to_usize,
        validations::{ check_index_against_input, check_vec_len },
    },
};

#[cfg(feature = "bytecode")]
use crate::{ utils::params::AtpParamTypes };

/// TUCS - To Uppercase Single
///
/// Uppercases a single character in `input` identified by `index`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::tucs::Tucs};
///
/// let token = Tucs::params(1);
///
/// assert_eq!(token.transform("banana"), Ok("bAnana".to_string()));
///
/// ```

#[derive(Clone, Copy, Default)]
pub struct Tucs {
    index: usize,
}

impl Tucs {
    pub fn params(index: usize) -> Self {
        Tucs {
            index,
        }
    }
}

impl TokenMethods for Tucs {
    fn get_string_repr(&self) -> &'static str {
        "tucs"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("tucs {};\n", self.index).into()
    }
    fn transform(&self, input: &str) -> Result<String, AtpError> {
        check_index_against_input(self.index, input)?;
        let result: String = input
            .char_indices()
            .map(|(i, c)| {
                if i == self.index { c.to_uppercase().to_string() } else { c.to_string() }
            })
            .collect();
        Ok(result)
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        check_vec_len(&line, 2)?;
        if line[0] == "tucs" {
            self.index = string_to_usize(&line[1])?;
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
        0x14
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

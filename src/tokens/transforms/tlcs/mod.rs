#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::InstructionMethods,
    utils::{ errors::{ AtpError }, validations::{ check_index_against_input, check_vec_len } },
};

use crate::utils::params::AtpParamTypes;

/// TLCS - To Lowercase Single
///
/// Lowercases a single character in `input` identified by `index`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::tlcs::Tlcs};
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

impl InstructionMethods for Tlcs {
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
                if i == self.index { c.to_lowercase().to_string() } else { c.to_string() }
            })
            .collect();

        Ok(result)
    }

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 1, "tlcs", "")?;

        self.index = parse_args!(params, 0, Usize, "Index should be of usize type");
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x15
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [AtpParamTypes::Usize(self.index)]);
        result
    }
}

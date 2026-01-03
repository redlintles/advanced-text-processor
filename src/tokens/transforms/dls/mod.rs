#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{
        errors::{ AtpError },
        validations::{ check_index_against_input, check_vec_len },
    },
};

use crate::utils::params::AtpParamTypes;

/// DLS - Delete Single
///
/// Delete's a single character specified by `index` in `input`
///
/// It will throw an `AtpError` if index does not exists in `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::dls::Dls};
///
/// let token = Dls::params(3);
///
/// assert_eq!(token.transform("banana"), Ok("banna".to_string()));
/// ```
#[derive(Clone, Copy, Default)]
pub struct Dls {
    pub index: usize,
}

impl Dls {
    pub fn params(index: usize) -> Self {
        Dls { index }
    }
}

impl TokenMethods for Dls {
    fn get_string_repr(&self) -> &'static str {
        "dls"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("dls {};\n", self.index).into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        check_index_against_input(self.index, input)?;
        Ok(
            input
                .chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if self.index == i {
                        return None;
                    } else {
                        return Some(c);
                    }
                })
                .collect()
        )
    }

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 1, "dls", "")?;

        self.index = parse_args!(params, 0, Usize, "Index should be of usize type");
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x32
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [AtpParamTypes::Usize(self.index)]);
        result
    }
}

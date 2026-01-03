#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::utils::params::{ AtpParamTypes };

use crate::utils::validations::check_vec_len;
use crate::{ tokens::TokenMethods, utils::{ errors::{ AtpError, AtpErrorCode } } };
/// Ins - Insert
///
/// Inserts `text` after `index` position in `input`
///
/// If index does not exists in current string, `AtpError` is returned
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::ins::Ins};
///
/// let token = Ins::params(2,"laranja");
///
/// assert_eq!(token.transform("banana"), Ok("banlaranjaana".to_string()));
/// ```
#[derive(Clone, Default)]
pub struct Ins {
    index: usize,
    text_to_insert: String,
}

impl Ins {
    pub fn params(index: usize, text_to_insert: &str) -> Self {
        Ins {
            index,
            text_to_insert: text_to_insert.to_string(),
        }
    }
}
impl TokenMethods for Ins {
    fn get_string_repr(&self) -> &'static str {
        "ins"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("ins {} {};\n", self.index, self.text_to_insert).into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        if self.index > input.chars().count() {
            return Err(
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        format!(
                            "Index does not exist in current string, for the input {}, only indexes between 0-{} are allowed",
                            input,
                            input.chars().count() - 1
                        ).into()
                    ),
                    self.to_atp_line(),
                    input.to_string()
                )
            );
        }
        let byte_index = input
            .char_indices()
            .nth(self.index + 1)
            .map(|(i, _)| i)
            .unwrap_or(input.len());

        let (before, after) = input.split_at(byte_index);

        let result = format!("{}{}{}", before, self.text_to_insert, after);

        Ok(result)
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 2, "ins", "")?;

        self.index = parse_args!(params, 0, Usize, "Index should be of usize type");
        self.text_to_insert = parse_args!(
            params,
            1,
            String,
            "Text_to_insert should be of String type"
        );

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x28
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::Usize(self.index),
            AtpParamTypes::String(self.text_to_insert.clone()),
        ]);
        result
    }
}

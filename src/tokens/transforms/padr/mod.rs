#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{ errors::AtpError, transforms::extend_string, validations::check_vec_len },
};

use crate::utils::params::AtpParamTypes;
/// PADR - Pad Right
///
/// Repeats `text` characters until `max_len` is reached, and then insert the result at the end of `input`
///
/// See Also:
///
/// - [`Padr` - Pad Left](crate::tokens::transforms::padr)
///
/// # Example:
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::padr::Padr};
///
/// let token = Padr::new("xy", 7);
///
/// assert_eq!(token.transform("banana"), Ok("bananax".to_string()));
/// ```
#[derive(Clone, Default)]
pub struct Padr {
    pub text: String,
    pub max_len: usize,
    params: Vec<AtpParamTypes>,
}

impl Padr {
    pub fn new(text: &str, max_len: usize) -> Self {
        Padr {
            text: text.to_string(),
            max_len,
            params: vec![text.to_string().into(), max_len.into()],
        }
    }
}

impl InstructionMethods for Padr {
    fn get_params(&self) -> &Vec<AtpParamTypes> {
        &self.params
    }
    fn get_string_repr(&self) -> &'static str {
        "padr"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("padr {} {};\n", self.text, self.max_len).into()
    }
    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        let character_count = input.chars().count();

        if character_count >= self.max_len {
            return Ok(input.to_string());
        }
        let ml = self.max_len - character_count;
        let s = extend_string(&self.text, ml);

        Ok(format!("{}{}", input, s))
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        check_vec_len(&params, 2, "padr", "")?;

        self.text = parse_args!(params, 0, String, "Text_to_insert should be of String type");
        self.max_len = parse_args!(params, 1, Usize, "Index should be of usize type");

        return Ok(());
    }
    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x30
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::String(self.text.clone()),
            AtpParamTypes::Usize(self.max_len),
        ]);
        result
    }
}

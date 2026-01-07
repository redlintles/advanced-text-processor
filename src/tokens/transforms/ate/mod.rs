#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{ tokens::InstructionMethods, utils::{ errors::{ AtpError }, validations::check_vec_len } };

use crate::utils::params::AtpParamTypes;
/// Token `Ate` — Add to End
///
/// Appends `text` to the end of `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::ate::Ate};
///
/// let token = Ate::params(" bar");
/// assert_eq!(token.transform("foo"), Ok("foo bar".to_string()));
/// ```

#[derive(Clone, Default)]
pub struct Ate {
    pub text: String,
}

impl Ate {
    pub fn params(text: &str) -> Self {
        Ate {
            text: text.to_string(),
        }
    }
}

impl InstructionMethods for Ate {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("ate {};\n", self.text).into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let mut s = String::from(input);
        s.push_str(&self.text);
        Ok(s)
    }

    fn get_string_repr(&self) -> &'static str {
        "ate"
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;
        use crate::utils::params::AtpParamTypesJoin;

        check_vec_len(&params, 1, "ate", params.join(""))?;

        self.text = parse_args!(params, 0, String, "Text should be of string type");

        Ok(())
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x02
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::String(self.text.clone()),
        ]);
        result
    }
}

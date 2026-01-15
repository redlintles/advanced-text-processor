#[cfg(feature = "test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, validations::check_vec_len },
};

use crate::utils::params::AtpParamTypes;
/// URLD - URL Decode
///
/// Decodes `input` from the URL Encoding Format
///
/// # Example
///
/// ```rust
/// use atp::tokens::{InstructionMethods, transforms::urld::Urld};
///
/// let token = Urld::default();
///
/// assert_eq!(token.transform("banana%20laranja"), Ok("banana laranja".to_string()));
/// ```
///

#[derive(Clone, Default)]
pub struct Urld {
    params: Vec<AtpParamTypes>,
}

impl InstructionMethods for Urld {
    fn get_params(&self) -> &Vec<AtpParamTypes> {
        &self.params
    }
    fn get_string_repr(&self) -> &'static str {
        "urld"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "urld;\n".into()
    }
    fn transform(&self, input: &str, _: &mut GlobalExecutionContext) -> Result<String, AtpError> {
        // Validação de percent encoding
        let bytes = input.as_bytes();
        let len = bytes.len();

        let mut i = 0;
        while i < len {
            if bytes[i] == b'%' {
                if
                    i + 2 >= len ||
                    !bytes[i + 1].is_ascii_hexdigit() ||
                    !bytes[i + 2].is_ascii_hexdigit()
                {
                    return Err(
                        AtpError::new(
                            AtpErrorCode::TextParsingError("Failed parsing URL string".into()),
                            "urld",
                            input.to_string()
                        )
                    );
                }
                i += 3;
                continue;
            }
            i += 1;
        }

        Ok(
            urlencoding
                ::decode(input)
                .map_err(|_| {
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed parsing URL string".into()),
                        "urld",
                        input.to_string()
                    )
                })?
                .to_string()
        )
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x21
    }
    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        check_vec_len(&params, 0, "urld", "")?;
        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        use crate::to_bytecode;
        let result: Vec<u8> = to_bytecode!(self.get_opcode(), []);
        result
    }
}

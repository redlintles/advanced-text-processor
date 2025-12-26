#[cfg(feature = "test_access")]
pub mod test;

use std::{ borrow::Cow };

use crate::{
    globals::table::{ QuerySource, QueryTarget, TOKEN_TABLE, TargetValue },
    to_bytecode,
    tokens::{ TokenMethods, transforms::dlf::Dlf },
};

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;
use crate::utils::errors::{ AtpError, AtpErrorCode };

/// Ifdc - If Do Contains
///
/// if `input` contains `text`, the `inner` token will be executed, otherwise `input` is returned with no changes
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, instructions::ifdc::Ifdc};
///
/// let token = Ifdc::params("xy", "atb laranja;");
///
/// assert_eq!(token.transform("larryxy"), Ok("laranjalarryxy".to_string())); // Adds laranja to the beginning
/// assert_eq!(token.transform("banana"), Ok("banana".to_string())); // Does nothing
///
/// ```
#[derive(Clone)]
pub struct Ifdc {
    text: String,
    inner: Box<dyn TokenMethods>,
}

impl Default for Ifdc {
    fn default() -> Self {
        Ifdc { text: "teste".to_string(), inner: Box::new(Dlf::default()) }
    }
}

impl Ifdc {
    pub fn params(text: &str, inner: Box<dyn TokenMethods>) -> Self {
        Ifdc {
            text: text.to_string(),
            inner,
        }
    }
}

impl TokenMethods for Ifdc {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("ifdc {} do {}", self.text, self.inner.to_atp_line()).into()
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line.get(0).map(|s| s.as_str()) == Some("ifdc") {
            if line.len() < 4 || line.get(2).map(|s| s.as_str()) != Some("do") {
                return Err(
                    AtpError::new(
                        AtpErrorCode::InvalidParameters(
                            "Expected: ifdc <text> do <token...>".into()
                        ),
                        "ifdc".to_string(),
                        line.join(" ")
                    )
                );
            }

            self.text = line[1].clone();
            let inner_token_text = line[3..].to_vec();

            let query_result = TOKEN_TABLE.find((
                QuerySource::Identifier(inner_token_text[0].clone().into()),
                QueryTarget::Token,
            ))?;

            match query_result {
                TargetValue::Token(inner_token_ref) => {
                    let mut inner_token = inner_token_ref.into_box();
                    inner_token.from_vec_params(inner_token_text)?;
                    self.inner = inner_token;
                }
                _ => unreachable!("Invalid query result"),
            }

            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid parser for this token".into()),
                "ifdc".to_string(),
                line.join(" ")
            )
        )
    }
    fn get_string_repr(&self) -> &'static str {
        "ifdc"
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        if input.contains(&self.text) {
            return Ok(self.inner.transform(input)?);
        }

        Ok(input.to_string())
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x33
    }
    #[cfg(feature = "bytecode")]
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        if instruction.len() != 2 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );
        }

        self.text = parse_args!(instruction, 0, String, "");

        self.inner = parse_args!(instruction, 1, Token, "");

        Ok(())
    }
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        let result = to_bytecode!(self.get_opcode(), [
            AtpParamTypes::String(self.text.clone()),
            AtpParamTypes::Token(self.inner.clone()),
        ]);

        result
    }
}

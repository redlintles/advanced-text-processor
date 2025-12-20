use std::borrow::Cow;

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeTokenMethods };
use crate::tokens::{ TokenMethods, transforms::dlf::Dlf };

#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;
use crate::utils::errors::{ AtpError, AtpErrorCode };
use crate::utils::mapping::get_supported_default_tokens;

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
/// assert_eq!(token.parse("larryxy"), Ok("laranjalarryxy".to_string())); // Adds laranja to the beginning
/// assert_eq!(token.parse("banana"), Ok("banana".to_string())); // Does nothing
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
        if line[0] == "ifdc" {
            self.text = line[1].clone();

            let inner_token_text = line[3..].to_vec();

            let mut inner_token = get_supported_default_tokens()
                .get(inner_token_text[0].as_str())
                .ok_or_else(||
                    AtpError::new(
                        AtpErrorCode::TokenNotFound("Token Not Found".into()),
                        self.to_atp_line(),
                        inner_token_text.join(" ")
                    )
                )?();

            inner_token.from_vec_params(inner_token_text)?;
            self.inner = inner_token;
        }

        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid parser for this token".into()),
                "IFDC".to_string(),
                line.join(" ")
            )
        )
    }
    fn get_string_repr(&self) -> &'static str {
        "ifdc"
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        if input.contains(&self.text) {
            return Ok(self.inner.parse(input)?);
        }

        Ok(input.to_string())
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Ifdc {
    fn get_opcode(&self) -> u32 {
        0x33
    }

    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        if instruction.len() != 2 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );
        }

        match &instruction[0] {
            AtpParamTypes::String(payload) => {
                self.text = payload.clone();
            }
            _ => {
                return Err(
                    AtpError::new(
                        AtpErrorCode::InvalidParameters(
                            "This token expected a String as argument at this position".into()
                        ),
                        self.to_atp_line(),
                        ""
                    )
                );
            }
        }

        Ok(())
    }

    fn to_bytecode(&self) -> Vec<u8> {
        use crate::utils::transforms::token_to_bytecode_token;

        let mut result = Vec::new();

        let instruction_type: u32 = self.get_opcode() as u32;

        let first_param_type: u32 = 0x02;
        let first_param_payload = self.text.as_str().as_bytes();
        let first_param_payload_size: u32 = first_param_payload.len() as u32;

        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

        let second_param_type: u32 = 0x03;
        let second_param_payload = token_to_bytecode_token(&self.inner).unwrap().to_bytecode();
        let second_param_payload_size: u32 = second_param_payload.len() as u32;

        let second_param_total_size: u64 = 4 + 4 + (second_param_payload_size as u64);

        let instruction_total_size: u64 = 4 + 1 + first_param_total_size + second_param_total_size;

        // Instruction Total Size
        result.extend_from_slice(&instruction_total_size.to_be_bytes());
        // Instruction Type
        result.extend_from_slice(&instruction_type.to_be_bytes());
        // Param Count
        result.push(2);
        // First Param Total Size
        result.extend_from_slice(&first_param_total_size.to_be_bytes());
        // First Param Type
        result.extend_from_slice(&first_param_type.to_be_bytes());
        // First Param Payload Size
        result.extend_from_slice(&first_param_payload_size.to_be_bytes());
        // First Param Payload
        result.extend_from_slice(&first_param_payload);

        // Second Param Total Size
        result.extend_from_slice(&second_param_total_size.to_be_bytes());
        // Second Param Type
        result.extend_from_slice(&second_param_type.to_be_bytes());
        // Second Param Payload Size
        result.extend_from_slice(&second_param_payload_size.to_be_bytes());
        // Second Param Payload
        result.extend_from_slice(&second_param_payload);

        result
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod ifdc_tests {
    #[test]
    fn it_works_correctly() {}
}

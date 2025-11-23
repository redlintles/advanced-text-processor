use std::borrow::Cow;

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeTokenMethods, BytecodeInstruction };
use crate::tokens::TokenMethods;

use crate::text::reader::read_from_text;

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
/// assert_eq!(token.parse("larryxy"), Ok("laranjalarryxy".to_string())); // Adds laranja to the beginning
/// assert_eq!(token.parse("banana"), Ok("banana".to_string())); // Does nothing
///
/// ```
#[derive(Clone, Default)]
pub struct Ifdc {
    text: String,
    inner: String,
}

impl Ifdc {
    pub fn params(text: &str, inner: &str) -> Self {
        Ifdc {
            text: text.to_string(),
            inner: inner.to_string(),
        }
    }
}

impl TokenMethods for Ifdc {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("ifdc {} do {}", self.text, self.inner).into()
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), crate::utils::errors::AtpError> {
        if line[0] == "ifdc" {
            self.text = line[1].clone();
            self.inner = line[3..].join(" ");

            return Ok(());
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
        let token = read_from_text(&self.inner)?;
        if input.contains(&self.text) {
            return Ok(token.parse(input)?);
        }

        Ok(input.to_string())
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Ifdc {
    fn get_opcode(&self) -> u8 {
        0x33
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == self.get_opcode() {
            self.text = instruction.operands[1].clone();
            self.inner = instruction.operands[3..].join(" ");
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid parser for this token".into()),
                "IFDC".to_string(),
                instruction.operands.join(" ")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: self.get_opcode(),
            operands: [
                [self.text.clone()].to_vec(),
                self.inner
                    .split_whitespace()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            ].concat(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod ifdc_tests {
    #[test]
    fn it_works_correctly() {}
}

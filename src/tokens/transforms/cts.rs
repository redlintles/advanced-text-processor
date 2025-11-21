use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{
        transforms::{ capitalize, string_to_usize },
        validations::{ check_index_against_input, check_vec_len },
    },
};

use crate::utils::errors::{ AtpError, AtpErrorCode };

#[cfg(feature = "bytecode")]
use crate::{ bytecode::{ BytecodeInstruction, BytecodeTokenMethods } };

/// Token `Cts` — Capitalize Single
///
/// Capitalizes a single word at the given index `i` within the input string.
///
/// Words are defined as sequences of characters separated by whitespace,
/// following the behavior of `input.split_whitespace()`.
///
/// If `i` is out of bounds for the number of words in the input, an `AtpError` is returned.
///
/// # Example
///
/// ```rust
/// use atp_project::tokens::{TokenMethods, transforms::cts::Cts};
/// let token = Cts::params(1);
/// assert_eq!(token.parse("foo bar"), Ok("foo Bar".to_string()));
/// ```

#[derive(Clone, Default)]
pub struct Cts {
    pub index: usize,
}

impl Cts {
    pub fn params(index: usize) -> Self {
        Cts {
            index,
        }
    }
}

impl TokenMethods for Cts {
    fn get_string_repr(&self) -> &'static str {
        "cts"
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        check_index_against_input(self.index, input)?;
        let v = input.split_whitespace().collect::<Vec<_>>();

        Ok(
            v
                .iter()
                .enumerate()
                .map(|(index, word)| {
                    if index == self.index { capitalize(word) } else { word.to_string() }
                })
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        check_vec_len(&line, 2)?;
        if line[0] == "cts" {
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

    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("cts {};\n", self.index).into()
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Cts {
    fn get_opcode(&self) -> u8 {
        0x1d
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode::BytecodeInstruction
    ) -> Result<(), AtpError> {
        check_vec_len(&instruction.operands, 1)?;
        if instruction.op_code == Cts::default().get_opcode() {
            self.index = string_to_usize(&instruction.operands[0])?;
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("".into()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> crate::bytecode::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Cts::default().get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }
}
#[cfg(feature = "test_access")]
#[cfg(test)]
mod cts_tests {
    use crate::{ tokens::{ TokenMethods, transforms::cts::Cts } };

    #[test]
    fn test_capitalize_single() {
        let mut token = Cts::params(3);

        assert_eq!(
            token.parse("banana laranja vermelha azul"),
            Ok("banana laranja vermelha Azul".to_string()),
            "It works correctly with expected inputs"
        );
        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if the string does not have the current token index"
        );

        assert_eq!(
            token.to_atp_line(),
            "cts 3;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "cts".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["cts".to_string(), (3).to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[test]
    fn test_capitalize_single_bytecode() {
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Cts::params(3);

        let instruction = BytecodeInstruction {
            op_code: 0x1d,
            operands: [(3).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x1d, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.token_from_bytecode_instruction(instruction.clone()),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.token_to_bytecode_instruction(),
            instruction,
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

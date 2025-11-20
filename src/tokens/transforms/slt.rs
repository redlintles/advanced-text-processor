use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::transforms::string_to_usize,
    utils::validations::check_chunk_bound_indexes,
};

use crate::utils::errors::{ AtpError, AtpErrorCode };
#[cfg(feature = "bytecode")]
use crate::{ bytecode::{ BytecodeInstruction, BytecodeTokenMethods } };

/// Slt - Select
///
/// Selects a subslice of `input` delimited by `start_index` and `end_index`(inclusive) discarding the rest in the process
/// If end_index is bigger than the length of the string, the subslice will include up to the last character of `input`
///
/// # Example
///
/// ```rust
/// use atp_project::tokens::{TokenMethods,transforms::slt::Slt};
///
/// let token = Slt::params(1,9999).unwrap();
///
/// assert_eq!(token.parse("banàna"), Ok("anàn".to_string()));
///
///
/// ```
#[derive(Clone, Default)]
pub struct Slt {
    pub start_index: usize,
    pub end_index: usize,
}

impl Slt {
    pub fn params(start_index: usize, end_index: usize) -> Result<Self, AtpError> {
        check_chunk_bound_indexes(start_index, end_index, None)?;
        Ok(Slt {
            start_index,
            end_index,
        })
    }
}

impl TokenMethods for Slt {
    fn get_string_repr(&self) -> &'static str {
        "slt"
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let total_chars = input.chars().count();
        let last_char_index = total_chars.saturating_sub(1);

        check_chunk_bound_indexes(self.start_index, self.end_index, Some(input))?;

        let subslice_start = input
            .char_indices()
            .nth(self.start_index)
            .map(|(i, _)| i)
            .unwrap_or(0);

        let subslice_end = if self.end_index > last_char_index {
            last_char_index
        } else {
            input
                .char_indices()
                .nth(self.end_index)
                .map(|(i, _)| i)
                .unwrap_or(input.len())
        };

        Ok(input[subslice_start..=subslice_end].to_string())
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "slt" {
            let start_index = string_to_usize(&line[1])?;
            let end_index = string_to_usize(&line[2])?;

            check_chunk_bound_indexes(start_index, end_index, None)?;

            self.start_index = start_index;
            self.end_index = end_index;
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid parser for this token".to_string()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        Cow::Owned(format!("slt {} {};\n", self.start_index, self.end_index))
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Slt {
    fn get_opcode(&self) -> u8 {
        0x11
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Slt::default().get_opcode() && instruction.operands.len() == 2 {
            let start_index = string_to_usize(&instruction.operands[0])?;
            let end_index = string_to_usize(&instruction.operands[1])?;

            check_chunk_bound_indexes(start_index, end_index, None)?;

            self.start_index = start_index;
            self.end_index = end_index;
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("".to_string()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> crate::bytecode::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Slt::default().get_opcode(),
            operands: [self.start_index.to_string(), self.end_index.to_string()].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod slt_tests {
    use crate::tokens::{ TokenMethods, transforms::slt::Slt };

    #[test]
    fn select() {
        let mut token = Slt::params(1, 5).unwrap();
        assert!(
            matches!(Slt::params(5, 1), Err(_)),
            "it throws an error if start_index is bigger than end_index"
        );

        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if start_index does not exists in input"
        );

        assert_eq!(
            token.parse("banànalaranjacheiadecanja"),
            Ok("anàna".to_string()),
            "It works with expected inputs"
        );

        assert_eq!(
            token.to_atp_line(),
            "slt 1 5;\n".to_string(),
            "conversion to atp_line works correctly"
        );

        assert_eq!(token.get_string_repr(), "slt".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["slt".to_string(), (5).to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["slt".to_string(), (1).to_string(), (5).to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn select_bytecode() {
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Slt::params(1, 5).unwrap();

        let mut instruction = BytecodeInstruction {
            op_code: 0x11,
            operands: [(1).to_string(), (5).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x11, "get_opcode does not disrepect ATP token mapping");

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

        instruction.operands = [(5).to_string(), (1).to_string()].to_vec();

        assert!(
            matches!(token.token_from_bytecode_instruction(instruction.clone()), Err(_)),
            "It throws an error for invalid operands"
        );

        instruction.op_code = 0x01;
        assert!(
            matches!(token.token_from_bytecode_instruction(instruction.clone()), Err(_)),
            "It throws an error for invalid op_code"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["slt".to_string(), (5).to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
    }
}

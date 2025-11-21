use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::transforms::string_to_usize,
    utils::validations::check_chunk_bound_indexes,
};

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };
use crate::utils::errors::{ AtpError, AtpErrorCode };
/// Dlc - Delete Chunk
///
/// Deletes an specific subslice of `input` delimited by `start_index` and `end_index`(inclusive)
///
/// # Example
///
/// ```rust
/// use atp_project::tokens::{TokenMethods, transforms::dlc::Dlc};
///
/// let token = Dlc::params(1,5).unwrap();
///
/// assert_eq!(token.parse("bananalaranjacheiadecanja"), Ok("blaranjacheiadecanja".to_string()))
///
/// ```
#[derive(Clone, Copy, Default)]
pub struct Dlc {
    pub start_index: usize,
    pub end_index: usize,
}

impl Dlc {
    pub fn params(start_index: usize, end_index: usize) -> Result<Self, AtpError> {
        check_chunk_bound_indexes(start_index, end_index, None)?;
        Ok(Dlc {
            start_index,
            end_index,
        })
    }
}

impl TokenMethods for Dlc {
    fn to_atp_line(&self) -> Cow<'static, str> {
        Cow::Owned(format!("dlc {} {};\n", self.start_index, self.end_index))
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        check_chunk_bound_indexes(self.start_index, self.end_index, Some(input))?;
        let start_index = input
            .char_indices()
            .nth(self.start_index)
            .map(|(i, _)| i)
            .ok_or_else(||
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        format!(
                            "Invalid Index for this specific input, supported indexes 0-{}, entered index {}",
                            input.char_indices().count().saturating_sub(1),
                            self.start_index
                        ).into()
                    ),
                    self.to_atp_line(),
                    input.to_string()
                )
            )?;
        let end_index = input
            .char_indices()
            .nth(self.end_index + 1)
            .map(|(i, _)| i)
            .ok_or_else(||
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        format!(
                            "Invalid Index for this specific input, supported indexes 0-{}, entered index {}",
                            input.chars().count().saturating_sub(1),
                            self.end_index
                        ).into()
                    ),
                    self.to_atp_line(),
                    input.to_string()
                )
            )?;

        let before = &input[..start_index.min(input.len())];
        let after = &input[end_index.min(input.len())..];

        Ok(format!("{}{}", before, after))
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "dlc;"

        if line[0] == "dlc" {
            let start_index = string_to_usize(&line[1])?;
            let end_index = string_to_usize(&line[2])?;

            check_chunk_bound_indexes(start_index, end_index, None)?;
            self.start_index = start_index;
            self.end_index = end_index;
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

    fn get_string_repr(&self) -> &'static str {
        "dlc"
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Dlc {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Dlc::default().get_opcode() {
            if !instruction.operands.is_empty() {
                let start_index = string_to_usize(&instruction.operands[0])?;
                let end_index = string_to_usize(&instruction.operands[1])?;

                check_chunk_bound_indexes(start_index, end_index, None)?;

                self.start_index = start_index;
                self.end_index = end_index;
                return Ok(());
            }

            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidOperands("Invalid operands for this instruction".into()),
                    instruction.op_code.to_string(),
                    instruction.operands.join(" ")
                )
            );
        }

        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("".into()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Dlc::default().get_opcode(),
            operands: [self.start_index.to_string(), self.end_index.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x08
    }
}
#[cfg(feature = "test_access")]
#[cfg(test)]
mod dlc_tests {
    use crate::tokens::{ TokenMethods, transforms::dlc::Dlc };

    #[test]
    fn delete_chunk() {
        let mut token = Dlc::params(1, 5).unwrap();
        assert!(
            matches!(Dlc::params(5, 1), Err(_)),
            "it throws an error if start_index is bigger than end_index"
        );

        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if start_index does not exists in input"
        );

        assert_eq!(
            token.parse("bananalaranjacheiadecanja"),
            Ok("blaranjacheiadecanja".to_string()),
            "It works with expected inputs"
        );

        assert_eq!(
            token.to_atp_line(),
            "dlc 1 5;\n".to_string(),
            "conversion to atp_line works correctly"
        );

        assert_eq!(token.get_string_repr(), "dlc".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["dlc".to_string(), (5).to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["dlc".to_string(), (1).to_string(), (5).to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn delete_chunk_bytecode() {
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Dlc::params(1, 5).unwrap();

        let mut instruction = BytecodeInstruction {
            op_code: 0x08,
            operands: [(1).to_string(), (5).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x08, "get_opcode does not disrepect ATP token mapping");

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
                    ["dlc".to_string(), (5).to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
    }
}

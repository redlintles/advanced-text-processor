#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

use crate::{
    token_data::TokenMethods,
    utils::{ transforms::string_to_usize, errors::{ AtpError, AtpErrorCode } },
};
/// Ins - Insert
///
/// Inserts `text` after `index` position in `input`
///
/// If index does not exists in current string, `AtpError` is returned
///
/// # Example
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::ins::Ins};
///
/// let token = Ins::params(2,"laranja");
///
/// assert_eq!(token.parse("banana"), Ok("banlaranjaana".to_string()));
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
    fn get_string_repr(&self) -> String {
        "ins".to_string()
    }
    fn to_atp_line(&self) -> String {
        format!("ins {} {};\n", self.index, self.text_to_insert)
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "ins" {
            self.index = string_to_usize(&line[1])?;
            self.text_to_insert = line[2].clone();
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("".to_string()),
                "ins".to_string(),
                line.join(" ")
            )
        )
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        if self.index > input.chars().count() {
            return Err(
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        format!(
                            "Index does not exist in current string, for the input {}, only indexes between 0-{} are allowed",
                            input,
                            input.len().saturating_sub(1)
                        )
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
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Ins {
    fn get_opcode(&self) -> u8 {
        0x28
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Ins::default().get_opcode() {
            self.index = string_to_usize(&instruction.operands[0])?;
            self.text_to_insert = instruction.operands[1].clone();
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

    fn token_to_bytecode_instruction(&self) -> crate::bytecode_parser::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Ins::default().get_opcode(),
            operands: [self.index.to_string(), self.text_to_insert.clone()].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod ins_tests {
    use crate::token_data::{ TokenMethods, token_defs::ins::Ins };
    #[test]
    fn insert_tests() {
        let mut token = Ins::params(2, "laranja");
        assert_eq!(
            token.parse("banana"),
            Ok("banlaranjaana".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "ins 2 laranja;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "ins".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["ins".to_string(), (2).to_string(), "laranja".to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn insert_bytecode_tests() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Ins::params(2, "laranja");

        let instruction = BytecodeInstruction {
            op_code: 0x28,
            operands: [(2).to_string(), "laranja".to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x28, "get_opcode does not disrepect ATP token mapping");

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

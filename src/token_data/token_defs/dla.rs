use crate::{ token_data::TokenMethods, utils::transforms::string_to_usize };

use crate::utils::errors::{ AtpError, AtpErrorCode };

#[cfg(feature = "bytecode")]
use crate::{ bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods } };
/// Dla - Delete After
/// Delete all characters after `index` in the specified `input`
///
/// It will throw an `AtpError` if index does not exists in the current `input`
///
/// # Example:
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::dla::Dla};
///
/// let token = Dla::params(3);
///
/// assert_eq!(token.parse("banana laranja vermelha azul"), Ok("bana".to_string()))
///
/// ```
#[derive(Clone, Copy, Default)]
pub struct Dla {
    pub index: usize,
}

impl Dla {
    pub fn params(index: usize) -> Self {
        Dla {
            index,
        }
    }
}

impl TokenMethods for Dla {
    fn token_to_atp_line(&self) -> String {
        format!("dla {};\n", self.index)
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        if !(0..input.len()).contains(&self.index) {
            return Err(
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        "self.index does not exist in current input".to_string()
                    ),
                    self.token_to_atp_line(),
                    input.to_string()
                )
            );
        }

        let mut s = String::from(input);
        if
            let Some(byte_index) = s
                .char_indices()
                .nth(self.index + 1)
                .map(|(i, _)| i)
        {
            s.drain(byte_index..);
            return Ok(s);
        }
        Err(
            AtpError::new(
                AtpErrorCode::IndexOutOfRange(
                    "Index is out of range for the desired string".to_string()
                ),
                self.token_to_atp_line(),
                input.to_string()
            )
        )
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "dla;"

        if line[0] == "dla" {
            self.index = string_to_usize(&line[1])?;
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

    fn get_string_repr(&self) -> String {
        "dla".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Dla {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        use AtpErrorCode;

        if instruction.op_code == Dla::default().get_opcode() {
            use AtpErrorCode;

            if !instruction.operands[0].is_empty() {
                self.index = string_to_usize(&instruction.operands[0])?;
                return Ok(());
            }

            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidOperands(
                        "Invalid operands for this instruction".to_string()
                    ),
                    instruction.op_code.to_string(),
                    instruction.operands.join(" ")
                )
            );
        }

        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("".to_string()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Dla::default().get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }

    fn get_opcode(&self) -> u8 {
        0x09
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod dla_tests {
    use crate::token_data::{ TokenMethods, token_defs::dla::Dla };
    #[test]
    fn delete_after_test() {
        let mut token = Dla::params(3);

        assert_eq!(
            token.parse("banana laranja vermelha azul"),
            Ok("bana".to_string()),
            "It works correctly with expected inputs"
        );
        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if the string does not have the current token index"
        );

        assert_eq!(
            token.token_to_atp_line(),
            "dla 3;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "dla".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.token_from_vec_params(["dla".to_string(), (3).to_string()].to_vec()),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }
    #[test]
    fn delete_after_bytecode() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Dla::params(3);

        let instruction = BytecodeInstruction {
            op_code: 0x09,
            operands: [(3).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x09, "get_opcode does not disrepect ATP token mapping");

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

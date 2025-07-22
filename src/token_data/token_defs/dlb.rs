use crate::{ token_data::TokenMethods, utils::transforms::string_to_usize };
use crate::utils::errors::{ AtpError, AtpErrorCode };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
/// Dlb - Delete Before
/// Delete all characters before `index` in the specified `input`
///
/// It will throw an `AtpError` if index does not exists in the current `input`
///
/// # Example:
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::dlb::Dlb};
///
/// let token = Dlb::params(3);
///
/// assert_eq!(token.parse("banana laranja vermelha azul"), Ok("ana laranja vermelha azul".to_string()))
///
/// ```
#[derive(Clone, Copy, Default)]
pub struct Dlb {
    pub index: usize,
}

impl Dlb {
    pub fn params(index: usize) -> Self {
        Dlb {
            index,
        }
    }
}

impl TokenMethods for Dlb {
    fn token_to_atp_line(&self) -> String {
        format!("dlb {};\n", self.index)
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let mut s = String::from(input);

        if
            let Some(byte_index) = s
                .char_indices()
                .nth(self.index)
                .map(|(i, _)| i)
        {
            s.drain(0..byte_index);
            return Ok(s);
        }

        Err(
            AtpError::new(
                AtpErrorCode::IndexOutOfRange(
                    format!(
                        "Supported indexes 0-{}, entered index {}",
                        input.chars().count().saturating_sub(1),
                        self.index
                    )
                ),
                self.token_to_atp_line(),
                input.to_string()
            )
        )
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "dlb;"

        if line[0] == "dlb" {
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
        "dlb".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Dlb {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Dlb::default().get_opcode() {
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
            op_code: Dlb::default().get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x0a
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod dlb_tests {
    use crate::token_data::{ TokenMethods, token_defs::dlb::Dlb };
    #[test]
    fn delete_after_test() {
        let mut token = Dlb::params(3);

        assert_eq!(
            token.parse("banana laranja vermelha azul"),
            Ok("ana laranja vermelha azul".to_string()),
            "It works correctly with expected inputs"
        );
        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if the string does not have the current token index"
        );

        assert_eq!(
            token.token_to_atp_line(),
            "dlb 3;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "dlb".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.token_from_vec_params(["dlb".to_string(), (3).to_string()].to_vec()),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }
    #[test]
    fn delete_ater_bytecode() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Dlb::params(3);

        let instruction = BytecodeInstruction {
            op_code: 0x0a,
            operands: [(3).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x0a, "get_opcode does not disrepect ATP token mapping");

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

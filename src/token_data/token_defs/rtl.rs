use crate::{ token_data::TokenMethods, utils::transforms::string_to_usize };

use crate::utils::errors::{ AtpError, AtpErrorCode };

#[cfg(feature = "bytecode")]
use crate::{ bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods } };
/// RTL - Rotate Left
///
/// Rotates `input` to the left `n` times
///
/// # Example
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::rtl::Rtl};
///
/// let token = Rtl::params(3);
///
/// assert_eq!(token.parse("banana"),Ok("anaban".to_string()));
///
/// ```
#[derive(Clone, Default)]
pub struct Rtl {
    pub times: usize,
}

impl Rtl {
    pub fn params(times: usize) -> Rtl {
        Rtl {
            times,
        }
    }
}

impl TokenMethods for Rtl {
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        if input.is_empty() {
            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidParameters("Input is empty".to_string()),
                    self.token_to_atp_line(),
                    "\" \"".to_string()
                )
            );
        }

        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();
        let times = self.times % len;

        Ok(
            chars[times..]
                .iter()
                .chain(&chars[..times])
                .collect()
        )
    }

    fn token_to_atp_line(&self) -> String {
        format!("rtl {};\n", self.times)
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "rtl" {
            self.times = string_to_usize(&line[1])?;
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
        "rtl".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Rtl {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Rtl::default().get_opcode() {
            if !(instruction.operands[0].is_empty() && instruction.operands.len() == 1) {
                self.times = string_to_usize(&instruction.operands[0])?;
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
            op_code: Rtl::default().get_opcode(),
            operands: [self.times.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x0e
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod rtl_tests {
    use crate::token_data::{ TokenMethods, token_defs::rtl::Rtl };
    #[test]
    fn repeat_tests() {
        let mut token = Rtl::params(3);

        assert_eq!(token.parse("banana"), Ok("anaban".to_string()), "It supports expected inputs");
        assert_eq!(
            token.token_to_atp_line(),
            "rtl 3;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "rtl".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.token_from_vec_params(["rtl".to_string(), (3).to_string()].to_vec()),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );

        assert_eq!(
            token.parse("banana"),
            Ok("anaban".to_string()),
            "token_from_vec_params parses the argument list correctly"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn repeat_bytecode_tests() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Rtl::params(3);

        let instruction = BytecodeInstruction {
            op_code: 0x0e,
            operands: [(3).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x0e, "get_opcode does not disrepect ATP token mapping");

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

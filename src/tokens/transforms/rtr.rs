use crate::{
    token_data::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::string_to_usize },
};

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
/// RTR - Rotate Right
///
/// Rotates `input` to the right `n` times
///
/// # Example
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::rtr::Rtr};
///
/// let token = Rtr::params(2);
///
/// assert_eq!(token.parse("banana"),Ok("nabana".to_string()));
///
/// ```
#[derive(Clone, Default)]
pub struct Rtr {
    pub times: usize,
}

impl Rtr {
    pub fn params(times: usize) -> Rtr {
        Rtr {
            times,
        }
    }
}

impl TokenMethods for Rtr {
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        if input.is_empty() {
            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidParameters("Input is empty".to_string()),
                    self.to_atp_line(),
                    "\" \"".to_string()
                )
            );
        }

        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();
        let times = self.times % len;

        Ok(
            chars[len - times..]
                .iter()
                .chain(&chars[..len - times])
                .collect()
        )
    }

    fn to_atp_line(&self) -> String {
        format!("rtr {};\n", self.times)
    }
    fn get_string_repr(&self) -> String {
        "rtr".to_string()
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "rtr" {
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
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Rtr {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Rtr::default().get_opcode() {
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
            op_code: Rtr::default().get_opcode(),
            operands: [self.times.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x0f
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod rtr_tests {
    use crate::token_data::{ TokenMethods, transforms::rtr::Rtr };
    #[test]
    fn rotate_right_tests() {
        let mut token = Rtr::params(2);

        assert_eq!(token.parse("banana"), Ok("nabana".to_string()), "It supports expected inputs");
        assert_eq!(
            token.to_atp_line(),
            "rtr 2;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "rtr".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["rtr".to_string(), (2).to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );

        assert_eq!(
            token.parse("banana"),
            Ok("nabana".to_string()),
            "from_vec_params parses the argument list correctly"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn rotate_right_bytecode_tests() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Rtr::params(2);

        let instruction = BytecodeInstruction {
            op_code: 0x0f,
            operands: [(2).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x0f, "get_opcode does not disrepect ATP token mapping");

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

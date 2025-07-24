use crate::{
    token_data::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::string_to_usize },
};

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

/// RPT - Repeat
///
/// Repeats `input` n `times`
///
/// # Example
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::rpt::Rpt};
///
/// let token = Rpt::params(3);
///
/// assert_eq!(token.parse("banana"),Ok("bananabananabanana".to_string()));
///
/// ```
#[derive(Clone, Default)]
pub struct Rpt {
    pub times: usize,
}

impl Rpt {
    pub fn params(times: usize) -> Self {
        Rpt { times }
    }
}

impl TokenMethods for Rpt {
    fn token_to_atp_line(&self) -> String {
        format!("rpt {};\n", self.times)
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.repeat(self.times))
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "rpt" {
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
        "rpt".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Rpt {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Rpt::default().get_opcode() {
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
            op_code: Rpt::default().get_opcode(),
            operands: [self.times.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x0d
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod rpt_tests {
    use crate::token_data::{ TokenMethods, token_defs::rpt::Rpt };
    #[test]
    fn repeat_tests() {
        let mut token = Rpt::params(3);

        assert_eq!(
            token.parse("banana"),
            Ok("bananabananabanana".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.token_to_atp_line(),
            "rpt 3;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "rpt".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.token_from_vec_params(["rpt".to_string(), (3).to_string()].to_vec()),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );

        assert_eq!(
            token.parse("banana"),
            Ok("bananabananabanana".to_string()),
            "token_from_vec_params parses the argument list correctly"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn repeat_bytecode_tests() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Rpt::params(3);

        let instruction = BytecodeInstruction {
            op_code: 0x0d,
            operands: [(3).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x0d, "get_opcode does not disrepect ATP token mapping");

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

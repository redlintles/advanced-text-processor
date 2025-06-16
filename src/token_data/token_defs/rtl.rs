use crate::{ token_data::TokenMethods, utils::transforms::string_to_usize };

#[cfg(feature = "bytecode")]
use crate::{
    bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods },
    utils::errors::{ AtpError, AtpErrorCode },
};
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
                    crate::utils::errors::AtpErrorCode::InvalidParameters(
                        "Input is empty".to_string()
                    ),
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
                crate::utils::errors::AtpErrorCode::TokenNotFound(
                    "Invalid parser for this token".to_string()
                ),
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
            if !(instruction.operands[0].is_empty() || instruction.operands[1].is_empty()) {
                self.times = string_to_usize(&instruction.operands[1])?;
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

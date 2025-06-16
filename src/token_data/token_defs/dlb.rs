use crate::{ token_data::TokenMethods, utils::{ errors::AtpError, transforms::string_to_usize } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
use crate::utils::errors::AtpErrorCode;
// Delete before
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
            s.drain(..=byte_index);
        }

        Err(
            AtpError::new(
                crate::utils::errors::AtpErrorCode::IndexOutOfRange(
                    format!("Supported indexes 0-{}, entered index {}", input.len() - 1, self.index)
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
                crate::utils::errors::AtpErrorCode::TokenNotFound(
                    "Invalid parser for this token".to_string()
                ),
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
                self.index = string_to_usize(&instruction.operands[1])?;
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

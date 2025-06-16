use crate::{ token_data::TokenMethods, utils::transforms::string_to_usize };

#[cfg(feature = "bytecode")]
use crate::{
    bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods },
    utils::errors::AtpError,
};
// Delete after
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
        let mut s = String::from(input);

        if
            let Some(byte_index) = s
                .char_indices()
                .nth(self.index)
                .map(|(i, _)| i)
        {
            s.drain(byte_index..);
            return Ok(s);
        }
        Err(
            AtpError::new(
                crate::utils::errors::AtpErrorCode::IndexOutOfRange(
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
                crate::utils::errors::AtpErrorCode::TokenNotFound(
                    "Invalid parser for this token".to_string()
                ),
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
        use crate::utils::errors::AtpErrorCode;

        if instruction.op_code == Dla::default().get_opcode() {
            use crate::utils::errors::AtpErrorCode;

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
            op_code: Dla::default().get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }

    fn get_opcode(&self) -> u8 {
        0x09
    }
}

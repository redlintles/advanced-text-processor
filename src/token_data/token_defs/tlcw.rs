use crate::{ token_data::TokenMethods, utils::errors::AtpError };
#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

#[derive(Clone, Default, Copy)]
pub struct Tlcw {
    index: usize,
}

impl Tlcw {
    pub fn params(index: usize) -> Self {
        Tlcw { index }
    }
}
impl TokenMethods for Tlcw {
    fn get_string_repr(&self) -> String {
        "tlcw".to_string()
    }

    fn token_to_atp_line(&self) -> String {
        "tlcw;\n".to_string()
    }

    fn parse(&self, input: &str) -> Result<String, crate::utils::errors::AtpError> {
        Ok(
            input
                .split_whitespace()
                .enumerate()
                .map(|(i, w)| {
                    if i == self.index { w.to_lowercase() } else { w.to_string() }
                })
                .collect::<Vec<_>>()
                .join(" ")
                .to_string()
        )
    }

    fn token_from_vec_params(
        &mut self,
        line: Vec<String>
    ) -> Result<(), crate::utils::errors::AtpError> {
        if line[0] == "tlcw" {
            return Ok(());
        }

        Err(
            AtpError::new(
                crate::utils::errors::AtpErrorCode::TokenNotFound(
                    "Invalid Parser for this token".to_string()
                ),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Tlcw {
    fn get_opcode(&self) -> u8 {
        0x29
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Tlcw::default().get_opcode() {
            return Ok(());
        }

        Err(
            AtpError::new(
                crate::utils::errors::AtpErrorCode::BytecodeNotFound(
                    "Invalid Parser for this token".to_string()
                ),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }
    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Tlcw::default().get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }
}

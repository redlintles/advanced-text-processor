use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };
#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

#[derive(Clone, Default, Copy)]
pub struct Tucw {
    index: usize,
}

impl Tucw {
    pub fn params(index: usize) -> Self {
        Tucw { index }
    }
}
impl TokenMethods for Tucw {
    fn get_string_repr(&self) -> String {
        "tucw".to_string()
    }

    fn token_to_atp_line(&self) -> String {
        "tucw;\n".to_string()
    }

    fn parse(&self, input: &str) -> Result<String, crate::utils::errors::AtpError> {
        Ok(
            input
                .split_whitespace()
                .enumerate()
                .map(|(i, w)| {
                    if i == self.index { w.to_uppercase() } else { w.to_string() }
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
        if line[0] == "tucw" {
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid Parser for this token".to_string()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Tucw {
    fn get_opcode(&self) -> u8 {
        0x2a
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Tucw::default().get_opcode() {
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".to_string()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }
    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Tucw::default().get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }
}

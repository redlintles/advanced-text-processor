use crate::{ utils::errors::{ AtpError, AtpErrorCode }, token_data::TokenMethods };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

#[derive(Clone, Copy, Default)]
pub struct Dls {
    pub index: usize,
}

impl Dls {
    pub fn params(index: usize) -> Self {
        Dls {
            index,
        }
    }
}

impl TokenMethods for Dls {
    fn get_string_repr(&self) -> String {
        "dls".to_string()
    }
    fn token_to_atp_line(&self) -> String {
        "dls;\n".to_string()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        if !(0..input.chars().count()).contains(&self.index) {
            return Err(
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        format!(
                            "O índice {} não existe na string {}, que só permite índices no intervalo {}-{}",
                            self.index,
                            input,
                            0,
                            input.chars().count()
                        )
                    ),
                    self.get_string_repr(),
                    input.to_string()
                )
            );
        }
        Ok(
            input
                .chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if self.index == i {
                        return None;
                    } else {
                        return Some(c);
                    }
                })
                .collect()
        )
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "dls" {
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
impl BytecodeTokenMethods for Dls {
    fn get_opcode(&self) -> u8 {
        0x32
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == self.get_opcode() {
            use crate::utils::transforms::string_to_usize;

            self.index = string_to_usize(&instruction.operands[1])?;
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
            op_code: self.get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }
}

use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeTokenMethods, BytecodeInstruction };

#[derive(Clone, Default)]
pub struct Padr {
    pub text: String,
    pub times: usize,
}

impl Padr {
    pub fn params(text: &str, times: usize) -> Self {
        Padr {
            text: text.to_string(),
            times,
        }
    }
}

impl TokenMethods for Padr {
    fn get_string_repr(&self) -> String {
        "padr".to_string()
    }
    fn token_to_atp_line(&self) -> String {
        "padr;\n".to_string()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let s = self.text.repeat(self.times);

        Ok(format!("{}{}", input, s))
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "padr" {
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
impl BytecodeTokenMethods for Padr {
    fn get_opcode(&self) -> u8 {
        0x30
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), crate::utils::errors::AtpError> {
        if instruction.op_code == self.get_opcode() {
            use crate::utils::transforms::string_to_usize;

            self.text = instruction.operands[1].clone();
            self.times = string_to_usize(&instruction.operands[2])?;
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
            operands: [self.text.clone(), self.times.to_string()].to_vec(),
        }
    }
}

use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeTokenMethods, BytecodeInstruction };

#[derive(Clone, Default)]
pub struct Padl {
    pub text: String,
    pub times: usize,
}

impl Padl {
    pub fn params(text: &str, times: usize) -> Self {
        Padl {
            text: text.to_string(),
            times,
        }
    }
}

impl TokenMethods for Padl {
    fn get_string_repr(&self) -> String {
        "padl".to_string()
    }
    fn token_to_atp_line(&self) -> String {
        "padl;\n".to_string()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let s = self.text.repeat(self.times);

        Ok(format!("{}{}", s, input))
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "padl" {
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
impl BytecodeTokenMethods for Padl {
    fn get_opcode(&self) -> u8 {
        0x2f
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

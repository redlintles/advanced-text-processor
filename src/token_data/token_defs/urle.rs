use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeTokenMethods, BytecodeInstruction };

#[derive(Copy, Clone, Default)]
pub struct Urle {}

impl TokenMethods for Urle {
    fn get_string_repr(&self) -> String {
        "urle".to_string()
    }

    fn token_to_atp_line(&self) -> String {
        "urle;\n".to_string()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(urlencoding::encode(input).to_string())
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "urle" {
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
impl BytecodeTokenMethods for Urle {
    fn get_opcode(&self) -> u8 {
        0x20
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Urle::default().get_opcode() {
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("".to_string()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> crate::bytecode_parser::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Urle::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

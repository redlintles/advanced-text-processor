use crate::{
    token_data::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::capitalize },
};

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

// Join to Kebab Case
#[derive(Clone, Copy, Default)]
pub struct Jpsc {}

impl TokenMethods for Jpsc {
    fn get_string_repr(&self) -> String {
        "jpsc".to_string()
    }

    fn token_to_atp_line(&self) -> String {
        "jpsc;\n".to_string()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let v = input.split_whitespace().collect::<Vec<_>>();

        let processed = v
            .iter()
            .map(|w| { capitalize(w) })
            .collect::<Vec<_>>()
            .join("");

        Ok(processed)
    }

    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "jpsc" {
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
impl BytecodeTokenMethods for Jpsc {
    fn get_opcode(&self) -> u8 {
        0x2e
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Jpsc::default().get_opcode() {
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
            op_code: Jpsc::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

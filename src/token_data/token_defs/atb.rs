use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

// add to beginning
#[derive(Clone, Default)]
pub struct Atb {
    pub text: String,
}

impl Atb {
    pub fn params(text: String) -> Self {
        Atb {
            text,
        }
    }
}

impl TokenMethods for Atb {
    fn token_to_atp_line(&self) -> String {
        format!("atb {};\n", self.text)
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let mut s = String::from(&self.text);
        s.push_str(input);
        Ok(s)
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "atb;"

        if line[0] == "atb" {
            self.text = line[1].clone();
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Token not recognized".to_string()),
                self.token_to_atp_line(),
                line.join(" ")
            )
        )
    }

    fn get_string_repr(&self) -> String {
        "atb".to_string()
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Atb {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Atb::default().get_opcode() {
            if !instruction.operands[0].is_empty() {
                self.text = instruction.operands[1].clone();
                return Ok(());
            }

            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidOperands(
                        "Invalid operands for this instruction! expected {text}".to_string()
                    ),
                    self.token_to_bytecode_instruction().to_bytecode_line(),
                    instruction.operands.join(" ")
                )
            );
        }

        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid code for this parser!".to_string()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Atb::default().get_opcode(),
            operands: [self.text.clone()].to_vec(),
        }
    }

    fn get_opcode(&self) -> u8 {
        0x01
    }
}

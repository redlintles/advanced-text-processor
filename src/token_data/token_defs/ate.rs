use crate::{ token_data::TokenMethods, utils::errors::AtpError };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
// add to end
#[derive(Clone, Default)]
pub struct Ate {
    pub text: String,
}

impl Ate {
    pub fn params(text: String) -> Self {
        Ate {
            text,
        }
    }
}

impl TokenMethods for Ate {
    fn token_to_atp_line(&self) -> String {
        format!("ate {};\n", self.text)
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let mut s = String::from(input);
        s.push_str(&self.text);
        Ok(s)
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "ate;"

        if line[0] == "ate" {
            self.text = line[1].clone();
            return Ok(());
        }
        Err(
            AtpError::new(
                crate::utils::errors::AtpErrorCode::TokenNotFound(
                    "Invalid parser for this token".to_string()
                ),
                line.join(" "),
                line.join(" ")
            )
        )
    }

    fn get_string_repr(&self) -> String {
        "ate".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Ate {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Ate::default().get_opcode() {
            if !instruction.operands[0].is_empty() {
                self.text = instruction.operands[1].clone();
                return Ok(());
            }

            return Err(
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::InvalidOperands(
                        "An ATP Bytecode parsing error ocurred: Invalid Operands".to_string()
                    ),
                    instruction.to_bytecode_line(),
                    instruction.operands.join(" ")
                )
            );
        }

        Err(
            AtpError::new(
                crate::utils::errors::AtpErrorCode::TokenNotFound(
                    "An ATP Bytecode parsing error ocurred: Invalid Token".to_string()
                ),
                instruction.to_bytecode_line(),
                instruction.operands.join("")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Ate::default().get_opcode(),
            operands: [self.text.clone()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x02
    }
}

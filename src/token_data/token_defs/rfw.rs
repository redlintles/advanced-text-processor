use regex::Regex;

use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
// Replace first with
#[derive(Clone)]
pub struct Rfw {
    pub pattern: Regex,
    pub text_to_replace: String,
}

impl Rfw {
    pub fn params(pattern: String, text_to_replace: String) -> Result<Self, String> {
        let pattern = Regex::new(&pattern).map_err(|x| x.to_string())?;
        Ok(Rfw {
            text_to_replace,
            pattern,
        })
    }
}

impl Default for Rfw {
    fn default() -> Self {
        Rfw {
            pattern: Regex::new("").unwrap(),
            text_to_replace: "_".to_string(),
        }
    }
}

impl TokenMethods for Rfw {
    fn token_to_atp_line(&self) -> String {
        format!("rfw {} {};\n", self.pattern, self.text_to_replace)
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(self.pattern.replace(input, &self.text_to_replace).to_string())
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "rfw;"

        if line[0] == "rfw" {
            self.pattern = Regex::new(&line[1]).map_err(|_|
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::TextParsingError(
                        "Failed creating regex".to_string()
                    ),
                    line[0].to_string(),
                    line.join(" ")
                )
            )?;
            self.text_to_replace = line[2].clone();
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
        "rfw".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Rfw {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Rfw::default().get_opcode() {
            if !(instruction.operands[0].is_empty() || instruction.operands[1].is_empty()) {
                self.pattern = Regex::new(&instruction.operands[0].clone()).map_err(|_|
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::TextParsingError(
                            "Failed creating regex".to_string()
                        ),
                        instruction.op_code.to_string(),
                        instruction.operands.join(" ")
                    )
                )?;
                self.text_to_replace = instruction.operands[1].clone();
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
            op_code: Rfw::default().get_opcode(),
            operands: [self.pattern.to_string(), self.text_to_replace.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x0c
    }
}

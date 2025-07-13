use regex::Regex;

use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
// Replace first with
#[derive(Clone)]
pub struct Rlw {
    pub pattern: Regex,
    pub text_to_replace: String,
}

impl Rlw {
    pub fn params(pattern: String, text_to_replace: String) -> Result<Self, String> {
        let pattern = Regex::new(&pattern).map_err(|x| x.to_string())?;
        Ok(Rlw {
            text_to_replace,
            pattern,
        })
    }
}

impl Default for Rlw {
    fn default() -> Self {
        Rlw {
            pattern: Regex::new("").unwrap(),
            text_to_replace: "_".to_string(),
        }
    }
}

impl TokenMethods for Rlw {
    fn token_to_atp_line(&self) -> String {
        format!("rlw {} {};\n", self.pattern, self.text_to_replace)
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let caps: Vec<_> = self.pattern.find_iter(input).collect();

        if let Some(m) = caps.last() {
            let (start, end) = (m.start(), m.end());

            let mut result = String::with_capacity(
                input.len() - (end - start) + self.text_to_replace.len()
            );
            result.push_str(&input[..start]);
            result.push_str(&self.text_to_replace);
            result.push_str(&input[end..]);
            return Ok(result);
        }
        Ok(input.to_string())
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "rlw;"

        if line[0] == "rlw" {
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
        "rlw".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Rlw {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Rlw::default().get_opcode() {
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
            op_code: Rlw::default().get_opcode(),
            operands: [self.pattern.to_string(), self.text_to_replace.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x1e
    }
}

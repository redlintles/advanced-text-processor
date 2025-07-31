use regex::Regex;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };
/// RAW - Replace All With
///
/// Replace all ocurrences of `pattern` in `input` with `text_to_replace`
///
/// See Also:
///
/// - [`RCW` - Replace Count With](crate::tokens::transforms::rcw)
/// - [`RFW` - Replace First With](crate::tokens::transforms::rfw)
/// - [`RLW` - Replace Last With](crate::tokens::transforms::rlw)
/// - [`RNW` - Replace Nth With](crate::tokens::transforms::rnw)
///
/// # Example:
///
/// ```rust
/// use atp_project::tokens::{TokenMethods, transforms::raw::Raw};
///
/// let token = Raw::params(&"a", "b").unwrap();
///
/// assert_eq!(token.parse("aaaaa"), Ok("bbbbb".to_string()));
/// ```
///
#[derive(Clone)]
pub struct Raw {
    pub pattern: Regex,
    pub text_to_replace: String,
}

impl Raw {
    pub fn params(pattern: &str, text_to_replace: &str) -> Result<Self, String> {
        let pattern = Regex::new(&pattern).map_err(|x| x.to_string())?;
        Ok(Raw {
            text_to_replace: text_to_replace.to_string(),
            pattern,
        })
    }
}

impl Default for Raw {
    fn default() -> Self {
        Raw {
            pattern: Regex::new("").unwrap(),
            text_to_replace: "_".to_string(),
        }
    }
}

impl TokenMethods for Raw {
    fn to_atp_line(&self) -> String {
        format!("raw {} {};\n", self.pattern, self.text_to_replace)
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(self.pattern.replace_all(input, &self.text_to_replace).to_string())
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "raw;"

        if line[0] == "raw" {
            self.pattern = Regex::new(&line[1]).map_err(|_|
                AtpError::new(
                    AtpErrorCode::TextParsingError("Failed creating regex".to_string()),
                    line[0].to_string(),
                    line.join(" ")
                )
            )?;
            self.text_to_replace = line[2].clone();
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

    fn get_string_repr(&self) -> String {
        "raw".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Raw {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Raw::default().get_opcode() {
            if !(instruction.operands[0].is_empty() || instruction.operands[1].is_empty()) {
                self.pattern = Regex::new(&instruction.operands[0].clone()).map_err(|_|
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed creating regex".to_string()),
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
            op_code: Raw::default().get_opcode(),
            operands: [self.pattern.to_string(), self.text_to_replace.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x0b
    }
}

#[cfg(test)]
#[cfg(feature = "test_access")]
mod raw_tests {
    use crate::tokens::{ TokenMethods, transforms::raw::Raw };
    #[test]
    fn replace_all_with_tests() {
        let mut token = Raw::params("a", "b").unwrap();
        assert_eq!(token.parse("aaaaa"), Ok("bbbbb".to_string()), "It supports expected inputs");

        assert_eq!(
            token.to_atp_line(),
            "raw a b;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "raw".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["raw".to_string(), "a".to_string(), "b".to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn replace_all_with_bytecode_tests() {
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Raw::params("a", "b").unwrap();

        let instruction = BytecodeInstruction {
            op_code: 0x0b,
            operands: ["a".to_string(), "b".to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x0b, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.token_from_bytecode_instruction(instruction.clone()),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.token_to_bytecode_instruction(),
            instruction,
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

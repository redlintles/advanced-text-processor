use std::borrow::Cow;

use regex::Regex;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };
/// RLW - Replace Last With
///
/// Replace the last ocurrency of `pattern` in `input` with `text_to_replace`
///
/// See Also:
///
/// - [`RAW` - Replace All With](crate::tokens::transforms::raw)
/// - [`RCW` - Replace First With](crate::tokens::transforms::rcw)
/// - [`RFW` - Replace Last With](crate::tokens::transforms::rfw)
/// - [`RNW` - Replace Nth With](crate::tokens::transforms::rnw)
///
/// # Example:
///
/// ```rust
/// use atp_project::tokens::{TokenMethods, transforms::rlw::Rlw};
///
/// let token = Rlw::params(&"a", "b").unwrap();
///
/// assert_eq!(token.parse("aaaaa"), Ok("aaaab".to_string()));
/// ```
///
#[derive(Clone)]
pub struct Rlw {
    pub pattern: Regex,
    pub text_to_replace: String,
}

impl Rlw {
    pub fn params(pattern: &str, text_to_replace: &str) -> Result<Self, String> {
        let pattern = Regex::new(&pattern).map_err(|x| x.to_string())?;
        Ok(Rlw {
            text_to_replace: text_to_replace.to_string(),
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
    fn to_atp_line(&self) -> Cow<'static, str> {
        Cow::Owned(format!("rlw {} {};\n", self.pattern, self.text_to_replace))
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
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "rlw;"

        if line[0] == "rlw" {
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

    fn get_string_repr(&self) -> &'static str {
        "rlw"
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
            op_code: Rlw::default().get_opcode(),
            operands: [self.pattern.to_string(), self.text_to_replace.to_string()].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x1e
    }
}

#[cfg(test)]
#[cfg(feature = "test_access")]
mod rlw_tests {
    use crate::tokens::{ TokenMethods, transforms::rlw::Rlw };
    #[test]
    fn replace_last_with_tests() {
        let mut token = Rlw::params("a", "b").unwrap();
        assert_eq!(token.parse("aaaaa"), Ok("aaaab".to_string()), "It supports expected inputs");

        assert_eq!(
            token.to_atp_line(),
            "rlw a b;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "rlw".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["rlw".to_string(), "a".to_string(), "b".to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn replace_last_with_bytecode_tests() {
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Rlw::params("a", "b").unwrap();

        let instruction = BytecodeInstruction {
            op_code: 0x1e,
            operands: ["a".to_string(), "b".to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x1e, "get_opcode does not disrepect ATP token mapping");

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

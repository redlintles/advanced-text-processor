use regex::Regex;

use crate::{ token_data::TokenMethods, utils::transforms::string_to_usize };
use crate::utils::errors::{ AtpError, AtpErrorCode };
#[cfg(feature = "bytecode")]
use crate::{ bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods } };

/// RCW - Replace Count With
///
/// Replace `count` ocurrences of `pattern` in `input` with `text_to_replace`
///
/// See Also:
///
/// - [`RAW` - Replace All With](crate::token_data::token_defs::rcw)
/// - [`RFW` - Replace First With](crate::token_data::token_defs::rfw)
/// - [`RLW` - Replace Last With](crate::token_data::token_defs::rlw)
/// - [`RNW` - Replace Nth With](crate::token_data::token_defs::rnw)
///
/// # Example:
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::rcw::Rcw};
///
/// let token = Rcw::params(&"a", "b", 3).unwrap();
///
/// assert_eq!(token.parse("aaaaa"), Ok("bbbaa".to_string()));
/// ```
///
#[derive(Clone)]
pub struct Rcw {
    pub pattern: Regex,
    pub count: usize,
    pub text_to_replace: String,
}

impl Rcw {
    pub fn params(pattern: &str, text_to_replace: &str, count: usize) -> Result<Self, String> {
        let pattern = Regex::new(&pattern).map_err(|x| x.to_string())?;
        Ok(Rcw {
            text_to_replace: text_to_replace.to_string(),
            pattern,
            count,
        })
    }
}

impl Default for Rcw {
    fn default() -> Self {
        Rcw {
            pattern: Regex::new("").unwrap(),
            text_to_replace: "_".to_string(),
            count: 0 as usize,
        }
    }
}

impl TokenMethods for Rcw {
    fn to_atp_line(&self) -> String {
        format!("rcw {} {} {};\n", self.pattern, self.text_to_replace, self.count)
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(self.pattern.replacen(input, self.count, &self.text_to_replace).to_string())
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "rcw;"

        if line[0] == "rcw" {
            self.pattern = Regex::new(&line[1]).map_err(|_|
                AtpError::new(
                    AtpErrorCode::TextParsingError("Failed creating regex".to_string()),
                    line[0].to_string(),
                    line.join(" ")
                )
            )?;
            self.text_to_replace = line[2].clone();
            self.count = string_to_usize(&line[3])?;
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
        "rcw".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Rcw {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Rcw::default().get_opcode() {
            if !(instruction.operands[0].is_empty() || instruction.operands[1].is_empty()) {
                self.pattern = Regex::new(&instruction.operands[0].clone()).map_err(|_|
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed creating regex".to_string()),
                        instruction.op_code.to_string(),
                        instruction.operands.join(" ")
                    )
                )?;
                self.text_to_replace = instruction.operands[1].clone();
                self.count = string_to_usize(&instruction.operands[2])?;
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
            op_code: Rcw::default().get_opcode(),
            operands: [
                self.pattern.to_string(),
                self.text_to_replace.to_string(),
                self.count.to_string(),
            ].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x10
    }
}

#[cfg(test)]
#[cfg(feature = "test_access")]
mod rcw_tests {
    use crate::token_data::{ TokenMethods, token_defs::rcw::Rcw };
    #[test]
    fn replace_count_with_tests() {
        let mut token = Rcw::params("a", "b", 3).unwrap();
        assert_eq!(token.parse("aaaaa"), Ok("bbbaa".to_string()), "It supports expected inputs");

        assert_eq!(
            token.to_atp_line(),
            "rcw a b 3;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "rcw".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["rcw".to_string(), "a".to_string(), "b".to_string(), (3).to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn replace_count_with_bytecode_tests() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Rcw::params("a", "b", 3).unwrap();

        let instruction = BytecodeInstruction {
            op_code: 0x10,
            operands: ["a".to_string(), "b".to_string(), (3).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x10, "get_opcode does not disrepect ATP token mapping");

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

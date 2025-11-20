use std::borrow::Cow;

use regex::Regex;

use crate::utils::validations::check_vec_len;
use crate::{ tokens::TokenMethods, utils::transforms::string_to_usize };

use crate::utils::errors::{ AtpError, AtpErrorCode };
#[cfg(feature = "bytecode")]
use crate::{ bytecode::{ BytecodeInstruction, BytecodeTokenMethods } };

/// SSLT - Split Select
///
/// Splits `input` by `pattern and return `index` of the resulting vec,
/// *discarding* the rest of the text in the process.
///
/// # Example:
///
/// ```rust
/// use atp_project::tokens::{TokenMethods, transforms::sslt::Sslt};
///
/// let token = Sslt::params("_", 1).unwrap();
///
/// assert_eq!(token.parse("foobar_foo_bar_bar_foo_barfoo"), Ok("foo".to_string()));
///
/// ```
#[derive(Clone)]
pub struct Sslt {
    pub pattern: Regex,
    pub index: usize,
}

impl Sslt {
    pub fn params(pattern: &str, index: usize) -> Result<Self, String> {
        let pattern = Regex::new(&pattern).map_err(|x| x.to_string())?;
        Ok(Sslt {
            pattern,
            index,
        })
    }
}

impl Default for Sslt {
    fn default() -> Self {
        Sslt {
            pattern: Regex::new("").unwrap(),
            index: 0,
        }
    }
}

impl TokenMethods for Sslt {
    fn get_string_repr(&self) -> &'static str {
        "sslt"
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let s = self.pattern
            .split(input)

            .collect::<Vec<_>>();

        if !(0..s.len()).contains(&self.index) {
            return Err(
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        "Index does not exist in the splitted vec".to_string()
                    ),
                    self.to_atp_line(),
                    input.to_string()
                )
            );
        }
        let i = match self.index >= s.len() {
            true => s.len() - 1,
            false => self.index,
        };

        let item = s
            .get(i)
            .ok_or_else(||
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange("Item not found".to_string()),
                    "sslt".to_string(),
                    input.to_string()
                )
            )?;
        Ok(item.to_string())
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        check_vec_len(&line, 3)?;
        if line[0] == "sslt" {
            self.pattern = Regex::new(&line[1]).map_err(|_|
                AtpError::new(
                    AtpErrorCode::TextParsingError("Failed to create regex".to_string()),
                    "sslt".to_string(),
                    String::from(&line[1])
                )
            )?;
            self.index = string_to_usize(&line[2])?;
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

    fn to_atp_line(&self) -> Cow<'static, str> {
        Cow::Owned(format!("sslt {} {};\n", self.pattern, self.index))
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Sslt {
    fn get_opcode(&self) -> u8 {
        0x1a
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode::BytecodeInstruction
    ) -> Result<(), AtpError> {
        check_vec_len(&instruction.operands, 2)?;
        if instruction.op_code == Sslt::default().get_opcode() {
            self.pattern = Regex::new(&instruction.operands[0]).map_err(|_|
                AtpError::new(
                    AtpErrorCode::TextParsingError("Failed to create regex".to_string()),
                    "sslt".to_string(),
                    String::from(&instruction.operands[0])
                )
            )?;
            self.index = string_to_usize(&instruction.operands[1])?;
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

    fn token_to_bytecode_instruction(&self) -> crate::bytecode::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Sslt::default().get_opcode(),
            operands: [self.pattern.to_string(), self.index.to_string()].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod sslt_tests {
    use crate::tokens::{ TokenMethods, transforms::sslt::Sslt };

    #[test]
    fn split_select_tests() {
        let mut token = Sslt::params("_", 5).unwrap();

        assert_eq!(token.parse("foobar_foo_bar_bar_foo_barfoo"), Ok("barfoo".to_string()));

        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if start_index does not exists in input"
        );

        assert_eq!(
            token.to_atp_line(),
            "sslt _ 5;\n".to_string(),
            "conversion to atp_line works correctly"
        );

        assert_eq!(
            token.get_string_repr(),
            "sslt".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["sslt".to_string(), "(".to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["sslt".to_string(), "_".to_string(), (5).to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn split_select_bytecode_tests() {
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Sslt::params("_", 5).unwrap();

        let mut instruction = BytecodeInstruction {
            op_code: 0x1a,
            operands: ["_".to_string(), (5).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x1a, "get_opcode does not disrepect ATP token mapping");

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

        instruction.operands = ["(".to_string(), (1).to_string()].to_vec();

        assert!(
            matches!(token.token_from_bytecode_instruction(instruction.clone()), Err(_)),
            "It throws an error for invalid operands"
        );

        instruction.op_code = 0x01;
        assert!(
            matches!(token.token_from_bytecode_instruction(instruction.clone()), Err(_)),
            "It throws an error for invalid op_code"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["sslt".to_string(), "(".to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid param vec"
        );
    }
}

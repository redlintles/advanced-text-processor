use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

/// Token `Atb` — Add to Beginning
///
/// Adds `text` to the beginning of `input`
///
/// # Example
///
/// ```rust
/// use atp_project::tokens::{TokenMethods, transforms::atb::Atb};
///
/// let token = Atb::params("foo");
/// assert_eq!(token.parse(" bar"), Ok("foo bar".to_string()));
/// ```
#[derive(Clone, Default)]
pub struct Atb {
    pub text: String,
}

impl Atb {
    pub fn params(text: &str) -> Self {
        Atb {
            text: text.to_string(),
        }
    }
}

impl TokenMethods for Atb {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("atb {};\n", self.text).into()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let mut s = String::from(&self.text);
        s.push_str(input);
        Ok(s)
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "atb;"

        if line[0] == "atb" {
            self.text = line[1].clone();
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Token not recognized".into()),
                self.to_atp_line(),
                line.join(" ")
            )
        )
    }

    fn get_string_repr(&self) -> &'static str {
        "atb"
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Atb {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Atb::default().get_opcode() {
            if !instruction.operands.is_empty() {
                self.text = instruction.operands[0].clone();
                return Ok(());
            }

            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidOperands(
                        "Invalid operands for this instruction! expected {text}".into()
                    ),
                    self.token_to_bytecode_instruction().to_bytecode_line(),
                    instruction.operands.join(" ")
                )
            );
        }

        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid code for this parser!".into()),
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

#[cfg(feature = "test_access")]
#[cfg(test)]
mod atb_tests {
    mod test_text_version {
        use crate::{ tokens::TokenMethods, tokens::transforms::atb::Atb };
        #[test]
        fn test_with_inputs() {
            let random_text = random_string::generate(6, ('a'..'z').collect::<String>());
            let token = Atb::params("banana");

            assert_eq!(
                token.parse(&random_text),
                Ok(format!("{}{}", "banana", random_text)),
                "It works with random inputs"
            );

            assert_eq!(
                token.parse("coxinha"),
                Ok("bananacoxinha".to_string()),
                "It works with expected inputs"
            );

            assert_eq!(
                token.parse("bànánà"),
                Ok("bananabànánà".to_string()),
                "It supports utf-8 strings"
            )
        }

        #[test]
        fn test_to_atp_line() {
            let token = Atb::params("banana");

            assert_eq!(
                token.to_atp_line(),
                "atb banana;\n".to_string(),
                "conversion to atp_line works correctly"
            );
        }

        #[test]
        fn test_get_string_repr() {
            let token = Atb::default();
            assert_eq!(
                token.get_string_repr(),
                "atb".to_string(),
                "get_string_repr works as expected"
            );
        }

        #[test]
        fn test_from_vec_params() {
            let mut token = Atb::params("laranja");
            assert!(
                matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
                "It throws an error for invalid vec_params"
            );
            assert!(
                matches!(
                    token.from_vec_params(["atb".to_string(), "banana".to_string()].to_vec()),
                    Ok(_)
                ),
                "It does not throws an error for valid vec_params"
            );

            assert_eq!(
                token.parse("coxinha"),
                Ok("bananacoxinha".to_string()),
                "from_vec_params call fill token params correctly"
            );
        }
    }

    #[cfg(feature = "bytecode")]
    mod test_bytecode_version {
        use crate::{ tokens::{ TokenMethods, transforms::atb::Atb } };
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        #[test]
        fn test_to_bytecode_instruction() {
            let token = Atb::params("banana");

            let instruction = BytecodeInstruction {
                op_code: 0x01,
                operands: ["banana".to_string()].to_vec(),
            };

            assert_eq!(
                token.token_to_bytecode_instruction(),
                instruction,
                "Conversion to bytecode instruction works perfectly!"
            );
        }

        #[test]
        fn test_get_op_code() {
            let token = Atb::default();
            assert_eq!(token.get_opcode(), 0x01, "get_opcode does not disrepect ATP token mapping");
        }

        #[test]
        fn test_from_bytecode_instruction() {
            let mut token = Atb::params("laranja");

            let mut instruction = BytecodeInstruction {
                op_code: 0x01,
                operands: ["banana".to_string()].to_vec(),
            };

            assert_eq!(
                token.token_from_bytecode_instruction(instruction.clone()),
                Ok(()),
                "Parsing from bytecode to token works correctly!"
            );
            assert_eq!(
                token.parse("coxinha"),
                Ok("bananacoxinha".to_string()),
                "from_bytecode_instruction fills token params correctly"
            );

            instruction.op_code = 0x02;

            assert!(
                matches!(token.token_from_bytecode_instruction(instruction.clone()), Err(_)),
                "Throws an error for invalid op_code"
            );

            instruction.op_code = 0x01;
            instruction.operands = [].to_vec();

            assert!(
                matches!(token.token_from_bytecode_instruction(instruction.clone()), Err(_)),
                "Throws an error for invalid operands"
            );
        }
    }
}

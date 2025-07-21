use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

// add to beginning
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
            if !instruction.operands.is_empty() {
                self.text = instruction.operands[0].clone();
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

#[cfg(feature = "test_access")]
#[cfg(test)]
mod atb_tests {
    use crate::{ token_data::TokenMethods, token_data::token_defs::atb::Atb };

    #[test]
    fn test_add_to_beginning() {
        let mut token = Atb::params("banana");

        assert_eq!(token.parse("coxinha"), Ok("bananacoxinha".to_string()));

        assert_eq!(
            token.token_to_atp_line(),
            "atb banana;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "atb".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.token_from_vec_params(["atb".to_string(), "banana".to_string()].to_vec()),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_add_to_beginning_bytecode() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Atb::params("banana");

        let instruction = BytecodeInstruction {
            op_code: 0x01,
            operands: ["banana".to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x01, "get_opcode does not disrepect ATP token mapping");

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

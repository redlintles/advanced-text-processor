use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };
// add to end
#[derive(Clone, Default)]
pub struct Ate {
    pub text: String,
}

impl Ate {
    pub fn params(text: &str) -> Self {
        Ate {
            text: text.to_string(),
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
                AtpErrorCode::TokenNotFound("Invalid parser for this token".to_string()),
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
            if !instruction.operands.is_empty() {
                self.text = instruction.operands[0].clone();
                return Ok(());
            }

            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidOperands(
                        "An ATP Bytecode parsing error ocurred: Invalid Operands".to_string()
                    ),
                    instruction.to_bytecode_line(),
                    instruction.operands.join(" ")
                )
            );
        }

        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound(
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

#[cfg(feature = "test_access")]
#[cfg(test)]
mod ate_tests {
    use crate::{ token_data::TokenMethods, token_data::token_defs::ate::Ate };

    #[test]
    fn test_add_to_end() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());
        let mut token = Ate::params("banana");

        assert_eq!(token.parse(&random_text), Ok(format!("{}{}", random_text, "banana")));

        assert_eq!(token.parse("coxinha"), Ok("coxinhabanana".to_string()));

        assert_eq!(
            token.token_to_atp_line(),
            "ate banana;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "ate".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.token_from_vec_params(["ate".to_string(), "banana".to_string()].to_vec()),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_add_to_end_bytecode() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Ate::params("banana");

        let instruction = BytecodeInstruction {
            op_code: 0x02,
            operands: ["banana".to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x02, "get_opcode does not disrepect ATP token mapping");

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

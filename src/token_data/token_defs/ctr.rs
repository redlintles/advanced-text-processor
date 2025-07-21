use crate::{ token_data::TokenMethods, utils::transforms::{ capitalize, string_to_usize } };
use crate::utils::errors::{ AtpError, AtpErrorCode };
#[cfg(feature = "bytecode")]
use crate::{ bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods } };
#[derive(Clone, Default)]
pub struct Ctr {
    pub start_index: usize,
    pub end_index: usize,
}

impl Ctr {
    pub fn params(start_index: usize, end_index: usize) -> Result<Self, AtpError> {
        if start_index > end_index {
            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidIndex(
                        "Start index must be smaller than end index".to_string()
                    ),
                    format!("ctc {} {};", start_index, end_index),
                    format!("Start Index: {}, End Index: {}", start_index, end_index)
                )
            );
        }
        Ok(Ctr {
            start_index,
            end_index,
        })
    }
}

impl TokenMethods for Ctr {
    fn get_string_repr(&self) -> String {
        "ctr".to_string()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        if self.start_index >= input.chars().count() {
            return Err(
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        "start_index does not exist in current string".to_string()
                    ),
                    self.token_to_atp_line(),
                    input.to_string()
                )
            );
        }
        // Since the user will probably not know the length of the string in the middle of the processing
        // Better simply adjust end_index to input.len() if its bigger. instead of throwing an "hard to debug" error

        let mut end = self.end_index;
        let total = input.split_whitespace().count();
        if end > total {
            end = total;
        }

        let result = input
            .split_whitespace()
            .enumerate()
            .map(|(i, c)| {
                if (self.start_index..=end).contains(&i) { capitalize(c) } else { c.to_string() }
            })
            .collect::<Vec<_>>()
            .join(" ");

        Ok(result)
    }

    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "ctr" {
            let start_index = string_to_usize(&line[1])?;
            let end_index = string_to_usize(&line[2])?;
            if start_index > end_index {
                return Err(
                    AtpError::new(
                        AtpErrorCode::InvalidIndex(
                            "Start index must be smaller than end index".to_string()
                        ),
                        format!("ctc {} {};", start_index, end_index),
                        format!("Start Index: {}, End Index: {}", start_index, end_index)
                    )
                );
            }

            self.start_index = start_index;
            self.end_index = end_index;

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

    fn token_to_atp_line(&self) -> String {
        format!("ctr {} {};\n", self.start_index, self.end_index)
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Ctr {
    fn get_opcode(&self) -> u8 {
        0x1c
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Ctr::default().get_opcode() && !instruction.operands.is_empty() {
            let start_index = string_to_usize(&instruction.operands[0])?;
            let end_index = string_to_usize(&instruction.operands[1])?;
            if start_index > end_index {
                return Err(
                    AtpError::new(
                        AtpErrorCode::InvalidIndex(
                            "Start index must be smaller than end index".to_string()
                        ),
                        format!("ctc {} {};", start_index, end_index),
                        format!("Start Index: {}, End Index: {}", start_index, end_index)
                    )
                );
            }

            self.start_index = start_index;
            self.end_index = end_index;

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

    fn token_to_bytecode_instruction(&self) -> crate::bytecode_parser::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Ctr::default().get_opcode(),
            operands: [self.start_index.to_string(), self.end_index.to_string()].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod ctr_tests {
    use crate::{ token_data::TokenMethods, token_data::token_defs::ctr::Ctr };

    #[test]
    fn test_capitalize_range() {
        let mut token = Ctr::params(1, 5).unwrap();

        assert!(
            matches!(Ctr::params(5, 1), Err(_)),
            "it throws an error if start_index is bigger than end_index"
        );

        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if start_index does not exists in input"
        );

        assert_eq!(
            token.parse("banana bananosa bananinha laranjinha vermelhinha azulzinha e fresquinha"),
            Ok(
                "banana Bananosa Bananinha Laranjinha Vermelhinha Azulzinha e fresquinha".to_string()
            )
        );
        assert_eq!(
            token.parse("banana bananosa bananinha laranjinha"),
            Ok("banana Bananosa Bananinha Laranjinha".to_string()),
            "It works with expected inputs"
        );

        assert_eq!(
            token.token_to_atp_line(),
            "ctr 1 5;\n".to_string(),
            "conversion to atp_line works correctly"
        );

        assert_eq!(token.get_string_repr(), "ctr".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.token_from_vec_params(
                    ["ctr".to_string(), (5).to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
        assert!(
            matches!(
                token.token_from_vec_params(
                    ["ctr".to_string(), (1).to_string(), (5).to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn test_capitalize_range_bytecode() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Ctr::params(1, 5).unwrap();

        let mut instruction = BytecodeInstruction {
            op_code: 0x1c,
            operands: [(1).to_string(), (5).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x1c, "get_opcode does not disrepect ATP token mapping");

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

        instruction.operands = [(5).to_string(), (1).to_string()].to_vec();

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
                token.token_from_vec_params(
                    ["ctr".to_string(), (5).to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
    }
}

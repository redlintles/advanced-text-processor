use crate::{ token_data::TokenMethods, utils::transforms::{ capitalize, string_to_usize } };
use crate::utils::errors::{ AtpError, AtpErrorCode };

#[cfg(feature = "bytecode")]
use crate::{ bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods } };
/// Token `Ctc` — Capitalize chunk
///
/// takes a subslice of input and capitalize every word contained in it(inclusive)
/// and then, rebuild the original input containing that subslice
///
/// Words are defined as sequences of characters separated by whitespace,
/// following the behavior of `input.split_whitespace()`.
///
/// If `start_index` is out of bounds for the number of words in the `input``, an `AtpError` is returned.
/// If `end_index` is out of bound for the number of words in the input, it's clamped up to the number of words in `input`
///
/// # Example
///
/// ```rust
/// use atp_project::token_data::{TokenMethods,token_defs::ctc::Ctc};
/// let token = Ctc::params(5,20).unwrap();
/// assert_eq!(token.parse("foo bar mar war"), Ok("foo bAr Mar War".to_string()));
/// ```
///
#[derive(Clone, Default)]
pub struct Ctc {
    pub start_index: usize,
    pub end_index: usize,
}

impl Ctc {
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
        Ok(Ctc {
            start_index,
            end_index,
        })
    }
}

impl TokenMethods for Ctc {
    fn get_string_repr(&self) -> String {
        "ctc".to_string()
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
        let total = input.chars().count();

        if end > total {
            end = input.len();
        }

        let slice: String = (&input[self.start_index..end]).to_string();

        let capitalized_chunk = slice
            .split_whitespace()
            .map(|w| capitalize(w).to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let prefix: String = input.chars().take(self.start_index).collect();
        let suffix: String = input.chars().skip(end).collect();

        let result = format!("{}{}{}", prefix, capitalized_chunk, suffix);

        Ok(result)
    }

    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "ctc" {
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
                crate::utils::errors::AtpErrorCode::TokenNotFound(
                    "Invalid parser for this token".to_string()
                ),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }

    fn token_to_atp_line(&self) -> String {
        format!("ctc {} {};\n", self.start_index, self.end_index)
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Ctc {
    fn get_opcode(&self) -> u8 {
        0x1b
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Ctc::default().get_opcode() && !instruction.operands.is_empty() {
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
            op_code: Ctc::default().get_opcode(),
            operands: [self.start_index.to_string(), self.end_index.to_string()].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod ctc_tests {
    use crate::{ token_data::TokenMethods, token_data::token_defs::ctc::Ctc };

    #[test]
    fn test_capitalize_chunk() {
        let mut token = Ctc::params(1, 5).unwrap();

        assert!(
            matches!(Ctc::params(5, 1), Err(_)),
            "it throws an error if start_index is bigger than end_index"
        );

        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if start_index does not exists in input"
        );

        assert_eq!(
            token.parse("bananabananosa"),
            Ok("bAnanabananosa".to_string()),
            "It works with expected inputs"
        );

        assert_eq!(
            token.token_to_atp_line(),
            "ctc 1 5;\n".to_string(),
            "conversion to atp_line works correctly"
        );

        assert_eq!(token.get_string_repr(), "ctc".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.token_from_vec_params(
                    ["ctc".to_string(), (5).to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
        assert!(
            matches!(
                token.token_from_vec_params(
                    ["ctc".to_string(), (1).to_string(), (5).to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn test_capitalize_chunk_bytecode() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Ctc::params(1, 5).unwrap();

        let mut instruction = BytecodeInstruction {
            op_code: 0x1b,
            operands: [(1).to_string(), (5).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x1b, "get_opcode does not disrepect ATP token mapping");

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
                    ["ctc".to_string(), (5).to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
    }
}

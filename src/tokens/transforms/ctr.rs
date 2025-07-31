use crate::{
    tokens::TokenMethods,
    utils::transforms::{ capitalize, string_to_usize },
    utils::validations::check_chunk_bound_indexes,
};
use crate::utils::errors::{ AtpError, AtpErrorCode };
#[cfg(feature = "bytecode")]
use crate::{ bytecode::{ BytecodeInstruction, BytecodeTokenMethods } };

/// Token `Ctr` — Capitalize Range
///
/// Capitalizes a range of words delimited by `start_index` and `end_index`(inclusive)
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
/// use atp_project::tokens::{TokenMethods,transforms::ctr::Ctr};
/// let token = Ctr::params(1,5).unwrap();
/// assert_eq!(token.parse("foo bar mar"), Ok("foo Bar Mar".to_string()));
/// ```
///
#[derive(Clone, Default)]
pub struct Ctr {
    pub start_index: usize,
    pub end_index: usize,
}

impl Ctr {
    pub fn params(start_index: usize, end_index: usize) -> Result<Self, AtpError> {
        check_chunk_bound_indexes(start_index, end_index, None)?;
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
        check_chunk_bound_indexes(self.start_index, self.end_index, Some(input))?;
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

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "ctr" {
            let start_index = string_to_usize(&line[1])?;
            let end_index = string_to_usize(&line[2])?;
            check_chunk_bound_indexes(start_index, end_index, None)?;

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

    fn to_atp_line(&self) -> String {
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
        instruction: crate::bytecode::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Ctr::default().get_opcode() && !instruction.operands.is_empty() {
            let start_index = string_to_usize(&instruction.operands[0])?;
            let end_index = string_to_usize(&instruction.operands[1])?;
            check_chunk_bound_indexes(start_index, end_index, None)?;

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

    fn token_to_bytecode_instruction(&self) -> crate::bytecode::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Ctr::default().get_opcode(),
            operands: [self.start_index.to_string(), self.end_index.to_string()].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod ctr_tests {
    use crate::{ tokens::TokenMethods, tokens::transforms::ctr::Ctr };

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
            token.to_atp_line(),
            "ctr 1 5;\n".to_string(),
            "conversion to atp_line works correctly"
        );

        assert_eq!(token.get_string_repr(), "ctr".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["ctr".to_string(), (5).to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
        assert!(
            matches!(
                token.from_vec_params(
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
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

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
                token.from_vec_params(
                    ["ctr".to_string(), (5).to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
    }
}

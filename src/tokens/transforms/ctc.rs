use crate::{
    tokens::TokenMethods,
    utils::transforms::{ capitalize, string_to_usize },
    utils::validations::check_chunk_bound_indexes,
};
use crate::utils::errors::{ AtpError };

#[cfg(feature = "bytecode")]
use crate::{
    bytecode::{ BytecodeInstruction, BytecodeTokenMethods },
    utils::errors::AtpErrorCode,
};
/// Token `Ctc` — Capitalize Chunk
///
/// Capitalizes every word in a character slice of the input, defined by `start_index` and `end_index` (inclusive).
///
/// The range is applied directly to the character indices of the original string. The extracted chunk is then split
/// into words (using `split_whitespace()`), capitalized individually, and finally reinserted into the original string.
///
/// - If `start_index` is out of bounds for the number of characters in the input, an `AtpError` is returned.
/// - If `end_index` exceeds the input's length, it will be clamped to the input's character count.
///
/// # Example
///
/// ```rust
/// use atp_project::tokens::{TokenMethods, transforms::ctc::Ctc};
///
/// let token = Ctc::params(1, 5).unwrap();
/// assert_eq!(token.parse("bananabananosa"), Ok("bAnanabananosa".to_string()));
/// ```
#[derive(Clone, Default)]
pub struct Ctc {
    pub start_index: usize,
    pub end_index: usize,
}

impl Ctc {
    pub fn params(start_index: usize, end_index: usize) -> Result<Self, AtpError> {
        check_chunk_bound_indexes(start_index, end_index, None)?;
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
        let total_chars = input.chars().count();

        check_chunk_bound_indexes(self.start_index, self.end_index, Some(input))?;

        let end_index = self.end_index.min(total_chars); // clamp to avoid overflow

        // Convert char indices to byte indices
        let start_byte = input
            .char_indices()
            .nth(self.start_index)
            .map(|(byte_idx, _)| byte_idx)
            .unwrap(); // safe: start_index < total_chars

        let end_byte = if end_index == total_chars {
            input.len() // go to the end
        } else {
            input
                .char_indices()
                .nth(end_index)
                .map(|(byte_idx, _)| byte_idx)
                .unwrap()
        };

        // Extract slice safely
        let slice = &input[start_byte..end_byte];

        // Capitalize all words in the slice
        let capitalized_chunk = slice
            .split_whitespace()
            .map(|w| capitalize(w))
            .collect::<Vec<_>>()
            .join(" ");

        // Rebuild final string
        let prefix = &input[..start_byte];
        let suffix = &input[end_byte..];

        let result = format!("{}{}{}", prefix, capitalized_chunk, suffix);

        Ok(result)
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "ctc" {
            let start_index = string_to_usize(&line[1])?;
            let end_index = string_to_usize(&line[2])?;

            check_chunk_bound_indexes(start_index, end_index, None)?;
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

    fn to_atp_line(&self) -> String {
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
        instruction: crate::bytecode::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Ctc::default().get_opcode() && !instruction.operands.is_empty() {
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
            op_code: Ctc::default().get_opcode(),
            operands: [self.start_index.to_string(), self.end_index.to_string()].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod ctc_tests {
    use crate::{ tokens::TokenMethods, tokens::transforms::ctc::Ctc };

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
            token.to_atp_line(),
            "ctc 1 5;\n".to_string(),
            "conversion to atp_line works correctly"
        );

        assert_eq!(token.get_string_repr(), "ctc".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["ctc".to_string(), (5).to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
        assert!(
            matches!(
                token.from_vec_params(
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
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

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
                token.from_vec_params(
                    ["ctc".to_string(), (5).to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
    }
}

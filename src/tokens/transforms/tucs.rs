use crate::{
    token_data::TokenMethods,
    utils::{
        errors::{ AtpError, AtpErrorCode },
        transforms::string_to_usize,
        validations::{ check_index_against_input, check_vec_len },
    },
};

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

/// TUCS - To Uppercase Single
///
/// Uppercases a single character in `input` identified by `index`
///
/// # Example
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::tucs::Tucs};
///
/// let token = Tucs::params(1);
///
/// assert_eq!(token.parse("banana"), Ok("bAnana".to_string()));
///
/// ```

#[derive(Clone, Copy, Default)]
pub struct Tucs {
    index: usize,
}

impl Tucs {
    pub fn params(index: usize) -> Self {
        Tucs {
            index,
        }
    }
}

impl TokenMethods for Tucs {
    fn get_string_repr(&self) -> String {
        "tucs".to_string()
    }

    fn to_atp_line(&self) -> String {
        format!("tucs {};\n", self.index)
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        check_index_against_input(self.index, input)?;
        let result: String = input
            .char_indices()
            .map(|(i, c)| {
                if i == self.index { c.to_uppercase().to_string() } else { c.to_string() }
            })
            .collect();
        Ok(result)
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        check_vec_len(&line, 2)?;
        if line[0] == "tucs" {
            self.index = string_to_usize(&line[1])?;
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
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Tucs {
    fn get_opcode(&self) -> u8 {
        0x14
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Tucs::default().get_opcode() && instruction.operands.len() == 1 {
            self.index = string_to_usize(&instruction.operands[0])?;
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

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Tucs::default().get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod tucs_tests {
    use crate::token_data::{ TokenMethods, transforms::tucs::Tucs };
    #[test]
    fn to_uppercase_single_tests() {
        let mut token = Tucs::params(1);

        assert_eq!(token.parse("banana"), Ok("bAnana".to_string()));

        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if start_index does not exists in input"
        );

        assert_eq!(
            token.to_atp_line(),
            "tucs 1;\n".to_string(),
            "conversion to atp_line works correctly"
        );

        assert_eq!(
            token.get_string_repr(),
            "tucs".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(["tucs".to_string(), "banana".to_string()].to_vec()),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
        assert!(
            matches!(token.from_vec_params(["tucs".to_string(), (1).to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn to_uppercase_single_bytecode_tests() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Tucs::params(1);

        let mut instruction = BytecodeInstruction {
            op_code: 0x14,
            operands: [(1).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x14, "get_opcode does not disrepect ATP token mapping");

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
                    ["tucs".to_string(), "(".to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid param vec"
        );
    }
}

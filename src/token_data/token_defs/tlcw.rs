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

/// TLCW - To Lowercase Word
///
/// Lowercase a single word of string
///
/// # Example:
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::tlcw::Tlcw};
///
/// let token = Tlcw::params(1);
///
/// assert_eq!(token.parse("BANANA LARANJA CHEIA DE CANJA"), Ok("BANANA laranja CHEIA DE CANJA".to_string()));
///
/// ```
#[derive(Clone, Default, Copy)]
pub struct Tlcw {
    index: usize,
}

impl Tlcw {
    pub fn params(index: usize) -> Self {
        Tlcw { index }
    }
}
impl TokenMethods for Tlcw {
    fn get_string_repr(&self) -> String {
        "tlcw".to_string()
    }

    fn to_atp_line(&self) -> String {
        format!("tlcw {};\n", self.index)
    }

    fn parse(&self, input: &str) -> Result<String, crate::utils::errors::AtpError> {
        check_index_against_input(self.index, input)?;
        Ok(
            input
                .split_whitespace()
                .enumerate()
                .map(|(i, w)| {
                    if i == self.index { w.to_lowercase() } else { w.to_string() }
                })
                .collect::<Vec<_>>()
                .join(" ")
                .to_string()
        )
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), crate::utils::errors::AtpError> {
        check_vec_len(&line, 2)?;
        if line[0] == "tlcw" {
            self.index = string_to_usize(&line[1])?;
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid Parser for this token".to_string()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Tlcw {
    fn get_opcode(&self) -> u8 {
        0x29
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        use crate::utils::validations::check_vec_len;

        check_vec_len(&instruction.operands, 1)?;
        if instruction.op_code == self.get_opcode() {
            use crate::utils::transforms::string_to_usize;

            self.index = string_to_usize(&instruction.operands[0])?;
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".to_string()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }
    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: self.get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod tlcw_tests {
    use crate::token_data::{ TokenMethods, token_defs::tlcw::Tlcw };
    #[test]
    fn to_lowercase_word_tests() {
        let mut token = Tlcw::params(1);

        assert_eq!(
            token.parse("BANANA LARANJA CHEIA DE CANJA"),
            Ok("BANANA laranja CHEIA DE CANJA".to_string())
        );

        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if start_index does not exists in input"
        );

        assert_eq!(
            token.to_atp_line(),
            "tlcw 1;\n".to_string(),
            "conversion to atp_line works correctly"
        );

        assert_eq!(
            token.get_string_repr(),
            "tlcw".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(["tlcw".to_string(), "banana".to_string()].to_vec()),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
        assert!(
            matches!(token.from_vec_params(["tlcw".to_string(), (1).to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn to_lowercase_word_bytecode_tests() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Tlcw::params(1);

        let mut instruction = BytecodeInstruction {
            op_code: 0x29,
            operands: [(1).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x29, "get_opcode does not disrepect ATP token mapping");

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
                    ["tlcw".to_string(), "(".to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid param vec"
        );
    }
}

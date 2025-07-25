use crate::{
    token_data::TokenMethods,
    utils::errors::{ AtpError, AtpErrorCode },
    utils::transforms::string_to_usize,
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
/// use atp_project::token_data::{TokenMethods, token_defs::tucw::Tucw};
///
/// let token = Tucw::params(1);
///
/// assert_eq!(token.parse("banana laranja cheia de canja"), Ok("banana LARANJA cheia de canja".to_string()));
///
/// ```
#[derive(Clone, Default, Copy)]
pub struct Tucw {
    index: usize,
}

impl Tucw {
    pub fn params(index: usize) -> Self {
        Tucw { index }
    }
}
impl TokenMethods for Tucw {
    fn get_string_repr(&self) -> String {
        "tucw".to_string()
    }

    fn token_to_atp_line(&self) -> String {
        format!("tucw {};\n", self.index)
    }

    fn parse(&self, input: &str) -> Result<String, crate::utils::errors::AtpError> {
        if !(0..input.chars().count()).contains(&self.index) {
            return Err(
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        "Index does not exist in the splitted vec".to_string()
                    ),
                    self.token_to_atp_line(),
                    input.to_string()
                )
            );
        }
        Ok(
            input
                .split_whitespace()
                .enumerate()
                .map(|(i, w)| {
                    if i == self.index { w.to_uppercase() } else { w.to_string() }
                })
                .collect::<Vec<_>>()
                .join(" ")
                .to_string()
        )
    }

    fn token_from_vec_params(
        &mut self,
        line: Vec<String>
    ) -> Result<(), crate::utils::errors::AtpError> {
        if line[0] == "tucw" {
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
impl BytecodeTokenMethods for Tucw {
    fn get_opcode(&self) -> u8 {
        0x2a
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Tucw::default().get_opcode() && instruction.operands.len() == 1 {
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
            op_code: Tucw::default().get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod tucw_tests {
    use crate::token_data::{ TokenMethods, token_defs::tucw::Tucw };
    #[test]
    fn to_uppercase_word_tests() {
        let mut token = Tucw::params(1);

        assert_eq!(
            token.parse("banana laranja cheia de canja"),
            Ok("banana LARANJA cheia de canja".to_string())
        );

        assert!(
            matches!(token.parse(""), Err(_)),
            "It throws an error if start_index does not exists in input"
        );

        assert_eq!(
            token.token_to_atp_line(),
            "tucw 1;\n".to_string(),
            "conversion to atp_line works correctly"
        );

        assert_eq!(
            token.get_string_repr(),
            "tucw".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.token_from_vec_params(["tucw".to_string(), "banana".to_string()].to_vec()),
                Err(_)
            ),
            "It throws an error for invalid operands"
        );
        assert!(
            matches!(
                token.token_from_vec_params(["tucw".to_string(), (1).to_string()].to_vec()),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn to_uppercase_word_bytecode_tests() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Tucw::params(1);

        let mut instruction = BytecodeInstruction {
            op_code: 0x2a,
            operands: [(1).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x2a, "get_opcode does not disrepect ATP token mapping");

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
                token.token_from_vec_params(
                    ["tucw".to_string(), "(".to_string(), (1).to_string()].to_vec()
                ),
                Err(_)
            ),
            "It throws an error for invalid param vec"
        );
    }
}

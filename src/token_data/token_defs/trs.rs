use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

// Trim right side
#[derive(Clone, Copy, Default)]
pub struct Trs {}

impl TokenMethods for Trs {
    fn token_to_atp_line(&self) -> String {
        "trs;\n".to_string()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(String::from(input.trim_end()))
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "trs;"

        if line[0] == "trs" {
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

    fn get_string_repr(&self) -> String {
        "trs".to_string()
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Trs {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        use crate::utils::errors::AtpErrorCode;

        if instruction.op_code == Trs::default().get_opcode() {
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
            op_code: Trs::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x07
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod trs_tests {
    #[test]
    fn test_trim_right_side() {
        use crate::token_data::{ token_defs::trs::Trs, TokenMethods };
        use rand::Rng;
        let mut token = Trs::default();

        let mut rng = rand::rng();

        let random_number: usize = rng.random_range(0..100);
        let spaces = " ".repeat(random_number);
        let mut text = String::from("banana");

        text.push_str(&spaces);

        assert_eq!(token.parse("banana     "), Ok("banana".to_string()));
        assert_eq!(token.parse(&text), Ok("banana".to_string()));
        assert_eq!(token.token_to_atp_line(), "trs;\n".to_string());
        assert_eq!(token.get_string_repr(), "trs".to_string());
        assert!(matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)));
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_bytecode_trim_right_side() {
        use crate::token_data::{ token_defs::trs::Trs };
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Trs::default();

        let instruction = BytecodeInstruction {
            op_code: 0x07,
            operands: [].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x07);

        assert_eq!(token.token_from_bytecode_instruction(instruction.clone()), Ok(()));

        assert_eq!(token.token_to_bytecode_instruction(), instruction);
    }
}

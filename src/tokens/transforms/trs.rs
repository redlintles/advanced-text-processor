use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

/// TLS - Trim left sides
///
/// Trim the right side of `input`, removing all spaces from the end
///
/// # Example:
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::trs::Trs};
///
/// let token = Trs::default();
///
/// assert_eq!(token.parse("   banana   "), Ok("   banana".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Trs {}

impl TokenMethods for Trs {
    fn to_atp_line(&self) -> String {
        "trs;\n".to_string()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(String::from(input.trim_end()))
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
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
        use crate::token_data::{ transforms::trs::Trs, TokenMethods };
        use rand::Rng;
        let mut token = Trs::default();

        let mut rng = rand::rng();

        let random_number: usize = rng.random_range(0..100);
        let spaces = " ".repeat(random_number);
        let mut text = String::from("banana");

        text.push_str(&spaces);

        assert_eq!(
            token.parse("banana     "),
            Ok("banana".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(token.parse(&text), Ok("banana".to_string()), "It supports random inputs");
        assert_eq!(
            token.to_atp_line(),
            "trs;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "trs".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["trs".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_bytecode_trim_right_side() {
        use crate::token_data::{ transforms::trs::Trs };
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Trs::default();

        let instruction = BytecodeInstruction {
            op_code: 0x07,
            operands: [].to_vec(),
        };
        assert_eq!(token.get_opcode(), 0x07, "get_opcode does not disrepect ATP token mapping");

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

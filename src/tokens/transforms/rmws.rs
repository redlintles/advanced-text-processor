use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

/// RMWS - Remove Whitespace
///
/// Removes all whitespaces in `input`
///
/// # Example
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::rmws::Rmws};
///
/// let token = Rmws::default();
///
/// assert_eq!(token.parse("banana laranja cheia de canja"), Ok("bananalaranjacheiadecanja".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Rmws {}

impl TokenMethods for Rmws {
    fn get_string_repr(&self) -> String {
        "rmws".to_string()
    }
    fn to_atp_line(&self) -> String {
        "rmws;\n".to_string()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(input.split_whitespace().collect::<Vec<_>>().join(""))
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "rmws" {
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
impl BytecodeTokenMethods for Rmws {
    fn get_opcode(&self) -> u8 {
        0x31
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == self.get_opcode() {
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
        BytecodeInstruction { op_code: self.get_opcode(), operands: [].to_vec() }
    }
}
#[cfg(feature = "test_access")]
#[cfg(test)]
mod rmws_tests {
    use crate::token_data::{ TokenMethods, transforms::rmws::Rmws };
    #[test]
    fn remove_whitespace_tests() {
        let mut token = Rmws::default();

        assert_eq!(
            token.parse("banana laranja cheia de canja"),
            Ok("bananalaranjacheiadecanja".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "rmws;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "rmws".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["rmws".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn remove_whitespace_bytecode_tests() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Rmws::default();

        let instruction = BytecodeInstruction {
            op_code: 0x31,
            operands: [].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x31, "get_opcode does not disrepect ATP token mapping");

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

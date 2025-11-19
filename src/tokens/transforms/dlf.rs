use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };
/// DLF - Delete First
///
/// Deletes the first character of `input`
///
/// # Example
///
/// ```rust
/// use atp_project::tokens::{TokenMethods, transforms::dlf::Dlf};
///
/// let token = Dlf::default();
///
/// assert_eq!(token.parse("banana"), Ok("anana".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Dlf {}

impl TokenMethods for Dlf {
    fn to_atp_line(&self) -> String {
        "dlf;\n".to_string()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let mut s = String::from(input);
        s.drain(..1);
        Ok(s)
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "dlf;"

        if line[0] == "dlf" {
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

    fn get_string_repr(&self) -> &'static str {
        "dlf"
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Dlf {
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Dlf::default().get_opcode() {
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
            op_code: Dlf::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
    fn get_opcode(&self) -> u8 {
        0x03
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod dlf_tests {
    use crate::tokens::{ transforms::dlf::Dlf, TokenMethods };

    #[test]
    fn test_delete_first() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut expected_output = random_text.clone();

        expected_output.drain(..1);

        let mut token = Dlf::default();

        assert_eq!(
            token.parse(&random_text),
            Ok(expected_output.to_string()),
            "It supports random inputs"
        );
        assert_eq!(token.parse("banana"), Ok("anana".to_string()), "It supports expected inputs");
        assert_eq!(
            token.to_atp_line(),
            "dlf;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "dlf".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["dlf".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_delete_first_bytecode() {
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Dlf::default();

        let instruction = BytecodeInstruction {
            op_code: 0x03,
            operands: [].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x03, "get_opcode does not disrepect ATP token mapping");

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

use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::capitalize },
};

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

/// JPSC - Join to PascalCase
///
/// If `input` is a string whose words are separated by spaces, join `input` as a PascalCase string
///
/// # Example
///
/// ```rust
/// use atp_project::tokens::{TokenMethods, transforms::jpsc::Jpsc};
///
/// let token = Jpsc::default();
///
/// assert_eq!(token.parse("banana laranja cheia de canja"), Ok("BananaLaranjaCheiaDeCanja".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Jpsc {}

impl TokenMethods for Jpsc {
    fn get_string_repr(&self) -> &'static str {
        "jpsc"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        Cow::Borrowed("jpsc;\n")
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let v = input.split_whitespace().collect::<Vec<_>>();

        let processed = v
            .iter()
            .map(|w| { capitalize(w) })
            .collect::<Vec<_>>()
            .join("");

        Ok(processed)
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "jpsc" {
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid Parser for this token".into()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Jpsc {
    fn get_opcode(&self) -> u8 {
        0x2e
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Jpsc::default().get_opcode() {
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }
    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: Jpsc::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod jpsc_tests {
    use crate::tokens::{ TokenMethods, transforms::jpsc::Jpsc };
    #[test]
    fn join_to_pascal_case_tests() {
        let mut token = Jpsc::default();
        assert_eq!(
            token.parse("banana laranja cheia de canja"),
            Ok("BananaLaranjaCheiaDeCanja".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "jpsc;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "jpsc".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["jpsc".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn join_to_pascal_case_bytecode_tests() {
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Jpsc::default();

        let instruction = BytecodeInstruction {
            op_code: 0x2e,
            operands: [].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x2e, "get_opcode does not disrepect ATP token mapping");

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

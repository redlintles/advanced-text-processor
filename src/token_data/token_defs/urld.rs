use crate::{ token_data::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeTokenMethods, BytecodeInstruction };
/// URLD - URL Decode
///
/// Decodes `input` from the URL Encoding Format
///
/// # Example
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::urld::Urld};
///
/// let token = Urld::default();
///
/// assert_eq!(token.parse("banana%20laranja"), Ok("banana laranja".to_string()));
/// ```
///

#[derive(Copy, Clone, Default)]
pub struct Urld {}

impl TokenMethods for Urld {
    fn get_string_repr(&self) -> String {
        "urld".to_string()
    }

    fn to_atp_line(&self) -> String {
        "urld;\n".to_string()
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(
            urlencoding
                ::decode(input)
                .map_err(|_|
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed parsing URL string".to_string()),
                        "urld".to_string(),
                        input.to_string()
                    )
                )?
                .to_string()
        )
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "urld" {
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
impl BytecodeTokenMethods for Urld {
    fn get_opcode(&self) -> u8 {
        0x21
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Urld::default().get_opcode() {
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

    fn token_to_bytecode_instruction(&self) -> crate::bytecode_parser::BytecodeInstruction {
        BytecodeInstruction {
            op_code: Urld::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod urld_tests {
    use crate::token_data::{ token_defs::urld::Urld, TokenMethods };
    #[test]
    fn test_url_decode() {
        let mut token = Urld::default();

        assert_eq!(
            token.parse("banana%20laranja"),
            Ok("banana laranja".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "urld;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "urld".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["urld".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_bytecode_url_decode() {
        use crate::token_data::{ token_defs::urld::Urld };
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Urld::default();

        let instruction = BytecodeInstruction {
            op_code: 0x21,
            operands: [].to_vec(),
        };
        assert_eq!(token.get_opcode(), 0x21, "get_opcode does not disrepect ATP token mapping");

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

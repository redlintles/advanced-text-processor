use crate::{ token_data::TokenMethods };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

use crate::utils::errors::{ AtpError, AtpErrorCode };

#[derive(Clone, Copy, Default)]
pub struct Jsone {}

impl TokenMethods for Jsone {
    fn get_string_repr(&self) -> String {
        "jsone".to_string()
    }
    fn token_to_atp_line(&self) -> String {
        "jsone;\n".to_string()
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "jsone" {
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

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        Ok(
            serde_json
                ::to_string(input)
                .map_err(|_|
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed to serialize to JSON".to_string()),
                        "serde_json::to_string".to_string(),
                        input.to_string()
                    )
                )?
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Jsone {
    fn get_opcode(&self) -> u8 {
        0x26
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Jsone::default().get_opcode() {
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
            op_code: Jsone::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod jsone_tests {
    use crate::{ token_data::{ token_defs::jsone::Jsone, TokenMethods } };

    #[test]
    fn test_json_escape() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut token = Jsone::default();

        assert_eq!(
            token.parse(&random_text),
            Ok(serde_json::to_string(&random_text).unwrap()),
            "It supports random inputs"
        );
        assert_eq!(
            token.parse("banana bananosa"),
            Ok(serde_json::to_string("banana bananosa").unwrap()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.token_to_atp_line(),
            "jsone;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "jsone".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.token_from_vec_params(["jsone".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_json_escape_bytecode() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Jsone::default();

        let instruction = BytecodeInstruction {
            op_code: 0x26,
            operands: [].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x26, "get_opcode does not disrepect ATP token mapping");

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

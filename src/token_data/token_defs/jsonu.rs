use crate::{ token_data::TokenMethods };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

use crate::utils::errors::{ AtpError, AtpErrorCode };

#[derive(Clone, Copy, Default)]
pub struct Jsonu {}

impl TokenMethods for Jsonu {
    fn get_string_repr(&self) -> String {
        "jsonu".to_string()
    }
    fn token_to_atp_line(&self) -> String {
        "jsonu;\n".to_string()
    }
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "jsonu" {
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
                ::from_str::<String>(input)
                .map_err(|_|
                    AtpError::new(
                        AtpErrorCode::TextParsingError("Failed to deserialize to JSON".to_string()),
                        "serde_json::from_str".to_string(),
                        input.to_string()
                    )
                )?
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Jsonu {
    fn get_opcode(&self) -> u8 {
        0x27
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: crate::bytecode_parser::BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Jsonu::default().get_opcode() {
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
            op_code: Jsonu::default().get_opcode(),
            operands: [].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod jsonu_tests {
    use crate::{ token_data::{ token_defs::jsonu::Jsonu, TokenMethods } };
    use crate::utils::errors::{ AtpError, AtpErrorCode };

    #[test]
    fn test_json_decode() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut token = Jsonu::default();

        assert_eq!(
            token.parse(&random_text),
            Err(
                AtpError::new(
                    AtpErrorCode::TextParsingError("Failed to deserialize to JSON".to_string()),
                    "serde_json::from_str".to_string(),
                    random_text.to_string()
                )
            ),
            "It supports random inputs"
        );
        assert_eq!(
            token.parse("banana bananosa"),
            Err(
                AtpError::new(
                    AtpErrorCode::TextParsingError("Failed to deserialize to JSON".to_string()),
                    "serde_json::from_str".to_string(),
                    "banana bananosa".to_string()
                )
            ),
            "It supports expected inputs"
        );
        assert_eq!(
            token.token_to_atp_line(),
            "jsonu;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "jsonu".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.token_from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.token_from_vec_params(["jsonu".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_json_decode_bytecode() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Jsonu::default();

        let instruction = BytecodeInstruction {
            op_code: 0x27,
            operands: [].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x27, "get_opcode does not disrepect ATP token mapping");

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

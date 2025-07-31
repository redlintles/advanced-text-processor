use crate::{
    token_data::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::capitalize },
};

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeTokenMethods, BytecodeInstruction };
/// Token `Cfw` — Capitalize First Word
///
/// Capitalizes the first word of `input`
///
/// # Example
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::cfw::Cfw};
///
/// let token = Cfw::default();
/// assert_eq!(token.parse("foo bar"), Ok("Foo bar".to_string()));
/// ```
#[derive(Copy, Clone, Default)]
pub struct Cfw {}

impl TokenMethods for Cfw {
    fn get_string_repr(&self) -> String {
        "cfw".to_string()
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "cfw" {
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
        Ok(capitalize(input))
    }

    fn to_atp_line(&self) -> String {
        "cfw;\n".to_string()
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Cfw {
    fn get_opcode(&self) -> u8 {
        0x18
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        if instruction.op_code == Cfw::default().get_opcode() {
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
        BytecodeInstruction { op_code: Cfw::default().get_opcode(), operands: [].to_vec() }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod cfw_tests {
    use crate::{
        token_data::{ transforms::cfw::Cfw, TokenMethods },
        utils::transforms::capitalize,
    };

    #[test]
    fn test_capitalize_first_word() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut token = Cfw::default();

        assert_eq!(
            token.parse(&random_text),
            Ok(capitalize(&random_text)),
            "It supports random inputs"
        );
        assert_eq!(token.parse("banana"), Ok("Banana".to_string()), "It supports expected inputs");
        assert_eq!(
            token.to_atp_line(),
            "cfw;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "cfw".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["cfw".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_capitalize_first_word_bytecode() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Cfw::default();

        let instruction = BytecodeInstruction {
            op_code: 0x18,
            operands: [].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x18, "get_opcode does not disrepect ATP token mapping");

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

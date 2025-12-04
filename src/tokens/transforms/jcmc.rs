use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::capitalize },
};

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeTokenMethods };

/// JCMC - Join to Camel Case
///
/// If `input` is a string whose words are separated by spaces, join `input` as a camelCase string
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::jcmc::Jcmc};
///
/// let token = Jcmc::default();
///
/// assert_eq!(token.parse("banana laranja cheia de canja"), Ok("bananaLaranjaCheiaDeCanja".to_string()));
/// ```
///
#[derive(Clone, Copy, Default)]
pub struct Jcmc {}

impl TokenMethods for Jcmc {
    fn get_string_repr(&self) -> &'static str {
        "jcmc"
    }

    fn to_atp_line(&self) -> Cow<'static, str> {
        "jcmc;\n".into()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let v = input.split_whitespace().collect::<Vec<_>>();

        let processed = v
            .iter()
            .enumerate()
            .map(|(i, w)| {
                if i >= 1 { capitalize(w) } else { w.to_string() }
            })
            .collect::<Vec<_>>()
            .join("");

        Ok(processed)
    }

    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "jcmc" {
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
impl BytecodeTokenMethods for Jcmc {
    fn get_opcode(&self) -> u8 {
        0x2d
    }

    fn token_from_bytecode_instruction(&mut self, instruction: Vec<u8>) -> Result<(), AtpError> {
        if instruction[0] == Jcmc::default().get_opcode() {
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                instruction[0].to_string(),
                instruction
                    .iter()
                    .map(|b| b.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        )
    }
    fn token_to_bytecode_instruction(&self) -> Vec<u8> {
        vec![Jcmc::default().get_opcode(), 0]
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod jcmc_tests {
    use crate::tokens::{ TokenMethods, transforms::jcmc::Jcmc };
    #[test]
    fn join_to_camel_case_tests() {
        let mut token = Jcmc::default();
        assert_eq!(
            token.parse("banana laranja cheia de canja"),
            Ok("bananaLaranjaCheiaDeCanja".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "jcmc;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "jcmc".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["jcmc".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn join_to_camel_case_bytecode_tests() {
        use crate::bytecode::{ BytecodeTokenMethods };

        let mut token = Jcmc::default();

        let instruction = vec![0x2d, 0];

        assert_eq!(token.get_opcode(), 0x2d, "get_opcode does not disrepect ATP token mapping");

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

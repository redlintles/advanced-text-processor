#[cfg(feature="test_access")]
pub mod test;

use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::{ utils::params::AtpParamTypes };

/// Token `Atb` — Add to Beginning
///
/// Adds `text` to the beginning of `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::atb::Atb};
///
/// let token = Atb::params("foo");
/// assert_eq!(token.transform(" bar"), Ok("foo bar".to_string()));
/// ```
#[derive(Clone, Default)]
pub struct Atb {
    pub text: String,
}

impl Atb {
    pub fn params(text: &str) -> Self {
        Atb {
            text: text.to_string(),
        }
    }
}

impl TokenMethods for Atb {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("atb {};\n", self.text).into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let mut s = String::from(&self.text);
        s.push_str(input);
        Ok(s)
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "atb;"

        if line[0] == "atb" {
            self.text = line[1].clone();
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Token not recognized".into()),
                self.to_atp_line(),
                line.join(" ")
            )
        )
    }

    fn get_string_repr(&self) -> &'static str {
        "atb"
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x01
    }

    #[cfg(feature = "bytecode")]
    fn from_params(&mut self, instruction: &Vec<AtpParamTypes>) -> Result<(), AtpError> {
        use crate::parse_args;

        if instruction.len() != 1 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );
        }

        self.text = parse_args!(instruction, 0, String, "Text should be of string type");

        Ok(())
    }

    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8> {
        let mut result = Vec::new();

        let instruction_type: u32 = self.get_opcode() as u32;

        let first_param_type: u32 = 0x01;
        let first_param_payload = self.text.as_bytes();
        let first_param_payload_size: u32 = first_param_payload.len() as u32;

        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);
        let instruction_total_size: u64 = 4 + 1 + first_param_total_size;

        // Instruction Total Size
        result.extend_from_slice(&instruction_total_size.to_be_bytes());
        // Instruction Type
        result.extend_from_slice(&instruction_type.to_be_bytes());
        // Param Count
        result.push(1);
        // First Param Total Size
        result.extend_from_slice(&first_param_total_size.to_be_bytes());
        // First Param Type
        result.extend_from_slice(&first_param_type.to_be_bytes());
        // First Param Payload Size
        result.extend_from_slice(&first_param_payload_size.to_be_bytes());
        // First Param Payload
        result.extend_from_slice(&first_param_payload);

        result
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod atb_tests {
    mod test_text_version {
        use crate::{ tokens::TokenMethods, tokens::transforms::atb::Atb };
        #[test]
        fn test_with_inputs() {
            let random_text = random_string::generate(6, ('a'..'z').collect::<String>());
            let token = Atb::params("banana");

            assert_eq!(
                token.transform(&random_text),
                Ok(format!("{}{}", "banana", random_text)),
                "It works with random inputs"
            );

            assert_eq!(
                token.transform("coxinha"),
                Ok("bananacoxinha".to_string()),
                "It works with expected inputs"
            );

            assert_eq!(
                token.transform("bànánà"),
                Ok("bananabànánà".to_string()),
                "It supports utf-8 strings"
            )
        }

        #[test]
        fn test_to_atp_line() {
            let token = Atb::params("banana");

            assert_eq!(
                token.to_atp_line(),
                "atb banana;\n".to_string(),
                "conversion to atp_line works correctly"
            );
        }

        #[test]
        fn test_get_string_repr() {
            let token = Atb::default();
            assert_eq!(
                token.get_string_repr(),
                "atb".to_string(),
                "get_string_repr works as expected"
            );
        }

        #[test]
        fn test_from_vec_params() {
            let mut token = Atb::params("laranja");
            assert!(
                matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
                "It throws an error for invalid vec_params"
            );
            assert!(
                matches!(
                    token.from_vec_params(["atb".to_string(), "banana".to_string()].to_vec()),
                    Ok(_)
                ),
                "It does not throws an error for valid vec_params"
            );

            assert_eq!(
                token.transform("coxinha"),
                Ok("bananacoxinha".to_string()),
                "from_vec_params call fill token params correctly"
            );
        }
    }

    #[cfg(feature = "bytecode")]
    mod test_bytecode_version {
        use crate::{ tokens::{ TokenMethods, transforms::atb::Atb }, utils::params::AtpParamTypes };

        #[test]
        fn test_to_bytecode_instruction() {
            let token = Atb::params("banana");

            let first_param_type: u32 = 0x01;
            let first_param_payload = "banana".as_bytes();
            let first_param_payload_size = first_param_payload.len() as u32;
            let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

            let instruction_type: u32 = 0x01;
            let param_count: u8 = 0x01;

            let instruction_total_size: u64 = 4 + 1 + first_param_total_size;

            let mut expected_output: Vec<u8> = vec![];

            expected_output.extend_from_slice(&instruction_total_size.to_be_bytes());
            expected_output.extend_from_slice(&instruction_type.to_be_bytes());
            expected_output.push(param_count);
            expected_output.extend_from_slice(&first_param_total_size.to_be_bytes());
            expected_output.extend_from_slice(&first_param_type.to_be_bytes());
            expected_output.extend_from_slice(&first_param_payload_size.to_be_bytes());
            expected_output.extend_from_slice(&first_param_payload);

            assert_eq!(
                token.to_bytecode(),
                expected_output,
                "Conversion to bytecode instruction works perfectly!"
            );
        }

        #[test]
        fn test_get_op_code() {
            let token = Atb::default();
            assert_eq!(token.get_opcode(), 0x01, "get_opcode does not disrepect ATP token mapping");
        }

        #[test]
        fn test_from_bytecode_instruction() {
            let mut token = Atb::params("laranja");

            let instruction: Vec<AtpParamTypes> = vec![
                AtpParamTypes::String("laranja".to_string())
            ];

            assert_eq!(
                token.from_params(&instruction),
                Ok(()),
                "Parsing from bytecode to token works correctly!"
            );
            assert_eq!(
                token.transform("coxinha"),
                Ok("bananacoxinha".to_string()),
                "from_bytecode_instruction fills token params correctly"
            );

            // assert!(
            //     matches!(token.token_from_bytecode_instruction(instruction.clone()), Err(_)),
            //     "Throws an error for invalid op_code"
            // );

            // instruction.op_code = 0x01;
            // instruction.operands = [].to_vec();

            // assert!(
            //     matches!(token.token_from_bytecode_instruction(instruction.clone()), Err(_)),
            //     "Throws an error for invalid operands"
            // );
        }
    }
}

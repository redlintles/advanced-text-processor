use std::borrow::Cow;

use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, AtpErrorCode } };

#[cfg(feature = "bytecode")]
use crate::{ utils::params::AtpParamTypes };
/// Token `Ate` — Add to End
///
/// Appends `text` to the end of `input`
///
/// # Example
///
/// ```rust
/// use atp::tokens::{TokenMethods, transforms::ate::Ate};
///
/// let token = Ate::params(" bar");
/// assert_eq!(token.transform("foo"), Ok("foo bar".to_string()));
/// ```

#[derive(Clone, Default)]
pub struct Ate {
    pub text: String,
}

impl Ate {
    pub fn params(text: &str) -> Self {
        Ate {
            text: text.to_string(),
        }
    }
}

impl TokenMethods for Ate {
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("ate {};\n", self.text).into()
    }

    fn transform(&self, input: &str) -> Result<String, AtpError> {
        let mut s = String::from(input);
        s.push_str(&self.text);
        Ok(s)
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        // "ate;"

        if line[0] == "ate" {
            self.text = line[1].clone();
            return Ok(());
        }
        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid parser for this token".into()),
                line.join(" "),
                line.join(" ")
            )
        )
    }

    fn get_string_repr(&self) -> &'static str {
        "ate"
    }

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32 {
        0x02
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
mod ate_tests {
    use crate::{ tokens::TokenMethods, tokens::transforms::ate::Ate };

    #[test]
    fn test_add_to_end() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());
        let mut token = Ate::params("banana");

        assert_eq!(token.transform(&random_text), Ok(format!("{}{}", random_text, "banana")));

        assert_eq!(token.transform("coxinha"), Ok("coxinhabanana".to_string()));

        assert_eq!(
            token.to_atp_line(),
            "ate banana;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "ate".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(["ate".to_string(), "banana".to_string()].to_vec()),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    mod test_bytecode_version {
        use crate::{ tokens::{ TokenMethods, transforms::ate::Ate }, utils::params::AtpParamTypes };

        #[test]
        fn test_to_bytecode_instruction() {
            let token = Ate::params("banana");

            let first_param_type: u32 = 0x01;
            let first_param_payload = "banana".as_bytes();
            let first_param_payload_size = first_param_payload.len() as u32;
            let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

            let instruction_type: u32 = 0x02;
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
            let token = Ate::default();
            assert_eq!(token.get_opcode(), 0x02, "get_opcode does not disrepect ATP token mapping");
        }

        #[test]
        fn test_from_bytecode_instruction() {
            let mut token = Ate::params("laranja");

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

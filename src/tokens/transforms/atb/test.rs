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

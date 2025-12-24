#[cfg(test)]
#[cfg(feature = "test_access")]
mod rlw_tests {
    use crate::tokens::{ TokenMethods, transforms::rlw::Rlw };
    #[test]
    fn replace_last_with_tests() {
        let mut token = Rlw::params("a", "b").unwrap();
        assert_eq!(
            token.transform("aaaaa"),
            Ok("aaaab".to_string()),
            "It supports expected inputs"
        );

        assert_eq!(
            token.to_atp_line(),
            "rlw a b;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "rlw".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["rlw".to_string(), "a".to_string(), "b".to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn replace_all_with_bytecode_tests() {
        use crate::{ utils::params::AtpParamTypes };

        let mut token = Rlw::params("banana", "laranja").unwrap();

        let instruction: Vec<AtpParamTypes> = vec![
            AtpParamTypes::Usize(3),
            AtpParamTypes::String("Banana".to_string())
        ];

        assert_eq!(token.get_opcode(), 0x0b, "get_opcode does not disrepect ATP token mapping");

        let first_param_type: u32 = *&instruction[0].get_param_type_code();
        let first_param_payload = "banana".as_bytes();
        let first_param_payload_size = first_param_payload.len() as u32;
        let first_param_total_size: u64 = 4 + 4 + (first_param_payload_size as u64);

        let second_param_type: u32 = *&instruction[1].get_param_type_code();
        let second_param_payload = "laranja".as_bytes();
        let second_param_payload_size = second_param_payload.len() as u32;
        let second_param_total_size: u64 = 4 + 4 + (second_param_payload_size as u64);

        let instruction_type: u32 = 0x0b;
        let param_count: u8 = 0x02;

        let instruction_total_size: u64 =
            8 + 4 + 1 + first_param_total_size + second_param_total_size;

        let mut expected_output: Vec<u8> = vec![];

        expected_output.extend_from_slice(&instruction_total_size.to_be_bytes());

        expected_output.extend_from_slice(&instruction_type.to_be_bytes());

        expected_output.push(param_count);

        expected_output.extend_from_slice(&first_param_total_size.to_be_bytes());
        expected_output.extend_from_slice(&first_param_type.to_be_bytes());
        expected_output.extend_from_slice(&first_param_payload_size.to_be_bytes());
        expected_output.extend_from_slice(&first_param_payload);

        expected_output.extend_from_slice(&second_param_total_size.to_be_bytes());
        expected_output.extend_from_slice(&second_param_type.to_be_bytes());
        expected_output.extend_from_slice(&second_param_payload_size.to_be_bytes());
        expected_output.extend_from_slice(&second_param_payload);

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            expected_output,
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

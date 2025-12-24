#[cfg(feature = "test_access")]
#[cfg(test)]
mod jkbc_tests {
    use crate::tokens::{ TokenMethods, transforms::jkbc::Jkbc };
    #[test]
    fn join_to_kebab_case_tests() {
        let mut token = Jkbc::default();
        assert_eq!(
            token.transform("banana laranja cheia de canja"),
            Ok("banana-laranja-cheia-de-canja".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "jkbc;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "jkbc".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["jkbc".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn join_to_kebab_case_bytecode_tests() {
        use crate::{ utils::params::AtpParamTypes };

        let mut token = Jkbc::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x2b, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            [
                // Instruction Total size
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d,
                // Instruction Type
                0x00, 0x00, 0x00, 0x2b,
                // Param Count
                0x00,
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

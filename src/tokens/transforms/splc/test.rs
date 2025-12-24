#[cfg(feature = "test_access")]
#[cfg(test)]
mod splc_tests {
    use crate::tokens::{ TokenMethods, transforms::splc::Splc };
    #[test]
    fn split_characters_tests() {
        let mut token = Splc::default();
        assert_eq!(
            token.transform("banana"),
            Ok("b a n a n a".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "splc;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "splc".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["splc".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
        assert_eq!(
            token.transform("banana"),
            Ok("b a n a n a".to_string()),
            "It works correctly after re-parsing"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn split_characters_bytecode_tests() {
        use crate::{ utils::params::AtpParamTypes };

        let mut token = Splc::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x23, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            vec![
                // Instruction total size
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x0d,
                // Instruction Type
                0x00,
                0x00,
                0x00,
                0x23,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

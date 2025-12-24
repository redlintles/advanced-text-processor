#[cfg(feature = "test_access")]
#[cfg(test)]
mod urle_tests {
    use crate::tokens::{ transforms::urle::Urle, TokenMethods };
    #[test]
    fn test_url_encode() {
        let mut token = Urle::default();

        assert_eq!(
            token.transform("banana laranja"),
            Ok("banana%20laranja".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "urle;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "urle".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["urle".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_bytecode_url_encode() {
        use crate::tokens::{ transforms::urle::Urle };

        use crate::utils::params::AtpParamTypes;

        let mut token = Urle::default();

        let instruction: Vec<AtpParamTypes> = vec![];
        assert_eq!(token.get_opcode(), 0x20, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            vec![
                // Instruction Total Size
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
                0x20,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

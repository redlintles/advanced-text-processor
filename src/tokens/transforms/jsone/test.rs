#[cfg(feature = "test_access")]
#[cfg(test)]
mod jsone_tests {
    use crate::{ tokens::{ transforms::jsone::Jsone, TokenMethods } };

    #[test]
    fn test_json_escape() {
        let mut token = Jsone::default();

        let expected_output = "\"{banana: '10'}\"".to_string();

        assert_eq!(
            token.transform("{banana: '10'}"),
            Ok(expected_output),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "jsone;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "jsone".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["jsone".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_json_escape_bytecode() {
        use crate::{ utils::params::AtpParamTypes };

        let mut token = Jsone::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x26, "get_opcode does not disrepect ATP token mapping");

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
                0x26,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

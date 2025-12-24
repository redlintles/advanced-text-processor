#[cfg(feature = "test_access")]
#[cfg(test)]
mod trs_tests {
    #[test]
    fn test_trim_right_side() {
        use crate::tokens::{ transforms::trs::Trs, TokenMethods };
        use rand::Rng;
        let mut token = Trs::default();

        let mut rng = rand::rng();

        let random_number: usize = rng.random_range(0..100);
        let spaces = " ".repeat(random_number);
        let mut text = String::from("banana");

        text.push_str(&spaces);

        assert_eq!(
            token.transform("banana     "),
            Ok("banana".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(token.transform(&text), Ok("banana".to_string()), "It supports random inputs");
        assert_eq!(
            token.to_atp_line(),
            "trs;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "trs".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["trs".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_bytecode_trim_right_side() {
        use crate::tokens::TokenMethods;
        use crate::tokens::{ transforms::trs::Trs };
        use crate::utils::params::AtpParamTypes;

        let mut token = Trs::default();

        let instruction: Vec<AtpParamTypes> = vec![];
        assert_eq!(token.get_opcode(), 0x07, "get_opcode does not disrepect ATP token mapping");

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
                0x07,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

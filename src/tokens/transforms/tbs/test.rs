#[cfg(feature = "test_access")]
#[cfg(test)]
mod tbs_tests {
    #[test]
    fn test_trim_both_sides() {
        use crate::tokens::{ transforms::tbs::Tbs, TokenMethods };
        use rand::Rng;
        let mut token = Tbs::default();

        let mut rng = rand::rng();

        let random_number_1: usize = rng.random_range(0..100);
        let random_number_2: usize = rng.random_range(0..100);
        let spaces_start = " ".repeat(random_number_1);
        let spaces_end = " ".repeat(random_number_2);
        let mut text = String::from("banana");

        text = format!("{}{}{}", spaces_start, text, spaces_end);

        assert_eq!(
            token.transform("     banana  "),
            Ok("banana".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(token.transform(&text), Ok("banana".to_string()), "It supports random inputs");
        assert_eq!(
            token.to_atp_line(),
            "tbs;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "tbs".to_string(), "get_string_repr works correctly");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["tbs".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_bytecode_trim_both_sides() {
        use crate::tokens::TokenMethods;
        use crate::tokens::{ transforms::tbs::Tbs };
        use crate::utils::params::AtpParamTypes;

        let mut token = Tbs::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x05, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            vec![
                // tamanho total da instrução (8 bytes)
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x0d,
                // opcode (4 bytes)
                0x00,
                0x00,
                0x00,
                0x05,
                // número de parâmetros
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

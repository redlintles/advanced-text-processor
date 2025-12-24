#[cfg(feature = "test_access")]
#[cfg(test)]
mod tls_tests {
    #[test]
    fn test_trim_left_side() {
        use crate::tokens::{ transforms::tls::Tls, TokenMethods };
        use rand::Rng;
        let mut token = Tls::default();

        let mut rng = rand::rng();

        let random_number: usize = rng.random_range(0..100);
        let spaces = " ".repeat(random_number);
        let mut text = String::from("banana");

        text = format!("{}{}", spaces, text);

        assert_eq!(
            token.transform("     banana"),
            Ok("banana".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(token.transform(&text), Ok("banana".to_string()));
        assert_eq!(token.to_atp_line(), "tls;\n".to_string(), "It supports random inputs");
        assert_eq!(token.get_string_repr(), "tls".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["tls".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_bytecode_trim_left_side() {
        use crate::tokens::TokenMethods;
        use crate::tokens::{ transforms::tls::Tls };
        use crate::utils::params::AtpParamTypes;

        let mut token = Tls::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x06, "get_opcode does not disrepect ATP token mapping");

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
                0x06,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod tla_tests {
    use crate::tokens::{ transforms::tla::Tla, TokenMethods };
    #[test]
    fn test_to_lowercase_all() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut token = Tla::default();

        assert_eq!(
            token.transform("BANANA"),
            Ok("banana".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.transform(&random_text),
            Ok(random_text.to_lowercase()),
            "It supports random inputs"
        );

        assert_eq!(
            token.to_atp_line(),
            "tla;\n".to_string(),
            "Conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "tla".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["tla".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_to_lowercase_all_bytecode() {
        use crate::{ utils::params::AtpParamTypes };

        let mut token = Tla::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x13, "get_opcode does not disrepect ATP token mapping");

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
                0x13,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

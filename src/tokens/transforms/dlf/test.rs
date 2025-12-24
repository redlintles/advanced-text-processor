#[cfg(feature = "test_access")]
#[cfg(test)]
mod dlf_tests {
    use crate::tokens::{ transforms::dlf::Dlf, TokenMethods };

    #[test]
    fn test_delete_first() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut expected_output = random_text.clone();

        expected_output.drain(..1);

        let mut token = Dlf::default();

        assert_eq!(
            token.transform(&random_text),
            Ok(expected_output.to_string()),
            "It supports random inputs"
        );
        assert_eq!(
            token.transform("banana"),
            Ok("anana".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "dlf;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "dlf".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["dlf".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_delete_first_bytecode() {
        let mut token = Dlf::default();

        let instruction = vec![];

        assert_eq!(token.get_opcode(), 0x03, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            vec![
                // Tamanho total do token
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x0d,
                // TIpo do token
                0x00,
                0x00,
                0x00,
                0x03,
                // Número de parâmetros
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

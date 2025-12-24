#[cfg(feature = "test_access")]
#[cfg(test)]
mod dll_tests {
    use crate::tokens::{ transforms::dll::Dll, TokenMethods };

    #[test]
    fn test_delete_last() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut expected_output = random_text.clone();

        if let Some((x, _)) = expected_output.char_indices().next_back() {
            expected_output.drain(x..);
        }

        let mut token = Dll::default();

        assert_eq!(
            token.transform(&random_text),
            Ok(expected_output.to_string()),
            "It supports random inputs"
        );
        assert_eq!(
            token.transform("banana"),
            Ok("banan".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "dll;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "dll".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["dll".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_delete_last_bytecode() {
        use crate::{ utils::params::AtpParamTypes };

        let mut token = Dll::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x04, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            vec![
                // Tamanho total da instrução
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x0d,
                // Tipo da instrução
                0x00,
                0x00,
                0x00,
                0x04,
                // Número de parâmetros da instrução
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

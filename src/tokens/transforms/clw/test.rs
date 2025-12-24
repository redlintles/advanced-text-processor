#[cfg(feature = "test_access")]
#[cfg(test)]
mod clw_tests {
    use crate::{ tokens::{ transforms::clw::Clw, TokenMethods }, utils::transforms::capitalize };

    #[test]
    fn test_capitalize_last_word() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut token = Clw::default();

        assert_eq!(
            token.transform(&random_text),
            Ok(
                random_text
                    .split_whitespace()
                    .rev()
                    .enumerate()
                    .map(|(i, w)| if i == 0 { capitalize(w) } else { w.to_string() })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            "It supports random inputs"
        );
        assert_eq!(
            token.transform("banana bananosa"),
            Ok("banana Bananosa".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "clw;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "clw".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["clw".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_capitalize_last_word_bytecode() {
        use crate::{ utils::params::AtpParamTypes };

        let mut token = Clw::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x19, "get_opcode does not disrepect ATP token mapping");

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
                0x19,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

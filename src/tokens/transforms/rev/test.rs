#[cfg(feature = "test_access")]
#[cfg(test)]
mod rev_tests {
    use crate::{ tokens::{ transforms::rev::Rev, TokenMethods } };

    #[test]
    fn test_reverse() {
        let random_text = random_string::generate(6, ('a'..'z').collect::<String>());

        let mut token = Rev::default();

        assert_eq!(
            token.transform(&random_text),
            Ok(
                random_text
                    .chars()
                    .rev()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            ),
            "It supports random inputs"
        );
        assert_eq!(
            token.transform("banana"),
            Ok("ananab".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "rev;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "rev".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["rev".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn test_reverse_bytecode() {
        use crate::{ utils::params::AtpParamTypes };

        let mut token = Rev::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x22, "get_opcode does not disrepect ATP token mapping");

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
                0x22,
                // Param Count
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

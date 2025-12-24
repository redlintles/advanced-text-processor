#[cfg(feature = "test_access")]
#[cfg(test)]
mod htmle_tests {
    use crate::tokens::{ TokenMethods, transforms::htmle::Htmle };
    #[test]
    fn html_escape_test() {
        let mut token = Htmle::default();

        assert_eq!(
            token.transform("<div>banana</div>"),
            Ok("&lt;div&gt;banana&lt;/div&gt;".to_string()),
            "It supports expected inputs!"
        );
        assert_eq!(
            token.to_atp_line(),
            "htmle;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "htmle".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["htmle".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn html_escape_bytecode_test() {
        use crate::{ utils::params::AtpParamTypes };

        let mut token = Htmle::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x24, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            vec![
                // Token total size
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x0d,
                // Token Type
                0x00,
                0x00,
                0x00,
                0x24,
                // Token Params
                0x00
            ],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

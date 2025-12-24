#[cfg(feature = "test_access")]
#[cfg(test)]
mod htmlu_tests {
    use crate::tokens::{ TokenMethods, transforms::htmlu::Htmlu };
    #[test]
    fn html_unescape_test() {
        let mut token = Htmlu::default();

        assert_eq!(
            token.transform("&lt;div&gt;banana&lt;/div&gt;"),
            Ok("<div>banana</div>".to_string()),
            "It supports expected inputs"
        );
        assert_eq!(
            token.to_atp_line(),
            "htmlu;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "htmlu".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["htmlu".to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn html_unescape_bytecode_test() {
        use crate::{ utils::params::AtpParamTypes };

        let mut token = Htmlu::default();

        let instruction: Vec<AtpParamTypes> = vec![];

        assert_eq!(token.get_opcode(), 0x25, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.from_params(&instruction),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.to_bytecode(),
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x25, 0x00],
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}

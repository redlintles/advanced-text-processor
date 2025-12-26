// src/tokens/transforms/jkbc/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::transforms::jkbc::Jkbc;
    use crate::tokens::TokenMethods;
    use crate::utils::errors::{ AtpError, AtpErrorCode };

    #[test]
    fn get_string_repr_is_jkbc() {
        let t = Jkbc::default();
        assert_eq!(t.get_string_repr(), "jkbc");
    }

    #[test]
    fn to_atp_line_is_constant() {
        let t = Jkbc::default();
        assert_eq!(t.to_atp_line().as_ref(), "jkbc;\n");
    }

    #[test]
    fn from_vec_params_accepts_jkbc_identifier() {
        let mut t = Jkbc::default();
        let line = vec!["jkbc".to_string()];

        assert_eq!(t.from_vec_params(line), Ok(()));
    }

    #[test]
    fn from_vec_params_rejects_wrong_identifier() {
        let mut t = Jkbc::default();
        let line = vec!["nope".to_string()];

        let got = t.from_vec_params(line.clone());

        let expected = Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid Parser for this token".into()),
                line[0].to_string(),
                line.join(" ")
            )
        );

        assert_eq!(got, expected);
    }

    #[test]
    #[should_panic]
    fn from_vec_params_panics_if_line_is_empty() {
        // acessa line[0] sem checar tamanho
        let mut t = Jkbc::default();
        let line: Vec<String> = vec![];
        let _ = t.from_vec_params(line);
    }

    #[test]
    fn transform_matches_doc_example() {
        let t = Jkbc::default();
        assert_eq!(
            t.transform("banana laranja cheia de canja"),
            Ok("banana-laranja-cheia-de-canja".to_string())
        );
    }

    #[test]
    fn transform_single_word_lowercases() {
        let t = Jkbc::default();
        assert_eq!(t.transform("BaNaNa"), Ok("banana".to_string()));
    }

    #[test]
    fn transform_collapses_whitespace_and_lowercases() {
        let t = Jkbc::default();
        assert_eq!(
            t.transform("  Banana   LARANJA \n Cheia\tDe   Canja  "),
            Ok("banana-laranja-cheia-de-canja".to_string())
        );
    }

    #[test]
    fn transform_empty_string_returns_empty() {
        let t = Jkbc::default();
        assert_eq!(t.transform(""), Ok("".to_string()));
    }

    #[test]
    fn transform_unicode_lowercase_behavior() {
        let t = Jkbc::default();
        // unicode + lowercasing (Rust faz lowercase unicode-aware)
        assert_eq!(t.transform("MAÇÃ COM CANELA"), Ok("maçã-com-canela".to_string()));
    }

    // ============================
    // Bytecode-only tests (separados)
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;
        use crate::utils::errors::AtpErrorCode;
        use crate::utils::params::AtpParamTypes;

        #[test]
        fn get_opcode_is_2b() {
            let t = Jkbc::default();
            assert_eq!(t.get_opcode(), 0x2b);
        }

        #[test]
        fn from_params_accepts_empty_param_list() {
            let mut t = Jkbc::default();
            let params: Vec<AtpParamTypes> = vec![];

            assert_eq!(t.from_params(&params), Ok(()));
        }

        #[test]
        fn from_params_rejects_any_params() {
            let mut t = Jkbc::default();
            let params = vec![AtpParamTypes::Usize(1)];

            let got = t.from_params(&params);

            let expected = Err(
                crate::utils::errors::AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );

            assert_eq!(got, expected);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_no_params() {
            let t = Jkbc::default();
            let bc = t.to_bytecode();

            // header mínimo: 8 + 4 + 1 = 13
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;

            // total_size = tamanho do "body" (opcode+count+params...)
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x2b);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

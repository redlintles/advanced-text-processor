// src/tokens/transforms/jsnc/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::TokenMethods;
    use crate::tokens::transforms::jsnc::Jsnc;

    #[test]
    fn get_string_repr_is_jsnc() {
        let t = Jsnc::default();
        assert_eq!(t.get_string_repr(), "jsnc");
    }

    #[test]
    fn to_atp_line_is_constant() {
        let t = Jsnc::default();
        assert_eq!(t.to_atp_line().as_ref(), "jsnc;\n");
    }

    #[test]
    fn transform_matches_doc_example() {
        let t = Jsnc::default();
        assert_eq!(
            t.transform("banana laranja cheia de canja"),
            Ok("banana_laranja_cheia_de_canja".to_string())
        );
    }

    #[test]
    fn transform_single_word_lowercases() {
        let t = Jsnc::default();
        assert_eq!(t.transform("Banana"), Ok("banana".to_string()));
    }

    #[test]
    fn transform_collapses_whitespace() {
        let t = Jsnc::default();
        assert_eq!(
            t.transform("  banana   laranja \n cheia\tde   canja  "),
            Ok("banana_laranja_cheia_de_canja".to_string())
        );
    }

    #[test]
    fn transform_empty_string_returns_empty() {
        let t = Jsnc::default();
        assert_eq!(t.transform(""), Ok("".to_string()));
    }

    #[test]
    fn transform_unicode_preserved_and_lowercased() {
        let t = Jsnc::default();
        assert_eq!(t.transform("Maçã Com Canela"), Ok("maçã_com_canela".to_string()));
    }

    // ============================
    // Bytecode-only tests
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;
        use crate::utils::errors::AtpErrorCode;
        use crate::utils::params::AtpParamTypes;

        #[test]
        fn get_opcode_is_2c() {
            let t = Jsnc::default();
            assert_eq!(t.get_opcode(), 0x2c);
        }

        #[test]
        fn from_params_accepts_empty_param_list() {
            let mut t = Jsnc::default();
            let params: Vec<AtpParamTypes> = vec![];

            assert_eq!(t.from_params(&params), Ok(()));
        }

        #[test]
        fn from_params_rejects_any_params() {
            let mut t = Jsnc::default();
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
            let t = Jsnc::default();
            let bc = t.to_bytecode();

            // header mínimo: 8 (size) + 4 (opcode) + 1 (param count)
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x2c);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

// src/tokens/transforms/jcmc/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::TokenMethods;
    use crate::tokens::transforms::jcmc::Jcmc;
    use crate::utils::errors::AtpErrorCode;
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_jcmc() {
        let t = Jcmc::default();
        assert_eq!(t.get_string_repr(), "jcmc");
    }

    #[test]
    fn to_atp_line_is_constant() {
        let t = Jcmc::default();
        assert_eq!(t.to_atp_line().as_ref(), "jcmc;\n");
    }

    #[test]
    fn transform_matches_doc_example() {
        let t = Jcmc::default();
        assert_eq!(
            t.transform("banana laranja cheia de canja"),
            Ok("bananaLaranjaCheiaDeCanja".to_string())
        );
    }

    #[test]
    fn transform_single_word_is_identity() {
        let t = Jcmc::default();
        assert_eq!(t.transform("banana"), Ok("banana".to_string()));
    }

    #[test]
    fn transform_trims_and_collapses_whitespace() {
        // split_whitespace() colapsa espaços/tabs/newlines
        let t = Jcmc::default();
        assert_eq!(
            t.transform("  banana   laranja \n cheia\tde   canja  "),
            Ok("bananaLaranjaCheiaDeCanja".to_string())
        );
    }

    #[test]
    fn transform_empty_string_returns_empty() {
        let t = Jcmc::default();
        assert_eq!(t.transform(""), Ok("".to_string()));
    }

    #[test]
    fn transform_unicode_preserved() {
        let t = Jcmc::default();
        // depende do capitalize() do seu projeto, mas normalmente:
        // "maçã" -> "Maçã"
        assert_eq!(t.transform("maçã com canela"), Ok("maçãComCanela".to_string()));
    }

    #[test]
    fn from_params_accepts_empty_param_list() {
        let mut t = Jcmc::default();
        let params: Vec<AtpParamTypes> = vec![];

        assert_eq!(t.from_params(&params), Ok(()));
    }

    #[test]
    fn from_params_rejects_any_params() {
        let mut t = Jcmc::default();
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

    // ============================
    // Bytecode-only tests (separados)
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;

        #[test]
        fn get_opcode_is_2d() {
            let t = Jcmc::default();
            assert_eq!(t.get_opcode(), 0x2d);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_no_params() {
            let t = Jcmc::default();
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
            assert_eq!(opcode, 0x2d);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

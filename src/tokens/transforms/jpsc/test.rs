// src/tokens/transforms/jpsc/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::InstructionMethods;
    use crate::tokens::transforms::jpsc::Jpsc;
    use crate::utils::errors::AtpErrorCode;
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_jpsc() {
        let t = Jpsc::default();
        assert_eq!(t.get_string_repr(), "jpsc");
    }

    #[test]
    fn to_atp_line_is_constant() {
        let t = Jpsc::default();
        assert_eq!(t.to_atp_line().as_ref(), "jpsc;\n");
    }

    #[test]
    fn transform_matches_doc_example() {
        let t = Jpsc::default();
        assert_eq!(
            t.transform("banana laranja cheia de canja"),
            Ok("BananaLaranjaCheiaDeCanja".to_string())
        );
    }

    #[test]
    fn transform_single_word_capitalizes() {
        let t = Jpsc::default();
        assert_eq!(t.transform("banana"), Ok("Banana".to_string()));
    }

    #[test]
    fn transform_collapses_whitespace() {
        // split_whitespace() colapsa espaços/tabs/newlines
        let t = Jpsc::default();
        assert_eq!(
            t.transform("  banana   laranja \n cheia\tde   canja  "),
            Ok("BananaLaranjaCheiaDeCanja".to_string())
        );
    }

    #[test]
    fn transform_empty_string_returns_empty() {
        let t = Jpsc::default();
        assert_eq!(t.transform(""), Ok("".to_string()));
    }

    #[test]
    fn transform_unicode_preserved() {
        let t = Jpsc::default();
        // depende do capitalize() do seu projeto; esperado típico:
        assert_eq!(t.transform("maçã com canela"), Ok("MaçãComCanela".to_string()));
    }

    #[test]
    fn from_params_accepts_empty_param_list() {
        let mut t = Jpsc::default();
        let params: Vec<AtpParamTypes> = vec![];

        assert_eq!(t.from_params(&params), Ok(()));
    }

    #[test]
    fn from_params_rejects_any_params() {
        let mut t = Jpsc::default();
        let params = vec![AtpParamTypes::Usize(1)];

        let err = t.from_params(&params).unwrap_err();

        assert!(matches!(err.error_code, AtpErrorCode::InvalidArgumentNumber(_)));
    }

    // ============================
    // Bytecode-only tests (separados)
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;

        #[test]
        fn get_opcode_is_2e() {
            let t = Jpsc::default();
            assert_eq!(t.get_opcode(), 0x2e);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_no_params() {
            let t = Jpsc::default();
            let bc = t.to_bytecode();

            // header mínimo: 8 + 4 + 1 = 13
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x2e);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

// src/tokens/transforms/rnw/test.rs

#[cfg(test)]
mod tests {
    use crate::context::execution_context::GlobalExecutionContext;
    use crate::tokens::InstructionMethods;
    use crate::tokens::transforms::rnw::Rnw;
    use crate::utils::errors::{ AtpErrorCode };
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_rnw() {
        let t = Rnw::default();
        assert_eq!(t.get_string_repr(), "rnw");
    }

    #[test]
    fn params_creates_valid_regex_and_fields() {
        let t = Rnw::params("a+", "b", 2).unwrap();
        assert_eq!(t.pattern.as_str(), "a+");
        assert_eq!(t.text_to_replace, "b".to_string());
        assert_eq!(t.index, 2);
    }

    #[test]
    fn params_rejects_invalid_regex() {
        let err = Rnw::params("(", "b", 0).unwrap_err();
        assert!(!err.is_empty());
    }

    #[test]
    fn to_atp_line_contains_pattern_replacement_and_index() {
        let t = Rnw::params("a+", "b", 2).unwrap();
        let line = t.to_atp_line();
        assert_eq!(line.as_ref(), "rnw a+ b 2;\n");
    }

    #[test]
    fn transform_replaces_nth_occurrence_doc_example_zero_based() {
        // No código, index é 0-based:
        // input "aaaaa" com pattern "a" tem matches: [0],[1],[2],[3],[4]
        // index=2 => troca o 3º 'a'
        let t = Rnw::params("a", "b", 2).unwrap();
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("aaaaa", &mut ctx), Ok("aabaa".to_string()));
    }

    #[test]
    fn transform_index_0_replaces_first_occurrence() {
        let t = Rnw::params("a", "b", 0).unwrap();
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("aaaaa", &mut ctx), Ok("baaaa".to_string()));
    }

    #[test]
    fn transform_large_index_no_match_returns_original() {
        let t = Rnw::params("a", "b", 999).unwrap();
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("aaaaa", &mut ctx), Ok("aaaaa".to_string()));
    }

    #[test]
    fn transform_when_pattern_not_found_returns_original() {
        let t = Rnw::params("z", "b", 0).unwrap();
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("aaaaa", &mut ctx), Ok("aaaaa".to_string()));
    }

    #[test]
    fn transform_replaces_correct_nth_for_multi_length_matches() {
        // matches "aa" in "aaaaaa": positions (0..2), (2..4), (4..6)
        // index=1 troca o segundo "aa"
        let t = Rnw::params("aa", "X", 1).unwrap();
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("aaaaaa", &mut ctx), Ok("aaXaa".to_string()));
    }

    #[test]
    fn transform_handles_utf8_safely() {
        // troca a 2ª ocorrência de "ã" (0-based index=1)
        let t = Rnw::params("ã", "A", 1).unwrap();
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("maçã maçã", &mut ctx), Ok("maçã maçA".to_string()));
    }

    #[test]
    fn transform_handles_zero_length_matches_without_crash() {
        // regex vazio costuma casar em "bordas" (inclusive fim),
        // o importante aqui é: não crashar e produzir algo determinístico.
        //
        // Para input "ab", matches vazios em posições 0,1,2 (dependendo do motor)
        // Vamos só testar um caso simples e estável: substituir o primeiro match (index 0)
        // normalmente insere no começo.
        let t = Rnw::params("", "X", 0).unwrap();
        let mut ctx = GlobalExecutionContext::new();

        let out = t.transform("ab", &mut ctx).unwrap();
        assert!(out.starts_with('X'));
    }

    #[test]
    fn from_params_parses_three_params() {
        let mut t = Rnw::default();

        let params = vec![
            AtpParamTypes::String("a+".to_string()),
            AtpParamTypes::String("b".to_string()),
            AtpParamTypes::Usize(2)
        ];

        assert_eq!(t.from_params(&params), Ok(()));
        assert_eq!(t.pattern.as_str(), "a+");
        assert_eq!(t.text_to_replace, "b".to_string());
        assert_eq!(t.index, 2);
    }

    #[test]
    fn from_params_rejects_wrong_param_count() {
        let mut t = Rnw::default();

        let params = vec![
            AtpParamTypes::String("a+".to_string()),
            AtpParamTypes::String("b".to_string())
        ];

        let err = t.from_params(&params).unwrap_err();

        assert!(matches!(err.error_code, AtpErrorCode::InvalidArgumentNumber(_)));
    }

    #[test]
    fn from_params_rejects_wrong_types() {
        let mut t = Rnw::default();

        let params = vec![
            AtpParamTypes::Usize(7), // deveria ser String(pattern)
            AtpParamTypes::String("b".to_string()),
            AtpParamTypes::Usize(2)
        ];

        let got = t.from_params(&params);

        let expected = Err(
            crate::utils::errors::AtpError::new(
                AtpErrorCode::InvalidParameters("Pattern should be of string type".into()),
                "",
                ""
            )
        );

        assert_eq!(got, expected);
    }

    #[test]
    fn from_params_rejects_invalid_regex_payload() {
        let mut t = Rnw::default();

        let params = vec![
            AtpParamTypes::String("(".to_string()),
            AtpParamTypes::String("b".to_string()),
            AtpParamTypes::Usize(2)
        ];

        let got = t.from_params(&params);

        let expected = Err(
            crate::utils::errors::AtpError::new(
                AtpErrorCode::TextParsingError("Failed to create regex".into()),
                "sslt",
                "(".to_string()
            )
        );

        assert_eq!(got, expected);
    }

    // ============================
    // Bytecode-only tests
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;

        #[test]
        fn get_opcode_is_0x1f() {
            let t = Rnw::default();
            assert_eq!(t.get_opcode(), 0x1f);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_three_params() {
            let t = Rnw::params("a+", "b", 2).unwrap();
            let bc = t.to_bytecode();

            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x1f);

            let param_count = bc[i] as usize;
            i += 1;
            assert_eq!(param_count, 3);

            // Param 1: String("a+")
            let _p1_total = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            let p1_type = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            let p1_payload_size = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap()) as usize;
            i += 4;
            assert_eq!(p1_type, 0x01);
            let p1_payload = &bc[i..i + p1_payload_size];
            i += p1_payload_size;
            assert_eq!(std::str::from_utf8(p1_payload).unwrap(), "a+");

            // Param 2: String("b")
            let _p2_total = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            let p2_type = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            let p2_payload_size = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap()) as usize;
            i += 4;
            assert_eq!(p2_type, 0x01);
            let p2_payload = &bc[i..i + p2_payload_size];
            i += p2_payload_size;
            assert_eq!(std::str::from_utf8(p2_payload).unwrap(), "b");

            // Param 3: Usize(2)
            let _p3_total = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            let p3_type = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            let p3_payload_size = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap()) as usize;
            i += 4;
            assert_eq!(p3_type, 0x02);
            assert_eq!(p3_payload_size, 8);
            let p3_payload = &bc[i..i + p3_payload_size];
            i += p3_payload_size;
            let val = u64::from_be_bytes(p3_payload.try_into().unwrap());
            assert_eq!(val, 2);
            assert_eq!(i, i); // Anti unused var warning, fix this for testing purposes on the bytecode size
        }
    }
}

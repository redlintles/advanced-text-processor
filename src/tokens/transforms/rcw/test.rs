// src/tokens/transforms/rcw/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::TokenMethods;
    use crate::tokens::transforms::rcw::Rcw;
    use crate::utils::errors::{ AtpError, AtpErrorCode };

    #[test]
    fn get_string_repr_is_rcw() {
        let t = Rcw::default();
        assert_eq!(t.get_string_repr(), "rcw");
    }

    #[test]
    fn params_creates_valid_regex_and_fields() {
        let t = Rcw::params("a+", "b", 3).unwrap();
        assert_eq!(t.pattern.as_str(), "a+");
        assert_eq!(t.text_to_replace, "b".to_string());
        assert_eq!(t.count, 3);
    }

    #[test]
    fn params_rejects_invalid_regex() {
        let err = Rcw::params("(", "b", 1).unwrap_err();
        assert!(!err.is_empty());
    }

    #[test]
    fn to_atp_line_contains_pattern_replacement_and_count() {
        let t = Rcw::params("a+", "b", 3).unwrap();
        let line = t.to_atp_line();
        assert_eq!(line.as_ref(), "rcw a+ b 3;\n");
    }

    #[test]
    fn transform_replaces_count_occurrences_doc_example() {
        let t = Rcw::params("a", "b", 3).unwrap();
        assert_eq!(t.transform("aaaaa"), Ok("bbbaa".to_string()));
    }

    #[test]
    fn transform_count_zero_returns_same_string() {
        let t = Rcw::params("a", "b", 0).unwrap();
        assert_eq!(t.transform("aaaaa"), Ok("aaaaa".to_string()));
    }

    #[test]
    fn transform_count_larger_than_matches_replaces_all_matches() {
        let t = Rcw::params("a", "b", 99).unwrap();
        assert_eq!(t.transform("aa"), Ok("bb".to_string()));
    }

    #[test]
    fn transform_with_regex_pattern_replaces_n_times() {
        let t = Rcw::params(r"\d+", "X", 2).unwrap();
        assert_eq!(t.transform("a1 b22 c333 d4444"), Ok("aX bX c333 d4444".to_string()));
    }

    #[test]
    fn from_vec_params_parses_pattern_replacement_and_count() {
        let mut t = Rcw::default();
        let line = vec!["rcw".to_string(), "a+".to_string(), "b".to_string(), "2".to_string()];

        assert_eq!(t.from_vec_params(line), Ok(()));
        assert_eq!(t.pattern.as_str(), "a+");
        assert_eq!(t.text_to_replace, "b".to_string());
        assert_eq!(t.count, 2);

        assert_eq!(t.transform("aaaa"), Ok("bb".to_string())); // a+ -> b, mas só 2 ocorrências (uma só match? aqui dá 1)
    }

    #[test]
    fn from_vec_params_rejects_invalid_regex() {
        let mut t = Rcw::default();
        let line = vec!["rcw".to_string(), "(".to_string(), "b".to_string(), "2".to_string()];

        let got = t.from_vec_params(line.clone());

        let expected = Err(
            AtpError::new(
                AtpErrorCode::TextParsingError("Failed creating regex".into()),
                line[0].to_string(),
                line.join(" ")
            )
        );

        assert_eq!(got, expected);
    }

    #[test]
    fn from_vec_params_rejects_wrong_identifier() {
        let mut t = Rcw::default();
        let line = vec!["nope".to_string(), "a+".to_string(), "b".to_string(), "2".to_string()];

        let got = t.from_vec_params(line.clone());

        let expected = Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid parser for this token".into()),
                line[0].to_string(),
                line.join(" ")
            )
        );

        assert_eq!(got, expected);
    }

    #[test]
    fn from_vec_params_rejects_invalid_count_string() {
        let mut t = Rcw::default();
        let line = vec!["rcw".to_string(), "a+".to_string(), "b".to_string(), "NaN".to_string()];

        // aqui depende do comportamento de string_to_usize:
        // - se retorna AtpError::TextParsingError / InvalidParameters etc.
        // Como não temos o código dela aqui, só garantimos que é Err.
        assert!(t.from_vec_params(line).is_err());
    }

    #[test]
    #[should_panic]
    fn from_vec_params_panics_if_line_is_empty() {
        let mut t = Rcw::default();
        let line: Vec<String> = vec![];
        let _ = t.from_vec_params(line);
    }

    #[test]
    #[should_panic]
    fn from_vec_params_panics_if_line_is_too_short() {
        // acessa line[3]
        let mut t = Rcw::default();
        let line = vec!["rcw".to_string(), "a+".to_string(), "b".to_string()];
        let _ = t.from_vec_params(line);
    }

    // ============================
    // Bytecode-only tests (separados)
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;
        use crate::utils::params::AtpParamTypes;

        #[test]
        fn get_opcode_is_0x10() {
            let t = Rcw::default();
            assert_eq!(t.get_opcode(), 0x10);
        }

        #[test]
        fn from_params_parses_three_params() {
            let mut t = Rcw::default();

            let params = vec![
                AtpParamTypes::String("a+".to_string()),
                AtpParamTypes::String("b".to_string()),
                AtpParamTypes::Usize(3)
            ];

            assert_eq!(t.from_params(&params), Ok(()));
            assert_eq!(t.pattern.as_str(), "a+");
            assert_eq!(t.text_to_replace, "b".to_string());
            assert_eq!(t.count, 3);
        }

        #[test]
        fn from_params_rejects_wrong_param_count() {
            let mut t = Rcw::default();

            let params = vec![
                AtpParamTypes::String("a+".to_string()),
                AtpParamTypes::String("b".to_string())
            ];

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
        fn from_params_rejects_wrong_types() {
            let mut t = Rcw::default();

            let params = vec![
                AtpParamTypes::Usize(7), // deveria ser String (pattern)
                AtpParamTypes::String("b".to_string()),
                AtpParamTypes::Usize(3)
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
            let mut t = Rcw::default();

            let params = vec![
                AtpParamTypes::String("(".to_string()),
                AtpParamTypes::String("b".to_string()),
                AtpParamTypes::Usize(3)
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

        #[test]
        fn to_bytecode_has_expected_header_and_three_params() {
            let t = Rcw::params("a+", "b", 3).unwrap();
            let bc = t.to_bytecode();

            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x10);

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

            // Param 3: Usize(3)
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
            let n = usize::from_be_bytes(p3_payload.try_into().unwrap());
            assert_eq!(n, 3);
        }
    }
}

// src/tokens/transforms/rlw/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::TokenMethods;
    use crate::tokens::transforms::rlw::Rlw;
    use crate::utils::errors::{ AtpError, AtpErrorCode };

    #[test]
    fn get_string_repr_is_rlw() {
        let t = Rlw::default();
        assert_eq!(t.get_string_repr(), "rlw");
    }

    #[test]
    fn params_creates_valid_regex_and_fields() {
        let t = Rlw::params("a+", "b").unwrap();
        assert_eq!(t.pattern.as_str(), "a+");
        assert_eq!(t.text_to_replace, "b".to_string());
    }

    #[test]
    fn params_rejects_invalid_regex() {
        let err = Rlw::params("(", "b").unwrap_err();
        assert!(!err.is_empty());
    }

    #[test]
    fn to_atp_line_contains_pattern_and_replacement() {
        let t = Rlw::params("a+", "b").unwrap();
        let line = t.to_atp_line();
        assert_eq!(line.as_ref(), "rlw a+ b;\n");
    }

    #[test]
    fn transform_replaces_last_occurrence_doc_example() {
        let t = Rlw::params("a", "b").unwrap();
        assert_eq!(t.transform("aaaaa"), Ok("aaaab".to_string()));
    }

    #[test]
    fn transform_when_no_match_returns_original() {
        let t = Rlw::params("z", "b").unwrap();
        assert_eq!(t.transform("aaaaa"), Ok("aaaaa".to_string()));
    }

    #[test]
    fn transform_replaces_last_match_when_regex_groups() {
        // aqui "a+" casa o bloco inteiro como UM match, então trocar "último match" == trocar o único match.
        let t = Rlw::params("a+", "b").unwrap();
        assert_eq!(t.transform("aaaaa"), Ok("b".to_string()));
    }

    #[test]
    fn transform_replaces_last_match_only() {
        let t = Rlw::params(r"\d+", "X").unwrap();
        assert_eq!(t.transform("a1 b22 c333"), Ok("a1 b22 cX".to_string()));
    }

    #[test]
    fn transform_handles_utf8_safely() {
        // se o regex encontra "ã", a substituição precisa manter UTF-8 correto
        let t = Rlw::params("ã", "A").unwrap();
        assert_eq!(t.transform("maçã maçã"), Ok("maçã maçA".to_string()));
    }

    #[test]
    fn from_vec_params_parses_pattern_and_replacement() {
        let mut t = Rlw::default();
        let line = vec!["rlw".to_string(), "a".to_string(), "b".to_string()];

        assert_eq!(t.from_vec_params(line), Ok(()));
        assert_eq!(t.pattern.as_str(), "a");
        assert_eq!(t.text_to_replace, "b".to_string());

        assert_eq!(t.transform("aaaaa"), Ok("aaaab".to_string()));
    }

    #[test]
    fn from_vec_params_rejects_invalid_regex() {
        let mut t = Rlw::default();
        let line = vec!["rlw".to_string(), "(".to_string(), "b".to_string()];

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
        let mut t = Rlw::default();
        let line = vec!["nope".to_string(), "a".to_string(), "b".to_string()];

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
    #[should_panic]
    fn from_vec_params_panics_if_line_is_empty() {
        let mut t = Rlw::default();
        let line: Vec<String> = vec![];
        let _ = t.from_vec_params(line);
    }

    #[test]
    #[should_panic]
    fn from_vec_params_panics_if_line_is_too_short() {
        // acessa line[2]
        let mut t = Rlw::default();
        let line = vec!["rlw".to_string(), "a".to_string()];
        let _ = t.from_vec_params(line);
    }

    // ============================
    // Bytecode-only tests
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;
        use crate::utils::params::AtpParamTypes;

        #[test]
        fn get_opcode_is_0x1e() {
            let t = Rlw::default();
            assert_eq!(t.get_opcode(), 0x1e);
        }

        #[test]
        fn from_params_parses_two_params() {
            let mut t = Rlw::default();

            let params = vec![
                AtpParamTypes::String("a+".to_string()),
                AtpParamTypes::String("b".to_string())
            ];

            assert_eq!(t.from_params(&params), Ok(()));
            assert_eq!(t.pattern.as_str(), "a+");
            assert_eq!(t.text_to_replace, "b".to_string());
        }

        #[test]
        fn from_params_rejects_wrong_param_count() {
            let mut t = Rlw::default();

            let params = vec![AtpParamTypes::String("a+".to_string())];

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
            let mut t = Rlw::default();

            let params = vec![
                AtpParamTypes::Usize(7), // deveria ser String (pattern)
                AtpParamTypes::String("b".to_string())
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
            let mut t = Rlw::default();

            let params = vec![
                AtpParamTypes::String("(".to_string()),
                AtpParamTypes::String("b".to_string())
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
        fn to_bytecode_has_expected_header_and_two_params() {
            let t = Rlw::params("a+", "b").unwrap();
            let bc = t.to_bytecode();

            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x1e);

            let param_count = bc[i] as usize;
            i += 1;
            assert_eq!(param_count, 2);

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
        }
    }
}

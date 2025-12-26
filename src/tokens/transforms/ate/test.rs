// src/tokens/transforms/ate/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::transforms::ate::Ate;
    use crate::tokens::TokenMethods;
    use crate::utils::errors::{ AtpError, AtpErrorCode };

    #[test]
    fn params_sets_text() {
        let t = Ate::params(" bar");
        assert_eq!(t.text, " bar".to_string());
    }

    #[test]
    fn get_string_repr_is_ate() {
        let t = Ate::default();
        assert_eq!(t.get_string_repr(), "ate");
    }

    #[test]
    fn to_atp_line_formats_correctly() {
        let t = Ate::params("xyz");
        assert_eq!(t.to_atp_line().as_ref(), "ate xyz;\n");
    }

    #[test]
    fn transform_appends_text() {
        let t = Ate::params(" bar");
        assert_eq!(t.transform("foo"), Ok("foo bar".to_string()));
    }

    #[test]
    fn transform_with_empty_text_keeps_input() {
        let t = Ate::params("");
        assert_eq!(t.transform("foo"), Ok("foo".to_string()));
    }

    #[test]
    fn transform_with_empty_input_returns_only_text() {
        let t = Ate::params("bar");
        assert_eq!(t.transform(""), Ok("bar".to_string()));
    }

    #[test]
    fn from_vec_params_parses_ok_when_identifier_matches() {
        let mut t = Ate::default();
        let line = vec!["ate".to_string(), " bar".to_string()];

        assert_eq!(t.from_vec_params(line), Ok(()));
        assert_eq!(t.text, " bar".to_string());
    }

    #[test]
    fn from_vec_params_returns_token_not_found_when_identifier_differs() {
        let mut t = Ate::default();
        let line = vec!["not_ate".to_string(), " bar".to_string()];

        let got = t.from_vec_params(line.clone());

        let expected = Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid parser for this token".into()),
                line.join(" "),
                line.join(" ")
            )
        );

        assert_eq!(got, expected);
    }

    #[test]
    #[should_panic]
    fn from_vec_params_panics_if_missing_text_param() {
        // do jeito que está hoje, line[1] causa panic se só vier "ate"
        let mut t = Ate::default();
        let line = vec!["ate".to_string()];
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
        fn get_opcode_is_02() {
            let t = Ate::default();
            assert_eq!(t.get_opcode(), 0x02);
        }

        #[test]
        fn from_params_rejects_wrong_param_count() {
            let mut t = Ate::default();
            let params = vec![
                AtpParamTypes::String("a".to_string()),
                AtpParamTypes::String("b".to_string())
            ];

            let got = t.from_params(&params);

            let expected = Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );

            assert_eq!(got, expected);
        }

        #[test]
        fn from_params_accepts_single_string_param() {
            let mut t = Ate::default();
            let params = vec![AtpParamTypes::String(" bar".to_string())];

            assert_eq!(t.from_params(&params), Ok(()));
            assert_eq!(t.text, " bar".to_string());
        }

        #[test]
        fn from_params_rejects_wrong_param_type() {
            let mut t = Ate::default();
            let params = vec![AtpParamTypes::Usize(123)];

            let got = t.from_params(&params);

            let expected = Err(
                AtpError::new(
                    AtpErrorCode::InvalidParameters("Text should be of string type".into()),
                    "",
                    ""
                )
            );

            assert_eq!(got, expected);
        }

        #[test]
        fn to_bytecode_has_expected_layout_and_decodes_param() {
            // Confere layout gerado por to_bytecode! + write_as_instruction_param:
            // [total_size u64][opcode u32][param_count u8][param...]
            // param = [param_total_size u64][param_type u32][payload_size u32][payload...]
            let t = Ate::params(" bar");
            let bc = t.to_bytecode();

            // Header mínimo: 8 + 4 + 1 = 13 bytes
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;

            // total_size = tamanho do "body" (opcode+count+params...)
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x02);

            let param_count = bc[i] as usize;
            i += 1;
            assert_eq!(param_count, 1);

            // param_total_size inclui os 8 bytes dele mesmo
            let param_total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap()) as usize;
            i += 8;

            // O resto do param tem (param_total_size - 8) bytes
            let param_start = i;
            let param_end = param_start + (param_total_size - 8);

            let param_payload_for_decoder = bc[param_start..param_end].to_vec();

            // Agora dá pra usar seu decoder de param:
            let decoded = AtpParamTypes::from_bytecode(param_payload_for_decoder).unwrap();

            match decoded {
                AtpParamTypes::String(s) => assert_eq!(s, " bar".to_string()),
                _ => panic!("Expected AtpParamTypes::String"),
            }
        }
    }
}

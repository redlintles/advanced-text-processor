// src/tokens/transforms/jsone/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::transforms::jsone::Jsone;
    use crate::tokens::TokenMethods;
    use crate::utils::errors::{ AtpError, AtpErrorCode };

    #[test]
    fn get_string_repr_is_jsone() {
        let t = Jsone::default();
        assert_eq!(t.get_string_repr(), "jsone");
    }

    #[test]
    fn to_atp_line_is_constant() {
        let t = Jsone::default();
        assert_eq!(t.to_atp_line().as_ref(), "jsone;\n");
    }

    #[test]
    fn from_vec_params_accepts_jsone_identifier() {
        let mut t = Jsone::default();
        let line = vec!["jsone".to_string()];

        assert_eq!(t.from_vec_params(line), Ok(()));
    }

    #[test]
    fn from_vec_params_rejects_wrong_identifier() {
        let mut t = Jsone::default();
        let line = vec!["nope".to_string()];

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
        // acessa line[0] sem checar tamanho
        let mut t = Jsone::default();
        let line: Vec<String> = vec![];
        let _ = t.from_vec_params(line);
    }

    #[test]
    fn transform_matches_doc_example() {
        let t = Jsone::default();
        let expected_output = "\"{banana: '10'}\"".to_string();

        assert_eq!(t.transform("{banana: '10'}"), Ok(expected_output));
    }

    #[test]
    fn transform_escapes_quotes_and_backslashes() {
        let t = Jsone::default();

        // input contém aspas e barra
        let input = r#"a "quote" and a \ slash"#;

        // serde_json::to_string gera string JSON com aspas externas e escapes internos
        let expected = "\"a \\\"quote\\\" and a \\\\ slash\"".to_string();

        assert_eq!(t.transform(input), Ok(expected));
    }

    #[test]
    fn transform_escapes_newlines_tabs_and_returns() {
        let t = Jsone::default();

        let input = "line1\nline2\tend\r";
        let expected = "\"line1\\nline2\\tend\\r\"".to_string();

        assert_eq!(t.transform(input), Ok(expected));
    }

    #[test]
    fn transform_empty_string_is_quoted_empty() {
        let t = Jsone::default();
        assert_eq!(t.transform(""), Ok("\"\"".to_string()));
    }

    #[test]
    fn transform_unicode_is_preserved() {
        let t = Jsone::default();

        let input = "maçã 🍎";
        // serde_json mantém unicode (não escapa como \uXXXX por padrão)
        let expected = "\"maçã 🍎\"".to_string();

        assert_eq!(t.transform(input), Ok(expected));
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
        fn get_opcode_is_26() {
            let t = Jsone::default();
            assert_eq!(t.get_opcode(), 0x26);
        }

        #[test]
        fn from_params_accepts_empty_param_list() {
            let mut t = Jsone::default();
            let params: Vec<AtpParamTypes> = vec![];

            assert_eq!(t.from_params(&params), Ok(()));
        }

        #[test]
        fn from_params_rejects_any_params() {
            let mut t = Jsone::default();
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
            let t = Jsone::default();
            let bc = t.to_bytecode();

            // header mínimo: 8 + 4 + 1 = 13
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x26);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

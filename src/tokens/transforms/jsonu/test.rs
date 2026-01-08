// src/tokens/transforms/jsonu/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::InstructionMethods;
    use crate::tokens::transforms::jsonu::Jsonu;
    use crate::utils::errors::{ AtpError, AtpErrorCode };
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_jsonu() {
        let t = Jsonu::default();
        assert_eq!(t.get_string_repr(), "jsonu");
    }

    #[test]
    fn to_atp_line_is_constant() {
        let t = Jsonu::default();
        assert_eq!(t.to_atp_line().as_ref(), "jsonu;\n");
    }

    #[test]
    fn transform_matches_doc_example() {
        let t = Jsonu::default();
        let expected_output = "{banana: '10'}".to_string();

        assert_eq!(t.transform("\"{banana: '10'}\""), Ok(expected_output));
    }

    #[test]
    fn transform_unescapes_quotes_backslashes_and_controls() {
        let t = Jsonu::default();

        // JSON string com escapes
        let input = "\"a \\\"quote\\\" and a \\\\ slash\\nline2\\tend\\r\"";
        let expected = "a \"quote\" and a \\ slash\nline2\tend\r".to_string();

        assert_eq!(t.transform(input), Ok(expected));
    }

    #[test]
    fn transform_empty_json_string_returns_empty() {
        let t = Jsonu::default();
        assert_eq!(t.transform("\"\""), Ok("".to_string()));
    }

    #[test]
    fn transform_unicode_is_preserved() {
        let t = Jsonu::default();

        let input = "\"maçã 🍎\"";
        let expected = "maçã 🍎".to_string();

        assert_eq!(t.transform(input), Ok(expected));
    }

    #[test]
    fn transform_returns_error_on_invalid_json_string() {
        let t = Jsonu::default();

        // não é uma string JSON válida (faltam aspas, ou JSON inválido)
        let input = "{banana: '10'}";

        let got = t.transform(input);

        let expected = Err(
            AtpError::new(
                AtpErrorCode::TextParsingError("Failed to deserialize to JSON".into()),
                "serde_json::from_str",
                input.to_string()
            )
        );

        assert_eq!(got, expected);
    }

    #[test]
    fn from_params_accepts_empty_param_list() {
        let mut t = Jsonu::default();
        let params: Vec<AtpParamTypes> = vec![];

        assert_eq!(t.from_params(&params), Ok(()));
    }

    #[test]
    fn from_params_rejects_any_params() {
        let mut t = Jsonu::default();
        let params = vec![AtpParamTypes::Usize(1)];

        let err = t.from_params(&params).unwrap_err();

        assert!(matches!(err.error_code, AtpErrorCode::InvalidArgumentNumber(_)));
    }

    #[test]
    fn roundtrip_jsone_then_jsonu_returns_original() {
        // teste de consistência do par encode/decode
        use crate::tokens::transforms::jsone::Jsone;

        let enc = Jsone::default();
        let dec = Jsonu::default();

        let original = "banana \"laranja\" \\ canja\n\tfim\rmaçã 🍎";

        let encoded = enc.transform(original).unwrap();
        let decoded = dec.transform(&encoded).unwrap();

        assert_eq!(decoded, original.to_string());
    }

    // ============================
    // Bytecode-only tests (separados)
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;

        #[test]
        fn get_opcode_is_27() {
            let t = Jsonu::default();
            assert_eq!(t.get_opcode(), 0x27);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_no_params() {
            let t = Jsonu::default();
            let bc = t.to_bytecode();

            // header mínimo: 8 + 4 + 1 = 13
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x27);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

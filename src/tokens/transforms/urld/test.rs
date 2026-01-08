// src/tokens/transforms/urld/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::InstructionMethods;
    use crate::tokens::transforms::urld::Urld;
    use crate::utils::errors::{ AtpError, AtpErrorCode };
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_urld() {
        let t = Urld::default();
        assert_eq!(t.get_string_repr(), "urld");
    }

    #[test]
    fn to_atp_line_is_constant() {
        let t = Urld::default();
        assert_eq!(t.to_atp_line().as_ref(), "urld;\n");
    }

    #[test]
    fn transform_matches_doc_example() {
        let t = Urld::default();
        assert_eq!(t.transform("banana%20laranja"), Ok("banana laranja".to_string()));
    }

    #[test]
    fn transform_decodes_reserved_characters() {
        let t = Urld::default();

        let input = "a%3Fb%3Dc%26d%2Fe%3Af";
        let expected = "a?b=c&d/e:f".to_string();

        assert_eq!(t.transform(input), Ok(expected));
    }

    #[test]
    fn transform_decodes_plus_literal_not_space() {
        // importante: urlencoding::decode NÃO converte '+' em espaço.
        // '+' permanece '+'; espaço é %20.
        let t = Urld::default();

        let input = "a%2Bb%20c";
        let expected = "a+b c".to_string();

        assert_eq!(t.transform(input), Ok(expected));
    }

    #[test]
    fn transform_empty_string_returns_empty() {
        let t = Urld::default();
        assert_eq!(t.transform(""), Ok("".to_string()));
    }

    #[test]
    fn transform_decodes_unicode_utf8() {
        let t = Urld::default();

        let input = "ma%C3%A7%C3%A3%20%F0%9F%8D%8E";
        let expected = "maçã 🍎".to_string();

        assert_eq!(t.transform(input), Ok(expected));
    }

    #[test]
    fn transform_returns_error_on_invalid_percent_encoding() {
        let t = Urld::default();

        // '%' seguido de algo que não forma um byte hex válido
        let input = "banana%2Glaranja";

        let got = t.transform(input);

        let expected = Err(
            AtpError::new(
                AtpErrorCode::TextParsingError("Failed parsing URL string".into()),
                "urld",
                input.to_string()
            )
        );

        assert_eq!(got, expected);
    }

    #[test]
    fn roundtrip_urle_then_urld_returns_original() {
        use crate::tokens::transforms::urle::Urle;

        let enc = Urle::default();
        let dec = Urld::default();

        let original = "banana \"laranja\" \\ canja\n\tfim\rmaçã 🍎 ?&=/:";

        let encoded = enc.transform(original).unwrap();
        let decoded = dec.transform(&encoded).unwrap();

        assert_eq!(decoded, original.to_string());
    }

    #[test]
    fn from_params_accepts_empty_param_list() {
        let mut t = Urld::default();
        let params: Vec<AtpParamTypes> = vec![];

        assert_eq!(t.from_params(&params), Ok(()));
    }

    #[test]
    fn from_params_rejects_any_params() {
        let mut t = Urld::default();
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
        fn get_opcode_is_21() {
            let t = Urld::default();
            assert_eq!(t.get_opcode(), 0x21);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_no_params() {
            let t = Urld::default();
            let bc = t.to_bytecode();

            // header mínimo: 8 + 4 + 1 = 13
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x21);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

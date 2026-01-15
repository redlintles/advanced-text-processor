// src/tokens/transforms/padr/test.rs

#[cfg(test)]
mod tests {
    use crate::context::execution_context::GlobalExecutionContext;
    use crate::tokens::InstructionMethods;
    use crate::tokens::transforms::padr::Padr;
    use crate::utils::errors::AtpErrorCode;
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_padr() {
        let t = Padr::default();
        assert_eq!(t.get_string_repr(), "padr");
    }

    #[test]
    fn to_atp_line_matches_params() {
        let t = Padr::new("xy", 7);
        assert_eq!(t.to_atp_line().as_ref(), "padr xy 7;\n");
    }

    #[test]
    fn transform_returns_input_unchanged_if_already_at_or_above_max_len() {
        let t = Padr::new("xy", 3);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banana", &mut ctx), Ok("banana".to_string())); // len 6 >= 3
    }

    #[test]
    fn transform_pads_right_until_max_len_doc_example() {
        // "banana" tem 6 chars, max_len=7 => precisa de 1 char de padding.
        // extend_string("xy", 1) => "x" (pelo exemplo da doc)
        let t = Padr::new("xy", 7);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banana", &mut ctx), Ok("bananax".to_string()));
    }

    #[test]
    fn transform_pads_right_multiple_chars() {
        // 6 -> 10 precisa de 4 chars
        // extend_string("xy", 4) => "xyxy" (comportamento esperado de repetição)
        let t = Padr::new("xy", 10);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banana", &mut ctx), Ok("bananaxyxy".to_string()));
    }

    #[test]
    fn from_params_accepts_text_then_max_len() {
        let mut t = Padr::default();

        let params = vec![AtpParamTypes::String("xy".to_string()), AtpParamTypes::Usize(7)];

        assert_eq!(t.from_params(&params), Ok(()));
        assert_eq!(t.text, "xy".to_string());
        assert_eq!(t.max_len, 7);
    }

    #[test]
    fn from_params_rejects_wrong_param_count() {
        let mut t = Padr::default();

        let params = vec![AtpParamTypes::String("xy".to_string())];

        let err = t.from_params(&params).unwrap_err();

        assert!(matches!(err.error_code, AtpErrorCode::InvalidArgumentNumber(_)));
    }

    #[test]
    fn from_params_rejects_wrong_param_types() {
        let mut t = Padr::default();

        // invertido propositalmente
        let params = vec![AtpParamTypes::Usize(7), AtpParamTypes::String("xy".to_string())];

        let got = t.from_params(&params);

        // parse_args! retorna InvalidParameters com a msg do callsite
        let expected = Err(
            crate::utils::errors::AtpError::new(
                AtpErrorCode::InvalidParameters("Text_to_insert should be of String type".into()),
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
        fn get_opcode_is_30() {
            let t = Padr::default();
            assert_eq!(t.get_opcode(), 0x30);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_two_params_in_correct_order() {
            let t = Padr::new("xy", 7);
            let bc = t.to_bytecode();

            // header mínimo: 8 + 4 + 1 = 13
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x30);

            let param_count = bc[i] as usize;
            i += 1;
            assert_eq!(param_count, 2);

            // Param 1: String("xy")
            let _p1_total = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            let p1_type = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            let p1_payload_size = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap()) as usize;
            i += 4;
            assert_eq!(p1_type, 0x01);
            let p1_payload = &bc[i..i + p1_payload_size];
            i += p1_payload_size;
            assert_eq!(std::str::from_utf8(p1_payload).unwrap(), "xy");

            // Param 2: Usize(7)
            let _p2_total = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            let p2_type = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            let p2_payload_size = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap()) as usize;
            i += 4;
            assert_eq!(p2_type, 0x02);
            let p2_payload = &bc[i..i + p2_payload_size];
            i += p2_payload_size;
            assert_eq!(usize::from_be_bytes(p2_payload.try_into().unwrap()), 7);
            assert_eq!(i, i); // Anti unused var warning, fix this for testing purposes on the bytecode size
        }
    }
}

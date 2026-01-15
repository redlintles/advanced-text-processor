// src/tokens/transforms/dls/test.rs

#[cfg(test)]
mod tests {
    use crate::context::execution_context::GlobalExecutionContext;
    use crate::tokens::InstructionMethods;
    use crate::tokens::transforms::dls::Dls;
    use crate::utils::errors::AtpErrorCode;
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn params_sets_index() {
        let t = Dls::new(3);
        assert_eq!(t.index, 3);
    }

    #[test]
    fn get_string_repr_is_dls() {
        let t = Dls::default();
        assert_eq!(t.get_string_repr(), "dls");
    }

    #[test]
    fn to_atp_line_formats_correctly() {
        let t = Dls::new(7);
        assert_eq!(t.to_atp_line().as_ref(), "dls 7;\n");
    }

    #[test]
    fn transform_deletes_single_char_basic_example() {
        // index 3 em "banana" remove 'a' => "banna"
        let t = Dls::new(3);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banana", &mut ctx), Ok("banna".to_string()));
    }

    #[test]
    fn transform_deletes_first_char() {
        let t = Dls::new(0);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banana", &mut ctx), Ok("anana".to_string()));
    }

    #[test]
    fn transform_deletes_last_char() {
        let t = Dls::new(5);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banana", &mut ctx), Ok("banan".to_string()));
    }

    #[test]
    fn transform_unicode_safe_deletes_accented_char() {
        // "ábc" removendo index 0 => "bc"
        let t = Dls::new(0);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("ábc", &mut ctx), Ok("bc".to_string()));
    }

    #[test]
    fn transform_unicode_safe_deletes_emoji() {
        // "a💥b" indices por char: 0:'a', 1:'💥', 2:'b'
        let t = Dls::new(1);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("a💥b", &mut ctx), Ok("ab".to_string()));
    }

    #[test]
    fn transform_errors_when_index_out_of_bounds() {
        let t = Dls::new(999);
        let mut ctx = GlobalExecutionContext::new();

        let got = t.transform("abc", &mut ctx);
        assert!(got.is_err());
    }

    #[test]
    fn from_params_rejects_wrong_param_count() {
        let mut t = Dls::default();
        let params = vec![AtpParamTypes::Usize(1), AtpParamTypes::Usize(2)];

        let err = t.from_params(&params).unwrap_err();

        assert!(matches!(err.error_code, AtpErrorCode::InvalidArgumentNumber(_)));
    }

    #[test]
    fn from_params_accepts_single_usize_param() {
        let mut t = Dls::default();
        let params = vec![AtpParamTypes::Usize(7)];

        assert_eq!(t.from_params(&params), Ok(()));
        assert_eq!(t.index, 7);
    }

    #[test]
    fn from_params_rejects_wrong_param_type() {
        let mut t = Dls::default();
        let params = vec![AtpParamTypes::String("x".to_string())];

        let got = t.from_params(&params);

        let expected = Err(
            crate::utils::errors::AtpError::new(
                AtpErrorCode::InvalidParameters("Index should be of usize type".into()),
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
        use crate::utils::params::AtpParamTypes;

        #[test]
        fn get_opcode_is_32() {
            let t = Dls::default();
            assert_eq!(t.get_opcode(), 0x32);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_decodes_one_param() {
            let t = Dls::new(7);
            let bc = t.to_bytecode();

            // header mínimo: 8 + 4 + 1 = 13
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x32);

            let param_count = bc[i] as usize;
            i += 1;
            assert_eq!(param_count, 1);

            // param 1
            let p1_total = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap()) as usize;
            i += 8;
            let p1_start = i;
            let p1_end = p1_start + (p1_total - 8);
            let p1_payload = bc[p1_start..p1_end].to_vec();

            let decoded = AtpParamTypes::from_bytecode(p1_payload).unwrap();
            match decoded {
                AtpParamTypes::Usize(n) => assert_eq!(n, 7),
                _ => panic!("Expected Usize param"),
            }
        }
    }
}

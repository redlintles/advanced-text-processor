// src/tokens/transforms/cts/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::TokenMethods;
    use crate::tokens::transforms::cts::Cts;
    use crate::utils::errors::{AtpError, AtpErrorCode};

    #[test]
    fn params_sets_index() {
        let t = Cts::params(3);
        assert_eq!(t.index, 3);
    }

    #[test]
    fn get_string_repr_is_cts() {
        let t = Cts::default();
        assert_eq!(t.get_string_repr(), "cts");
    }

    #[test]
    fn to_atp_line_formats_correctly() {
        let t = Cts::params(7);
        assert_eq!(t.to_atp_line().as_ref(), "cts 7;\n");
    }

    #[test]
    fn transform_capitalizes_word_at_index() {
        let t = Cts::params(1);
        assert_eq!(t.transform("foo bar"), Ok("foo Bar".to_string()));
    }

    #[test]
    fn transform_capitalizes_first_word() {
        let t = Cts::params(0);
        assert_eq!(t.transform("foo bar"), Ok("Foo bar".to_string()));
    }

    #[test]
    fn transform_capitalizes_last_word() {
        let t = Cts::params(2);
        assert_eq!(t.transform("a b c"), Ok("a b C".to_string()));
    }

    #[test]
    fn transform_collapses_whitespace_due_to_split_whitespace() {
        // split_whitespace normaliza espaços/tabs/newlines
        let t = Cts::params(1);
        assert_eq!(t.transform("foo   bar"), Ok("foo Bar".to_string()));
    }

    #[test]
    fn transform_errors_when_index_out_of_bounds() {
        let t = Cts::params(5);
        let got = t.transform("one two");
        assert!(got.is_err());
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
        fn get_opcode_is_1d() {
            let t = Cts::default();
            assert_eq!(t.get_opcode(), 0x1d);
        }

        #[test]
        fn from_params_rejects_wrong_param_count() {
            let mut t = Cts::default();
            let params = vec![AtpParamTypes::Usize(1), AtpParamTypes::Usize(2)];

            let got = t.from_params(&params);

            let expected = Err(crate::utils::errors::AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                "",
                "",
            ));

            assert_eq!(got, expected);
        }

        #[test]
        fn from_params_accepts_single_usize_param() {
            let mut t = Cts::default();
            let params = vec![AtpParamTypes::Usize(7)];

            assert_eq!(t.from_params(&params), Ok(()));
            assert_eq!(t.index, 7);
        }

        #[test]
        fn from_params_rejects_wrong_param_type() {
            let mut t = Cts::default();
            let params = vec![AtpParamTypes::String("x".to_string())];

            let got = t.from_params(&params);

            let expected = Err(crate::utils::errors::AtpError::new(
                AtpErrorCode::InvalidParameters("Index should be of usize type".into()),
                "",
                "",
            ));

            assert_eq!(got, expected);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_decodes_one_param() {
            let t = Cts::params(7);
            let bc = t.to_bytecode();

            // header mínimo: 8 + 4 + 1 = 13
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x1d);

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

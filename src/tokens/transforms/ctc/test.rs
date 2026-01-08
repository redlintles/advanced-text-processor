// src/tokens/transforms/ctc/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::InstructionMethods;
    use crate::tokens::transforms::ctc::Ctc;
    use crate::utils::errors::{ AtpError, AtpErrorCode };
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn params_accepts_valid_range() {
        let t = Ctc::params(0, 1).unwrap();
        assert_eq!(t.start_index, 0);
        assert_eq!(t.end_index, 1);
    }

    #[test]
    fn get_string_repr_is_ctc() {
        let t = Ctc::default();
        assert_eq!(t.get_string_repr(), "ctc");
    }

    #[test]
    fn to_atp_line_formats_correctly() {
        let t = Ctc::params(2, 7).unwrap();
        assert_eq!(t.to_atp_line().as_ref(), "ctc 2 7;\n");
    }

    #[test]
    fn transform_capitalizes_chunk_simple_letters() {
        // exemplo do doc: 1..5 em "bananabananosa" => "bAnanabananosa"
        let t = Ctc::params(1, 5).unwrap();
        assert_eq!(t.transform("bananabananosa"), Ok("bAnanabananosa".to_string()));
    }

    #[test]
    fn transform_capitalizes_words_inside_chunk_and_reinserts() {
        // chunk pega "foo bar" e vira "Foo Bar"
        let input = "xx foo bar yy";
        // índices por caractere:
        // "xx " = 3 chars => start 3
        // "xx foo bar" (sem o espaço antes de yy) termina no índice 10? vamos computar:
        // string: x(0) x(1) ' '(2) f(3) o(4) o(5) ' '(6) b(7) a(8) r(9) ' '(10) y(11) y(12)
        // chunk [3..10) => chars 3..10 => "foo bar" (até antes do espaço)
        let t = Ctc::params(3, 10).unwrap();
        assert_eq!(t.transform(input), Ok("xx Foo Bar yy".to_string()));
    }

    #[test]
    fn transform_end_index_clamps_to_len() {
        let input = "hello";
        let t = Ctc::params(0, 999).unwrap(); // end enorme, deve clamp
        assert_eq!(t.transform(input), Ok("Hello".to_string()));
    }

    #[test]
    fn transform_on_unicode_char_indices() {
        // garante que char_indices/byte slicing está correto
        // "á" é 1 char (2 bytes), mas índice por char deve funcionar
        let input = "ábc def";
        // capitalizar chunk "ábc" => "Ábc"
        let t = Ctc::params(0, 3).unwrap();
        assert_eq!(t.transform(input), Ok("Ábc def".to_string()));
    }

    #[test]
    fn transform_errors_on_out_of_bounds_start() {
        let input = "abc";
        let t = Ctc::params(5, 6).unwrap(); // params() só valida relação, não input
        let got: Result<String, AtpError> = t.transform(input);
        assert!(got.is_err());
    }

    #[test]
    fn from_params_rejects_wrong_param_count() {
        let mut t = Ctc::default();
        let params = vec![AtpParamTypes::Usize(1)];

        let err = t.from_params(&params).unwrap_err();

        assert!(matches!(err.error_code, AtpErrorCode::InvalidArgumentNumber(_)));
    }

    #[test]
    fn from_params_accepts_two_usize_params() {
        let mut t = Ctc::default();
        let params = vec![AtpParamTypes::Usize(1), AtpParamTypes::Usize(5)];

        assert_eq!(t.from_params(&params), Ok(()));
        assert_eq!(t.start_index, 1);
        assert_eq!(t.end_index, 5);
    }

    #[test]
    fn from_params_rejects_wrong_param_type() {
        let mut t = Ctc::default();
        let params = vec![AtpParamTypes::String("x".to_string()), AtpParamTypes::Usize(5)];

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
        fn get_opcode_is_1b() {
            let t = Ctc::default();
            assert_eq!(t.get_opcode(), 0x1b);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_decodes_two_params() {
            let t = Ctc::params(2, 7).unwrap();
            let bc = t.to_bytecode();

            // header mínimo: 8 + 4 + 1 = 13
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x1b);

            let param_count = bc[i] as usize;
            i += 1;
            assert_eq!(param_count, 2);

            // param 1
            let p1_total = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap()) as usize;
            i += 8;
            let p1_start = i;
            let p1_end = p1_start + (p1_total - 8);
            let p1_payload = bc[p1_start..p1_end].to_vec();
            i = p1_end;

            let decoded1 = AtpParamTypes::from_bytecode(p1_payload).unwrap();
            match decoded1 {
                AtpParamTypes::Usize(n) => assert_eq!(n, 2),
                _ => panic!("Expected Usize param #1"),
            }

            // param 2
            let p2_total = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap()) as usize;
            i += 8;
            let p2_start = i;
            let p2_end = p2_start + (p2_total - 8);
            let p2_payload = bc[p2_start..p2_end].to_vec();

            let decoded2 = AtpParamTypes::from_bytecode(p2_payload).unwrap();
            match decoded2 {
                AtpParamTypes::Usize(n) => assert_eq!(n, 7),
                _ => panic!("Expected Usize param #2"),
            }
        }
    }
}

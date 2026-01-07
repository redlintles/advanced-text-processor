// src/tokens/transforms/dlc/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::InstructionMethods;
    use crate::tokens::transforms::dlc::Dlc;
    use crate::utils::errors::{ AtpErrorCode };
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn params_sets_indices() {
        let t = Dlc::params(1, 3).unwrap();
        assert_eq!(t.start_index, 1);
        assert_eq!(t.end_index, 3);
    }

    #[test]
    fn get_string_repr_is_dlc() {
        let t = Dlc::default();
        assert_eq!(t.get_string_repr(), "dlc");
    }

    #[test]
    fn to_atp_line_formats_correctly() {
        let t = Dlc::params(2, 5).unwrap();
        assert_eq!(t.to_atp_line().as_ref(), "dlc 2 5;\n");
    }

    #[test]
    fn transform_removes_middle_chunk() {
        // remove "nana" (1..5) from "bananalaranja..."
        let t = Dlc::params(1, 5).unwrap();
        assert_eq!(
            t.transform("bananalaranjacheiadecanja"),
            Ok("blaranjacheiadecanja".to_string())
        );
    }

    #[test]
    fn transform_removes_entire_string() {
        let t = Dlc::params(0, 100).unwrap();
        assert_eq!(t.transform("abc").unwrap(), "");
    }

    #[test]
    fn transform_unicode_safe() {
        let t = Dlc::params(1, 2).unwrap();
        assert_eq!(t.transform("ábcd"), Ok("ád".to_string()));
    }

    #[test]
    fn transform_fails_on_invalid_index() {
        let t = Dlc::params(10, 20).unwrap();
        let got = t.transform("abc");
        assert!(got.is_err());
    }

    #[test]
    fn from_params_rejects_wrong_param_count() {
        let mut t = Dlc::default();
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
    fn from_params_accepts_two_usize_params() {
        let mut t = Dlc::default();
        let params = vec![AtpParamTypes::Usize(2), AtpParamTypes::Usize(4)];

        assert_eq!(t.from_params(&params), Ok(()));
        assert_eq!(t.start_index, 2);
        assert_eq!(t.end_index, 4);
    }

    #[test]
    fn from_params_rejects_wrong_param_type() {
        let mut t = Dlc::default();
        let params = vec![AtpParamTypes::String("x".to_string()), AtpParamTypes::Usize(2)];

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
    // Bytecode-only tests
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;
        use crate::utils::params::AtpParamTypes;

        #[test]
        fn get_opcode_is_08() {
            let t = Dlc::default();
            assert_eq!(t.get_opcode(), 0x08);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_decodes_params() {
            let t = Dlc::params(3, 6).unwrap();
            let bc = t.to_bytecode();

            // header mínimo: 8 + 4 + 1 = 13
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x08);

            let param_count = bc[i] as usize;
            i += 1;
            assert_eq!(param_count, 2);

            // param 1
            let p1_total = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap()) as usize;
            i += 8;
            let p1_start = i;
            let p1_end = p1_start + (p1_total - 8);
            let p1_payload = bc[p1_start..p1_end].to_vec();

            let decoded1 = AtpParamTypes::from_bytecode(p1_payload).unwrap();
            match decoded1 {
                AtpParamTypes::Usize(n) => assert_eq!(n, 3),
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
                AtpParamTypes::Usize(n) => assert_eq!(n, 6),
                _ => panic!("Expected Usize param #2"),
            }
        }
    }
}

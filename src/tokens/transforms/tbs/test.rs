#![cfg(feature = "test_access")]

#[cfg(test)]
mod tests {
    use crate::tokens::{TokenMethods, transforms::tbs::Tbs};
    use crate::utils::errors::{AtpError, AtpErrorCode};

    #[test]
    fn get_string_repr_is_tbs() {
        let t = Tbs::default();
        assert_eq!(t.get_string_repr(), "tbs");
    }

    #[test]
    fn to_atp_line_is_correct() {
        let t = Tbs::default();
        assert_eq!(t.to_atp_line().as_ref(), "tbs;\n");
    }

    #[test]
    fn transform_trims_both_sides_spaces() {
        let t = Tbs::default();
        assert_eq!(t.transform("   banana   ").unwrap(), "banana");
    }

    #[test]
    fn transform_trims_tabs_newlines_too() {
        let t = Tbs::default();
        assert_eq!(t.transform("\t\n  banana \r\n").unwrap(), "banana");
    }

    #[test]
    fn transform_no_outer_whitespace_unchanged() {
        let t = Tbs::default();
        assert_eq!(t.transform("banana").unwrap(), "banana");
    }

    #[test]
    fn transform_only_whitespace_becomes_empty() {
        let t = Tbs::default();
        assert_eq!(t.transform("     ").unwrap(), "");
    }

    #[test]
    fn transform_empty_is_empty() {
        let t = Tbs::default();
        assert_eq!(t.transform("").unwrap(), "");
    }

    // ============================
    // Bytecode tests
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;
        use crate::utils::params::AtpParamTypes;

        #[test]
        fn get_opcode_is_0x05() {
            let t = Tbs::default();
            assert_eq!(t.get_opcode(), 0x05);
        }

        #[test]
        fn from_params_accepts_empty() {
            let mut t = Tbs::default();
            let params: Vec<AtpParamTypes> = vec![];
            assert_eq!(t.from_params(&params), Ok(()));
        }

        #[test]
        fn from_params_rejects_any_params() {
            let mut t = Tbs::default();
            let params = vec![AtpParamTypes::Usize(1)];

            let got = t.from_params(&params);

            let expected = Err(AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                "",
                "",
            ));

            assert_eq!(got, expected);
        }

        #[test]
        fn to_bytecode_contains_opcode_and_zero_params() {
            let t = Tbs::default();
            let bc = t.to_bytecode();

            // Formato esperado: [u64 total_size_be][u32 opcode_be][u8 param_count]...
            assert!(bc.len() >= 13);

            let total_size = u64::from_be_bytes(bc[0..8].try_into().unwrap()) as usize;
            assert_eq!(total_size, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[8..12].try_into().unwrap());
            assert_eq!(opcode, 0x05);

            let param_count = bc[12] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

#![cfg(feature = "test_access")]

#[cfg(test)]
mod tests {
    use crate::context::execution_context::GlobalExecutionContext;
    use crate::tokens::{ InstructionMethods, transforms::slt::Slt };
    use crate::utils::errors::{ AtpErrorCode };
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_slt() {
        let t = Slt::default();
        assert_eq!(t.get_string_repr(), "slt");
    }

    #[test]
    fn to_atp_line_is_correct() {
        let t = Slt {
            start_index: 1,
            end_index: 3,
        };
        assert_eq!(t.to_atp_line().as_ref(), "slt 1 3;\n");
    }

    #[test]
    fn transform_selects_basic_slice() {
        let t = Slt::params(1, 3).unwrap();
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banana", &mut ctx).unwrap(), "ana");
    }

    #[test]
    fn transform_supports_unicode_safely() {
        // banàna => b a n à n a
        // indices: 0 b, 1 a, 2 n, 3 à, 4 n, 5 a
        let t = Slt::params(1, 4).unwrap();
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banàna", &mut ctx).unwrap(), "anàn");
    }

    #[test]
    fn transform_end_index_beyond_len_selects_until_end() {
        let t = Slt::params(1, 9999).unwrap();
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banana", &mut ctx).unwrap(), "anana");
    }

    #[test]
    fn transform_rejects_invalid_bounds() {
        // start > end deve falhar (quem define isso é seu check_chunk_bound_indexes)
        let t = Slt {
            start_index: 5,
            end_index: 1,
        };
        let mut ctx = GlobalExecutionContext::new();

        assert!(matches!(t.transform("banana", &mut ctx), Err(_)));
    }

    #[test]
    fn from_params_accepts_two_params() {
        let mut t = Slt::default();
        let params = vec![AtpParamTypes::Usize(1), AtpParamTypes::Usize(3)];
        assert_eq!(t.from_params(&params), Ok(()));
        assert_eq!(t.start_index, 1);
        assert_eq!(t.end_index, 3);
    }

    #[test]
    fn from_params_rejects_wrong_len() {
        let mut t = Slt::default();
        let params = vec![AtpParamTypes::Usize(1)];

        let err = t.from_params(&params).unwrap_err();

        assert!(matches!(err.error_code, AtpErrorCode::InvalidArgumentNumber(_)));
    }

    // ============================
    // Bytecode tests
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;

        #[test]
        fn get_opcode_is_0x11() {
            let t = Slt::default();
            assert_eq!(t.get_opcode(), 0x11);
        }

        #[test]
        fn to_bytecode_contains_opcode_and_two_params() {
            let t = Slt {
                start_index: 1,
                end_index: 3,
            };
            let bc = t.to_bytecode();

            // Formato: [u64 total_size_be][u32 opcode_be][u8 param_count]...
            assert!(bc.len() >= 13);

            let total_size = u64::from_be_bytes(bc[0..8].try_into().unwrap()) as usize;
            assert_eq!(total_size, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[8..12].try_into().unwrap());
            assert_eq!(opcode, 0x11);

            let param_count = bc[12] as usize;
            assert_eq!(param_count, 2);
        }
    }
}

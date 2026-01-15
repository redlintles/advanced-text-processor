#![cfg(feature = "test_access")]

#[cfg(test)]
mod tests {
    use crate::context::execution_context::GlobalExecutionContext;
    use crate::tokens::{ InstructionMethods, transforms::rtl::Rtl };
    use crate::utils::errors::{ AtpError, AtpErrorCode };
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_rtl() {
        let t = Rtl::default();
        assert_eq!(t.get_string_repr(), "rtl");
    }

    #[test]
    fn to_atp_line_contains_times() {
        let t = Rtl::new(3);
        assert_eq!(t.to_atp_line().as_ref(), "rtl 3;\n");
    }

    #[test]
    fn transform_rotates_left_basic() {
        let t = Rtl::new(3);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banana", &mut ctx).unwrap(), "anaban");
    }

    #[test]
    fn transform_times_zero_returns_same() {
        let t = Rtl::new(0);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banana", &mut ctx).unwrap(), "banana");
    }

    #[test]
    fn transform_times_equal_len_returns_same() {
        let t = Rtl::new(6); // len("banana") == 6
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banana", &mut ctx).unwrap(), "banana");
    }

    #[test]
    fn transform_times_greater_than_len_uses_modulo() {
        let t = Rtl::new(7); // 7 % 6 = 1
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banana", &mut ctx).unwrap(), "ananab");
    }

    #[test]
    fn transform_single_char_always_same() {
        let t = Rtl::new(999);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("x", &mut ctx).unwrap(), "x");
    }

    #[test]
    fn transform_unicode_safe_rotation() {
        // "áβç" (3 chars) rotate 1 => "βçá"
        let t = Rtl::new(1);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("áβç", &mut ctx).unwrap(), "βçá");
    }

    #[test]
    fn transform_empty_input_returns_error() {
        let t = Rtl::new(1);
        let mut ctx = GlobalExecutionContext::new();

        let got = t.transform("", &mut ctx);

        let expected = Err(
            AtpError::new(
                AtpErrorCode::InvalidParameters("Input is empty".into()),
                t.to_atp_line(),
                "\" \""
            )
        );

        assert_eq!(got, expected);
    }

    #[test]
    fn from_params_parses_single_usize() {
        let mut t = Rtl::default();

        let params = vec![AtpParamTypes::Usize(5)];

        assert_eq!(t.from_params(&params), Ok(()));
        assert_eq!(t.times, 5);
    }

    #[test]
    fn from_params_rejects_wrong_param_count() {
        let mut t = Rtl::default();

        let params = vec![];

        let err = t.from_params(&params).unwrap_err();

        assert!(matches!(err.error_code, AtpErrorCode::InvalidArgumentNumber(_)));
    }

    #[test]
    fn from_params_rejects_wrong_type() {
        let mut t = Rtl::default();

        let params = vec![AtpParamTypes::String("x".to_string())];

        assert!(t.from_params(&params).is_err());
    }
    // ============================
    // Bytecode tests
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;

        #[test]
        fn get_opcode_is_0x0e() {
            let t = Rtl::default();
            assert_eq!(t.get_opcode(), 0x0e);
        }

        #[test]
        fn to_bytecode_contains_opcode_and_one_param() {
            let t = Rtl::new(3);
            let bc = t.to_bytecode();

            assert!(!bc.is_empty());
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x0e);

            let param_count = bc[i] as usize;
            i += 1;
            assert_eq!(param_count, 1);

            // Param 1: Usize
            let _p_total = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;

            let p_type = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(p_type, 0x02);

            let p_size = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap()) as usize;
            i += 4;
            assert_eq!(p_size, 8);

            let payload = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            assert_eq!(payload, 3);
        }
    }
}

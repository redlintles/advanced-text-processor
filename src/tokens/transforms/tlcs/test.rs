#![cfg(feature = "test_access")]

#[cfg(test)]
mod tests {
    use crate::context::execution_context::GlobalExecutionContext;
    use crate::tokens::{ InstructionMethods, transforms::tlcs::Tlcs };
    use crate::utils::errors::{ AtpErrorCode };
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_tlcs() {
        let t = Tlcs::default();
        assert_eq!(t.get_string_repr(), "tlcs");
    }

    #[test]
    fn to_atp_line_is_correct() {
        let t = Tlcs::new(1);
        assert_eq!(t.to_atp_line().as_ref(), "tlcs 1;\n");
    }

    #[test]
    fn transform_lowercases_single_char_ascii() {
        let t = Tlcs::new(1);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("BANANA", &mut ctx), Ok("BaNANA".to_string()));
    }

    #[test]
    fn transform_lowercases_single_char_unicode() {
        // Índices por CHAR: 0 b, 1 a, 2 n, 3 à, 4 n, 5 a
        let t = Tlcs::new(3);
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("banÀna", &mut ctx), Ok("banàna".to_string()));
    }

    #[test]
    fn from_params_accepts_one_usize() {
        let mut t = Tlcs::default();
        let params = vec![AtpParamTypes::Usize(1)];

        assert_eq!(t.from_params(&params), Ok(()));
        assert_eq!(t.to_atp_line().as_ref(), "tlcs 1;\n");
    }

    #[test]
    fn from_params_rejects_wrong_len() {
        let mut t = Tlcs::default();
        let params: Vec<AtpParamTypes> = vec![];

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
        fn get_opcode_is_0x15() {
            let t = Tlcs::default();
            assert_eq!(t.get_opcode(), 0x15);
        }

        #[test]
        fn to_bytecode_has_opcode_and_one_param() {
            let t = Tlcs::new(7);
            let bc = t.to_bytecode();

            assert!(bc.len() >= 13);

            let total_size = u64::from_be_bytes(bc[0..8].try_into().unwrap()) as usize;
            assert_eq!(total_size, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[8..12].try_into().unwrap());
            assert_eq!(opcode, 0x15);

            let param_count = bc[12] as usize;
            assert_eq!(param_count, 1);
        }
    }
}

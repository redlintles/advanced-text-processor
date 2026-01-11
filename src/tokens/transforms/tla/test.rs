#![cfg(feature = "test_access")]

#[cfg(test)]
mod tests {
    use crate::context::execution_context::GlobalExecutionContext;
    use crate::tokens::{ InstructionMethods, transforms::tla::Tla };
    use crate::utils::errors::{ AtpErrorCode };
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_tla() {
        let t = Tla::default();
        assert_eq!(t.get_string_repr(), "tla");
    }

    #[test]
    fn to_atp_line_is_correct() {
        let t = Tla::default();
        assert_eq!(t.to_atp_line().as_ref(), "tla;\n");
    }

    #[test]
    fn transform_lowercases_ascii() {
        let t = Tla::default();
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("BANANA", &mut ctx).unwrap(), "banana");
    }

    #[test]
    fn transform_preserves_non_letters() {
        let t = Tla::default();
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("BA-NA_NA 123!", &mut ctx).unwrap(), "ba-na_na 123!");
    }

    #[test]
    fn transform_empty_is_empty() {
        let t = Tla::default();
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("", &mut ctx).unwrap(), "");
    }

    #[test]
    fn transform_unicode_lowercase() {
        // Unicode casefolding/lowercasing
        let t = Tla::default();
        let mut ctx = GlobalExecutionContext::new();

        assert_eq!(t.transform("ÁÉÍÓÚ Ç", &mut ctx).unwrap(), "áéíóú ç");
    }

    #[test]
    fn from_params_accepts_empty() {
        let mut t = Tla::default();
        let params: Vec<AtpParamTypes> = vec![];
        assert_eq!(t.from_params(&params), Ok(()));
    }

    #[test]
    fn from_params_rejects_any_params() {
        let mut t = Tla::default();
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
        fn get_opcode_is_0x13() {
            let t = Tla::default();
            assert_eq!(t.get_opcode(), 0x13);
        }

        #[test]
        fn to_bytecode_contains_opcode_and_zero_params() {
            let t = Tla::default();
            let bc = t.to_bytecode();

            assert!(!bc.is_empty());
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x13);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

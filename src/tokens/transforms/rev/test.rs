#![cfg(feature = "test_access")]

#[cfg(test)]
mod tests {
    use crate::tokens::{ InstructionMethods, transforms::rev::Rev };
    use crate::utils::errors::{ AtpError, AtpErrorCode };
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_rev() {
        let t = Rev::default();
        assert_eq!(t.get_string_repr(), "rev");
    }

    #[test]
    fn to_atp_line_is_correct() {
        let t = Rev::default();
        assert_eq!(t.to_atp_line().as_ref(), "rev;\n");
    }

    #[test]
    fn transform_reverses_ascii() {
        let t = Rev::default();
        assert_eq!(t.transform("foobar").unwrap(), "raboof");
    }

    #[test]
    fn transform_empty_is_empty() {
        let t = Rev::default();
        assert_eq!(t.transform("").unwrap(), "");
    }

    #[test]
    fn transform_single_char_is_same() {
        let t = Rev::default();
        assert_eq!(t.transform("x").unwrap(), "x");
    }

    #[test]
    fn transform_unicode_safe() {
        // chars() => reversão por scalar values (não por byte)
        let t = Rev::default();
        assert_eq!(t.transform("áβç").unwrap(), "çβá");
    }

    #[test]
    fn from_params_accepts_empty() {
        let mut t = Rev::default();
        let params: Vec<AtpParamTypes> = vec![];
        assert_eq!(t.from_params(&params), Ok(()));
    }

    #[test]
    fn from_params_rejects_any_params() {
        let mut t = Rev::default();
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
        fn get_opcode_is_0x22() {
            let t = Rev::default();
            assert_eq!(t.get_opcode(), 0x22);
        }

        #[test]
        fn to_bytecode_contains_opcode_and_zero_params() {
            let t = Rev::default();
            let bc = t.to_bytecode();

            assert!(!bc.is_empty());
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x22);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

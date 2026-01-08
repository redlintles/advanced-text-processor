// src/tokens/transforms/dll/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::InstructionMethods;
    use crate::tokens::transforms::dll::Dll;
    use crate::utils::errors::AtpErrorCode;
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_dll() {
        let t = Dll::default();
        assert_eq!(t.get_string_repr(), "dll");
    }

    #[test]
    fn to_atp_line_is_constant() {
        let t = Dll::default();
        assert_eq!(t.to_atp_line().as_ref(), "dll;\n");
    }

    #[test]
    fn transform_deletes_last_char_basic() {
        let t = Dll::default();
        assert_eq!(t.transform("banana"), Ok("banan".to_string()));
    }

    #[test]
    fn transform_empty_string_stays_empty() {
        let t = Dll::default();
        assert_eq!(t.transform(""), Ok("".to_string()));
    }

    #[test]
    fn transform_single_char_becomes_empty() {
        let t = Dll::default();
        assert_eq!(t.transform("a"), Ok("".to_string()));
    }

    #[test]
    fn transform_unicode_last_char_removed_safely_accented() {
        let t = Dll::default();
        assert_eq!(t.transform("abá"), Ok("ab".to_string()));
    }

    #[test]
    fn transform_unicode_last_char_removed_safely_emoji() {
        let t = Dll::default();
        assert_eq!(t.transform("boom💥"), Ok("boom".to_string()));
    }

    #[test]
    fn from_params_accepts_empty_param_list() {
        let mut t = Dll::default();
        let params: Vec<AtpParamTypes> = vec![];

        assert_eq!(t.from_params(&params), Ok(()));
    }

    #[test]
    fn from_params_rejects_any_params() {
        let mut t = Dll::default();
        let params = vec![AtpParamTypes::Usize(1)];

        let err = t.from_params(&params).unwrap_err();

        assert!(matches!(err.error_code, AtpErrorCode::InvalidArgumentNumber(_)));
    }

    // ============================
    // Bytecode-only tests (separados)
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;

        #[test]
        fn get_opcode_is_04() {
            let t = Dll::default();
            assert_eq!(t.get_opcode(), 0x04);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_no_params() {
            let t = Dll::default();
            let bc = t.to_bytecode();

            // header mínimo: 8 + 4 + 1 = 13
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;

            // total_size = tamanho do "body" (opcode+count+params...)
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x04);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

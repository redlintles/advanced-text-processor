// src/tokens/transforms/dlf/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::TokenMethods;
    use crate::tokens::transforms::dlf::Dlf;

    #[test]
    fn get_string_repr_is_dlf() {
        let t = Dlf::default();
        assert_eq!(t.get_string_repr(), "dlf");
    }

    #[test]
    fn to_atp_line_is_constant() {
        let t = Dlf::default();
        assert_eq!(t.to_atp_line().as_ref(), "dlf;\n");
    }

    #[test]
    fn transform_deletes_first_char_basic() {
        let t = Dlf::default();
        assert_eq!(t.transform("banana"), Ok("anana".to_string()));
    }

    #[test]
    fn transform_empty_string_stays_empty() {
        let t = Dlf::default();
        assert_eq!(t.transform(""), Ok("".to_string()));
    }

    #[test]
    fn transform_single_char_becomes_empty() {
        let t = Dlf::default();
        assert_eq!(t.transform("a"), Ok("".to_string()));
    }

    #[test]
    fn transform_unicode_first_char_removed_safely_accented() {
        let t = Dlf::default();
        assert_eq!(t.transform("ábc"), Ok("bc".to_string()));
    }

    #[test]
    fn transform_unicode_first_char_removed_safely_emoji() {
        let t = Dlf::default();
        assert_eq!(t.transform("💥boom"), Ok("boom".to_string()));
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
        fn get_opcode_is_03() {
            let t = Dlf::default();
            assert_eq!(t.get_opcode(), 0x03);
        }

        #[test]
        fn from_params_accepts_empty_param_list() {
            let mut t = Dlf::default();
            let params: Vec<AtpParamTypes> = vec![];

            assert_eq!(t.from_params(&params), Ok(()));
        }

        #[test]
        fn from_params_rejects_any_params() {
            let mut t = Dlf::default();
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
        fn to_bytecode_has_expected_header_and_no_params() {
            let t = Dlf::default();
            let bc = t.to_bytecode();

            // header mínimo: 8 + 4 + 1 = 13 bytes
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;

            // total_size = tamanho do "body" (opcode+count+params...)
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x03);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

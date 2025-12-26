#![cfg(feature = "test_access")]

#[cfg(test)]
mod tests {
    use crate::tokens::{ transforms::tlcc::Tlcc, TokenMethods };
    use crate::utils::errors::{ AtpError, AtpErrorCode };

    #[test]
    fn get_string_repr_is_tlcc() {
        let t = Tlcc::default();
        assert_eq!(t.get_string_repr(), "tlcc");
    }

    #[test]
    fn to_atp_line_is_correct() {
        let t = Tlcc::params(1, 4).unwrap();
        assert_eq!(t.to_atp_line().as_ref(), "tlcc 1 4;\n");
    }

    #[test]
    fn transform_lowercases_only_the_chunk() {
        let t = Tlcc::params(1, 4).unwrap();
        // BANANA => BananA (1..4 inclusive)
        assert_eq!(t.transform("BANANA"), Ok("BananA".to_string()));
    }

    #[test]
    fn transform_supports_unicode_safely() {
        // "ÁBÇDÊ" lowercasing chunk should not break UTF-8
        let t = Tlcc::params(1, 3).unwrap();
        // indexes: 0 Á, 1 B, 2 Ç, 3 D, 4 Ê
        assert_eq!(t.transform("ÁBÇDÊ"), Ok("ÁbçdÊ".to_string()));
    }

    #[test]
    fn transform_errors_on_invalid_bounds() {
        // start > end (depende de como seu validator define)
        let got = Tlcc::params(4, 1);
        assert!(matches!(got, Err(_)));
    }

    #[test]
    fn from_vec_params_parses_ok() {
        let mut t = Tlcc::default();
        let line = vec!["tlcc".to_string(), "2".to_string(), "3".to_string()];

        assert_eq!(t.from_vec_params(line), Ok(()));
        assert_eq!(t.to_atp_line().as_ref(), "tlcc 2 3;\n");
    }

    #[test]
    fn from_vec_params_rejects_wrong_token() {
        let mut t = Tlcc::default();
        let line = vec!["nope".to_string(), "1".to_string(), "2".to_string()];

        let got = t.from_vec_params(line.clone());

        let expected = Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid parser for this token".into()),
                line[0].to_string(),
                line.join(" ")
            )
        );

        assert_eq!(got, expected);
    }

    // ============================
    // Bytecode tests
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;
        use crate::utils::params::AtpParamTypes;

        #[test]
        fn get_opcode_is_0x17() {
            let t = Tlcc::default();
            assert_eq!(t.get_opcode(), 0x17);
        }

        #[test]
        fn from_params_accepts_two_usizes() {
            let mut t = Tlcc::default();
            let params = vec![AtpParamTypes::Usize(1), AtpParamTypes::Usize(4)];

            assert_eq!(t.from_params(&params), Ok(()));
            assert_eq!(t.to_atp_line().as_ref(), "tlcc 1 4;\n");
        }

        #[test]
        fn from_params_rejects_wrong_len() {
            let mut t = Tlcc::default();
            let params = vec![AtpParamTypes::Usize(1)];

            let got = t.from_params(&params);

            let expected = Err(
                AtpError::new(
                    AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                    "",
                    ""
                )
            );

            assert_eq!(got, expected);
        }

        #[test]
        fn to_bytecode_has_opcode_and_two_params() {
            let t = Tlcc::params(1, 4).unwrap();
            let bc = t.to_bytecode();

            assert!(bc.len() >= 13);

            let total_size = u64::from_be_bytes(bc[0..8].try_into().unwrap()) as usize;
            assert_eq!(total_size, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[8..12].try_into().unwrap());
            assert_eq!(opcode, 0x17);

            let param_count = bc[12] as usize;
            assert_eq!(param_count, 2);
        }
    }
}

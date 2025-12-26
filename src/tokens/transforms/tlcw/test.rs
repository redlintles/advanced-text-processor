#![cfg(feature = "test_access")]

#[cfg(test)]
mod tests {
    use crate::tokens::{ transforms::tlcw::Tlcw, TokenMethods };
    use crate::utils::errors::{ AtpError, AtpErrorCode };

    #[test]
    fn get_string_repr_is_tlcw() {
        let t = Tlcw::default();
        assert_eq!(t.get_string_repr(), "tlcw");
    }

    #[test]
    fn to_atp_line_is_correct() {
        let t = Tlcw::params(2);
        assert_eq!(t.to_atp_line().as_ref(), "tlcw 2;\n");
    }

    #[test]
    fn transform_lowercases_one_word_by_index() {
        let t = Tlcw::params(1);
        let input = "BANANA LARANJA CHEIA DE CANJA";
        let expected = "BANANA laranja CHEIA DE CANJA".to_string();

        assert_eq!(t.transform(input), Ok(expected));
    }

    #[test]
    fn transform_index_zero_lowercases_first_word() {
        let t = Tlcw::params(0);
        assert_eq!(t.transform("BANANA LARANJA"), Ok("banana LARANJA".to_string()));
    }

    #[test]
    fn from_vec_params_parses_ok() {
        let mut t = Tlcw::default();
        let line = vec!["tlcw".to_string(), "3".to_string()];

        assert_eq!(t.from_vec_params(line), Ok(()));
        assert_eq!(t.to_atp_line().as_ref(), "tlcw 3;\n");
    }

    #[test]
    fn from_vec_params_rejects_wrong_token() {
        let mut t = Tlcw::default();
        let line = vec!["nope".to_string(), "1".to_string()];

        let got = t.from_vec_params(line.clone());

        let expected = Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid Parser for this token".into()),
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
        fn get_opcode_is_0x29() {
            let t = Tlcw::default();
            assert_eq!(t.get_opcode(), 0x29);
        }

        #[test]
        fn from_params_accepts_one_usize() {
            let mut t = Tlcw::default();
            let params = vec![AtpParamTypes::Usize(1)];

            assert_eq!(t.from_params(&params), Ok(()));
            assert_eq!(t.to_atp_line().as_ref(), "tlcw 1;\n");
        }

        #[test]
        fn from_params_rejects_wrong_len() {
            let mut t = Tlcw::default();
            let params: Vec<AtpParamTypes> = vec![];

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
        fn to_bytecode_has_opcode_and_one_param() {
            let t = Tlcw::params(7);
            let bc = t.to_bytecode();

            assert!(bc.len() >= 13);

            let total_size = u64::from_be_bytes(bc[0..8].try_into().unwrap()) as usize;
            assert_eq!(total_size, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[8..12].try_into().unwrap());
            assert_eq!(opcode, 0x29);

            let param_count = bc[12] as usize;
            assert_eq!(param_count, 1);
        }
    }
}

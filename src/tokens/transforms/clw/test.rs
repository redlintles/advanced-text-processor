// src/tokens/transforms/clw/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::TokenMethods;
    use crate::tokens::transforms::clw::Clw;
    use crate::utils::errors::{AtpError, AtpErrorCode};

    #[test]
    fn get_string_repr_is_clw() {
        let t = Clw::default();
        assert_eq!(t.get_string_repr(), "clw");
    }

    #[test]
    fn to_atp_line_is_constant() {
        let t = Clw::default();
        assert_eq!(t.to_atp_line().as_ref(), "clw;\n");
    }

    #[test]
    fn transform_capitalizes_last_word_basic_case() {
        let t = Clw::default();
        assert_eq!(t.transform("foo bar"), Ok("foo Bar".to_string()));
    }

    #[test]
    fn transform_empty_input_stays_empty() {
        let t = Clw::default();
        assert_eq!(t.transform(""), Ok("".to_string()));
    }

    #[test]
    fn transform_single_word_capitalizes_that_word() {
        let t = Clw::default();
        assert_eq!(t.transform("hello"), Ok("Hello".to_string()));
    }

    #[test]
    fn transform_preserves_previous_words() {
        let t = Clw::default();
        assert_eq!(t.transform("foo bar baz"), Ok("foo bar Baz".to_string()));
    }

    #[test]
    fn transform_trailing_space_edge_case_current_behavior() {
        // Observação: como o split é por ' ' (espaço literal),
        // "foo " vira ["foo", ""] e o "último" vira "".
        // Então o resultado fica "foo " (permanece igual).
        let t = Clw::default();
        assert_eq!(t.transform("foo "), Ok("foo ".to_string()));
    }

    // ============================
    // Bytecode-only tests (separados)
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;
        use crate::utils::params::AtpParamTypes;

        #[test]
        fn get_opcode_is_19() {
            let t = Clw::default();
            assert_eq!(t.get_opcode(), 0x19);
        }

        #[test]
        fn from_params_accepts_empty_param_list() {
            let mut t = Clw::default();
            let params: Vec<AtpParamTypes> = vec![];

            assert_eq!(t.from_params(&params), Ok(()));
        }

        #[test]
        fn from_params_rejects_any_params() {
            let mut t = Clw::default();
            let params = vec![AtpParamTypes::String("x".to_string())];

            let got = t.from_params(&params);

            let expected = Err(AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                "",
                "",
            ));

            assert_eq!(got, expected);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_no_params() {
            let t = Clw::default();
            let bc = t.to_bytecode();

            // Header mínimo: 8 + 4 + 1 = 13 bytes
            assert!(bc.len() >= 13);

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;

            // total_size = tamanho do "body" (opcode+count+params...)
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x19);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

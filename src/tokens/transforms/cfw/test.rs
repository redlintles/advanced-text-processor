// src/tokens/transforms/cfw/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::InstructionMethods;
    use crate::tokens::transforms::cfw::Cfw;
    use crate::utils::errors::{ AtpError, AtpErrorCode };
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_cfw() {
        let t = Cfw::default();
        assert_eq!(t.get_string_repr(), "cfw");
    }

    #[test]
    fn to_atp_line_is_constant() {
        let t = Cfw::default();
        assert_eq!(t.to_atp_line().as_ref(), "cfw;\n");
    }

    #[test]
    fn transform_capitalizes_first_word() {
        let t = Cfw::default();
        assert_eq!(t.transform("foo bar"), Ok("Foo bar".to_string()));
    }

    #[test]
    fn transform_empty_input_stays_empty() {
        let t = Cfw::default();
        assert_eq!(t.transform(""), Ok("".to_string()));
    }

    #[test]
    fn transform_single_word() {
        let t = Cfw::default();
        assert_eq!(t.transform("hello"), Ok("Hello".to_string()));
    }

    // Se sua função capitalize() lida com espaços/pontuação de um jeito específico,
    // dá pra ir refinando esses testes depois. Por enquanto é um “smoke test”.
    #[test]
    fn transform_preserves_rest_of_string_basic_case() {
        let t = Cfw::default();
        assert_eq!(t.transform("foo bar baz"), Ok("Foo bar baz".to_string()));
    }

    #[test]
    fn from_params_accepts_empty_param_list() {
        let mut t = Cfw::default();
        let params: Vec<AtpParamTypes> = vec![];

        assert_eq!(t.from_params(&params), Ok(()));
    }

    #[test]
    fn from_params_rejects_any_params() {
        let mut t = Cfw::default();
        let params = vec![AtpParamTypes::String("x".to_string())];

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
        fn get_opcode_is_18() {
            let t = Cfw::default();
            assert_eq!(t.get_opcode(), 0x18);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_no_params() {
            let t = Cfw::default();
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
            assert_eq!(opcode, 0x18);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

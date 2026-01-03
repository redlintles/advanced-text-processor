#![cfg(feature = "test_access")]

#[cfg(test)]
mod tests {
    use crate::tokens::{ TokenMethods, transforms::splc::Splc };
    use crate::utils::errors::{ AtpError, AtpErrorCode };
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_splc() {
        let t = Splc::default();
        assert_eq!(t.get_string_repr(), "splc");
    }

    #[test]
    fn to_atp_line_is_correct() {
        let t = Splc::default();
        assert_eq!(t.to_atp_line().as_ref(), "splc;\n");
    }

    #[test]
    fn transform_splits_ascii_chars() {
        let t = Splc::default();
        assert_eq!(t.transform("banana").unwrap(), "b a n a n a");
    }

    #[test]
    fn transform_keeps_existing_spaces_as_chars() {
        let t = Splc::default();
        assert_eq!(t.transform("a b").unwrap(), "a   b");
        // explicação: chars = ['a',' ','b'] => "a" + " " + " " + " " + "b" => "a␠␠␠b"
    }

    #[test]
    fn transform_unicode_chars_ok() {
        let t = Splc::default();
        assert_eq!(t.transform("áβ🍌").unwrap(), "á β 🍌");
    }

    #[test]
    fn transform_empty_is_empty() {
        let t = Splc::default();
        assert_eq!(t.transform("").unwrap(), "");
    }

    #[test]
    fn from_params_accepts_empty() {
        let mut t = Splc::default();
        let params: Vec<AtpParamTypes> = vec![];
        assert_eq!(t.from_params(&params), Ok(()));
    }

    #[test]
    fn from_params_rejects_any_params() {
        let mut t = Splc::default();
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

    // ============================
    // Bytecode tests
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;

        #[test]
        fn get_opcode_is_0x23() {
            let t = Splc::default();
            assert_eq!(t.get_opcode(), 0x23);
        }

        #[test]
        fn to_bytecode_contains_opcode_and_zero_params() {
            let t = Splc::default();
            let bc = t.to_bytecode();

            // Formato esperado: [u64 total_size_be][u32 opcode_be][u8 param_count]...
            assert!(bc.len() >= 13);

            let total_size = u64::from_be_bytes(bc[0..8].try_into().unwrap()) as usize;
            assert_eq!(total_size, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[8..12].try_into().unwrap());
            assert_eq!(opcode, 0x23);

            let param_count = bc[12] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

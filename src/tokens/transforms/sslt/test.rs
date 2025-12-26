#![cfg(feature = "test_access")]

#[cfg(test)]
mod tests {
    use crate::tokens::{ transforms::sslt::Sslt, TokenMethods };
    use crate::utils::errors::{ AtpError, AtpErrorCode };

    #[test]
    fn get_string_repr_is_sslt() {
        let t = Sslt::default();
        assert_eq!(t.get_string_repr(), "sslt");
    }

    #[test]
    fn to_atp_line_is_correctish() {
        let t = Sslt::params("_", 1).unwrap();
        // Regex Display imprime o pattern, então isso deve bater.
        assert_eq!(t.to_atp_line().as_ref(), "sslt _ 1;\n");
    }

    #[test]
    fn transform_selects_expected_piece() {
        let t = Sslt::params("_", 1).unwrap();
        assert_eq!(t.transform("foobar_foo_bar_bar_foo_barfoo"), Ok("foo".to_string()));
    }

    #[test]
    fn transform_supports_empty_segments() {
        // "a__b" split "_" => ["a", "", "b"]
        let t = Sslt::params("_", 1).unwrap();
        assert_eq!(t.transform("a__b"), Ok("".to_string()));
    }

    #[test]
    fn transform_errors_on_out_of_range() {
        let t = Sslt::params("_", 99).unwrap();

        let got = t.transform("a_b");

        let expected = Err(
            AtpError::new(
                AtpErrorCode::IndexOutOfRange("Index does not exist in the splitted vec".into()),
                t.to_atp_line(),
                "a_b".to_string()
            )
        );

        assert_eq!(got, expected);
    }

    #[test]
    fn from_vec_params_accepts_valid() {
        let mut t = Sslt::default();

        let line = vec!["sslt".to_string(), "_".to_string(), "2".to_string()];
        assert_eq!(t.from_vec_params(line), Ok(()));
        assert_eq!(t.index, 2);
        assert_eq!(t.pattern.to_string(), "_".to_string());
    }

    #[test]
    fn from_vec_params_rejects_wrong_token() {
        let mut t = Sslt::default();
        let line = vec!["nope".to_string(), "_".to_string(), "0".to_string()];

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

    #[test]
    fn from_vec_params_rejects_invalid_regex() {
        let mut t = Sslt::default();
        let line = vec!["sslt".to_string(), "(".to_string(), "0".to_string()]; // regex inválida

        let got = t.from_vec_params(line.clone());

        let expected = Err(
            AtpError::new(
                AtpErrorCode::TextParsingError("Failed to create regex".into()),
                "sslt",
                line[1].to_string()
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
        fn get_opcode_is_0x1a() {
            let t = Sslt::default();
            assert_eq!(t.get_opcode(), 0x1a);
        }

        #[test]
        fn from_params_accepts_two_params() {
            let mut t = Sslt::default();
            let params = vec![AtpParamTypes::Usize(1), AtpParamTypes::String("_".to_string())];

            assert_eq!(t.from_params(&params), Ok(()));
            assert_eq!(t.index, 1);
            assert_eq!(t.pattern.to_string(), "_".to_string());
        }

        #[test]
        fn from_params_rejects_wrong_len() {
            let mut t = Sslt::default();
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
            let t = Sslt::params("_", 1).unwrap();
            let bc = t.to_bytecode();

            // Formato: [u64 total_size_be][u32 opcode_be][u8 param_count]...
            assert!(bc.len() >= 13);

            let total_size = u64::from_be_bytes(bc[0..8].try_into().unwrap()) as usize;
            assert_eq!(total_size, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[8..12].try_into().unwrap());
            assert_eq!(opcode, 0x1a);

            let param_count = bc[12] as usize;
            assert_eq!(param_count, 2);
        }
    }
}

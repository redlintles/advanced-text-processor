#![cfg(feature = "test_access")]
#[cfg(test)]
mod tests {
    use crate::tokens::{ TokenMethods, transforms::rpt::Rpt };
    use crate::utils::errors::{ AtpError, AtpErrorCode };

    #[test]
    fn get_string_repr_is_rpt() {
        let t = Rpt::default();
        assert_eq!(t.get_string_repr(), "rpt");
    }

    #[test]
    fn to_atp_line_contains_times() {
        let t = Rpt::params(3);
        assert_eq!(t.to_atp_line().as_ref(), "rpt 3;\n");
    }

    #[test]
    fn transform_repeats_input_n_times() {
        let t = Rpt::params(3);
        assert_eq!(t.transform("banana").unwrap(), "bananabananabanana");
    }

    #[test]
    fn transform_zero_times_returns_empty_string() {
        let t = Rpt::params(0);
        assert_eq!(t.transform("banana").unwrap(), "");
    }

    #[test]
    fn transform_empty_input_still_empty() {
        let t = Rpt::params(5);
        assert_eq!(t.transform("").unwrap(), "");
    }

    #[test]
    fn from_vec_params_parses_times() {
        let mut t = Rpt::default();

        let line = vec!["rpt".to_string(), "4".to_string()];

        assert_eq!(t.from_vec_params(line), Ok(()));
        assert_eq!(t.times, 4);
        assert_eq!(t.transform("a").unwrap(), "aaaa");
    }

    #[test]
    fn from_vec_params_rejects_wrong_token() {
        let mut t = Rpt::default();

        let line = vec!["nope".to_string(), "3".to_string()];

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
    fn from_vec_params_rejects_non_numeric_times() {
        let mut t = Rpt::default();

        let line = vec!["rpt".to_string(), "NaN".to_string()];

        assert!(t.from_vec_params(line).is_err());
    }

    // Documenta comportamento atual: indexação direta -> panic se faltar argumento
    #[test]
    #[should_panic]
    fn from_vec_params_panics_if_missing_times() {
        let mut t = Rpt::default();
        let line = vec!["rpt".to_string()];
        let _ = t.from_vec_params(line);
    }

    // ============================
    // Bytecode tests
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;
        use crate::utils::params::AtpParamTypes;

        #[test]
        fn get_opcode_is_0x0d() {
            let t = Rpt::default();
            assert_eq!(t.get_opcode(), 0x0d);
        }

        #[test]
        fn from_params_parses_single_usize() {
            let mut t = Rpt::default();

            let params = vec![AtpParamTypes::Usize(5)];

            assert_eq!(t.from_params(&params), Ok(()));
            assert_eq!(t.times, 5);
        }

        #[test]
        fn from_params_rejects_wrong_param_count() {
            let mut t = Rpt::default();

            let params = vec![];

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
        fn from_params_rejects_wrong_type() {
            let mut t = Rpt::default();

            let params = vec![AtpParamTypes::String("x".to_string())];

            assert!(t.from_params(&params).is_err());
        }

        #[test]
        fn to_bytecode_contains_opcode_and_one_param() {
            let t = Rpt::params(3);
            let bc = t.to_bytecode();

            assert!(!bc.is_empty());
            assert!(bc.len() >= 13); // header mínimo

            let mut i = 0;

            let total_size = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;
            assert_eq!(total_size as usize, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(opcode, 0x0d);

            let param_count = bc[i] as usize;
            i += 1;
            assert_eq!(param_count, 1);

            // Param 1: Usize
            let _p_total = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            i += 8;

            let p_type = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap());
            i += 4;
            assert_eq!(p_type, 0x02);

            let p_size = u32::from_be_bytes(bc[i..i + 4].try_into().unwrap()) as usize;
            i += 4;
            assert_eq!(p_size, 8);

            let payload = u64::from_be_bytes(bc[i..i + 8].try_into().unwrap());
            assert_eq!(payload, 3);
        }
    }
}

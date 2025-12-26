#![cfg(feature = "test_access")]

#[cfg(test)]
mod tests {
    use crate::tokens::{ TokenMethods, transforms::dlf::Dlf };
    use crate::utils::errors::{ AtpError, AtpErrorCode };
    use crate::tokens::instructions::ifdc::Ifdc;

    #[test]
    fn to_atp_line_ok() {
        let token = Ifdc::params("xy", Box::new(Dlf::default()));
        let s = token.to_atp_line();
        assert!(s.contains("ifdc xy do"), "ifdc header ok");
    }

    #[test]
    fn transform_executes_inner_if_contains() {
        // Se Dlf faz "prefixo laranja" ou algo diferente, troque esse teste.
        // Aqui eu só testo o fluxo: contém => chama inner, não contém => retorna input
        let token = Ifdc::params("xy", Box::new(Dlf::default()));

        let a = token.transform("abcxydef");
        assert!(a.is_ok(), "contains -> inner executed (at least does not fail)");

        let b = token.transform("banana").unwrap();
        assert_eq!(b, "banana".to_string(), "does nothing when not contains");
    }

    #[test]
    fn from_vec_params_rejects_wrong_head() {
        let mut t = Ifdc::default();
        let line = vec!["nope".to_string(), "xy".to_string(), "do".to_string(), "dlf".to_string()];

        let got = t.from_vec_params(line.clone());
        let expected = Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid parser for this token".into()),
                "ifdc".to_string(),
                line.join(" ")
            )
        );

        assert_eq!(got, expected);
    }

    #[test]
    fn from_vec_params_parses_ok() {
        let mut t = Ifdc::default();
        let line = vec!["ifdc".to_string(), "xy".to_string(), "do".to_string(), "dlf".to_string()];

        assert_eq!(t.from_vec_params(line), Ok(()));
        assert_eq!(t.get_string_repr(), "ifdc");
    }

    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;
        use crate::utils::params::AtpParamTypes;

        #[test]
        fn opcode_ok() {
            let t = Ifdc::default();
            assert_eq!(t.get_opcode(), 0x33);
        }

        #[test]
        fn from_params_rejects_wrong_len() {
            let mut t = Ifdc::default();
            let params: Vec<AtpParamTypes> = vec![AtpParamTypes::String("xy".to_string())];

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
        fn from_params_accepts_string_as_first_param() {
            let mut t = Ifdc::default();
            let params: Vec<AtpParamTypes> = vec![
                AtpParamTypes::String("xy".to_string()),
                // depende de como você representa tokens no bytecode:
                AtpParamTypes::Token(Box::new(Dlf::default()))
            ];

            assert_eq!(t.from_params(&params), Ok(()));
        }
    }
}

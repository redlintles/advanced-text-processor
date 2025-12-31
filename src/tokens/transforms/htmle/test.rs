// src/tokens/transforms/htmle/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::TokenMethods;
    use crate::tokens::transforms::htmle::Htmle;

    #[test]
    fn get_string_repr_is_htmle() {
        let t = Htmle::default();
        assert_eq!(t.get_string_repr(), "htmle");
    }

    #[test]
    fn to_atp_line_is_constant() {
        let t = Htmle::default();
        assert_eq!(t.to_atp_line().as_ref(), "htmle;\n");
    }

    #[test]
    fn transform_escapes_html_like_doc_example() {
        let t = Htmle::default();
        assert_eq!(
            t.transform("<div>banana</div>"),
            Ok("&lt;div&gt;banana&lt;/div&gt;".to_string())
        );
    }

    #[test]
    fn transform_escapes_quotes_and_ampersand() {
        let t = Htmle::default();
        assert_eq!(
            t.transform(r#"<a href="x&y">"#),
            Ok("&lt;a href=&quot;x&amp;y&quot;&gt;".to_string())
        );
    }

    #[test]
    fn transform_no_special_chars_is_identity() {
        let t = Htmle::default();
        assert_eq!(t.transform("banana"), Ok("banana".to_string()));
    }

    #[test]
    fn transform_preserves_unicode_text() {
        let t = Htmle::default();
        assert_eq!(t.transform("maçã & pão"), Ok("maçã &amp; pão".to_string()));
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
        fn get_opcode_is_24() {
            let t = Htmle::default();
            assert_eq!(t.get_opcode(), 0x24);
        }

        #[test]
        fn from_params_accepts_empty_param_list() {
            let mut t = Htmle::default();
            let params: Vec<AtpParamTypes> = vec![];

            assert_eq!(t.from_params(&params), Ok(()));
        }

        #[test]
        fn from_params_rejects_any_params() {
            let mut t = Htmle::default();
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
            let t = Htmle::default();
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
            assert_eq!(opcode, 0x24);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

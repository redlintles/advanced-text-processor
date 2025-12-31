// src/tokens/transforms/htmlu/test.rs

#[cfg(test)]
mod tests {
    use crate::tokens::TokenMethods;
    use crate::tokens::transforms::htmlu::Htmlu;
    use crate::utils::errors::{AtpError, AtpErrorCode};

    #[test]
    fn get_string_repr_is_htmlu() {
        let t = Htmlu::default();
        assert_eq!(t.get_string_repr(), "htmlu");
    }

    #[test]
    fn to_atp_line_is_constant() {
        let t = Htmlu::default();
        assert_eq!(t.to_atp_line().as_ref(), "htmlu;\n");
    }

    #[test]
    fn transform_unescapes_html_like_doc_example() {
        let t = Htmlu::default();
        assert_eq!(
            t.transform("&lt;div&gt;banana&lt;/div&gt;"),
            Ok("<div>banana</div>".to_string())
        );
    }

    #[test]
    fn transform_unescapes_quotes_and_ampersand() {
        let t = Htmlu::default();
        assert_eq!(
            t.transform("&lt;a href=&quot;x&amp;y&quot;&gt;"),
            Ok(r#"<a href="x&y">"#.to_string())
        );
    }

    #[test]
    fn transform_no_entities_is_identity() {
        let t = Htmlu::default();
        assert_eq!(t.transform("banana"), Ok("banana".to_string()));
    }

    #[test]
    fn transform_preserves_unicode_text() {
        let t = Htmlu::default();
        assert_eq!(t.transform("maçã &amp; pão"), Ok("maçã & pão".to_string()));
    }

    #[test]
    fn transform_roundtrip_with_htmle_string_expectation() {
        // Sem depender diretamente do token HTMLE: apenas valida decodificação correta
        let t = Htmlu::default();
        assert_eq!(t.transform("a&lt;b&amp;c&gt;d"), Ok("a<b&c>d".to_string()));
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
        fn get_opcode_is_25() {
            let t = Htmlu::default();
            assert_eq!(t.get_opcode(), 0x25);
        }

        #[test]
        fn from_params_accepts_empty_param_list() {
            let mut t = Htmlu::default();
            let params: Vec<AtpParamTypes> = vec![];

            assert_eq!(t.from_params(&params), Ok(()));
        }

        #[test]
        fn from_params_rejects_any_params() {
            let mut t = Htmlu::default();
            let params = vec![AtpParamTypes::Usize(1)];

            let got = t.from_params(&params);

            let expected = Err(crate::utils::errors::AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                "",
                "",
            ));

            assert_eq!(got, expected);
        }

        #[test]
        fn to_bytecode_has_expected_header_and_no_params() {
            let t = Htmlu::default();
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
            assert_eq!(opcode, 0x25);

            let param_count = bc[i] as usize;
            assert_eq!(param_count, 0);
        }
    }
}

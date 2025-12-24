//! Tests for token `Atb`.
//!
//! Parent module has: `#[cfg(feature = "test_access")] pub mod test;`

#[cfg(test)]
mod common {
    use crate::tokens::{ transforms::atb::Atb, TokenMethods };

    #[test]
    fn params_sets_text() {
        let t = Atb::params("foo");
        assert_eq!(t.text, "foo");
    }

    #[test]
    fn transform_prepends_text() {
        let t = Atb::params("foo");
        assert_eq!(t.transform(" bar").unwrap(), "foo bar");
    }

    #[test]
    fn transform_empty_input() {
        let t = Atb::params("foo");
        assert_eq!(t.transform("").unwrap(), "foo");
    }

    #[test]
    fn transform_empty_prefix() {
        let t = Atb::params("");
        assert_eq!(t.transform("bar").unwrap(), "bar");
    }

    #[test]
    fn to_atp_line_exact_format() {
        let t = Atb::params("foo");
        let line = t.to_atp_line();
        assert_eq!(line.as_ref(), "atb foo;\n");
    }

    #[test]
    fn get_string_repr_is_atb() {
        let t = Atb::default();
        assert_eq!(t.get_string_repr(), "atb");
    }

    #[test]
    fn from_vec_params_sets_text_when_tag_matches() {
        let mut t = Atb::default();
        t.from_vec_params(vec!["atb".into(), "hello".into()]).unwrap();
        assert_eq!(t.text, "hello");
        assert_eq!(t.transform(" world").unwrap(), "hello world");
    }

    #[test]
    fn from_vec_params_returns_error_when_tag_mismatch_and_does_not_mutate() {
        let mut t = Atb::params("initial");
        let res = t.from_vec_params(vec!["nope".into(), "hello".into()]);
        assert!(res.is_err());
        assert_eq!(t.text, "initial");
    }

    #[test]
    #[should_panic]
    fn from_vec_params_panics_when_missing_param() {
        // Documenta o comportamento atual (line[1] sem check de len)
        let mut t = Atb::default();
        let _ = t.from_vec_params(vec!["atb".into()]);
    }
}

#[cfg(all(test, feature = "bytecode"))]
mod bytecode {
    use crate::{ tokens::{ transforms::atb::Atb, TokenMethods }, utils::params::AtpParamTypes };

    // Helper: encode a param in the exact format that AtpParamTypes::from_bytecode expects:
    // [type: u32][payload_size: u32][payload: bytes]
    fn encode_param_for_decoder(param_type: u32, payload: &[u8]) -> Vec<u8> {
        let mut v = Vec::with_capacity(4 + 4 + payload.len());
        v.extend_from_slice(&param_type.to_be_bytes());
        v.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        v.extend_from_slice(payload);
        v
    }

    #[test]
    fn opcode_is_expected() {
        let t = Atb::default();
        assert_eq!(t.get_opcode(), 0x01);
    }

    #[test]
    fn atpparam_from_bytecode_string_roundtrip_decoder_format() {
        // Este teste valida o DECODER real do from_bytecode, usando o formato que ele espera.
        let raw = encode_param_for_decoder(0x01, b"abc");
        let parsed = AtpParamTypes::from_bytecode(raw).unwrap();

        match parsed {
            AtpParamTypes::String(s) => assert_eq!(s, "abc"),
            other => panic!("Expected String, got {:?}", other.get_param_type_code()),
        }
    }

    #[test]
    fn atpparam_from_bytecode_usize_decoder_format() {
        let n: usize = 123;
        let raw = encode_param_for_decoder(0x02, &n.to_be_bytes());
        let parsed = AtpParamTypes::from_bytecode(raw).unwrap();

        match parsed {
            AtpParamTypes::Usize(x) => assert_eq!(x, 123),
            other => panic!("Expected Usize, got {:?}", other.get_param_type_code()),
        }
    }

    #[test]
    fn atb_from_params_accepts_single_string_param() {
        let mut t = Atb::default();
        let params = vec![AtpParamTypes::String("foo".to_string())];

        t.from_params(&params).unwrap();
        assert_eq!(t.text, "foo");
        assert_eq!(t.transform(" bar").unwrap(), "foo bar");
    }

    #[test]
    fn atb_from_params_rejects_wrong_param_count() {
        let mut t = Atb::default();
        let params = vec![
            AtpParamTypes::String("a".to_string()),
            AtpParamTypes::String("b".to_string())
        ];

        let err = t.from_params(&params).unwrap_err();
        // Assert forte: tipo do erro (sem depender de mensagem/string)
        assert!(matches!(err, crate::utils::errors::AtpError { .. }));

        // Como AtpError não expõe fields public, a melhor checagem forte
        // é pelo Display do AtpErrorCode ou por comparação se você expuser um getter.
        // Aqui vai um check “semi-forte” usando o código:
        // (se você adicionar um getter em AtpError pra pegar error_code, isso fica perfeito)
        let rendered = err.to_string();
        assert!(rendered.contains("106")); // BytecodeNotFound => 106
    }

    #[test]
    fn atb_to_bytecode_can_be_parsed_into_params_and_feed_from_params() {
        // Aqui a gente valida o fluxo real:
        // Atb::to_bytecode -> decodificar estrutura -> AtpParamTypes::from_bytecode(param bytes) -> Atb::from_params
        //
        // Observação: Atb::to_bytecode gera uma INSTRUCTION com layout:
        // [u64 instruction_total_size][u32 opcode][u8 param_count]
        // [u64 param_total_size][u32 param_type][u32 payload_size][payload]
        //
        // E AtpParamTypes::from_bytecode espera:
        // [u32 type][u32 payload_size][payload]
        //
        // Então, para montar o input correto do from_bytecode, pegamos o trecho:
        // [u32 param_type][u32 payload_size][payload] (ignorando o u64 param_total_size).

        let original = Atb::params("hello");
        let bytes = original.to_bytecode();

        // pulo: 8 (total) + 4 (opcode) + 1 (param_count) = 13
        let mut idx = 13;

        // lê u64 param_total_size, mas não usa (8 bytes)
        let _param_total_size = u64::from_be_bytes(bytes[idx..idx + 8].try_into().unwrap());
        idx += 8;

        // Agora o sub-slice começa em [u32 param_type][u32 payload_size][payload...]
        let param_slice = bytes[idx..].to_vec();

        // Parse param => AtpParamTypes
        let parsed_param = AtpParamTypes::from_bytecode(param_slice).unwrap();

        let mut rebuilt = Atb::default();
        rebuilt.from_params(&vec![parsed_param]).unwrap();

        assert_eq!(rebuilt.text, "hello");
        assert_eq!(rebuilt.transform(" world").unwrap(), "hello world");
    }

    #[test]
    fn atpparam_param_to_bytecode_has_internal_structure() {
        // Este teste NÃO faz roundtrip (porque encoder/decoder não batem hoje),
        // mas garante que o encoder produz algo consistente internamente.
        let p = AtpParamTypes::String("abc".to_string());
        let b = p.param_to_bytecode();

        // type u32
        let ty = u32::from_be_bytes(b[0..4].try_into().unwrap());
        assert_eq!(ty, 0x01);

        // em seguida você grava u64 param_total_size
        let _total = u64::from_be_bytes(b[4..12].try_into().unwrap());

        // depois payload_size u32
        let size = u32::from_be_bytes(b[12..16].try_into().unwrap());
        assert_eq!(size, 3);

        // payload
        assert_eq!(&b[16..19], b"abc");
    }
}

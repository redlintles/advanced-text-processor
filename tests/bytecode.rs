#[cfg(feature = "test_access")]
#[cfg(test)]
pub mod bytecode {
    use std::{ fs::File, io::Read, path::Path };

    use atp::{ api::atp_processor::AtpProcessorMethods, tokens::TokenMethods };

    #[cfg(test)]
    mod write_bytecode_to_file_tests {
        use std::borrow::Cow;
        use std::fs;
        use std::path::PathBuf;

        use atp::bytecode::writer::write_bytecode_to_file;
        use atp::tokens::TokenMethods;

        use tempfile::tempdir;

        /// Minimal dummy token for these tests.
        #[derive(Clone)]
        struct DummyToken {
            atp: String,
            bc: Vec<u8>,
        }

        impl DummyToken {
            fn new(atp: &str, bc: &[u8]) -> Self {
                Self {
                    atp: atp.into(),
                    bc: bc.to_vec(),
                }
            }
        }

        impl atp::tokens::TokenMethods for DummyToken {
            fn to_bytecode(&self) -> Vec<u8> {
                self.bc.clone()
            }

            fn to_atp_line(&self) -> Cow<'static, str> {
                self.atp.clone().into()
            }

            fn transform(&self, _input: &str) -> Result<String, atp::utils::errors::AtpError> {
                todo!()
            }

            fn get_string_repr(&self) -> &'static str {
                todo!()
            }

            fn from_params(
                &mut self,
                _instruction: &Vec<atp::utils::params::AtpParamTypes>
            ) -> Result<(), atp::utils::errors::AtpError> {
                todo!()
            }

            fn get_opcode(&self) -> u32 {
                todo!()
            }
        }

        fn parse_header(bytes: &[u8]) -> (Vec<u8>, u64, u32, &[u8]) {
            assert!(bytes.len() >= 20);
            let magic = bytes[0..8].to_vec();
            let protocol = u64::from_be_bytes(bytes[8..16].try_into().unwrap());
            let count = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
            let rest = &bytes[20..];
            (magic, protocol, count, rest)
        }

        /// Helper: create an empty file first so `canonicalize()` doesn't fail in `check_file_path`.
        fn touch(path: &PathBuf) {
            // create(true) + truncate(true) ensures it exists and is empty
            fs::OpenOptions::new().create(true).truncate(true).write(true).open(path).unwrap();
        }

        #[test]
        fn writes_header_and_all_token_bytecodes_in_order() {
            let dir = tempdir().unwrap();
            let path: PathBuf = dir.path().join("out.atpbc");

            // IMPORTANT: your check_file_path canonicalizes, so the file must exist already
            touch(&path);

            let tokens: Vec<Box<dyn TokenMethods>> = vec![
                Box::new(DummyToken::new("tok1", &[0xaa, 0xbb])),
                Box::new(DummyToken::new("tok2", &[0x10])),
                Box::new(DummyToken::new("tok3", &[])),
                Box::new(DummyToken::new("tok4", &[0xff, 0x00, 0x01]))
            ];

            write_bytecode_to_file(&path, tokens).unwrap();

            let bytes = fs::read(&path).unwrap();
            let (magic, protocol, count, rest) = parse_header(&bytes);

            let expected_magic = vec![38, 235, 245, 8, 244, 137, 1, 179];
            assert_eq!(magic, expected_magic);
            assert_eq!(protocol, 1);
            assert_eq!(count, 4);

            let expected_payload: Vec<u8> = vec![
                0xaa,
                0xbb, // tok1
                0x10, // tok2
                // tok3 empty
                0xff,
                0x00,
                0x01 // tok4
            ];
            assert_eq!(rest, expected_payload.as_slice());
        }

        #[test]
        fn instruction_count_is_zero_when_no_tokens() {
            let dir = tempdir().unwrap();
            let path: PathBuf = dir.path().join("empty.atpbc");

            // same reason: canonicalize requires existence
            touch(&path);

            let tokens: Vec<Box<dyn TokenMethods>> = vec![];
            write_bytecode_to_file(&path, tokens).unwrap();

            let bytes = fs::read(&path).unwrap();
            let (_magic, _protocol, count, rest) = parse_header(&bytes);

            assert_eq!(count, 0);
            assert!(rest.is_empty(), "no tokens => no payload bytes");
        }

        #[test]
        fn invalid_extension_is_rejected_by_check_file_path() {
            let dir = tempdir().unwrap();
            let path: PathBuf = dir.path().join("wrong.txt");

            // Create it anyway, because your validator canonicalizes first.
            // This ensures the error is about extension, not about "no such file".
            touch(&path);

            let tokens: Vec<Box<dyn TokenMethods>> = vec![
                Box::new(DummyToken::new("tok", &[1, 2, 3]))
            ];

            let err = write_bytecode_to_file(&path, tokens).unwrap_err();

            let msg = format!("{err:?}");
            assert!(
                msg.contains("ValidationError") ||
                    msg.contains("check_file_path") ||
                    msg.contains("atpbc"),
                "expected an extension/path validation error, got: {msg}"
            );
        }

        #[test]
        fn directory_path_is_rejected_by_check_file_path() {
            let dir = tempdir().unwrap();
            let path_is_dir: PathBuf = dir.path().join("some_dir.atpbc");

            fs::create_dir_all(&path_is_dir).unwrap();

            let tokens: Vec<Box<dyn TokenMethods>> = vec![Box::new(DummyToken::new("tok", &[1]))];

            let err = write_bytecode_to_file(&path_is_dir, tokens).unwrap_err();

            // Your real behavior: ValidationError("Path is a directory, not a file")
            let msg = format!("{err:?}");
            assert!(
                msg.contains("Path is a directory") || msg.contains("ValidationError"),
                "expected directory validation error, got: {msg}"
            );
        }
    }
    #[test]
    fn test_write_bytecode_to_file() {
        use atp::bytecode::writer::write_bytecode_to_file;
        use atp::tokens::transforms::{ atb::Atb, ate::Ate, rpt::Rpt };
        use tempfile::Builder;
        let file = Builder::new().suffix(".atpbc").prefix("output_").tempfile().unwrap();

        let path = file.path();

        let tokens: Vec<Box<dyn TokenMethods>> = vec![
            Box::new(Atb {
                text: "Banana".to_string(),
            }),
            Box::new(Ate {
                text: "Laranja".to_string(),
            }),
            Box::new(Rpt { times: 3 as usize })
        ];

        let _ = write_bytecode_to_file(path, tokens);

        let mut opened_file = File::open(path).unwrap();

        let mut content = String::new();
        opened_file.read_to_string(&mut content).unwrap();

        let expected_content = "0x01 Banana\n0x02 Laranja\n0x0d 3\n";

        assert_eq!(
            content,
            expected_content,
            "Unexpected Output in test_write_to_file: content differs"
        );
    }

    #[test]
    fn test_read_bytecode_from_file() {
        use atp::{ api::atp_processor::AtpProcessor, bytecode::reader::read_bytecode_from_file };
        let tokens = match read_bytecode_from_file(Path::new("./banana.atpbc")) {
            Ok(x) => x,
            Err(e) => panic!("{}", format!("{}", e)),
        };

        let input = "Coxinha";

        let expected_output = "BananaCoxinhaLaranjaBananaCoxinhaLaranjaBananaCoxinhaLaranja";

        let mut processor: Box<dyn AtpProcessorMethods> = Box::new(AtpProcessor::new());

        let identifier = processor.add_transform(tokens.to_vec());

        let output = processor.process_all_bytecode_with_debug(&identifier, input).unwrap();

        assert_eq!(output, expected_output);
    }
}

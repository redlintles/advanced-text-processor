#[cfg(feature = "test_access")]
#[cfg(test)]
pub mod bytecode {
    use std::{ fs::File, io::Read };

    use atp::{ tokens::{ InstructionMethods, transforms::rpt } };

    #[cfg(test)]
    mod write_bytecode_to_file_tests {
        use std::borrow::Cow;
        use std::fs;
        use std::path::PathBuf;

        use atp::bytecode::writer::write_bytecode_to_file;
        use atp::context::execution_context::{ GlobalExecutionContext };
        use atp::tokens::InstructionMethods;

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

        impl atp::tokens::InstructionMethods for DummyToken {
            fn to_bytecode(&self) -> Vec<u8> {
                self.bc.clone()
            }

            fn to_atp_line(&self) -> Cow<'static, str> {
                self.atp.clone().into()
            }

            fn transform(
                &self,
                _input: &str,
                _context: &mut GlobalExecutionContext
            ) -> Result<String, atp::utils::errors::AtpError> {
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

            let tokens: Vec<Box<dyn InstructionMethods>> = vec![
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

            let tokens: Vec<Box<dyn InstructionMethods>> = vec![];
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

            let tokens: Vec<Box<dyn InstructionMethods>> = vec![
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

            let tokens: Vec<Box<dyn InstructionMethods>> = vec![
                Box::new(DummyToken::new("tok", &[1]))
            ];

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

        let tokens: Vec<Box<dyn InstructionMethods>> = vec![
            Box::new(Atb {
                text: "Banana".to_string(),
            }),
            Box::new(Ate {
                text: "Laranja".to_string(),
            }),
            Box::new(Rpt { times: 3 as usize })
        ];

        let mut header: Vec<u8> = Vec::new();

        let magic_number: Vec<u8> = vec![38, 235, 245, 8, 244, 137, 1, 179];

        let protocol_version = (1 as u64).to_be_bytes();

        let instruction_count = (tokens.len() as u32).to_be_bytes();

        header.extend_from_slice(&magic_number);
        header.extend_from_slice(&protocol_version);
        header.extend_from_slice(&instruction_count);

        let mut expected_content: Vec<u8> = vec![];

        expected_content.extend_from_slice(&header);

        for t in tokens.iter() {
            expected_content.extend_from_slice(&t.to_bytecode());
        }
        let _ = write_bytecode_to_file(path, tokens);

        let mut opened_file = File::open(path).unwrap();

        let mut content: Vec<u8> = Vec::new();
        opened_file.read_to_end(&mut content).unwrap();

        assert_eq!(
            content,
            expected_content,
            "Unexpected Output in test_write_to_file: content differs"
        );
    }

    #[test]
    fn test_read_bytecode_from_file() {
        use std::path::Path;
        use tempfile::Builder;

        use atp::{
            api::atp_processor::{ AtpProcessor, AtpProcessorMethods },
            bytecode::{ reader::read_bytecode_from_file, writer::write_bytecode_to_file },
            tokens::transforms::{ atb, ate }, // ajuste pros tokens reais que você quer
            tokens::InstructionMethods,
        };

        // 1) cria tokens em memória (exemplo)
        let tokens: Vec<Box<dyn InstructionMethods>> = vec![
            Box::new(atb::Atb::params("Banana")),
            Box::new(ate::Ate::params("Laranja")),
            Box::new(rpt::Rpt::params(3))
        ];

        let tmp = Builder::new().prefix("banana_").suffix(".atpbc").tempfile().unwrap();

        let file_path = tmp.path().to_path_buf();
        write_bytecode_to_file(Path::new(&file_path), tokens).unwrap();

        use std::fs;

        let data = fs::read(&file_path).unwrap();
        eprintln!("len = {}", data.len());
        eprintln!("header bytes = {:02x?}", &data[..(20).min(data.len())]); // 8+8+4 = 20
        eprintln!("first body bytes = {:02x?}", &data[20..(20 + 16).min(data.len())]);

        assert!(file_path.exists(), "writer não criou o arquivo: {:?}", file_path);

        // 3) lê de volta
        let read_tokens = read_bytecode_from_file(Path::new(&file_path)).unwrap();

        // 4) processa
        let input = "Coxinha";
        let expected_output = "BananaCoxinhaLaranjaBananaCoxinhaLaranjaBananaCoxinhaLaranja";

        let mut processor: Box<dyn AtpProcessorMethods> = Box::new(AtpProcessor::new());
        println!("read_tokens len {}", read_tokens.len());
        let identifier = processor.add_transform(read_tokens);

        let output = processor.process_all_bytecode_with_debug(&identifier, input).unwrap();

        assert_eq!(output, expected_output);
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
pub mod processor {
    use std::{ path::Path };

    use atp::{
        api::{ AtpBuilderMethods, atp_processor::{ AtpProcessor, AtpProcessorMethods } },
        tokens::{ InstructionMethods, transforms::{ atb::Atb, ate::Ate, raw::Raw, rpt::Rpt } },
        utils::errors::AtpError,
    };
    use uuid::Uuid;

    #[test]
    fn test_process_all() -> Result<(), AtpError> {
        let mut processor = AtpProcessor::new();
        let identifier = processor
            .create_pipeline()
            .add_to_beginning("Banana")?
            .add_to_end("Laranja")?
            .repeat(3 as usize)?
            .build();
        let input = "Carimbo verde de deus";

        let output = processor.process_all(&identifier, input)?;

        let expected_output =
            "BananaCarimbo verde de deusLaranjaBananaCarimbo verde de deusLaranjaBananaCarimbo verde de deusLaranja";

        assert_eq!(output, expected_output, "Unexpected output in process_all");

        Ok(())
    }
    #[test]
    fn test_process_all_with_debug() -> Result<(), AtpError> {
        let mut processor = AtpProcessor::new();
        let identifier = processor
            .create_pipeline()
            .add_to_beginning("Banana")?
            .add_to_end("Laranja")?
            .repeat(3 as usize)?
            .build();
        let input = "Carimbo verde de deus";

        let output = processor.process_all_with_debug(&identifier, input)?;

        let expected_output =
            "BananaCarimbo verde de deusLaranjaBananaCarimbo verde de deusLaranjaBananaCarimbo verde de deusLaranja";

        assert_eq!(output, expected_output, "Unexpected output in process_all");

        Ok(())
    }

    #[test]
    fn test_process_single() -> Result<(), AtpError> {
        let mut processor = AtpProcessor::new();
        let token: Box<dyn InstructionMethods> = Box::new(
            Raw::params("a", "b").map_err(|e|
                AtpError::new(
                    atp::utils::errors::AtpErrorCode::TextParsingError("".into()),
                    "",
                    e.to_string()
                )
            )?
        );

        let input = "a".repeat(100);

        let output = processor.process_single(token, &input)?;

        let expected_output = "b".repeat(100);

        assert_eq!(output, expected_output);

        Ok(())
    }
    #[test]
    fn test_process_single_with_debug() -> Result<(), AtpError> {
        let mut processor: Box<dyn AtpProcessorMethods> = Box::new(AtpProcessor::new());
        let token: Box<dyn InstructionMethods> = Box::new(
            Raw::params("a", "b").map_err(|e|
                AtpError::new(
                    atp::utils::errors::AtpErrorCode::TextParsingError("".into()),
                    "",
                    e.to_string()
                )
            )?
        );

        let input = "a".repeat(100);

        let output = processor.process_single_with_debug(token, &input)?;

        let expected_output = "b".repeat(100);

        assert_eq!(output, expected_output);

        Ok(())
    }

    #[test]
    fn test_read_from_file() -> Result<(), AtpError> {
        let mut processor = AtpProcessor::new();

        let identifier = processor.read_from_text_file(Path::new("instructions.atp"))?;

        let input_string = "Banana";
        let expected_output = "BznzbonanzanzBznznz";

        let output = processor.process_all(&identifier, input_string)?;

        println!("{} => {} == {}", input_string, output, expected_output);

        assert_eq!(output, expected_output, "Unexpected Output in read_from_file");

        Ok(())
    }

    #[test]
    fn test_write_to_file() -> Result<(), AtpError> {
        use std::fs::File;
        use std::io::Read;

        use tempfile::Builder;

        let file = Builder::new().suffix(".atp").tempfile().expect("Error opening archive");

        let path = file.path();

        let mut processor = AtpProcessor::new();
        let identifier = processor
            .create_pipeline()
            .add_to_beginning("Banana")?
            .add_to_end("Laranja")?
            .repeat(3usize)?
            .build();

        // ✅ não ignore o erro
        processor.write_to_text_file(&identifier, path)?;

        let mut opened_file = File::open(path).unwrap();
        let mut content = String::new();
        opened_file.read_to_string(&mut content).unwrap();

        // ✅ normaliza CRLF só por segurança
        let content = content.replace("\r\n", "\n");

        let expected_content = "atb Banana;\nate Laranja;\nrpt 3;\n";

        assert_eq!(
            content,
            expected_content,
            "Unexpected Output in test_write_to_file: content differs"
        );
        Ok(())
    }

    #[test]
    fn test_add_transform() {
        use uuid::Variant;
        let mut tokens: Vec<Box<dyn InstructionMethods>> = Vec::new();

        tokens.push(Box::new(Atb::params("Banana")));
        tokens.push(Box::new(Ate::params("Laranja")));
        tokens.push(Box::new(Rpt::params(3)));

        let mut processor = AtpProcessor::new();

        let identifier = processor.add_transform(tokens);

        let parsed = Uuid::parse_str(&identifier).expect(
            "Unexpected output in test_add_transform: Non valid UUID generated by add_transform"
        );

        assert_eq!(
            parsed.get_variant(),
            Variant::RFC4122,
            "Unexpected output in test_add_transform: Non UUID_V4"
        );
        assert_eq!(
            parsed.get_version_num(),
            4 as usize,
            "Unexpected output in test_add_transform: UUID is from different version"
        );
    }
}

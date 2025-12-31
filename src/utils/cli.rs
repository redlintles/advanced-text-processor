use std::borrow::Cow;

use crate::{
    builder::atp_processor::{ AtpProcessor, AtpProcessorMethods },
    utils::{ errors::AtpError, transforms::get_safe_utf8_char_index },
};

fn process_run(
    processor: &mut AtpProcessor,
    identifier: &str,
    input: &str,
    debug: bool
) -> Result<String, AtpError> {
    if debug {
        return Ok(processor.process_all_with_debug(identifier, input)?);
    } else {
        return Ok(processor.process_all(identifier, input)?);
    }
}

pub fn process_input_single_chunk(
    processor: &mut AtpProcessor,
    identifier: &str,
    input: &str,
    debug: bool
) -> Result<String, AtpError> {
    if input.is_empty() {
        return Ok(String::new());
    }
    Ok(process_run(processor, identifier, input, debug)?)
}

pub fn process_input_line_by_line(
    processor: &mut AtpProcessor,
    identifier: &str,
    input: &str,
    debug: bool
) -> Result<String, AtpError> {
    if input.is_empty() {
        return Ok(String::new());
    }
    let mut text_vec = input
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    for line in text_vec.iter_mut() {
        *line = process_run(processor, identifier, line, debug)?;
    }

    Ok(text_vec.join("\n"))
}

pub fn process_input_by_chunks(
    processor: &mut AtpProcessor,
    identifier: &str,
    input: &str,
    chunk_size: usize,
    debug: bool
) -> Result<String, AtpError> {
    if input.is_empty() {
        return Ok(String::new());
    }
    if chunk_size == 0 {
        return Err(
            AtpError::new(
                super::errors::AtpErrorCode::ZeroDivisionError("chunk size == 0".into()),
                Cow::Borrowed("process_input_by_chunks"),
                Cow::Owned(input.to_string())
            )
        );
    }
    let character_count = input.chars().count();
    let iterations = character_count.div_ceil(chunk_size);

    let mut processed = Vec::new();

    for i in 0..iterations - 1 {
        let left_chunk_bound = get_safe_utf8_char_index(i * chunk_size, input)?;
        let right_chunk_bound = get_safe_utf8_char_index((i + 1) * chunk_size, input)?;

        let result = process_run(
            processor,
            identifier,
            &input[left_chunk_bound..right_chunk_bound],
            debug
        )?;

        processed.push(result);
    }

    // Último chunk
    let left_chunk_bound = get_safe_utf8_char_index((iterations - 1) * chunk_size, input)?;
    let result = process_run(processor, identifier, &input[left_chunk_bound..], debug)?;
    processed.push(result);

    Ok(processed.join(""))
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod cli_tests {
    mod process_input_by_chunks_tests {
        use crate::{
            builder::{ atp_builder::{ AtpBuilder }, AtpBuilderMethods },
            utils::{ cli::process_input_by_chunks, errors::AtpError },
        };

        #[test]
        fn it_works_correctly() -> Result<(), AtpError> {
            let (mut processor, id) = AtpBuilder::new()
                .add_to_beginning("b")?
                .add_to_end("l")?
                .build();

            let input = "coxinha";
            let expected_output = "bcolbxilbnhlbal".to_string();
            let result = process_input_by_chunks(&mut processor, &id, input, 2, true)?;

            println!("Resultado: {}", result);

            assert_eq!(result, expected_output, "It works correctly");
            Ok(())
        }
    }

    mod process_input_line_by_line_tests {
        use crate::{
            builder::{ atp_builder::{ AtpBuilder }, AtpBuilderMethods },
            utils::{ cli::process_input_line_by_line, errors::AtpError },
        };

        #[test]
        fn it_works_correctly() -> Result<(), AtpError> {
            let (mut processor, id) = AtpBuilder::new()
                .add_to_beginning("b")?
                .add_to_end("l")?
                .build();

            let input = "coxinha\nlaranja";
            let expected_output = "bcoxinhal\nblaranjal".to_string();
            let result = process_input_line_by_line(&mut processor, &id, input, true)?;

            println!("Resultado: {}", result);

            assert_eq!(result, expected_output, "It works correctly");
            Ok(())
        }
    }

    mod process_input_single_chunk_tests {
        use crate::{
            builder::{ atp_builder::{ AtpBuilder }, AtpBuilderMethods },
            utils::{ cli::process_input_single_chunk, errors::AtpError },
        };

        #[test]
        fn it_works_correctly() -> Result<(), AtpError> {
            let (mut processor, id) = AtpBuilder::new()
                .add_to_beginning("b")?
                .add_to_end("l")?
                .build();

            let input = "coxinha";
            let expected_output = "bcoxinhal".to_string();
            let result = process_input_single_chunk(&mut processor, &id, input, true)?;

            println!("Resultado: {}", result);

            assert_eq!(result, expected_output, "It works correctly");
            Ok(())
        }
    }
}

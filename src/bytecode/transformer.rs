use std::{ fs::OpenOptions, io::Write, path::Path };

use crate::{
    text::reader::read_from_file,
    tokens::TokenMethods,
    utils::{ errors::AtpError, transforms::token_to_bytecode_token, validations::check_file_path },
};

use super::{ reader::read_bytecode_from_file };

pub fn atp_text_to_bytecode_file(input_file: &Path, output_file: &Path) -> Result<(), AtpError> {
    check_file_path(input_file, Some("atp"))?;
    check_file_path(output_file, Some("atpbc"))?;

    let tokens: Vec<Box<dyn TokenMethods>> = read_from_file(input_file)?;

    let mut new_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_file)
        .map_err(|_|
            AtpError::new(
                crate::utils::errors::AtpErrorCode::FileOpeningError("Failed opening File".into()),
                "",
                format!("{:?}", output_file)
            )
        )?;

    for token in tokens.into_iter() {
        let line = token_to_bytecode_token(&token)?
            .token_to_bytecode_instruction()
            .to_bytecode_line();

        match new_file.write(line.as_bytes()) {
            Ok(_) => (),
            Err(_) => {
                return Err(
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::FileWritingError(
                            "Failed writing text to atp file".into()
                        ),
                        "",
                        line
                    )
                );
            }
        }
    }
    Ok(())
}

pub fn atp_bytecode_to_atp_file(input_file: &Path, output_file: &Path) -> Result<(), AtpError> {
    check_file_path(input_file, Some("atpbc"))?;
    check_file_path(output_file, Some("atp"))?;

    let tokens = read_bytecode_from_file(input_file)?;

    let mut new_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_file)
        .map_err(|_|
            AtpError::new(
                crate::utils::errors::AtpErrorCode::FileOpeningError("Failed opening File".into()),
                "",
                format!("{:?}", output_file)
            )
        )?;

    for token in tokens.into_iter() {
        let line = token.to_atp_line();

        match new_file.write(line.as_bytes()) {
            Ok(_) => (),
            Err(_) => {
                return Err(
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::FileWritingError(
                            "Failed writing text to atp file".into()
                        ),
                        "",
                        line
                    )
                );
            }
        }
    }

    Ok(())
}

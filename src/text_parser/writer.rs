use std::{ fs::OpenOptions, io::Write, path::Path };

use crate::{ token_data::TokenMethods, utils::{ errors::AtpError, validations::check_file_path } };

pub fn write_to_file(path: &Path, tokens: &Vec<Box<dyn TokenMethods>>) -> Result<(), AtpError> {
    check_file_path(path, Some("atp"))?;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|_|
            AtpError::new(
                crate::utils::errors::AtpErrorCode::FileOpeningError(
                    "Failed opening File".to_string()
                ),
                "".to_string(),
                format!("{:?}", path)
            )
        )?;

    let mut success = true;

    for token in tokens.iter() {
        let line = token.token_to_atp_line();

        match file.write_all(line.as_bytes()) {
            Ok(_) => (),
            Err(_) => {
                success = false;
            }
        }
    }

    match success {
        true => Ok(()),
        false =>
            Err(
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::FileWritingError(
                        "Failed writing text to atp file".to_string()
                    ),
                    "".to_string(),
                    "".to_string()
                )
            ),
    }
}

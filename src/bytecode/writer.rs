use std::{ fs::OpenOptions, io::Write, path::Path };

use crate::{ tokens::TokenMethods, utils::{ errors::AtpError, validations::check_file_path } };

pub fn write_bytecode_to_file(
    path: &Path,
    tokens: Vec<Box<dyn TokenMethods>>
) -> Result<(), AtpError> {
    check_file_path(path, Some("atpbc"))?;

    let mut file = match OpenOptions::new().create(true).truncate(true).write(true).open(path) {
        Ok(x) => x,
        Err(_) => {
            return Err(
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::FileOpeningError(
                        "Failed opening File".into()
                    ),
                    "",
                    format!("{:?}", path)
                )
            );
        }
    };

    let mut header: Vec<u8> = Vec::new();

    let magic_number: Vec<u8> = vec![38, 235, 245, 8, 244, 137, 1, 179];

    let protocol_version = (1 as u64).to_be_bytes();

    let instruction_count = (tokens.len() as u32).to_be_bytes();

    header.extend_from_slice(&magic_number);
    header.extend_from_slice(&protocol_version);
    header.extend_from_slice(&instruction_count);

    match file.write(&header) {
        Ok(_) => (),
        Err(_) => {
            return Err(
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::FileWritingError(
                        "Failed writing text to atp file".into()
                    ),
                    "Write bytecode to file",
                    "Header writing error"
                )
            );
        }
    }

    for token in tokens.iter() {
        let line = token.to_bytecode();

        match file.write(&line) {
            Ok(_) => (),
            Err(_) => {
                return Err(
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::FileWritingError(
                            "Failed writing text to atp file".into()
                        ),
                        "Write bytecode to file",
                        token.to_atp_line()
                    )
                );
            }
        }
    }

    Ok(())
}

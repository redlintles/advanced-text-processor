use std::{ fs::OpenOptions, io::Write, path::Path };

use crate::utils::{ errors::AtpError, validations::check_file_path };

use super::BytecodeTokenMethods;

pub fn write_bytecode_to_file(
    path: &Path,
    tokens: Vec<Box<dyn BytecodeTokenMethods>>
) -> Result<(), AtpError> {
    check_file_path(path, Some("atpbc"))?;

    let mut file = match OpenOptions::new().create(true).truncate(true).write(true).open(path) {
        Ok(x) => x,
        Err(_) => {
            return Err(
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::FileOpeningError(
                        "Failed opening File".to_string()
                    ),
                    "".to_string(),
                    format!("{:?}", path)
                )
            );
        }
    };

    for token in tokens.iter() {
        let line = token.token_to_bytecode_instruction().to_bytecode_line();

        match file.write(line.as_bytes()) {
            Ok(_) => (),
            Err(_) => {
                return Err(
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::FileWritingError(
                            "Failed writing text to atp file".to_string()
                        ),
                        "".to_string(),
                        line.to_string()
                    )
                );
            }
        }
    }

    Ok(())
}

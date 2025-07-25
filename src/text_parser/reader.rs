use std::{ fs::OpenOptions, io::{ BufRead, BufReader }, path::Path };

use crate::{
    token_data::TokenMethods,
    utils::{
        errors::{ AtpError, AtpErrorCode },
        mapping::get_supported_default_tokens,
        validations::check_file_path,
    },
};

pub fn read_from_file(path: &Path) -> Result<Vec<Box<dyn TokenMethods>>, AtpError> {
    check_file_path(path, Some("atp"))?;
    let mut result: Vec<Box<dyn TokenMethods>> = Vec::new();

    let file = match OpenOptions::new().read(true).open(path) {
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

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut line_text = match line {
            Ok(x) => x,
            Err(_) => {
                return Err(
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::FileReadingError(
                            "Failed reading file line".to_string()
                        ),
                        "".to_string(),
                        "".to_string()
                    )
                );
            }
        };

        line_text.pop();

        let chunks = match shell_words::split(&line_text) {
            Ok(x) => x,
            Err(_) => {
                return Err(
                    AtpError::new(
                        AtpErrorCode::TextParsingError(
                            "An ATP Parsing error ocurred: Error splitting file line".to_string()
                        ),
                        "shell words split".to_string(),
                        line_text.to_string()
                    )
                );
            }
        };

        let supported_tokens = get_supported_default_tokens();
        let token_factory = match supported_tokens.get(&chunks[0]) {
            Some(x) => x,
            None => {
                return Err(
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::TokenNotFound(
                            "Token not recognized".to_string()
                        ),
                        "".to_string(),
                        chunks[0].to_string()
                    )
                );
            }
        };

        let mut token = token_factory();

        match token.from_vec_params(chunks) {
            Ok(x) => x,
            Err(_) => {
                return Err(
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::TextParsingError(
                            "Token not recognized".to_string()
                        ),
                        "".to_string(),
                        "".to_string()
                    )
                );
            }
        }

        result.push(token);
    }

    Ok(result)
}

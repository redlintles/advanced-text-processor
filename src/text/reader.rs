use std::{ fs::OpenOptions, io::{ BufRead, BufReader }, path::Path };

use crate::{
    tokens::TokenMethods,
    utils::{
        errors::{ AtpError, AtpErrorCode },
        mapping::get_supported_default_tokens,
        validations::check_file_path,
    },
};

pub fn read_from_text(token_string: &str) -> Result<Box<dyn TokenMethods>, AtpError> {
    let chunks = match
        shell_words::split(
            &token_string
                .strip_suffix(";")
                .ok_or_else(||
                    AtpError::new(
                        AtpErrorCode::TextParsingError(
                            "An ATP Parsing error ocurred: Error splitting file line".into()
                        ),
                        "shell words split",
                        token_string.to_string()
                    )
                )?
        )
    {
        Ok(x) => x,
        Err(_) => {
            return Err(
                AtpError::new(
                    AtpErrorCode::TextParsingError(
                        "An ATP Parsing error ocurred: Error splitting file line".into()
                    ),
                    "shell words split",
                    token_string.to_string()
                )
            );
        }
    };

    let supported_tokens = get_supported_default_tokens();
    let token_factory = match supported_tokens.get(chunks[0].as_str()) {
        Some(x) => x,
        None => {
            return Err(
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::TokenNotFound(
                        "Token not recognized".into()
                    ),
                    "",
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
                        "Token not recognized".into()
                    ),
                    "",
                    ""
                )
            );
        }
    }

    Ok(token)
}

pub fn read_from_text_vec(tokens: Vec<String>) -> Result<Vec<Box<dyn TokenMethods>>, AtpError> {
    let mut result: Vec<Box<dyn TokenMethods>> = Vec::new();

    for line_text in tokens.iter() {
        result.push(read_from_text(&line_text)?);
    }

    Ok(result)
}

pub fn read_from_file(path: &Path) -> Result<Vec<Box<dyn TokenMethods>>, AtpError> {
    check_file_path(path, Some("atp"))?;
    let mut result: Vec<Box<dyn TokenMethods>> = Vec::new();

    let file = match OpenOptions::new().read(true).open(path) {
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

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut line_text = match line {
            Ok(x) => x,
            Err(_) => {
                return Err(
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::FileReadingError(
                            "Failed reading file line".into()
                        ),
                        "",
                        ""
                    )
                );
            }
        };

        line_text.pop();

        result.push(read_from_text(&line_text)?);
    }

    Ok(result)
}

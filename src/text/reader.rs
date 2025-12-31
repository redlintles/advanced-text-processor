use std::{ fs::OpenOptions, io::{ BufRead, BufReader }, path::Path };

use crate::{
    globals::table::{ QuerySource, QueryTarget, TOKEN_TABLE, TargetValue },
    tokens::TokenMethods,
    utils::{
        errors::{ AtpError, AtpErrorCode },
        params::AtpParamTypes,
        validations::check_file_path,
    },
};

pub fn read_from_text(token_string: &str) -> Result<Box<dyn TokenMethods>, AtpError> {
    let chunks = match
        shell_words::split(
            &token_string
                .strip_suffix(";")
                .ok_or_else(|| {
                    AtpError::new(
                        AtpErrorCode::TextParsingError(
                            "An ATP Parsing error ocurred: Error splitting file line".into()
                        ),
                        "shell words split",
                        token_string.to_string()
                    )
                })?
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

    let token_query = TOKEN_TABLE.find((
        QuerySource::Identifier(chunks[0].clone().into()),
        QueryTarget::Token,
    ))?;

    let token_param_types = match
        TOKEN_TABLE.find((QuerySource::Identifier(chunks[0].clone().into()), QueryTarget::Params))?
    {
        TargetValue::Params(p) => p,
        _ => unreachable!(" Invalid Query result"),
    };

    match token_query {
        TargetValue::Token(token_ref) => {
            let mut token = token_ref.into_box();

            let parsed_params = AtpParamTypes::from_expected(token_param_types, &chunks[1..])?;

            token.from_params(&parsed_params)?;

            Ok(token)
        }
        _ => unreachable!("Invalid query result!"),
    }
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

use std::{ fs::OpenOptions, io::{ BufRead, BufReader }, path::Path };

use crate::utils::{
    errors::{ AtpError, AtpErrorCode },
    mapping::get_mapping_bytecode_to_token,
    transforms::token_from_hex_string,
    validations::check_file_path,
};

use super::{ BytecodeInstruction, BytecodeTokenMethods };

pub fn read_bytecode_from_text(
    token_string: &str
) -> Result<Box<dyn BytecodeTokenMethods>, AtpError> {
    let chunks = match shell_words::split(&token_string) {
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

    let brute_op_code = match chunks.first() {
        Some(x) => x,
        None => {
            return Err(
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::BytecodeParsingError(
                        "Failed Parsing op_code".into()
                    ),
                    "",
                    chunks.join(" ")
                )
            );
        }
    };

    let parsed_op_code = match token_from_hex_string(&brute_op_code) {
        Some(x) => x,
        None => {
            return Err(
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::BytecodeParsingError(
                        "Failed Parsing op_code".into()
                    ),
                    "",
                    chunks.join(" ")
                )
            );
        }
    };

    let instruction = BytecodeInstruction {
        op_code: parsed_op_code,
        operands: chunks,
    };

    let op_code_map = get_mapping_bytecode_to_token();

    let mapped_token = match op_code_map.get(&(parsed_op_code as u8)) {
        Some(x) => x,
        None => {
            return Err(
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::TokenNotFound(
                        "Token not recognized".into()
                    ),
                    "",
                    parsed_op_code.to_string()
                )
            );
        }
    };

    let mut new_token = mapped_token();

    match new_token.token_from_bytecode_instruction(instruction) {
        Ok(()) => {
            return Ok(new_token);
        }
        Err(_) => {
            return Err(
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::BytecodeParsingError(
                        "Token not recognized".into()
                    ),
                    "",
                    parsed_op_code.to_string()
                )
            );
        }
    }
}

pub fn read_bytecode_from_text_vec(
    tokens: Vec<String>
) -> Result<Vec<Box<dyn BytecodeTokenMethods>>, AtpError> {
    let mut result: Vec<Box<dyn BytecodeTokenMethods>> = Vec::new();

    for line_text in tokens.iter() {
        result.push(read_bytecode_from_text(&line_text)?);
    }

    Ok(result)
}

pub fn read_bytecode_from_file(
    path: &Path
) -> Result<Vec<Box<dyn BytecodeTokenMethods>>, AtpError> {
    check_file_path(path, Some("atpbc"))?;
    let mut result: Vec<Box<dyn BytecodeTokenMethods>> = Vec::new();

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
        let line_text = match line {
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

        result.push(read_bytecode_from_text(&line_text)?);
    }

    Ok(result)
}

use std::{ fs::OpenOptions, io::{ BufRead, BufReader }, path::Path };

use crate::utils::{
    errors::{ AtpError, AtpErrorCode },
    mapping::get_mapping_bytecode_to_token,
    transforms::token_from_hex_string,
    validations::check_file_path,
};

use super::{ BytecodeInstruction, BytecodeTokenMethods };

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
        let line_text = match line {
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

        let brute_op_code = match chunks.first() {
            Some(x) => x,
            None => {
                return Err(
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::BytecodeParsingError(
                            "Failed Parsing op_code".to_string()
                        ),
                        "".to_string(),
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
                            "Failed Parsing op_code".to_string()
                        ),
                        "".to_string(),
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
                            "Token not recognized".to_string()
                        ),
                        "".to_string(),
                        parsed_op_code.to_string()
                    )
                );
            }
        };

        let mut new_token = mapped_token();

        match new_token.token_from_bytecode_instruction(instruction) {
            Ok(()) => result.push(new_token),
            Err(_) => {
                return Err(
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::TokenNotFound(
                            "Token not recognized".to_string()
                        ),
                        "".to_string(),
                        parsed_op_code.to_string()
                    )
                );
            }
        }
    }

    Ok(result)
}

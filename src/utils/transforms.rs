pub fn string_to_usize(chunk: &str) -> Result<usize, AtpError> {
    let mut parsed_chunk = String::from(chunk);
    if chunk.ends_with(";") {
        parsed_chunk.pop();
    }

    match parsed_chunk.parse() {
        Ok(v) => Ok(v),
        Err(_) => {
            Err(
                AtpError::new(
                    super::errors::AtpErrorCode::TextParsingError(
                        "String to usize Parsing failed".to_string()
                    ),
                    chunk.to_string(),
                    chunk.to_string()
                )
            )
        }
    }
}

#[cfg(feature = "bytecode")]
pub fn token_from_hex_string(s: &str) -> Option<u8> {
    let stripped = s.strip_prefix("0x")?;

    let byte = u8::from_str_radix(stripped, 16).ok()?;

    Some(byte)
}
use crate::utils::errors::AtpError;
#[cfg(feature = "bytecode")]
use crate::{
    bytecode_parser::BytecodeTokenMethods,
    token_data::TokenMethods,
    utils::mapping::get_supported_bytecode_tokens,
};

pub fn capitalize(input: &str) -> String {
    let mut chars = input.chars();

    match chars.next() {
        Some(x) => {
            let f = x.to_uppercase().to_string();
            let r: String = chars.collect();
            format!("{}{}", f, r)
        }
        None => input.to_string(),
    }
}

pub fn extend_string(input: &str, max_len: usize) -> String {
    if input.is_empty() || max_len == 0 {
        return String::new();
    }
    let to_repeat = max_len.div_ceil(input.chars().count());

    let repeated_string = input.repeat(to_repeat).chars().take(max_len).collect();

    repeated_string
}

#[cfg(feature = "bytecode")]
pub fn token_to_bytecode_token(
    token: &Box<dyn TokenMethods>
) -> Result<Box<dyn BytecodeTokenMethods>, AtpError> {
    let mut line = token.token_to_atp_line().trim().to_string();

    if line.ends_with(";") {
        line = line.trim_end_matches(";").to_string();
    }

    let chunks = match shell_words::split(&line) {
        Ok(x) => x,
        Err(_) => {
            return Err(
                AtpError::new(
                    super::errors::AtpErrorCode::TextParsingError(
                        "Shell words split failed".to_string()
                    ),
                    "shell_words::split()".to_string(),
                    token.token_to_atp_line()
                )
            );
        }
    };

    let string_token_map = get_supported_bytecode_tokens();

    let factory = match string_token_map.get(&token.get_string_repr()) {
        Some(x) => x,
        None => {
            return Err(
                AtpError::new(
                    super::errors::AtpErrorCode::TokenNotFound(
                        "Token not found in the token map".to_string()
                    ),
                    token.token_to_atp_line(),
                    token.token_to_atp_line()
                )
            );
        }
    };

    let mut new_token = factory();

    new_token.token_from_vec_params(chunks)?;

    Ok(new_token)
}

#[cfg(feature = "bytecode")]
pub fn bytecode_token_to_token(
    token: &Box<dyn BytecodeTokenMethods>
) -> Result<Box<dyn TokenMethods>, AtpError> {
    use super::mapping::get_supported_default_tokens;

    let mut line = token.token_to_atp_line().trim().to_string();

    println!("DEBUG TRANSFORM: {:?}", line);

    if line.ends_with(";") {
        line = line.trim_end_matches(";").to_string();
    }

    let chunks = match shell_words::split(&line) {
        Ok(x) => x,
        Err(_) => {
            return Err(
                AtpError::new(
                    super::errors::AtpErrorCode::TextParsingError(
                        "Shell words split failed".to_string()
                    ),
                    "shell_words::split()".to_string(),
                    token.token_to_atp_line()
                )
            );
        }
    };
    let string_token_map = get_supported_default_tokens();

    let factory = match string_token_map.get(&token.get_string_repr()) {
        Some(x) => x,
        None => {
            return Err(
                AtpError::new(
                    super::errors::AtpErrorCode::TokenNotFound(
                        "Token not found in the token map".to_string()
                    ),
                    token.token_to_atp_line(),
                    token.token_to_atp_line()
                )
            );
        }
    };

    let mut new_token = factory();

    println!("DEBUG TRANSFORM: {:?}", chunks);
    new_token.token_from_vec_params(chunks)?;

    Ok(new_token)
}

#[cfg(feature = "bytecode")]
pub fn token_vec_to_bytecode_token_vec(
    tokens: &Vec<Box<dyn TokenMethods>>
) -> Result<Vec<Box<dyn BytecodeTokenMethods>>, AtpError> {
    let mut r: Vec<Box<dyn BytecodeTokenMethods>> = Vec::new();

    for tk in tokens {
        let parsed_token = token_to_bytecode_token(tk)?;

        r.push(parsed_token);
    }
    Ok(r)
}
#[cfg(feature = "bytecode")]
pub fn bytecode_token_vec_to_token_vec(
    tokens: &Vec<Box<dyn BytecodeTokenMethods>>
) -> Result<Vec<Box<dyn TokenMethods>>, AtpError> {
    let mut r: Vec<Box<dyn TokenMethods>> = Vec::new();

    for tk in tokens {
        let parsed_token = bytecode_token_to_token(tk)?;

        r.push(parsed_token);
    }
    Ok(r)
}

pub fn get_safe_utf8_char_index(index: usize, input: &str) -> Result<usize, AtpError> {
    Ok(
        input
            .char_indices()
            .nth(index)
            .map(|(i, _)| i)
            .ok_or_else(||
                AtpError::new(
                    super::errors::AtpErrorCode::IndexOutOfRange("".to_string()),
                    "Get safe utf-8 char index".to_string(),
                    input.to_string()
                )
            )?
    )
}

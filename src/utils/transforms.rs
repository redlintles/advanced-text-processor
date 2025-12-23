use std::borrow::Cow;
use crate::utils::errors::AtpError;

pub fn string_to_usize(chunk: &str) -> Result<usize, AtpError> {
    let mut parsed_chunk = String::from(chunk);
    if chunk.ends_with(";") {
        parsed_chunk.pop();
    }

    match parsed_chunk.parse() {
        Ok(v) => Ok(v),
        Err(_) => {
            let str_chunk = chunk.to_string();
            Err(
                AtpError::new(
                    super::errors::AtpErrorCode::TextParsingError(
                        "String to usize Parsing failed".into()
                    ),
                    Cow::Owned(str_chunk),
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

pub fn get_safe_utf8_char_index(index: usize, input: &str) -> Result<usize, AtpError> {
    Ok(
        input
            .char_indices()
            .nth(index)
            .map(|(i, _)| i)
            .ok_or_else(||
                AtpError::new(
                    super::errors::AtpErrorCode::IndexOutOfRange("".into()),
                    Cow::Borrowed("Get safe utf-8 char index"),
                    input.to_string()
                )
            )?
    )
}

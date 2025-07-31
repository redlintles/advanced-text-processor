use crate::utils::errors::AtpError;

use super::BytecodeTokenMethods;

pub fn parse_token(token: &dyn BytecodeTokenMethods, input: &str) -> Result<String, AtpError> {
    token.parse(input)
}

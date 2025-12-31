use crate::{tokens::TokenMethods, utils::errors::AtpError};

pub fn apply_token(token: &dyn TokenMethods, input: &str) -> Result<String, AtpError> {
    token.transform(input)
}

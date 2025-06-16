use crate::{ token_data::TokenMethods, utils::errors::AtpError };

pub fn parse_token(token: &dyn TokenMethods, input: &str) -> Result<String, AtpError> {
    token.parse(input)
}

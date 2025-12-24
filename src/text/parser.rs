use crate::{ tokens::TokenMethods, utils::errors::{ AtpError, ErrorManager } };

pub fn parse_token(
    token: &dyn TokenMethods,
    input: &str,
    error_manager: &mut ErrorManager
) -> Result<String, AtpError> {
    match token.transform(input) {
        Ok(x) => Ok(x),
        Err(e) => {
            error_manager.add_error(e.clone());
            Err(e)
        }
    }
}

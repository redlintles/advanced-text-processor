use crate::{
    context::execution_context::GlobalExecutionContext,
    globals::var::TokenWrapper,
    utils::errors::{ AtpError, ErrorManager },
};

pub fn apply_transform(
    token: &TokenWrapper,
    input: &str,
    error_manager: &mut ErrorManager,
    context: &mut GlobalExecutionContext
) -> Result<String, AtpError> {
    match token.apply_token(input, &mut *context) {
        Ok(x) => Ok(x),
        Err(e) => {
            error_manager.add_error(e.clone());
            Err(e)
        }
    }
}

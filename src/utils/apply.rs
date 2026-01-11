use crate::{
    context::execution_context::GlobalExecutionContext,
    tokens::InstructionMethods,
    utils::errors::{ AtpError, ErrorManager },
};

pub fn apply_transform(
    token: &dyn InstructionMethods,
    input: &str,
    error_manager: &mut ErrorManager,
    context: &mut GlobalExecutionContext
) -> Result<String, AtpError> {
    match token.transform(input, &mut *context) {
        Ok(x) => Ok(x),
        Err(e) => {
            error_manager.add_error(e.clone());
            Err(e)
        }
    }
}

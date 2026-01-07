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
    if token.needs_context() {
        return match token.transform_with_context(input, context) {
            Ok(x) => Ok(x),
            Err(e) => {
                error_manager.add_error(e.clone());
                Err(e)
            }
        };
    } else {
        return match token.transform(input) {
            Ok(x) => Ok(x),
            Err(e) => {
                error_manager.add_error(e.clone());
                Err(e)
            }
        };
    }
}

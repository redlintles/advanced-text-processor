use crate::{
    context::execution_context::{ GlobalContextMethods, GlobalExecutionContext, VarValues },
    globals::table::{ SyntaxDef, SyntaxToken, QuerySource, QueryTarget, TOKEN_TABLE, TargetValue },
    tokens::InstructionMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, params::AtpParamTypes },
};

pub enum ValType {
    Literal(AtpParamTypes),
    VarRef(&'static str),
}

fn effective_param_types(expected: &[SyntaxDef]) -> Vec<SyntaxToken> {
    expected
        .iter()
        .filter_map(|ip| {
            match ip.token {
                SyntaxToken::Literal(_) => None,
                other => Some(other),
            }
        })
        .collect()
}

impl ValType {
    #[allow(dead_code)]
    fn resolve_variables(
        t: &Box<dyn InstructionMethods>,
        values: Vec<ValType>,
        context: &mut GlobalExecutionContext
    ) -> Result<Vec<AtpParamTypes>, AtpError> {
        let mut result = Vec::new();

        let query_result = TOKEN_TABLE.find((
            QuerySource::Identifier(t.get_string_repr().into()),
            QueryTarget::Syntax,
        ))?;

        let expected_params = effective_param_types(
            &(match query_result {
                TargetValue::Syntax(x) => x,
                _ => unreachable!("Unreachable Code"),
            })
        );
        for (i, v) in values.iter().enumerate() {
            match v {
                ValType::Literal(literal) => {
                    match (literal, expected_params[i]) {
                        (AtpParamTypes::String(_), SyntaxToken::String) => {
                            result.push(literal.clone());
                        }
                        (AtpParamTypes::Usize(_), SyntaxToken::Usize) => {
                            result.push(literal.clone());
                        }
                        (AtpParamTypes::Token(_), SyntaxToken::Token) => {
                            result.push(literal.clone());
                        }
                        _ => {
                            return Err(
                                AtpError::new(
                                    AtpErrorCode::IncompatibleTypeError(
                                        "Literal type and required param type are different".into()
                                    ),
                                    "resolve_variables()",
                                    ""
                                )
                            );
                        }
                    }
                }
                ValType::VarRef(name) => {
                    let variable = context.get_var(name)?;
                    match (variable.value.clone(), expected_params[i]) {
                        (VarValues::String(v), SyntaxToken::String) => {
                            result.push(AtpParamTypes::String(v));
                        }
                        (VarValues::Usize(v), SyntaxToken::Usize) => {
                            result.push(AtpParamTypes::Usize(v));
                        }
                        (VarValues::Token(v), SyntaxToken::Token) => {
                            result.push(AtpParamTypes::Token(v));
                        }
                        _ => {
                            return Err(
                                AtpError::new(
                                    AtpErrorCode::IncompatibleTypeError(
                                        "Var type and required param type are different".into()
                                    ),
                                    "resolve_variables()",
                                    ""
                                )
                            );
                        }
                    }
                }
            }
        }

        Ok(result)
    }
}

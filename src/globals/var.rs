use std::ops::Deref;

use crate::{
    context::execution_context::{ GlobalContextMethods, GlobalExecutionContext, VarValues },
    globals::table::{ QuerySource, QueryTarget, SyntaxDef, SyntaxToken, TOKEN_TABLE, TargetValue },
    to_bytecode,
    tokens::{ InstructionMethods, transforms::dlf::Dlf },
    utils::{ errors::{ AtpError, AtpErrorCode }, params::AtpParamTypes },
};
#[derive(Clone)]
pub enum ValType {
    Literal(AtpParamTypes),
    VarRef(String),
}
#[derive(Clone)]
pub struct TokenWrapper {
    params: Vec<ValType>,
    pub token: Box<dyn InstructionMethods>,
}

impl Default for TokenWrapper {
    fn default() -> Self {
        TokenWrapper { params: Vec::new(), token: Box::new(Dlf::default()) }
    }
}

impl Deref for TokenWrapper {
    type Target = Box<dyn InstructionMethods>;
    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

impl From<Box<dyn InstructionMethods>> for TokenWrapper {
    fn from(value: Box<dyn InstructionMethods>) -> Self {
        TokenWrapper::new(value, None)
    }
}

impl From<TokenWrapper> for Box<dyn InstructionMethods> {
    fn from(value: TokenWrapper) -> Self {
        return value.token;
    }
}

impl TokenWrapper {
    pub fn get_default_token(&self) -> Box<dyn InstructionMethods> {
        self.token.clone()
    }
    pub fn new(token: Box<dyn InstructionMethods>, params: Option<Vec<ValType>>) -> Self {
        match params {
            Some(param_vec) => { TokenWrapper { params: param_vec, token } }
            None => {
                let token_params = token
                    .get_params()
                    .clone()
                    .into_iter()
                    .map(|i| ValType::Literal(i))
                    .collect::<Vec<ValType>>();
                TokenWrapper { params: token_params, token }
            }
        }
    }
    pub fn apply_token(
        &self,
        input: &str,
        context: &mut GlobalExecutionContext
    ) -> Result<String, AtpError> {
        let parsed_params = ValType::resolve_variables(&self.token, &self.params, &mut *context)?;
        let mut t = self.token.clone();
        t.from_params(&parsed_params)?;

        let result = t.transform(input, context)?;

        Ok(result)
    }

    pub fn to_text_line_resolved(
        &self,
        context: &mut GlobalExecutionContext
    ) -> Result<String, AtpError> {
        let parsed_params = ValType::resolve_variables(&self.token, &self.params, &mut *context)?;
        let mut t = self.token.clone();
        t.from_params(&parsed_params)?;

        Ok(t.to_atp_line().into())
    }

    pub fn to_text_line_unresolved(&self) -> Result<String, AtpError> {
        let mut parsed_params = Vec::new();

        for param in self.params.iter() {
            match param {
                ValType::Literal(x) => parsed_params.push(x.clone()),
                ValType::VarRef(var_name) => {
                    parsed_params.push(
                        AtpParamTypes::String(format!("{{{{{}}}}}", var_name.clone()))
                    );
                }
            }
        }

        let mut t = self.token.clone();

        t.from_params(&parsed_params)?;

        Ok(t.to_atp_line().into())
    }

    pub fn to_bytecode_resolved(
        &self,
        context: &mut GlobalExecutionContext
    ) -> Result<Vec<u8>, AtpError> {
        let parsed_params = ValType::resolve_variables(&self.token, &self.params, &mut *context)?;
        let mut t = self.token.clone();
        t.from_params(&parsed_params)?;

        Ok(t.to_bytecode())
    }

    pub fn to_bytecode_unresolved(&self) -> Result<Vec<u8>, AtpError> {
        let result: Vec<u8> = Vec::new();
        let mut unresolved_params: Vec<AtpParamTypes> = Vec::new();
        for val in self.params.iter() {
            match val {
                ValType::Literal(x) => unresolved_params.push(x.clone()),
                ValType::VarRef(name) =>
                    unresolved_params.push(AtpParamTypes::VarRef(name.to_string())),
            }
        }

        let x = to_bytecode!(self.get_opcode(), []);

        Ok(result)
    }
}

pub fn get_effective_param_types(expected: &[SyntaxDef]) -> Vec<SyntaxToken> {
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
        values: &Vec<ValType>,
        context: &mut GlobalExecutionContext
    ) -> Result<Vec<AtpParamTypes>, AtpError> {
        let mut result = Vec::new();

        let query_result = TOKEN_TABLE.find((
            QuerySource::Identifier(t.get_string_repr().into()),
            QueryTarget::Syntax,
        ))?;

        let expected_params = get_effective_param_types(
            &(match query_result {
                TargetValue::Syntax(x) => x,
                _ => unreachable!("Unreachable Code"),
            })
        );
        if values.len() != expected_params.len() {
            return Err(
                AtpError::new(
                    AtpErrorCode::InvalidParameters("Param count mismatch".into()),
                    "resolve_variables",
                    format!(
                        "token={}, expected={}, got={}",
                        t.get_string_repr(),
                        expected_params.len(),
                        values.len()
                    )
                )
            );
        }

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

// params.rs
// Reescrito para suportar:
// - Texto: retorna Vec<ValType> (Literal / VarRef) via sintaxe {{nome}}
// - Bytecode: 0x01 String, 0x02 Usize, 0x03 Token, 0x04 VarRef
// - PARAM_TOKEN: constrói TokenWrapper(params: Vec<ValType>, token: Box<dyn InstructionMethods>)
//   (não chama from_params aqui; isso fica pro runtime no TokenWrapper)

use core::str;
use std::{ array::TryFromSliceError, borrow::Cow, io::{ Cursor, Read }, sync::Arc };

use regex::Regex;

#[cfg(feature = "bytecode")]
use crate::context::execution_context::GlobalExecutionContext;

use crate::{
    globals::{
        table::{ QuerySource, QueryTarget, SyntaxDef, SyntaxToken, TOKEN_TABLE, TargetValue },
        var::{ TokenWrapper, ValType },
    },
    tokens::InstructionMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::string_to_usize },
};

/// Tipos resolvidos (sem variáveis pendentes)
#[derive(Clone)]
pub enum AtpParamTypes {
    String(String),
    Usize(usize),
    Token(TokenWrapper),
    VarRef(String),
}

// --------------------------
// Conversions
// --------------------------

impl From<String> for AtpParamTypes {
    fn from(value: String) -> Self {
        AtpParamTypes::String(value)
    }
}

impl From<usize> for AtpParamTypes {
    fn from(value: usize) -> Self {
        AtpParamTypes::Usize(value)
    }
}

impl From<TokenWrapper> for AtpParamTypes {
    fn from(value: TokenWrapper) -> Self {
        AtpParamTypes::Token(value)
    }
}
impl From<Box<dyn InstructionMethods>> for AtpParamTypes {
    fn from(value: Box<dyn InstructionMethods>) -> Self {
        AtpParamTypes::Token(TokenWrapper::new(value, None))
    }
}

impl TryFrom<AtpParamTypes> for String {
    type Error = AtpError;
    fn try_from(value: AtpParamTypes) -> Result<String, Self::Error> {
        Ok(match value {
            AtpParamTypes::String(v) => v,
            AtpParamTypes::Usize(v) => v.to_string(),
            AtpParamTypes::Token(v) => v.to_text_line_unresolved()?,
            AtpParamTypes::VarRef(v) => v,
        })
    }
}

// Nota: esse dummy_context só existe pra manter o From<AtpParamTypes> for String compilável
// se você realmente precisar converter TokenWrapper -> String sem contexto, troque por get_default_token().to_atp_line()
// ou remova completamente esse From.
fn dummy_context() -> GlobalExecutionContext {
    // Se seu GlobalExecutionContext não tiver Default, remova o dummy_context e ajuste o From.
    GlobalExecutionContext::new()
}

impl TryFrom<AtpParamTypes> for usize {
    type Error = AtpError;
    fn try_from(value: AtpParamTypes) -> Result<Self, AtpError> {
        match value {
            AtpParamTypes::Usize(v) => Ok(v),
            _ =>
                Err(
                    AtpError::new(
                        AtpErrorCode::TryIntoFailError(
                            "Failed conversion from AtpParamTypes to usize".into()
                        ),
                        "TryFrom<AtpParamTypes> for usize",
                        ""
                    )
                ),
        }
    }
}

impl TryFrom<AtpParamTypes> for TokenWrapper {
    type Error = AtpError;
    fn try_from(value: AtpParamTypes) -> Result<Self, AtpError> {
        match value {
            AtpParamTypes::Token(v) => Ok(v),
            _ =>
                Err(
                    AtpError::new(
                        AtpErrorCode::TryIntoFailError(
                            "Failed conversion from AtpParamTypes to TokenWrapper".into()
                        ),
                        "TryFrom<AtpParamTypes> for TokenWrapper",
                        ""
                    )
                ),
        }
    }
}

/// Debug customizado para evitar exigir Debug em dyn InstructionMethods
impl std::fmt::Debug for AtpParamTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtpParamTypes::String(s) => f.debug_tuple("String").field(s).finish(),
            AtpParamTypes::Usize(n) => f.debug_tuple("Usize").field(n).finish(),
            AtpParamTypes::Token(t) => f.debug_tuple("Token").field(&t.get_string_repr()).finish(),
            AtpParamTypes::VarRef(s) => f.debug_tuple("VarRef").field(s).finish(),
        }
    }
}

pub trait AtpParamTypesJoin {
    fn join(&self, sep: &str) -> String;
}

impl AtpParamTypesJoin for Vec<AtpParamTypes> {
    fn join(&self, sep: &str) -> String {
        let mut out = String::new();
        for (idx, item) in self.iter().enumerate() {
            if idx > 0 {
                out.push_str(sep);
            }
            out.push_str(&item.to_string());
        }
        out
    }
}

// --------------------------
// Política de aninhamento
// --------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AssocMode {
    Normal,
    AssocPayload,
}

const MAX_DEPTH_NORMAL: u8 = 1;
const MAX_DEPTH_ASSOC_PAYLOAD: u8 = 3;

// Bytecode param types
const PARAM_STRING: u32 = 0x01;
const PARAM_USIZE: u32 = 0x02;
const PARAM_TOKEN: u32 = 0x03;
const PARAM_VARREF: u32 = 0x04;

impl AtpParamTypes {
    pub fn to_string(&self) -> String {
        match self {
            AtpParamTypes::String(payload) => payload.to_string(),
            AtpParamTypes::VarRef(payload) => payload.to_string(),
            AtpParamTypes::Usize(payload) => payload.to_string(),
            AtpParamTypes::Token(payload) => payload.to_atp_line().into(),
        }
    }

    // --------------------------
    // Parsing de texto -> Vec<ValType>
    // --------------------------

    pub fn from_expected(
        expected: Arc<[SyntaxDef]>,
        chunks: &[String]
    ) -> Result<Vec<ValType>, AtpError> {
        let (parsed, consumed) = Self::parse_with_cursor(
            expected,
            chunks,
            0,
            0,
            AssocMode::Normal
        )?;
        if consumed != chunks.len() {
            return Err(
                AtpError::new(
                    AtpErrorCode::TextParsingError("Extra parameters after parsing".into()),
                    "AtpParamTypes::from_expected",
                    format!("consumed={}, total={}", consumed, chunks.len())
                )
            );
        }
        Ok(parsed)
    }

    fn parse_with_cursor(
        expected: Arc<[SyntaxDef]>,
        chunks: &[String],
        mut i: usize,
        token_depth: u8,
        assoc_mode: AssocMode
    ) -> Result<(Vec<ValType>, usize), AtpError> {
        // Regex compilada uma vez por chamada (ok por enquanto; se quiser otimizar, use OnceLock)
        let var_re = Regex::new(r"^\{\{(.+)\}\}$").map_err(|e| {
            AtpError::new(
                AtpErrorCode::TextParsingError("Error creating regex".into()),
                "AtpParamTypes::parse_with_cursor(regex)",
                e.to_string()
            )
        })?;

        let mut out: Vec<ValType> = Vec::with_capacity(expected.len());
        let this_is_block_like = Self::is_block_like_signature(&expected);

        for p in expected.iter() {
            match p.token {
                SyntaxToken::Literal(expected_literal) => {
                    let literal = chunks
                        .get(i)
                        .ok_or_else(|| {
                            AtpError::new(
                                AtpErrorCode::TextParsingError("Missing literal".into()),
                                "AtpParamTypes::parse_with_cursor",
                                format!("index={}", i)
                            )
                        })?;
                    if expected_literal != literal {
                        return Err(
                            AtpError::new(
                                AtpErrorCode::InvalidParameters("Invalid literal".into()),
                                "AtpParamTypes::parse_with_cursor",
                                format!(
                                    "expected={}, got={}, index={}",
                                    expected_literal,
                                    literal,
                                    i
                                )
                            )
                        );
                    }
                    i += 1;
                }

                SyntaxToken::String => {
                    let s = chunks
                        .get(i)
                        .ok_or_else(|| {
                            AtpError::new(
                                AtpErrorCode::TextParsingError("Missing String parameter".into()),
                                "AtpParamTypes::parse_with_cursor",
                                format!("index={}", i)
                            )
                        })?;

                    if let Some(caps) = var_re.captures(s) {
                        let name = caps
                            .get(1)
                            .map(|m| m.as_str().trim().to_string())
                            .unwrap_or_default();

                        if name.is_empty() {
                            return Err(
                                AtpError::new(
                                    AtpErrorCode::TextParsingError("Empty var name".into()),
                                    "AtpParamTypes::parse_with_cursor",
                                    format!("index={}", i)
                                )
                            );
                        }

                        out.push(ValType::VarRef(name));
                    } else {
                        out.push(ValType::Literal(AtpParamTypes::String(s.clone())));
                    }

                    i += 1;
                }

                SyntaxToken::Usize => {
                    let s = chunks
                        .get(i)
                        .ok_or_else(|| {
                            AtpError::new(
                                AtpErrorCode::TextParsingError("Missing Usize parameter".into()),
                                "AtpParamTypes::parse_with_cursor",
                                format!("index={}", i)
                            )
                        })?;
                    out.push(ValType::Literal(AtpParamTypes::Usize(string_to_usize(s)?)));
                    i += 1;
                }

                SyntaxToken::Token => {
                    let child_assoc_mode = if assoc_mode == AssocMode::AssocPayload {
                        AssocMode::AssocPayload
                    } else if this_is_block_like {
                        AssocMode::AssocPayload
                    } else {
                        AssocMode::Normal
                    };

                    let next_depth = token_depth + 1;
                    let max_depth = match child_assoc_mode {
                        AssocMode::Normal => MAX_DEPTH_NORMAL,
                        AssocMode::AssocPayload => MAX_DEPTH_ASSOC_PAYLOAD,
                    };

                    if next_depth > max_depth {
                        return Err(
                            AtpError::new(
                                AtpErrorCode::TextParsingError(
                                    "Nested token depth exceeded".into()
                                ),
                                "AtpParamTypes::parse_with_cursor",
                                format!(
                                    "depth={}, max={}, index={}, assoc={:?}",
                                    next_depth,
                                    max_depth,
                                    i,
                                    child_assoc_mode
                                )
                            )
                        );
                    }

                    // token id
                    let nested_id = chunks
                        .get(i)
                        .ok_or_else(|| {
                            AtpError::new(
                                AtpErrorCode::TextParsingError(
                                    "Missing nested token identifier".into()
                                ),
                                "AtpParamTypes::parse_with_cursor",
                                format!("index={}", i)
                            )
                        })?
                        .clone();
                    i += 1;

                    let nested_key: Cow<'static, str> = Cow::Owned(nested_id.clone());

                    let nested_expected = match
                        TOKEN_TABLE.find((
                            QuerySource::Identifier(nested_key.clone()),
                            QueryTarget::Syntax,
                        ))?
                    {
                        TargetValue::Syntax(p) => p,
                        _ => unreachable!("Invalid Query result (Syntax)"),
                    };

                    // Dentro de AssocPayload não pode existir outro token block-like
                    if
                        child_assoc_mode == AssocMode::AssocPayload &&
                        Self::is_block_like_signature(&nested_expected)
                    {
                        return Err(
                            AtpError::new(
                                AtpErrorCode::TextParsingError(
                                    "A block cannot contain another block".into()
                                ),
                                "AtpParamTypes::parse_with_cursor",
                                format!("nested_id={}, index={}", nested_id, i - 1)
                            )
                        );
                    }

                    let (nested_params, next_i) = Self::parse_with_cursor(
                        nested_expected,
                        chunks,
                        i,
                        next_depth,
                        child_assoc_mode
                    )?;
                    i = next_i;

                    let nested_token_ref = match
                        TOKEN_TABLE.find((QuerySource::Identifier(nested_key), QueryTarget::Token))?
                    {
                        TargetValue::Token(t) => t,
                        _ => unreachable!("Invalid Query result (Token)"),
                    };

                    let nested_token = nested_token_ref.into_box();
                    out.push(
                        ValType::Literal(
                            AtpParamTypes::Token(
                                TokenWrapper::new(nested_token, Some(nested_params))
                            )
                        )
                    );
                }
            }
        }

        Ok((out, i))
    }

    fn is_block_like_signature(expected: &Arc<[SyntaxDef]>) -> bool {
        expected.len() == 3 &&
            matches!(expected[0].token, SyntaxToken::String) &&
            matches!(expected[1].token, SyntaxToken::Literal("assoc")) &&
            matches!(expected[2].token, SyntaxToken::Token)
    }

    fn effective_syntax_tokens(expected: &Arc<[SyntaxDef]>) -> Vec<SyntaxToken> {
        let mut out = Vec::with_capacity(expected.len());
        for ip in expected.iter() {
            if matches!(ip.token, SyntaxToken::Literal(_)) {
                continue;
            }
            out.push(ip.token);
        }
        out
    }

    // --------------------------
    // Parsing de Bytecode -> AtpParamTypes (raiz) / ValType (params internos)
    // --------------------------

    pub fn from_bytecode(bytecode: Vec<u8>) -> Result<AtpParamTypes, AtpError> {
        Self::from_bytecode_with_policy(&bytecode, 0, AssocMode::Normal)
    }

    fn from_bytecode_with_policy(
        bytes: &[u8],
        token_depth: u8,
        assoc_mode: AssocMode
    ) -> Result<AtpParamTypes, AtpError> {
        // Layout novo: [u64 total][u32 type][u32 payload_size][payload]
        if bytes.len() >= 16 {
            if let Ok(total) = Self::peek_u64_be(&bytes[0..8]) {
                if (total as usize) == bytes.len() {
                    return Self::parse_param_new_layout(bytes, token_depth, assoc_mode);
                }
            }
        }
        // Layout antigo: [u32 type][u32 payload_size][payload]
        Self::parse_param_old_layout(bytes, token_depth, assoc_mode)
    }

    fn parse_param_new_layout(
        bytes: &[u8],
        token_depth: u8,
        assoc_mode: AssocMode
    ) -> Result<AtpParamTypes, AtpError> {
        let mut reader = Cursor::new(bytes);

        let total_size = Self::read_u64_be(
            &mut reader,
            "AtpParamTypes::from_bytecode(total)"
        )? as usize;
        if total_size != bytes.len() {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Param total size mismatch".into()),
                    "AtpParamTypes::from_bytecode(new_layout)",
                    format!("declared={}, actual={}", total_size, bytes.len())
                )
            );
        }

        let param_type = Self::read_u32_be(&mut reader, "AtpParamTypes::from_bytecode(type)")?;
        let payload_size = Self::read_u32_be(
            &mut reader,
            "AtpParamTypes::from_bytecode(payload_size)"
        )? as usize;

        let remaining = bytes.len().saturating_sub(reader.position() as usize);
        if payload_size > remaining {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Payload exceeds remaining".into()),
                    "AtpParamTypes::from_bytecode(new_layout)",
                    format!("payload_size={}, remaining={}", payload_size, remaining)
                )
            );
        }

        let payload = Self::read_exact_vec(
            &mut reader,
            payload_size,
            "AtpParamTypes::from_bytecode(payload)"
        )?;
        Self::decode_param_payload(param_type, payload, token_depth, assoc_mode)
    }

    fn parse_param_old_layout(
        bytes: &[u8],
        token_depth: u8,
        assoc_mode: AssocMode
    ) -> Result<AtpParamTypes, AtpError> {
        if bytes.len() < 8 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Param too small".into()),
                    "AtpParamTypes::from_bytecode(old_layout)",
                    format!("len={}", bytes.len())
                )
            );
        }

        let mut reader = Cursor::new(bytes);
        let param_type = Self::read_u32_be(&mut reader, "AtpParamTypes::from_bytecode(type)")?;
        let payload_size = Self::read_u32_be(
            &mut reader,
            "AtpParamTypes::from_bytecode(payload_size)"
        )? as usize;

        let remaining = bytes.len().saturating_sub(reader.position() as usize);
        if payload_size > remaining {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Payload exceeds remaining".into()),
                    "AtpParamTypes::from_bytecode(old_layout)",
                    format!("payload_size={}, remaining={}", payload_size, remaining)
                )
            );
        }

        let payload = Self::read_exact_vec(
            &mut reader,
            payload_size,
            "AtpParamTypes::from_bytecode(payload)"
        )?;
        Self::decode_param_payload(param_type, payload, token_depth, assoc_mode)
    }

    /// Decode para tipos resolvidos (não aceita VarRef aqui como raiz)
    fn decode_param_payload(
        param_type: u32,
        payload: Vec<u8>,
        token_depth: u8,
        assoc_mode: AssocMode
    ) -> Result<AtpParamTypes, AtpError> {
        match param_type {
            PARAM_STRING => {
                let text = str
                    ::from_utf8(&payload)
                    .map_err(|e| {
                        AtpError::new(
                            AtpErrorCode::BytecodeParamParsingError(
                                "Failed parsing bytes to UTF8 string".into()
                            ),
                            "AtpParamTypes::from_bytecode(String)",
                            e.to_string()
                        )
                    })?;
                Ok(AtpParamTypes::String(text.to_string()))
            }

            PARAM_USIZE => {
                let b: [u8; 8] = payload
                    .as_slice()
                    .try_into()
                    .map_err(|e: TryFromSliceError| {
                        AtpError::new(
                            AtpErrorCode::BytecodeParamParsingError(
                                "Failed parsing bytes to usize".into()
                            ),
                            "AtpParamTypes::from_bytecode(Usize)",
                            e.to_string()
                        )
                    })?;
                Ok(AtpParamTypes::Usize(usize::from_be_bytes(b)))
            }

            PARAM_VARREF => {
                // VarRef só deveria existir dentro de Token params (ValType),
                // mas se aparecer aqui como raiz, retorna erro claro.
                Err(
                    AtpError::new(
                        AtpErrorCode::BytecodeParamParsingError(
                            "VarRef cannot be a root AtpParamTypes".into()
                        ),
                        "AtpParamTypes::from_bytecode(VarRef)",
                        "Use decode_val_payload inside PARAM_TOKEN"
                    )
                )
            }

            PARAM_TOKEN => {
                let mut reader = Cursor::new(payload.as_slice());

                let opcode = Self::read_u32_be(
                    &mut reader,
                    "AtpParamTypes::from_bytecode(Token.opcode)"
                )?;
                let param_count = Self::read_u8(
                    &mut reader,
                    "AtpParamTypes::from_bytecode(Token.param_count)"
                )? as usize;

                // Sintaxe esperada (com literais)
                let expected = match
                    TOKEN_TABLE.find((QuerySource::Bytecode(opcode), QueryTarget::Syntax))?
                {
                    TargetValue::Syntax(p) => p,
                    _ => unreachable!(),
                };

                let expected_effective = Self::effective_syntax_tokens(&expected);
                if param_count != expected_effective.len() {
                    return Err(
                        AtpError::new(
                            AtpErrorCode::BytecodeParsingError("Param count mismatch".into()),
                            "AtpParamTypes::from_bytecode(Token.param_count)",
                            format!(
                                "opcode=0x{:X}, expected_effective={}, got={}",
                                opcode,
                                expected_effective.len(),
                                param_count
                            )
                        )
                    );
                }

                let this_is_block_like = Self::is_block_like_signature(&expected);
                let mut params: Vec<ValType> = Vec::with_capacity(param_count);

                for idx in 0..param_count {
                    // Cada parâmetro do token está no layout novo (começa com u64 total)
                    let size_u64 = Self::read_u64_be(
                        &mut reader,
                        "AtpParamTypes::from_bytecode(Token.param_total_size)"
                    )?;

                    let size_usize = usize
                        ::try_from(size_u64)
                        .map_err(|_| {
                            AtpError::new(
                                AtpErrorCode::BytecodeParsingError("Param size overflow".into()),
                                "AtpParamTypes::from_bytecode(Token.param_total_size)",
                                format!("size_u64={}", size_u64)
                            )
                        })?;

                    if size_usize < 16 {
                        // no layout novo, mínimo: 8+4+4
                        return Err(
                            AtpError::new(
                                AtpErrorCode::BytecodeParsingError(
                                    "Invalid param total size".into()
                                ),
                                "AtpParamTypes::from_bytecode(Token.param_total_size)",
                                format!("size={}", size_usize)
                            )
                        );
                    }

                    let rest_len = size_usize - 8;
                    let rest = Self::read_exact_vec(
                        &mut reader,
                        rest_len,
                        "AtpParamTypes::from_bytecode(Token.param_bytes)"
                    )?;

                    let mut full_param: Vec<u8> = Vec::with_capacity(size_usize);
                    full_param.extend_from_slice(&size_u64.to_be_bytes());
                    full_param.extend_from_slice(&rest);

                    // Decide assoc_mode do filho usando idx efetivo
                    let child_assoc_mode = if assoc_mode == AssocMode::AssocPayload {
                        AssocMode::AssocPayload
                    } else if
                        this_is_block_like &&
                        matches!(expected_effective[idx], SyntaxToken::Token)
                    {
                        AssocMode::AssocPayload
                    } else {
                        AssocMode::Normal
                    };

                    // Peek do tipo do filho para checar depth
                    #[allow(unused_parens)]
                    if
                        let Ok(next_param_type) = ({
                            let mut cursor = Cursor::new(full_param.as_slice());
                            cursor.set_position(8);
                            Self::read_u32_be(&mut cursor, "")
                        })
                    {
                        if next_param_type == PARAM_TOKEN {
                            let next_depth = token_depth + 1;
                            let max_depth = match child_assoc_mode {
                                AssocMode::Normal => MAX_DEPTH_NORMAL,
                                AssocMode::AssocPayload => MAX_DEPTH_ASSOC_PAYLOAD,
                            };
                            if next_depth > max_depth {
                                return Err(
                                    AtpError::new(
                                        AtpErrorCode::BytecodeParsingError(
                                            "Nested token depth exceeded".into()
                                        ),
                                        "AtpParamTypes::from_bytecode(Token)",
                                        format!(
                                            "depth={}, max_assoc_mode={:?}",
                                            next_depth,
                                            child_assoc_mode
                                        )
                                    )
                                );
                            }
                        }
                    }

                    // Agora decodifica o child param como ValType (pode ser VarRef)
                    let parsed_val = Self::decode_full_param_as_valtype(
                        &full_param,
                        token_depth + 1,
                        child_assoc_mode
                    )?;

                    // Regra: em AssocPayload não pode existir block-like (só se child for Token literal)
                    if child_assoc_mode == AssocMode::AssocPayload {
                        if let ValType::Literal(AtpParamTypes::Token(ref tok)) = parsed_val {
                            let nested_expected = match
                                TOKEN_TABLE.find((
                                    QuerySource::Identifier(
                                        Cow::Owned(tok.get_string_repr().to_string())
                                    ),
                                    QueryTarget::Syntax,
                                ))?
                            {
                                TargetValue::Syntax(p) => p,
                                _ => unreachable!(),
                            };

                            if Self::is_block_like_signature(&nested_expected) {
                                return Err(
                                    AtpError::new(
                                        AtpErrorCode::BytecodeParsingError(
                                            "A block cannot contain another block".into()
                                        ),
                                        "AtpParamTypes::from_bytecode(Token)",
                                        format!("nested_id={}", tok.get_string_repr())
                                    )
                                );
                            }
                        }
                    }

                    params.push(parsed_val);
                }

                // Token base (default), embrulhado com params não resolvidos
                let query_result = TOKEN_TABLE.find((
                    QuerySource::Bytecode(opcode),
                    QueryTarget::Token,
                ))?;
                match query_result {
                    TargetValue::Token(token_ref) => {
                        let token = token_ref.into_box();
                        Ok(AtpParamTypes::Token(TokenWrapper::new(token, Some(params))))
                    }
                    _ => unreachable!(),
                }
            }

            _ =>
                Err(
                    AtpError::new(
                        AtpErrorCode::BytecodeParamNotRecognized(
                            format!("Param Bytecode Not Recognized 0x{:X}", param_type).into()
                        ),
                        "AtpParamTypes::from_bytecode",
                        ""
                    )
                ),
        }
    }

    /// Decodifica um full_param (layout novo) para ValType:
    /// - 0x04 => VarRef(String)
    /// - outros => Literal(AtpParamTypes)
    fn decode_full_param_as_valtype(
        full_param: &[u8],
        token_depth: u8,
        assoc_mode: AssocMode
    ) -> Result<ValType, AtpError> {
        if full_param.len() < 16 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Param too small".into()),
                    "AtpParamTypes::decode_full_param_as_valtype",
                    format!("len={}", full_param.len())
                )
            );
        }

        let mut cursor = Cursor::new(full_param);
        let total = Self::read_u64_be(&mut cursor, "ValType.param.total")? as usize;
        if total != full_param.len() {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Param total size mismatch".into()),
                    "AtpParamTypes::decode_full_param_as_valtype",
                    format!("declared={}, actual={}", total, full_param.len())
                )
            );
        }

        let ty = Self::read_u32_be(&mut cursor, "ValType.param.type")?;
        let payload_size = Self::read_u32_be(&mut cursor, "ValType.param.payload_size")? as usize;

        let remaining = full_param.len().saturating_sub(cursor.position() as usize);
        if payload_size > remaining {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Payload exceeds remaining".into()),
                    "AtpParamTypes::decode_full_param_as_valtype",
                    format!("payload_size={}, remaining={}", payload_size, remaining)
                )
            );
        }

        let payload = Self::read_exact_vec(&mut cursor, payload_size, "ValType.param.payload")?;

        match ty {
            PARAM_VARREF => {
                let text = str
                    ::from_utf8(&payload)
                    .map_err(|e| {
                        AtpError::new(
                            AtpErrorCode::BytecodeParamParsingError(
                                "Failed parsing bytes to UTF8 string".into()
                            ),
                            "AtpParamTypes::from_bytecode(VarRef)",
                            e.to_string()
                        )
                    })?;

                let name = text.trim();
                if name.is_empty() {
                    return Err(
                        AtpError::new(
                            AtpErrorCode::BytecodeParamParsingError("Empty VarRef name".into()),
                            "AtpParamTypes::from_bytecode(VarRef)",
                            ""
                        )
                    );
                }
                Ok(ValType::VarRef(name.to_string()))
            }

            // Qualquer outro tipo é Literal(AtpParamTypes)
            _ =>
                Ok(
                    ValType::Literal(
                        Self::decode_param_payload(ty, payload, token_depth, assoc_mode)?
                    )
                ),
        }
    }

    // --------------------------
    // Helpers de leitura de bytecode
    // --------------------------

    fn read_exact_vec(
        reader: &mut Cursor<&[u8]>,
        len: usize,
        instruction: &'static str
    ) -> Result<Vec<u8>, AtpError> {
        let mut buf = vec![0u8; len];
        reader
            .read_exact(&mut buf)
            .map_err(|e| {
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed reading bytecode".into()),
                    instruction,
                    e.to_string()
                )
            })?;
        Ok(buf)
    }

    fn read_u8(reader: &mut Cursor<&[u8]>, instruction: &'static str) -> Result<u8, AtpError> {
        let mut b = [0u8; 1];
        reader
            .read_exact(&mut b)
            .map_err(|e| {
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed reading bytecode".into()),
                    instruction,
                    e.to_string()
                )
            })?;
        Ok(u8::from_be_bytes(b))
    }

    fn read_u32_be(reader: &mut Cursor<&[u8]>, instruction: &'static str) -> Result<u32, AtpError> {
        let mut b = [0u8; 4];
        reader
            .read_exact(&mut b)
            .map_err(|e| {
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed reading bytecode".into()),
                    instruction,
                    e.to_string()
                )
            })?;
        Ok(u32::from_be_bytes(b))
    }

    fn read_u64_be(reader: &mut Cursor<&[u8]>, instruction: &'static str) -> Result<u64, AtpError> {
        let mut b = [0u8; 8];
        reader
            .read_exact(&mut b)
            .map_err(|e| {
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed reading bytecode".into()),
                    instruction,
                    e.to_string()
                )
            })?;
        Ok(u64::from_be_bytes(b))
    }

    fn peek_u64_be(bytes: &[u8]) -> Result<u64, AtpError> {
        let b: [u8; 8] = bytes
            .try_into()
            .map_err(|_| {
                AtpError::new(
                    AtpErrorCode::BytecodeParamParsingError("Failed reading u64 header".into()),
                    "AtpParamTypes::peek_u64_be",
                    format!("len={}", bytes.len())
                )
            })?;
        Ok(u64::from_be_bytes(b))
    }

    // --------------------------
    // Bytecode writing (param)
    // --------------------------

    pub fn get_param_type_code(&self) -> u32 {
        match self {
            AtpParamTypes::String(_) => PARAM_STRING,
            AtpParamTypes::Usize(_) => PARAM_USIZE,
            AtpParamTypes::Token(_) => PARAM_TOKEN,
            AtpParamTypes::VarRef(_) => PARAM_VARREF,
        }
    }

    #[cfg(feature = "bytecode")]
    pub fn write_as_instruction_param(
        &self,
        out: &mut Vec<u8>,
        context: &mut GlobalExecutionContext
    ) -> Result<(), AtpError> {
        let param_type = self.get_param_type_code();

        let payload: Vec<u8> = match self {
            AtpParamTypes::String(s) => s.as_bytes().to_vec(),
            AtpParamTypes::Usize(n) => n.to_be_bytes().to_vec(),
            AtpParamTypes::Token(t) => t.to_bytecode_resolved(context)?,
            AtpParamTypes::VarRef(s) => s.as_bytes().to_vec(),
        };

        let payload_size_u32: u32 = payload.len() as u32;
        let total_size_u64: u64 = 8 + 4 + 4 + (payload.len() as u64);

        out.extend_from_slice(&total_size_u64.to_be_bytes());
        out.extend_from_slice(&param_type.to_be_bytes());
        out.extend_from_slice(&payload_size_u32.to_be_bytes());
        out.extend_from_slice(&payload);

        Ok(())
    }

    #[cfg(feature = "bytecode")]
    pub fn param_to_bytecode(
        &self,
        context: &mut GlobalExecutionContext
    ) -> Result<(u64, Vec<u8>), AtpError> {
        let mut result: Vec<u8> = Vec::new();
        self.write_as_instruction_param(&mut result, context)?;
        let total = u64::from_be_bytes(result[0..8].try_into().unwrap());
        Ok((total, result))
    }
}

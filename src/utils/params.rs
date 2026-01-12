use core::str;
use std::{ array::TryFromSliceError, borrow::Cow, io::{ Cursor, Read }, sync::Arc };

use crate::{
    globals::table::{ SyntaxDef, SyntaxToken, QuerySource, QueryTarget, TargetValue, TOKEN_TABLE },
    tokens::InstructionMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::string_to_usize },
};

/// Param types usados em parâmetros de token
#[derive(Clone)]
pub enum AtpParamTypes {
    String(String),
    Usize(usize),
    Token(Box<dyn InstructionMethods>),
}

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

impl From<Box<dyn InstructionMethods>> for AtpParamTypes {
    fn from(value: Box<dyn InstructionMethods>) -> Self {
        AtpParamTypes::Token(value)
    }
}

impl From<AtpParamTypes> for String {
    fn from(value: AtpParamTypes) -> String {
        match value {
            AtpParamTypes::String(v) => v.to_string(),
            AtpParamTypes::Usize(v) => v.to_string(),
            AtpParamTypes::Token(v) => v.to_atp_line().into(),
        }
    }
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
                            "Failed conversion from AtpParamTypes To Usize".into()
                        ),
                        "try_into",
                        ""
                    )
                ),
        }
    }
}

impl TryFrom<AtpParamTypes> for Box<dyn InstructionMethods> {
    type Error = AtpError;
    fn try_from(value: AtpParamTypes) -> Result<Self, AtpError> {
        match value {
            AtpParamTypes::Token(v) => Ok(v),
            _ =>
                Err(
                    AtpError::new(
                        AtpErrorCode::TryIntoFailError(
                            "Failed conversion from AtpParamTypes To Token".into()
                        ),
                        "try_into",
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

impl AtpParamTypes {
    pub fn to_string(&self) -> String {
        match self {
            AtpParamTypes::String(payload) => payload.to_string(),
            AtpParamTypes::Usize(payload) => payload.to_string(),
            AtpParamTypes::Token(payload) => payload.to_atp_line().into(),
        }
    }

    // --------------------------
    // Parsing de texto
    // --------------------------

    pub fn from_expected(
        expected: Arc<[SyntaxDef]>,
        chunks: &[String]
    ) -> Result<Vec<AtpParamTypes>, AtpError> {
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
    ) -> Result<(Vec<AtpParamTypes>, usize), AtpError> {
        let mut out = Vec::with_capacity(expected.len());

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
                    out.push(AtpParamTypes::String(s.clone()));
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
                    out.push(AtpParamTypes::Usize(string_to_usize(s)?));
                    i += 1;
                }

                SyntaxToken::Token => {
                    // Decide o modo do próximo nível
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

                    // Lê o identificador do token
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
                        _ => unreachable!("Invalid Query result (Params)"),
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

                    let mut nested_token = nested_token_ref.into_box();
                    nested_token.from_params(&nested_params)?;
                    out.push(AtpParamTypes::Token(nested_token));
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

    // --------------------------
    // Parsing de Bytecode
    // --------------------------

    pub fn from_bytecode(bytecode: Vec<u8>) -> Result<AtpParamTypes, AtpError> {
        Self::from_bytecode_with_policy(&bytecode, 0, AssocMode::Normal)
    }

    fn from_bytecode_with_policy(
        bytes: &[u8],
        token_depth: u8,
        assoc_mode: AssocMode
    ) -> Result<AtpParamTypes, AtpError> {
        // Tenta layout novo primeiro
        if bytes.len() >= 16 {
            if let Ok(total) = Self::peek_u64_be(&bytes[0..8]) {
                if (total as usize) == bytes.len() {
                    return Self::parse_param_new_layout(bytes, token_depth, assoc_mode);
                }
            }
        }
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

            PARAM_TOKEN => {
                // Leitura do header do token
                let mut reader = Cursor::new(payload.as_slice());

                let opcode = Self::read_u32_be(
                    &mut reader,
                    "AtpParamTypes::from_bytecode(Token.opcode)"
                )?;
                let param_count = Self::read_u8(
                    &mut reader,
                    "AtpParamTypes::from_bytecode(Token.param_count)"
                )? as usize;

                // Parametros esperados para esse token
                let expected = match
                    TOKEN_TABLE.find((QuerySource::Bytecode(opcode), QueryTarget::Syntax))?
                {
                    TargetValue::Syntax(p) => p,
                    _ => unreachable!(),
                };

                // Detecta se o token atual pode gerar assoc payload
                let this_is_block_like = Self::is_block_like_signature(&expected);

                let mut params: Vec<AtpParamTypes> = Vec::with_capacity(param_count);

                for idx in 0..param_count {
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

                    if size_usize < 8 {
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

                    // Decide modo do filho
                    let child_assoc_mode = if assoc_mode == AssocMode::AssocPayload {
                        AssocMode::AssocPayload
                    } else if
                        this_is_block_like &&
                        Self::param_index_is_token_in_signature(&expected, idx)
                    {
                        AssocMode::AssocPayload
                    } else {
                        AssocMode::Normal
                    };

                    // Se for um token real, incrementa depth e checa limite
                    // Verificamos se o next item é realmente um token
                    #[allow(unused_parens)]
                    if
                        let Ok(next_param_type) = ({
                            let mut cursor = Cursor::new(full_param.as_slice());
                            // pula total u64
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

                    // Recurre com depth incrementado para token
                    let parsed = Self::from_bytecode_with_policy(
                        &full_param,
                        token_depth + 1,
                        child_assoc_mode
                    )?;

                    // Nao permite block-like dentro de assoc payload
                    if child_assoc_mode == AssocMode::AssocPayload {
                        if let AtpParamTypes::Token(ref tok) = parsed {
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

                    params.push(parsed);
                }

                // Instancia token
                let query_result = TOKEN_TABLE.find((
                    QuerySource::Bytecode(opcode),
                    QueryTarget::Token,
                ))?;
                match query_result {
                    TargetValue::Token(token_ref) => {
                        let mut token = token_ref.into_box();
                        token.from_params(&params)?;
                        Ok(AtpParamTypes::Token(token))
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

    fn param_index_is_token_in_signature(expected: &Arc<[SyntaxDef]>, param_index: usize) -> bool {
        let mut effective_types: Vec<SyntaxToken> = Vec::with_capacity(expected.len());
        for ip in expected.iter() {
            if matches!(ip.token, SyntaxToken::Literal(_)) {
                continue;
            }
            effective_types.push(ip.token);
        }
        matches!(effective_types.get(param_index), Some(SyntaxToken::Token))
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
        }
    }

    #[cfg(feature = "bytecode")]
    pub fn write_as_instruction_param(&self, out: &mut Vec<u8>) {
        let param_type = self.get_param_type_code();

        let payload: Vec<u8> = match self {
            AtpParamTypes::String(s) => s.as_bytes().to_vec(),
            AtpParamTypes::Usize(n) => n.to_be_bytes().to_vec(),
            AtpParamTypes::Token(t) => t.to_bytecode(),
        };

        let payload_size_u32: u32 = payload.len() as u32;
        let total_size_u64: u64 = 8 + 4 + 4 + (payload.len() as u64);

        out.extend_from_slice(&total_size_u64.to_be_bytes());
        out.extend_from_slice(&param_type.to_be_bytes());
        out.extend_from_slice(&payload_size_u32.to_be_bytes());
        out.extend_from_slice(&payload);
    }

    #[cfg(feature = "bytecode")]
    pub fn param_to_bytecode(&self) -> (u64, Vec<u8>) {
        let mut result: Vec<u8> = Vec::new();

        let param_type = self.get_param_type_code();

        let payload: Vec<u8> = match self {
            AtpParamTypes::String(x) => x.as_bytes().to_vec(),
            AtpParamTypes::Usize(x) => x.to_be_bytes().to_vec(),
            AtpParamTypes::Token(x) => x.to_bytecode(),
        };

        let payload_size_u32: u32 = payload.len() as u32;
        let total_size_u64: u64 = 8 + 4 + 4 + (payload.len() as u64);

        result.extend_from_slice(&total_size_u64.to_be_bytes());
        result.extend_from_slice(&param_type.to_be_bytes());
        result.extend_from_slice(&payload_size_u32.to_be_bytes());
        result.extend_from_slice(&payload);

        (total_size_u64, result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::globals::table::{ SyntaxDef, QuerySource, QueryTarget, TargetValue, TOKEN_TABLE };
    use std::sync::Arc;

    // -----------------------------
    // Helpers (TOKEN TABLE)
    // -----------------------------

    fn expected_for(id: &str) -> Arc<[SyntaxDef]> {
        let key = std::borrow::Cow::Owned(id.to_string());
        match TOKEN_TABLE.find((QuerySource::Identifier(key), QueryTarget::Syntax)).unwrap() {
            TargetValue::Syntax(p) => p,
            _ => unreachable!("Expected Params"),
        }
    }

    fn opcode_for(id: &str) -> u32 {
        let key = std::borrow::Cow::Owned(id.to_string());
        match TOKEN_TABLE.find((QuerySource::Identifier(key), QueryTarget::Bytecode)).unwrap() {
            TargetValue::Bytecode(c) => c,
            _ => unreachable!("Expected Bytecode"),
        }
    }

    fn chunks(parts: &[&str]) -> Vec<String> {
        parts
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn is_text_err(code: &AtpErrorCode) -> bool {
        matches!(code, AtpErrorCode::TextParsingError(_))
    }

    fn is_bc_err(code: &AtpErrorCode) -> bool {
        matches!(
            code,
            AtpErrorCode::BytecodeParsingError(_) |
                AtpErrorCode::BytecodeParamParsingError(_) |
                AtpErrorCode::BytecodeParamNotRecognized(_)
        )
    }

    // -----------------------------
    // Bytecode Builders
    // -----------------------------

    fn bc_param(param_type: u32, payload: &[u8]) -> Vec<u8> {
        let total = 8u64 + 4 + 4 + (payload.len() as u64);
        let mut out = Vec::new();
        out.extend_from_slice(&total.to_be_bytes());
        out.extend_from_slice(&param_type.to_be_bytes());
        out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        out.extend_from_slice(payload);
        out
    }

    fn bc_string(s: &str) -> Vec<u8> {
        bc_param(PARAM_STRING, s.as_bytes())
    }
    #[allow(dead_code)]
    fn bc_usize(n: usize) -> Vec<u8> {
        bc_param(PARAM_USIZE, &n.to_be_bytes())
    }

    fn bc_token_param(opcode: u32, params: Vec<Vec<u8>>) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&opcode.to_be_bytes());
        payload.push(params.len() as u8);
        for p in params {
            payload.extend_from_slice(&p);
        }
        bc_param(PARAM_TOKEN, &payload)
    }

    // -----------------------------
    // TEXT tests
    // -----------------------------

    #[test]
    fn text_basic_ifdc_valid() {
        let expected = expected_for("ifdc");
        let parsed = AtpParamTypes::from_expected(
            expected,
            &chunks(&["banana", "do", "atb", "pizza"])
        ).unwrap();

        assert_eq!(parsed.len(), 2);
        match &parsed[0] {
            AtpParamTypes::String(s) => assert_eq!(s, "banana"),
            _ => panic!("Expected String"),
        }
        match &parsed[1] {
            AtpParamTypes::Token(t) => assert_eq!(t.get_string_repr(), "atb"),
            _ => panic!("Expected Token"),
        }
    }

    #[test]
    fn text_blk_assoc_valid() {
        let expected = expected_for("blk");
        let parsed = AtpParamTypes::from_expected(
            expected,
            &chunks(&["x", "assoc", "ifdc", "banana", "do", "tbs"])
        ).unwrap();

        assert_eq!(parsed.len(), 2);
        match &parsed[0] {
            AtpParamTypes::String(s) => assert_eq!(s, "x"),
            _ => panic!("Expected String"),
        }
        match &parsed[1] {
            AtpParamTypes::Token(t) => assert_eq!(t.get_string_repr(), "ifdc"),
            _ => panic!("Expected Token"),
        }
    }

    #[test]
    fn text_ifdc_nest_blk_assoc_raw() {
        let expected = expected_for("ifdc");
        let parsed = AtpParamTypes::from_expected(
            expected,
            &chunks(&["laranja", "do", "blk", "x", "assoc", "raw", "laranja", "abacaxi"])
        ).unwrap();

        match &parsed[1] {
            AtpParamTypes::Token(t) => assert_eq!(t.get_string_repr(), "blk"),
            _ => panic!("Expected Token(blk)"),
        }
    }

    #[test]
    fn text_ifdc_nest_blk_assoc_ifdc_raw_ok() {
        let expected = expected_for("ifdc");
        let parsed = AtpParamTypes::from_expected(
            expected,
            &chunks(
                &[
                    "laranja",
                    "do",
                    "blk",
                    "x",
                    "assoc",
                    "ifdc",
                    "pera",
                    "do",
                    "raw",
                    "laranja",
                    "abacaxi",
                ]
            )
        ).unwrap();

        match &parsed[1] {
            AtpParamTypes::Token(t) => assert_eq!(t.get_string_repr(), "blk"),
            _ => panic!("Expected Token(blk)"),
        }
    }

    #[test]
    fn text_rejects_ifdc_inside_ifdc() {
        let expected = expected_for("ifdc");
        let err = AtpParamTypes::from_expected(
            expected,
            &chunks(&["banana", "do", "ifdc", "coxinha", "do", "atb", "pizza"])
        ).unwrap_err();

        assert!(is_text_err(&err.error_code));
    }

    #[test]
    fn text_rejects_blk_inside_blk_assoc() {
        let expected = expected_for("blk");
        let err = AtpParamTypes::from_expected(
            expected,
            &chunks(&["x", "assoc", "blk", "y", "assoc", "atb", "banana"])
        ).unwrap_err();

        assert!(is_text_err(&err.error_code));
    }

    #[test]
    fn text_rejects_excessive_assoc_depth() {
        let expected = expected_for("blk");
        let err = AtpParamTypes::from_expected(
            expected,
            &chunks(
                &[
                    "x",
                    "assoc",
                    "ifdc",
                    "a",
                    "do",
                    "ifdc",
                    "b",
                    "do",
                    "ifdc",
                    "c",
                    "do",
                    "raw",
                    "d",
                    "e",
                ]
            )
        ).unwrap_err();

        assert!(is_text_err(&err.error_code));
    }

    // -----------------------------
    // BYTECODE tests
    // -----------------------------

    #[test]
    fn bytecode_string_roundtrip() {
        let p = AtpParamTypes::String("abc".to_string());
        let (_total, b) = p.param_to_bytecode();

        let total = u64::from_be_bytes(b[0..8].try_into().unwrap());
        assert_eq!(total as usize, b.len());

        let ty = u32::from_be_bytes(b[8..12].try_into().unwrap());
        assert_eq!(ty, PARAM_STRING);

        let size = u32::from_be_bytes(b[12..16].try_into().unwrap());
        assert_eq!(size, 3);

        let parsed = AtpParamTypes::from_bytecode(b).unwrap();
        match parsed {
            AtpParamTypes::String(s) => assert_eq!(s, "abc"),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn bytecode_usize_roundtrip() {
        let p = AtpParamTypes::Usize(42);
        let (_total, b) = p.param_to_bytecode();

        let parsed = AtpParamTypes::from_bytecode(b).unwrap();
        match parsed {
            AtpParamTypes::Usize(n) => assert_eq!(n, 42),
            _ => panic!("Expected Usize"),
        }
    }

    #[test]
    fn bytecode_token_ifdc_atb() {
        let atb_op = opcode_for("atb");
        let ifdc_op = opcode_for("ifdc");

        let atb_param = bc_token_param(atb_op, vec![bc_string("pizza")]);
        let ifdc_param = bc_token_param(ifdc_op, vec![bc_string("banana"), atb_param]);

        let parsed = AtpParamTypes::from_bytecode(ifdc_param).unwrap();
        println!("\n\n\n\n\n Banana LARANJA");
        match parsed {
            AtpParamTypes::Token(t) => {
                println!("{}", t.to_atp_line());
                assert_eq!(t.get_string_repr(), "ifdc")
            }
            _ => panic!("Expected Token(ifdc)"),
        }
    }

    #[test]
    fn bytecode_rejects_nested_ifdc_outside_blk_assoc() {
        let atb_op = opcode_for("atb");
        let ifdc_op = opcode_for("ifdc");

        let atb_inner = bc_token_param(atb_op, vec![bc_string("pizza")]);
        let ifdc_inner = bc_token_param(ifdc_op, vec![bc_string("coxinha"), atb_inner]);
        let ifdc_outer = bc_token_param(ifdc_op, vec![bc_string("banana"), ifdc_inner]);

        let err = AtpParamTypes::from_bytecode(ifdc_outer).unwrap_err();
        assert!(is_bc_err(&err.error_code));
    }

    #[test]
    fn bytecode_allows_ifdc_blk_assoc_raw() {
        let ifdc_op = opcode_for("ifdc");
        let blk_op = opcode_for("blk");
        let raw_op = opcode_for("raw");

        let raw_tok = bc_token_param(raw_op, vec![bc_string("laranja"), bc_string("abacaxi")]);
        let blk_tok = bc_token_param(blk_op, vec![bc_string("x"), raw_tok]);

        let ifdc_tok = bc_token_param(ifdc_op, vec![bc_string("laranja"), blk_tok]);

        let parsed = AtpParamTypes::from_bytecode(ifdc_tok).unwrap();
        match parsed {
            AtpParamTypes::Token(t) => assert_eq!(t.get_string_repr(), "ifdc"),
            _ => panic!("Expected Token(ifdc)"),
        }
    }

    #[test]
    fn bytecode_rejects_blk_inside_blk_assoc() {
        let blk_op = opcode_for("blk");
        let atb_op = opcode_for("atb");

        let atb_tok = bc_token_param(atb_op, vec![bc_string("banana")]);
        let blk_inner = bc_token_param(blk_op, vec![bc_string("y"), atb_tok]);

        let blk_outer = bc_token_param(blk_op, vec![bc_string("x"), blk_inner]);

        let err = AtpParamTypes::from_bytecode(blk_outer).unwrap_err();
        assert!(is_bc_err(&err.error_code));
    }

    #[test]
    fn bytecode_rejects_excessive_assoc_depth() {
        let blk_op = opcode_for("blk");
        let ifdc_op = opcode_for("ifdc");
        let raw_op = opcode_for("raw");

        let raw_tok = bc_token_param(raw_op, vec![bc_string("d"), bc_string("e")]);
        let ifdc3 = bc_token_param(ifdc_op, vec![bc_string("c"), raw_tok]);
        let ifdc2 = bc_token_param(ifdc_op, vec![bc_string("b"), ifdc3]);
        let ifdc1 = bc_token_param(ifdc_op, vec![bc_string("a"), ifdc2]);

        let blk_outer = bc_token_param(blk_op, vec![bc_string("x"), ifdc1]);

        let err = AtpParamTypes::from_bytecode(blk_outer).unwrap_err();
        assert!(is_bc_err(&err.error_code));
    }
}

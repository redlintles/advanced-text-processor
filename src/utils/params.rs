use core::str;
use std::{
    array::TryFromSliceError,
    io::{BufReader, Read},
    sync::Arc,
};

use crate::{
    globals::table::{
        InstructionParam, ParamType, QuerySource, QueryTarget, TOKEN_TABLE, TargetValue,
    },
    tokens::TokenMethods,
    utils::{
        errors::{AtpError, AtpErrorCode},
        transforms::string_to_usize,
    },
};

pub enum AtpParamTypes {
    String(String),
    Usize(usize),
    Token(Box<dyn TokenMethods>),
}

pub trait AtpParamTypesJoin {
    fn join(&self, sep: &str) -> String;
}

impl AtpParamTypesJoin for Vec<AtpParamTypes> {
    fn join(&self, sep: &str) -> String {
        self.iter()
            .map(|t| t.to_string() + sep)
            .collect::<String>()
            .trim_end()
            .to_string()
    }
}

impl AtpParamTypes {
    pub fn to_string(&self) -> String {
        match self {
            AtpParamTypes::String(payload) => payload.to_string(),
            AtpParamTypes::Usize(payload) => payload.to_string(),
            AtpParamTypes::Token(payload) => payload.to_atp_line().into(),
        }
    }

    pub fn from_expected(
        expected: Arc<[InstructionParam]>,
        chunks: &[String],
    ) -> Result<Vec<AtpParamTypes>, AtpError> {
        let (parsed, consumed) = Self::parse_with_cursor(expected, chunks, 0)?;

        // ✅ se você quiser ser rígido: não pode sobrar chunk “solto”
        if consumed != chunks.len() {
            return Err(AtpError::new(
                AtpErrorCode::TextParsingError("Extra parameters after parsing".into()),
                "AtpParamTypes::from_expected",
                format!("consumed={}, total={}", consumed, chunks.len()),
            ));
        }

        Ok(parsed)
    }

    fn parse_with_cursor(
        expected: Arc<[InstructionParam]>,
        chunks: &[String],
        mut i: usize,
    ) -> Result<(Vec<AtpParamTypes>, usize), AtpError> {
        let mut out = Vec::with_capacity(expected.len());

        for p in expected.iter() {
            match p.param_type {
                ParamType::String => {
                    let s = chunks.get(i).ok_or_else(|| {
                        AtpError::new(
                            AtpErrorCode::TextParsingError("Missing String parameter".into()),
                            "AtpParamTypes::parse_with_cursor",
                            format!("index={}", i),
                        )
                    })?;
                    out.push(AtpParamTypes::String(s.clone()));
                    i += 1;
                }

                ParamType::Usize => {
                    let s = chunks.get(i).ok_or_else(|| {
                        AtpError::new(
                            AtpErrorCode::TextParsingError("Missing Usize parameter".into()),
                            "AtpParamTypes::parse_with_cursor",
                            format!("index={}", i),
                        )
                    })?;
                    out.push(AtpParamTypes::Usize(string_to_usize(s)?));
                    i += 1;
                }

                ParamType::Token => {
                    // 1) ler o id do token aninhado (ex: "atb")
                    let nested_id = chunks
                        .get(i)
                        .ok_or_else(|| {
                            AtpError::new(
                                AtpErrorCode::TextParsingError(
                                    "Missing nested token identifier".into(),
                                ),
                                "AtpParamTypes::parse_with_cursor",
                                format!("index={}", i),
                            )
                        })?
                        .clone();
                    i += 1;

                    // 2) pegar params esperados do token aninhado
                    let nested_expected = match TOKEN_TABLE.find((
                        QuerySource::Identifier(nested_id.clone().into()),
                        QueryTarget::Params,
                    ))? {
                        TargetValue::Params(p) => p,
                        _ => unreachable!("Invalid Query result (Params)"),
                    };

                    // 3) parsear params do token aninhado consumindo do mesmo slice
                    let (nested_params, next_i) =
                        Self::parse_with_cursor(nested_expected, chunks, i)?;
                    i = next_i;

                    // 4) instanciar token aninhado e aplicar params
                    let nested_token_ref = match TOKEN_TABLE.find((
                        QuerySource::Identifier(nested_id.clone().into()),
                        QueryTarget::Token,
                    ))? {
                        TargetValue::Token(t) => t,
                        _ => unreachable!("Invalid Query result (Token)"),
                    };

                    let mut nested_token = nested_token_ref.into_box();
                    nested_token.from_params(&nested_params);

                    out.push(AtpParamTypes::Token(nested_token));
                }

                // Futuro
                ParamType::VarRef => todo!(),
                ParamType::BlockRef => todo!(),
            }
        }

        Ok((out, i))
    }

    pub fn from_bytecode(bytecode: Vec<u8>) -> Result<AtpParamTypes, AtpError> {
        let mut reader = BufReader::new(&bytecode[..]);

        let mut param_type_bytes = [0u8; 4];
        reader.read_exact(&mut param_type_bytes).map_err(|e| {
            AtpError::new(
                AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                "read_bytecode_from_file",
                e.to_string(),
            )
        })?;

        let param_type = u32::from_be_bytes(param_type_bytes);

        // Possível remoção no futuro ao substituir por read_to_end
        let mut param_payload_size_bytes = [0u8; 4];
        reader
            .read_exact(&mut param_payload_size_bytes)
            .map_err(|e| {
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                    "read_bytecode_from_file",
                    e.to_string(),
                )
            })?;

        let param_payload_size = u32::from_be_bytes(param_payload_size_bytes) as usize;

        let mut param_payload_bytes = vec![0u8; param_payload_size];

        reader.read_exact(&mut param_payload_bytes).map_err(|e| {
            AtpError::new(
                AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                "read_bytecode_from_file",
                e.to_string(),
            )
        })?;

        match param_type {
            0x01 => {
                let text = str::from_utf8(&param_payload_bytes).map_err(|e| {
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::BytecodeParamParsingError(
                            "Failed Parsing Bytes to UTF8 String".into(),
                        ),
                        "AtpParamTypes.from_bytecode()",
                        e.to_string(),
                    )
                })?;
                Ok(AtpParamTypes::String(text.to_string()))
            }
            0x02 => {
                let b: [u8; 8] =
                    param_payload_bytes
                        .as_slice()
                        .try_into()
                        .map_err(|e: TryFromSliceError| {
                            AtpError::new(
                                crate::utils::errors::AtpErrorCode::BytecodeParamParsingError(
                                    "Failed Parsing Bytes to Usize".into(),
                                ),
                                "AtpParamTypes.from_bytecode()",
                                e.to_string(),
                            )
                        })?;

                let n = usize::from_be_bytes(b);
                Ok(AtpParamTypes::Usize(n))
            }
            0x03 => {
                let mut reader = BufReader::new(&param_payload_bytes[..]);

                let mut instruction_type_bytes = [0u8; 4];

                reader
                    .read_exact(&mut instruction_type_bytes)
                    .map_err(|e| {
                        AtpError::new(
                            AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                            "read_bytecode_from_file",
                            e.to_string(),
                        )
                    })?;

                let instruction_type = u32::from_be_bytes(instruction_type_bytes);

                let mut param_count_bytes = [0u8; 1];

                reader.read_exact(&mut param_count_bytes).map_err(|e| {
                    AtpError::new(
                        AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                        "read_bytecode_from_file",
                        e.to_string(),
                    )
                })?;

                let param_count = u8::from_be_bytes(param_count_bytes) as usize;

                let mut params: Vec<AtpParamTypes> = Vec::with_capacity(param_count);

                for _ in 0..param_count {
                    let mut param_length_bytes = [0u8; 8];

                    reader.read_exact(&mut param_length_bytes).map_err(|e| {
                        AtpError::new(
                            AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                            "read_bytecode_from_file",
                            e.to_string(),
                        )
                    })?;

                    let param_length = usize::from_be_bytes(param_length_bytes);

                    let mut v = vec![0u8; param_length];

                    reader.read_exact(&mut v).map_err(|e| {
                        AtpError::new(
                            AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                            "read_bytecode_from_file",
                            e.to_string(),
                        )
                    })?;

                    let parsed_param = AtpParamTypes::from_bytecode(v)?;

                    params.push(parsed_param);
                }

                let query_result = TOKEN_TABLE
                    .find((QuerySource::Bytecode(instruction_type), QueryTarget::Token))?;

                match query_result {
                    TargetValue::Token(token_ref) => {
                        let mut token = token_ref.into_box();

                        token.from_params(&params)?;

                        Ok(AtpParamTypes::Token(token))
                    }
                    _ => unreachable!("Invalid Query Result!"),
                }
            }
            _ => Err(AtpError::new(
                crate::utils::errors::AtpErrorCode::BytecodeParamNotRecognized(
                    "Param Bytecode Not Recognized".into(),
                ),
                "",
                "",
            )),
        }
    }
}

impl AtpParamTypes {
    pub fn get_param_type_code(&self) -> u32 {
        match self {
            AtpParamTypes::String(_) => 0x01,
            AtpParamTypes::Usize(_) => 0x02,
            AtpParamTypes::Token(_) => 0x03,
        }
    }

    pub fn write_as_instruction_param(&self, out: &mut Vec<u8>) {
        let param_type = self.get_param_type_code();

        let payload: Vec<u8> = match self {
            AtpParamTypes::String(s) => s.as_bytes().to_vec(),
            AtpParamTypes::Usize(n) => n.to_be_bytes().to_vec(),
            AtpParamTypes::Token(t) => t.to_bytecode(), // token aninhado
        };

        let payload_size_u32: u32 = payload.len() as u32;

        // Param Total Size = 8 + 4 + 4 + payload
        let param_total_size_u64: u64 = 8 + 4 + 4 + (payload.len() as u64);

        out.extend_from_slice(&param_total_size_u64.to_be_bytes());
        out.extend_from_slice(&param_type.to_be_bytes());
        out.extend_from_slice(&payload_size_u32.to_be_bytes());
        out.extend_from_slice(&payload);
    }

    pub fn param_to_bytecode(&self) -> (u64, Vec<u8>) {
        let mut result: Vec<u8> = Vec::new();

        // Payload Type
        result.extend_from_slice(&self.get_param_type_code().to_be_bytes());
        let payload = match self {
            AtpParamTypes::String(x) => x.as_bytes().to_vec(),
            AtpParamTypes::Usize(x) => x.to_be_bytes().to_vec(),
            AtpParamTypes::Token(x) => x.to_bytecode(),
        };

        // Param Size
        let payload_size = &(payload.len() as u32);

        let param_total_size = ((4 + 4 + payload_size) as u64).to_be_bytes();

        result.extend_from_slice(&param_total_size);

        result.extend_from_slice(&payload_size.to_be_bytes());
        // Payload Content
        result.extend_from_slice(&payload);

        (u64::from_be_bytes(param_total_size), result)
    }
}

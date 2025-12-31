use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
    path::Path,
};

use crate::{
    globals::table::{ParamType, QuerySource, QueryTarget, TOKEN_TABLE, TargetValue},
    tokens::TokenMethods,
    utils::{
        errors::{AtpError, AtpErrorCode},
        params::AtpParamTypes,
        validations::check_file_path,
    },
};

fn param_type_from_code(code: u32) -> Option<ParamType> {
    match code {
        0x01 => Some(ParamType::String),
        0x02 => Some(ParamType::Usize),
        0x03 => Some(ParamType::Token),
        _ => None,
    }
}

pub fn read_bytecode_from_file(path: &Path) -> Result<Vec<Box<dyn TokenMethods>>, AtpError> {
    check_file_path(path, Some("atpbc"))?;

    let file = OpenOptions::new().read(true).open(path).map_err(|_| {
        AtpError::new(
            AtpErrorCode::FileOpeningError("Failed opening File".into()),
            "bytecode reader",
            format!("{:?}", path),
        )
    })?;

    let mut reader = BufReader::new(file);

    // --- header ---
    let mut magic_number = [0u8; 8];
    let expected_magic_number: [u8; 8] = [38, 235, 245, 8, 244, 137, 1, 179];
    reader.read_exact(&mut magic_number).map_err(|e| {
        AtpError::new(
            AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
            "read_bytecode_from_file",
            e.to_string(),
        )
    })?;

    if magic_number != expected_magic_number {
        return Err(AtpError::new(
            AtpErrorCode::FileReadingError("Incompatible Magic Number with ATP".into()),
            "bytecode reader",
            "",
        ));
    }

    let mut protocol_version_bytes = [0u8; 8];
    reader
        .read_exact(&mut protocol_version_bytes)
        .map_err(|e| {
            AtpError::new(
                AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                "read_bytecode_from_file",
                e.to_string(),
            )
        })?;
    let protocol_version = u64::from_be_bytes(protocol_version_bytes);

    let mut instruction_count_bytes = [0u8; 4];
    reader
        .read_exact(&mut instruction_count_bytes)
        .map_err(|e| {
            AtpError::new(
                AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                "read_bytecode_from_file",
                e.to_string(),
            )
        })?;
    let instruction_count = u32::from_be_bytes(instruction_count_bytes);

    if protocol_version != 1 {
        return Err(AtpError::new(
            AtpErrorCode::BytecodeParsingError("Unsupported protocol version".into()),
            "read_bytecode_from_file",
            protocol_version.to_string(),
        ));
    }

    let mut result: Vec<Box<dyn TokenMethods>> = Vec::with_capacity(instruction_count as usize);

    // --- body ---
    for _ in 0..instruction_count {
        // instruction type
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

        // param count
        let mut instruction_param_count_bytes = [0u8; 1];
        reader
            .read_exact(&mut instruction_param_count_bytes)
            .map_err(|e| {
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                    "read_bytecode_from_file",
                    e.to_string(),
                )
            })?;
        let instruction_param_count = u8::from_be_bytes(instruction_param_count_bytes) as usize;

        // expected params (schema)
        let expected = match TOKEN_TABLE
            .find((QuerySource::Bytecode(instruction_type), QueryTarget::Params))?
        {
            TargetValue::Params(p) => p,
            _ => unreachable!("Invalid query result (Params)"),
        };

        // validação de contagem (com opcionais)
        let min_required = expected.iter().filter(|p| !p.optional).count();
        let max_allowed = expected.len();

        if instruction_param_count < min_required || instruction_param_count > max_allowed {
            return Err(AtpError::new(
                AtpErrorCode::BytecodeParsingError("Invalid param count for instruction".into()),
                "read_bytecode_from_file",
                format!(
                    "instr=0x{:x} got={} expected=[min={}, max={}]",
                    instruction_type, instruction_param_count, min_required, max_allowed
                ),
            ));
        }

        // ler e validar params
        let mut params: Vec<AtpParamTypes> = Vec::with_capacity(instruction_param_count);

        // vamos consumir exatamente instruction_param_count params do bytecode,
        // e checar que eles batem com os tipos esperados, respeitando opcionais.
        let mut expected_i = 0usize;

        for param_i in 0..instruction_param_count {
            // lê Param Total Size
            let mut param_total_size_bytes = [0u8; 8];
            reader
                .read_exact(&mut param_total_size_bytes)
                .map_err(|e| {
                    AtpError::new(
                        AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                        "read_bytecode_from_file",
                        e.to_string(),
                    )
                })?;
            let param_total_size = u64::from_be_bytes(param_total_size_bytes);

            if param_total_size < 8 {
                return Err(AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Invalid param_total_size".into()),
                    "read_bytecode_from_file",
                    format!("param_total_size={}", param_total_size),
                ));
            }

            // ✅ CORREÇÃO: já consumimos 8 bytes do total_size, agora lemos o resto
            let remaining = (param_total_size - 8) as usize;
            let mut param_data_bytes: Vec<u8> = vec![0u8; remaining];
            reader.read_exact(&mut param_data_bytes).map_err(|e| {
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                    "read_bytecode_from_file",
                    e.to_string(),
                )
            })?;

            // valida tipo vs schema (olhando os 4 primeiros bytes: param_type)
            if param_data_bytes.len() < 4 {
                return Err(AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Param too short".into()),
                    "read_bytecode_from_file",
                    format!("instr=0x{:x} param_index={}", instruction_type, param_i),
                ));
            }

            let type_code = u32::from_be_bytes([
                param_data_bytes[0],
                param_data_bytes[1],
                param_data_bytes[2],
                param_data_bytes[3],
            ]);
            let actual_pt = param_type_from_code(type_code).ok_or_else(|| {
                AtpError::new(
                    AtpErrorCode::BytecodeParamNotRecognized(
                        "Param Bytecode Not Recognized".into(),
                    ),
                    "read_bytecode_from_file",
                    format!("type_code=0x{:x}", type_code),
                )
            })?;

            // avança expected_i até achar um tipo compatível (pulando opcionais)
            while expected_i < expected.len() {
                let expected_pt = expected[expected_i].param_type;
                if (expected_pt as u8) == (actual_pt as u8) {
                    break;
                }

                if expected[expected_i].optional {
                    expected_i += 1;
                    continue;
                }

                return Err(AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Param type mismatch".into()),
                    "read_bytecode_from_file",
                    format!(
                        "instr=0x{:x} param={} got={:?} expected={:?}",
                        instruction_type, param_i, actual_pt, expected_pt
                    ),
                ));
            }

            if expected_i >= expected.len() {
                return Err(AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Too many params (schema overflow)".into()),
                    "read_bytecode_from_file",
                    format!("instr=0x{:x} param_index={}", instruction_type, param_i),
                ));
            }

            // parse real (recursivo se for Token)
            params.push(AtpParamTypes::from_bytecode(param_data_bytes)?);
            expected_i += 1;
        }

        // instancia token
        let token_ref = match TOKEN_TABLE
            .find((QuerySource::Bytecode(instruction_type), QueryTarget::Token))?
        {
            TargetValue::Token(t) => t,
            _ => unreachable!("Invalid query result (Token)"),
        };

        let mut token = token_ref.into_box();
        token.from_params(&params).map_err(|e| {
            AtpError::new(
                AtpErrorCode::BytecodeParsingError("Failed applying params".into()),
                "read_bytecode_from_file",
                e.to_string(),
            )
        })?;

        result.push(token);
    }

    Ok(result)
}

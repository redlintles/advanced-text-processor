use std::{ fs::OpenOptions, io::{ BufReader, Read }, path::Path };

use crate::{
    globals::table::{ SyntaxToken, QuerySource, QueryTarget, TOKEN_TABLE, TargetValue },
    tokens::InstructionMethods,
    utils::{
        errors::{ AtpError, AtpErrorCode },
        params::AtpParamTypes,
        validations::check_file_path,
    },
};

fn param_type_from_code(code: u32) -> Option<SyntaxToken> {
    match code {
        0x01 => Some(SyntaxToken::String),
        0x02 => Some(SyntaxToken::Usize),
        0x03 => Some(SyntaxToken::Token),
        _ => None,
    }
}

fn bytecode_compatible(expected: &SyntaxToken, actual: &SyntaxToken) -> bool {
    match (expected, actual) {
        (SyntaxToken::String, SyntaxToken::String) => true,
        (SyntaxToken::Usize, SyntaxToken::Usize) => true,
        (SyntaxToken::Token, SyntaxToken::Token) => true,
        _ => false,
    }
}

pub fn read_bytecode_from_file(path: &Path) -> Result<Vec<Box<dyn InstructionMethods>>, AtpError> {
    check_file_path(path, Some("atpbc"))?;

    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|_| {
            AtpError::new(
                AtpErrorCode::FileOpeningError("Failed opening File".into()),
                "bytecode reader",
                format!("{:?}", path)
            )
        })?;

    let mut reader = BufReader::new(file);

    // --- header ---
    let mut magic_number = [0u8; 8];
    let expected_magic_number: [u8; 8] = [38, 235, 245, 8, 244, 137, 1, 179];

    reader
        .read_exact(&mut magic_number)
        .map_err(|e| {
            AtpError::new(
                AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                "read_bytecode_from_file",
                e.to_string()
            )
        })?;

    if magic_number != expected_magic_number {
        return Err(
            AtpError::new(
                AtpErrorCode::FileReadingError("Incompatible Magic Number with ATP".into()),
                "bytecode reader",
                ""
            )
        );
    }

    let mut protocol_version_bytes = [0u8; 8];
    reader
        .read_exact(&mut protocol_version_bytes)
        .map_err(|e| {
            AtpError::new(
                AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                "read_bytecode_from_file",
                e.to_string()
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
                e.to_string()
            )
        })?;
    let instruction_count = u32::from_be_bytes(instruction_count_bytes);

    if protocol_version != 1 {
        return Err(
            AtpError::new(
                AtpErrorCode::BytecodeParsingError("Unsupported protocol version".into()),
                "read_bytecode_from_file",
                protocol_version.to_string()
            )
        );
    }

    let mut result: Vec<Box<dyn InstructionMethods>> = Vec::with_capacity(
        instruction_count as usize
    );

    // --- body ---
    for _ in 0..instruction_count {
        // 1) instruction_total_size (u64)
        let mut instruction_total_size_bytes = [0u8; 8];
        reader
            .read_exact(&mut instruction_total_size_bytes)
            .map_err(|e| {
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                    "read_bytecode_from_file",
                    e.to_string()
                )
            })?;
        let instruction_total_size = u64::from_be_bytes(instruction_total_size_bytes);

        // mínimo = opcode(4) + param_count(1) = 5
        if instruction_total_size < 5 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Invalid instruction_total_size".into()),
                    "read_bytecode_from_file",
                    format!("instruction_total_size={}", instruction_total_size)
                )
            );
        }

        // Vamos contar bytes consumidos "dentro" da instrução (sem contar os 8 do total_size).
        let mut consumed_in_instruction: u64 = 0;

        // 2) opcode (u32)
        let mut instruction_type_bytes = [0u8; 4];
        reader
            .read_exact(&mut instruction_type_bytes)
            .map_err(|e| {
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                    "read_bytecode_from_file",
                    e.to_string()
                )
            })?;
        let instruction_type = u32::from_be_bytes(instruction_type_bytes);
        consumed_in_instruction += 4;

        if instruction_type == 0 {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Invalid instruction type (0)".into()),
                    "read_bytecode_from_file",
                    "instruction_type=0".to_string()
                )
            );
        }

        // 3) param_count (u8)
        let mut instruction_param_count_byte = [0u8; 1];
        reader
            .read_exact(&mut instruction_param_count_byte)
            .map_err(|e| {
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                    "read_bytecode_from_file",
                    e.to_string()
                )
            })?;
        let instruction_param_count = instruction_param_count_byte[0] as usize;
        consumed_in_instruction += 1;

        // debug opcional
        eprintln!(
            "instr_total_size={} opcode=0x{:08x} param_count={}",
            instruction_total_size,
            instruction_type,
            instruction_param_count
        );

        // expected params (schema)
        let expected = match
            TOKEN_TABLE.find((QuerySource::Bytecode(instruction_type), QueryTarget::Syntax))?
        {
            TargetValue::Syntax(p) => p,
            _ => unreachable!("Invalid query result (Params)"),
        };

        let min_required = expected
            .iter()
            .filter(|p| !p.optional)
            .count();
        let max_allowed = expected.len();

        if instruction_param_count < min_required || instruction_param_count > max_allowed {
            return Err(
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError(
                        "Invalid param count for instruction".into()
                    ),
                    "read_bytecode_from_file",
                    format!(
                        "instr=0x{:x} got={} expected=[min={}, max={}]",
                        instruction_type,
                        instruction_param_count,
                        min_required,
                        max_allowed
                    )
                )
            );
        }

        // ler e validar params
        let mut params: Vec<AtpParamTypes> = Vec::with_capacity(instruction_param_count);
        let mut expected_i = 0usize;

        for param_i in 0..instruction_param_count {
            // Param Total Size (u64)
            let mut param_total_size_bytes = [0u8; 8];
            reader
                .read_exact(&mut param_total_size_bytes)
                .map_err(|e| {
                    AtpError::new(
                        AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                        "read_bytecode_from_file",
                        e.to_string()
                    )
                })?;
            let param_total_size = u64::from_be_bytes(param_total_size_bytes);
            consumed_in_instruction += 8;

            if param_total_size < 8 {
                return Err(
                    AtpError::new(
                        AtpErrorCode::BytecodeParsingError("Invalid param_total_size".into()),
                        "read_bytecode_from_file",
                        format!("param_total_size={}", param_total_size)
                    )
                );
            }

            // já consumimos 8 bytes do total_size, agora lemos o restante do param
            let remaining = (param_total_size - 8) as usize;
            let mut param_data_bytes = vec![0u8; remaining];
            reader
                .read_exact(&mut param_data_bytes)
                .map_err(|e| {
                    AtpError::new(
                        AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                        "read_bytecode_from_file",
                        e.to_string()
                    )
                })?;
            consumed_in_instruction += remaining as u64;

            // valida tipo vs schema (primeiros 4 bytes do payload do param)
            if param_data_bytes.len() < 4 {
                return Err(
                    AtpError::new(
                        AtpErrorCode::BytecodeParsingError("Param too short".into()),
                        "read_bytecode_from_file",
                        format!("instr=0x{:x} param_index={}", instruction_type, param_i)
                    )
                );
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
                        "Param Bytecode Not Recognized".into()
                    ),
                    "read_bytecode_from_file",
                    format!("type_code=0x{:x}", type_code)
                )
            })?;

            while expected_i < expected.len() {
                let expected_pt = &expected[expected_i].token;

                if matches!(expected_pt, SyntaxToken::Literal(_)) {
                    expected_i += 1;
                    continue;
                }

                if bytecode_compatible(expected_pt, &actual_pt) {
                    break;
                }

                if expected[expected_i].optional {
                    expected_i += 1;
                    continue;
                }

                return Err(
                    AtpError::new(
                        AtpErrorCode::BytecodeParsingError("Param type mismatch".into()),
                        "read_bytecode_from_file",
                        format!(
                            "instr=0x{:x} param={} got={:?} expected={:?}",
                            instruction_type,
                            param_i,
                            actual_pt,
                            expected_pt
                        )
                    )
                );
            }

            if expected_i >= expected.len() {
                return Err(
                    AtpError::new(
                        AtpErrorCode::BytecodeParsingError(
                            "Too many params (schema overflow)".into()
                        ),
                        "read_bytecode_from_file",
                        format!("instr=0x{:x} param_index={}", instruction_type, param_i)
                    )
                );
            }

            params.push(AtpParamTypes::from_bytecode(param_data_bytes)?);
            expected_i += 1;
        }

        // ✅ guardrail: garante que consumimos exatamente instruction_total_size bytes (sem contar os 8 do total_size)
        if consumed_in_instruction != instruction_total_size {
            // se consumimos menos, podemos "pular" o resto; se consumimos mais, é erro fatal
            if consumed_in_instruction < instruction_total_size {
                let to_skip = (instruction_total_size - consumed_in_instruction) as usize;
                let mut skip_buf = vec![0u8; to_skip];
                reader
                    .read_exact(&mut skip_buf)
                    .map_err(|e| {
                        AtpError::new(
                            AtpErrorCode::BytecodeParsingError(
                                "Failed skipping instruction padding".into()
                            ),
                            "read_bytecode_from_file",
                            e.to_string()
                        )
                    })?;
            } else {
                return Err(
                    AtpError::new(
                        AtpErrorCode::BytecodeParsingError(
                            "Instruction size mismatch (over-read)".into()
                        ),
                        "read_bytecode_from_file",
                        format!(
                            "opcode=0x{:x} consumed={} expected={}",
                            instruction_type,
                            consumed_in_instruction,
                            instruction_total_size
                        )
                    )
                );
            }
        }

        // instancia token
        let token_ref = match
            TOKEN_TABLE.find((QuerySource::Bytecode(instruction_type), QueryTarget::Token))?
        {
            TargetValue::Token(t) => t,
            _ => unreachable!("Invalid query result (Token)"),
        };

        let mut token = token_ref.into_box();
        token
            .from_params(&params)
            .map_err(|e| {
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed applying params".into()),
                    "read_bytecode_from_file",
                    e.to_string()
                )
            })?;

        result.push(token);
    }

    Ok(result)
}

use std::{ fs::OpenOptions, io::{ BufReader, Read }, path::Path };

use crate::utils::{
    params::AtpParamTypes,
    errors::{ AtpError, AtpErrorCode },
    mapping::get_mapping_bytecode_to_token,
    validations::check_file_path,
};

use super::{ BytecodeTokenMethods };

pub fn read_bytecode_from_file(
    path: &Path
) -> Result<Vec<Box<dyn BytecodeTokenMethods>>, AtpError> {
    check_file_path(path, Some("atpbc"))?;
    let mut result: Vec<Box<dyn BytecodeTokenMethods>> = Vec::new();

    let file = match OpenOptions::new().read(true).open(path) {
        Ok(x) => x,
        Err(_) => {
            return Err(
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::FileOpeningError(
                        "Failed opening File".into()
                    ),
                    "",
                    format!("{:?}", path)
                )
            );
        }
    };

    let mut reader = BufReader::new(file);

    let mut magic_number = [0u8; 8];
    let expected_magic_number: [u8; 8] = [38, 235, 245, 8, 244, 137, 1, 179];

    reader
        .read_exact(&mut magic_number)
        .map_err(|e|
            AtpError::new(
                AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                "read_bytecode_from_file",
                e.to_string()
            )
        )?;

    if magic_number != expected_magic_number {
        return Err(
            AtpError::new(
                AtpErrorCode::FileReadingError("Incompatible Magic Number with ATP".into()),
                "bytecode reader",
                ""
            )
        );
    }

    let mut protocol_version_bytes: [u8; 8] = [0u8; 8];
    reader
        .read_exact(&mut protocol_version_bytes)
        .map_err(|e|
            AtpError::new(
                AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                "read_bytecode_from_file",
                e.to_string()
            )
        )?;

    let protocol_version = u64::from_be_bytes(protocol_version_bytes);

    let mut instruction_count_bytes = [0u8; 4];
    reader
        .read_exact(&mut instruction_count_bytes)
        .map_err(|e|
            AtpError::new(
                AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                "read_bytecode_from_file",
                e.to_string()
            )
        )?;

    let instruction_count = u32::from_be_bytes(instruction_count_bytes);

    if protocol_version == 1 {
        for _ in 0..instruction_count {
            // Tipo da instrução
            let mut instruction_type_bytes = [0u8; 4];
            reader
                .read_exact(&mut instruction_type_bytes)
                .map_err(|e|
                    AtpError::new(
                        AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                        "read_bytecode_from_file",
                        e.to_string()
                    )
                )?;
            let instruction_type = u32::from_be_bytes(instruction_type_bytes);

            // Quantidade de parâmetros da instrução

            let mut instruction_param_count_bytes = [0u8; 1];

            reader
                .read_exact(&mut instruction_param_count_bytes)
                .map_err(|e|
                    AtpError::new(
                        AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                        "read_bytecode_from_file",
                        e.to_string()
                    )
                )?;

            let instruction_param_count = u8::from_be_bytes(instruction_param_count_bytes);

            let mut params: Vec<AtpParamTypes> = Vec::new();

            for _ in 0..instruction_param_count {
                let mut param_total_size_bytes = [0u8; 8];
                reader
                    .read_exact(&mut param_total_size_bytes)
                    .map_err(|e|
                        AtpError::new(
                            AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                            "read_bytecode_from_file",
                            e.to_string()
                        )
                    )?;
                let param_total_size = u64::from_be_bytes(param_total_size_bytes);
                let mut param_data_bytes: Vec<u8> = vec![0u8; param_total_size as usize];
                reader
                    .read_exact(&mut param_data_bytes)
                    .map_err(|e|
                        AtpError::new(
                            AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                            "read_bytecode_from_file",
                            e.to_string()
                        )
                    )?;

                params.push(AtpParamTypes::from_bytecode(param_data_bytes)?);

                // Código final para fazer o parsing dos parâmetros
            }

            let mut token = get_mapping_bytecode_to_token()
                .get(&instruction_type)
                .ok_or_else(||
                    AtpError::new(
                        AtpErrorCode::TokenNotFound("Invalid Bytecode".into()),
                        "parse_bytecode_token",
                        instruction_type.to_string()
                    )
                )?();

            token
                .from_params(&params)
                .map_err(|e|
                    AtpError::new(
                        AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                        "read_bytecode_from_file",
                        e.to_string()
                    )
                )?;

            result.push(token);
        }
    }

    Ok(result)
}

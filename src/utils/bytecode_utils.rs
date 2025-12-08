use std::{ array::TryFromSliceError, str::Utf8Error };

use crate::{
    bytecode::BytecodeTokenMethods,
    utils::{ errors::AtpError, mapping::{ get_mapping_bytecode_to_token } },
};

pub enum AtpParamTypes {
    String(String),
    Usize(usize),
    Token(Box<dyn BytecodeTokenMethods>),
}

impl AtpParamTypes {
    pub fn get_param_type_code(x: AtpParamTypes) -> u32 {
        match x {
            AtpParamTypes::String(_) => 0x01,
            AtpParamTypes::Usize(_) => 0x02,
            AtpParamTypes::Token(_) => 0x03,
        }
    }
}

// [type, size, payload]

// Param Type - 4 Bytes
// Param Size - 4 bytes
// Param Payload - "Size" Bytes

/*
pub fn parse_bytecode_param(param_vec: Vec<u8>) -> Result<AtpParamTypes, AtpError> {
    let param_type_slice: [u8; 4] = param_vec[0..=3].try_into().map_err(|e: TryFromSliceError|
        AtpError::new(
            super::errors::AtpErrorCode::BytecodeParsingError(e.to_string().into()),
            "parse_bytecode_param usize",
            param_vec
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    )?;

    let param_size_slice: [u8; 4] = param_vec[4..=7].try_into().map_err(|e: TryFromSliceError|
        AtpError::new(
            super::errors::AtpErrorCode::BytecodeParsingError(e.to_string().into()),
            "parse_bytecode_param usize",
            param_vec
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    )?;
    let param_type = u32::from_be_bytes(param_type_slice);

    let param_size = u32::from_be_bytes(param_size_slice);

    match param_type {
        0x01 => {
            let slice = &param_vec[8..];

            let text = std::str::from_utf8(slice).map_err(|e: Utf8Error|
                AtpError::new(
                    super::errors::AtpErrorCode::BytecodeParsingError(e.to_string().into()),
                    "parse_bytecode_param string block",
                    param_vec
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            )?;

            Ok(AtpParamTypes::String(text.to_string()))
        }
        0x02 => {
            if param_size == 8 {
                let slice = &param_vec[8..];
                let arr: [u8; 8] = slice.try_into().map_err(|e: TryFromSliceError|
                    AtpError::new(
                        super::errors::AtpErrorCode::BytecodeParsingError(e.to_string().into()),
                        "parse_bytecode_param usize block",
                        param_vec
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    )
                )?;

                return Ok(AtpParamTypes::Usize(usize::from_be_bytes(arr)));
            }

            Err(
                AtpError::new(
                    super::errors::AtpErrorCode::BytecodeParsingError(
                        "Failed Parsing Usize, invalid param size".into()
                    ),
                    "parse_bytecode_param usize block",
                    ""
                )
            )
        }
        0x03 => { Ok(AtpParamTypes::Token(Box::new(Tua::default()))) }
        _ => {
            Err(
                AtpError::new(
                    super::errors::AtpErrorCode::BytecodeParamNotRecognized(
                        "Param type not recognized".into()
                    ),
                    param_vec[0].to_string(),
                    param_vec
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            )
        }
    }
}
*/

pub fn parse_bytecode_param(param_vec: Vec<u8>) -> Result<AtpParamTypes, AtpError> {
    // param_vec layout:
    // [ Param Total Size - 8 ]   <- já removido pela função chamadora
    // [ Param Type - 4 ]
    // [ Payload Size - 4 ]
    // [ Payload - N ]

    if param_vec.len() < 8 {
        return Err(
            AtpError::new(
                super::errors::AtpErrorCode::BytecodeParsingError("Param too small".into()),
                "parse_bytecode_param",
                format!("{:?}", param_vec)
            )
        );
    }

    let param_type_bytes: [u8; 4] = param_vec[0..4]
        .try_into()
        .map_err(|e: TryFromSliceError|
            AtpError::new(
                super::errors::AtpErrorCode::BytecodeParsingError(e.to_string().into()),
                "parse_bytecode_param type",
                format!("{:?}", param_vec)
            )
        )?;

    let payload_size_bytes: [u8; 4] = param_vec[4..8]
        .try_into()
        .map_err(|e: TryFromSliceError|
            AtpError::new(
                super::errors::AtpErrorCode::BytecodeParsingError(e.to_string().into()),
                "parse_bytecode_param payload size",
                format!("{:?}", param_vec)
            )
        )?;

    let param_type = u32::from_be_bytes(param_type_bytes);
    let payload_size = u32::from_be_bytes(payload_size_bytes) as usize;

    let payload_start = 8;
    let payload_end = payload_start + payload_size;

    if payload_end > param_vec.len() {
        return Err(
            AtpError::new(
                super::errors::AtpErrorCode::BytecodeParsingError("Payload exceeds buffer".into()),
                "parse_bytecode_param bounds",
                format!("{:?}", param_vec)
            )
        );
    }

    let payload = &param_vec[payload_start..payload_end];

    match param_type {
        0x01 => {
            // STRING
            let text = std::str
                ::from_utf8(payload)
                .map_err(|e: Utf8Error|
                    AtpError::new(
                        super::errors::AtpErrorCode::BytecodeParsingError(e.to_string().into()),
                        "parse_bytecode_param string",
                        format!("{:?}", param_vec)
                    )
                )?;
            Ok(AtpParamTypes::String(text.to_string()))
        }

        0x02 => {
            // USIZE
            if payload_size != 8 {
                return Err(
                    AtpError::new(
                        super::errors::AtpErrorCode::BytecodeParsingError(
                            "Invalid usize payload size".into()
                        ),
                        "parse_bytecode_param usize",
                        format!("{:?}", param_vec)
                    )
                );
            }

            let arr: [u8; 8] = payload
                .try_into()
                .map_err(|e: TryFromSliceError|
                    AtpError::new(
                        super::errors::AtpErrorCode::BytecodeParsingError(e.to_string().into()),
                        "parse_bytecode_param usize",
                        format!("{:?}", param_vec)
                    )
                )?;

            Ok(AtpParamTypes::Usize(usize::from_be_bytes(arr)))
        }

        0x03 => {
            // TOKEN (Transformation only; Instruction only if blk)
            let inner_token = parse_bytecode_token(payload.to_vec())?;
            Ok(AtpParamTypes::Token(inner_token))
        }

        _ =>
            Err(
                AtpError::new(
                    super::errors::AtpErrorCode::BytecodeParamNotRecognized(
                        "Unknown parameter type".into()
                    ),
                    param_type.to_string(),
                    format!("{:?}", param_vec)
                )
            ),
    }
}

// Instruction Type - 4 Bytes
// Param Count - 1 Byte
// Param Total Size - 8 bytes - 8 + 4 + 4 + N
// Param Type - 4 bytes
// Param Payload Size - 4 Bytes
// Param Payload - N Bytes
/*
pub fn parse_bytecode_token(token_vec: Vec<u8>) -> Result<Box<dyn BytecodeTokenMethods>, AtpError> {
    let bytecode_mapping = get_mapping_bytecode_to_token();

    let instruction_type_bytes: [u8; 4] = token_vec[0..=3].try_into().unwrap();
    let instruction_type = u32::from_be_bytes(instruction_type_bytes);
    let token = bytecode_mapping.get(&instruction_type).unwrap()();
    let param_count: usize = token_vec[4].try_into().unwrap();

    let mut offset: usize = 5;

    let mut parsed_params: Vec<AtpParamTypes> = vec![];

    for _ in 0..param_count {
        let param_total_size_end = offset + 8;
        let param_total_size_bytes: [u8; 8] = token_vec[offset..param_total_size_end]
            .try_into()
            .unwrap();
        let param_total_size: usize = usize::from_be_bytes(param_total_size_bytes);

        let param_end = offset + param_total_size;

        let param_bytes: Vec<u8> = token_vec[offset..param_end].to_vec();

        offset += param_total_size;

        parsed_params.push(parse_bytecode_param(param_bytes).unwrap());
    }

    token.token_from_bytecode_instruction(parsed_params);

    Ok(token)
}
*/

pub fn parse_bytecode_token(token_vec: Vec<u8>) -> Result<Box<dyn BytecodeTokenMethods>, AtpError> {
    // token_vec layout:
    // [ Instruction Type - 4 ]
    // [ Param Count - 1 ]
    // loop:
    //     [ Param Total Size - 8 ]
    //     [ Param Type - 4 ]
    //     [ Payload Size - 4 ]
    //     [ Payload - N ]

    if token_vec.len() < 5 {
        return Err(
            AtpError::new(
                super::errors::AtpErrorCode::BytecodeParsingError("Token too small".into()),
                "parse_bytecode_token",
                format!("{:?}", token_vec)
            )
        );
    }

    let mapping = get_mapping_bytecode_to_token();

    let instr_type_bytes: [u8; 4] = token_vec[0..4]
        .try_into()
        .map_err(|_|
            AtpError::new(
                super::errors::AtpErrorCode::BytecodeParsingError(
                    "Invalid instruction type".into()
                ),
                "parse_bytecode_token",
                format!("{:?}", token_vec)
            )
        )?;

    let instr_type = u32::from_be_bytes(instr_type_bytes);

    let constructor = mapping
        .get(&instr_type)
        .ok_or_else(||
            AtpError::new(
                super::errors::AtpErrorCode::BytecodeParsingError("Unknown instruction".into()),
                format!("{}", instr_type),
                format!("{:?}", token_vec)
            )
        )?;

    let mut token = constructor();

    let param_count = token_vec[4] as usize;

    let mut offset = 5;
    let mut parsed_params: Vec<AtpParamTypes> = Vec::new();

    for _ in 0..param_count {
        if offset + 8 > token_vec.len() {
            return Err(
                AtpError::new(
                    super::errors::AtpErrorCode::BytecodeParsingError(
                        "Param header truncated".into()
                    ),
                    "parse_bytecode_token param header",
                    format!("{:?}", token_vec)
                )
            );
        }

        // Read total param size
        let total_size_bytes: [u8; 8] = token_vec[offset..offset + 8].try_into().unwrap();

        let total_size = usize::from_be_bytes(total_size_bytes);

        if offset + total_size > token_vec.len() {
            return Err(
                AtpError::new(
                    super::errors::AtpErrorCode::BytecodeParsingError("Param out of bounds".into()),
                    "parse_bytecode_token param bounds",
                    format!("{:?}", token_vec)
                )
            );
        }

        let param_bytes = token_vec[offset + 8..offset + total_size].to_vec();
        offset += total_size;

        let parsed = parse_bytecode_param(param_bytes)?;
        parsed_params.push(parsed);
    }

    token.from_params(parsed_params)?;

    Ok(token)
}

use core::str;
use std::{ array::TryFromSliceError, io::{ BufReader, Read } };

use crate::{
    globals::table::{ TOKEN_TABLE, TableQuery, TokenTableMethods },
    tokens::TokenMethods,
    utils::errors::{ AtpError, AtpErrorCode },
};

pub enum AtpParamTypes {
    String(String),
    Usize(usize),
    Token(Box<dyn TokenMethods>),
}

impl AtpParamTypes {
    pub fn from_bytecode(bytecode: Vec<u8>) -> Result<AtpParamTypes, AtpError> {
        let mut reader = BufReader::new(&bytecode[..]);

        let mut param_type_bytes = [0u8; 4];
        reader
            .read_exact(&mut param_type_bytes)
            .map_err(|e|
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                    "read_bytecode_from_file",
                    e.to_string()
                )
            )?;

        let param_type = u32::from_be_bytes(param_type_bytes);

        // Possível remoção no futuro ao substituir por read_to_end
        let mut param_payload_size_bytes = [0u8; 4];
        reader
            .read_exact(&mut param_payload_size_bytes)
            .map_err(|e|
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                    "read_bytecode_from_file",
                    e.to_string()
                )
            )?;

        let param_payload_size = u32::from_be_bytes(param_payload_size_bytes) as usize;

        let mut param_payload_bytes = vec![0u8; param_payload_size];

        reader
            .read_exact(&mut param_payload_bytes)
            .map_err(|e|
                AtpError::new(
                    AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                    "read_bytecode_from_file",
                    e.to_string()
                )
            )?;

        match param_type {
            0x01 => {
                let text = str
                    ::from_utf8(&param_payload_bytes)
                    .map_err(|e|
                        AtpError::new(
                            crate::utils::errors::AtpErrorCode::BytecodeParamParsingError(
                                "Failed Parsing Bytes to UTF8 String".into()
                            ),
                            "AtpParamTypes.from_bytecode()",
                            e.to_string()
                        )
                    )?;
                Ok(AtpParamTypes::String(text.to_string()))
            }
            0x02 => {
                let b: [u8; 8] = param_payload_bytes
                    .as_slice()
                    .try_into()
                    .map_err(|e: TryFromSliceError|
                        AtpError::new(
                            crate::utils::errors::AtpErrorCode::BytecodeParamParsingError(
                                "Failed Parsing Bytes to Usize".into()
                            ),
                            "AtpParamTypes.from_bytecode()",
                            e.to_string()
                        )
                    )?;

                let n = usize::from_be_bytes(b);
                Ok(AtpParamTypes::Usize(n))
            }
            0x03 => {
                let mut reader = BufReader::new(&param_payload_bytes[..]);

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

                let mut param_count_bytes = [0u8; 1];

                reader
                    .read_exact(&mut param_count_bytes)
                    .map_err(|e|
                        AtpError::new(
                            AtpErrorCode::BytecodeParsingError("Failed Reading Bytecode".into()),
                            "read_bytecode_from_file",
                            e.to_string()
                        )
                    )?;

                let param_count = u8::from_be_bytes(param_count_bytes) as usize;

                let mut params: Vec<AtpParamTypes> = Vec::with_capacity(param_count);

                for _ in 0..param_count {
                    let mut param_length_bytes = [0u8; 8];

                    reader
                        .read_exact(&mut param_length_bytes)
                        .map_err(|e|
                            AtpError::new(
                                AtpErrorCode::BytecodeParsingError(
                                    "Failed Reading Bytecode".into()
                                ),
                                "read_bytecode_from_file",
                                e.to_string()
                            )
                        )?;

                    let param_length = usize::from_be_bytes(param_length_bytes);

                    let mut v = vec![0u8; param_length];

                    reader
                        .read_exact(&mut v)
                        .map_err(|e|
                            AtpError::new(
                                AtpErrorCode::BytecodeParsingError(
                                    "Failed Reading Bytecode".into()
                                ),
                                "read_bytecode_from_file",
                                e.to_string()
                            )
                        )?;

                    let parsed_param = AtpParamTypes::from_bytecode(v)?;

                    params.push(parsed_param);
                }

                let token_ref = TOKEN_TABLE.find(
                    TableQuery::Bytecode(instruction_type)
                )?.get_token();

                let mut token = token_ref.into_box();

                token.from_params(&params)?;

                Ok(AtpParamTypes::Token(token))
            }
            _ => {
                Err(
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::BytecodeParamNotRecognized(
                            "Param Bytecode Not Recognized".into()
                        ),
                        "",
                        ""
                    )
                )
            }
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
    pub fn param_to_bytecode(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(&self.get_param_type_code().to_be_bytes());
        let payload = match self {
            AtpParamTypes::String(x) => x.as_bytes().to_vec(),
            AtpParamTypes::Usize(x) => x.to_be_bytes().to_vec(),
            AtpParamTypes::Token(x) => x.to_bytecode(),
        };

        result.extend_from_slice(&(payload.len() as u32).to_be_bytes());

        result.extend_from_slice(&payload);

        result
    }
}

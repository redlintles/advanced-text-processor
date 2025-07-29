use std::path::Path;

use napi_derive::napi;

use crate::{
    builder::atp_processor::{ AtpProcessor, AtpProcessorMethods },
    bytecode_parser::reader::read_bytecode_from_text,
    text_parser::reader::{ read_from_text, read_from_text_vec },
};

#[cfg(feature = "bytecode")]
use crate::builder::atp_processor::AtpProcessorBytecodeMethods;

#[napi]
pub struct ATPProcessorJS {
    inner: AtpProcessor,
}

#[napi]
impl ATPProcessorJS {
    #[napi(constructor)]
    pub fn new() -> Self {
        ATPProcessorJS {
            inner: AtpProcessor::new(),
        }
    }
    #[napi]
    pub fn add_transform(&mut self, tokens: Vec<String>) -> napi::Result<String> {
        let parsed_tokens = read_from_text_vec(tokens)?;
        Ok(self.inner.add_transform(parsed_tokens))
    }

    #[napi]
    pub fn process_all(&mut self, id: String, input: String) -> napi::Result<String> {
        Ok(self.inner.process_all(&id, &input)?)
    }

    #[napi]
    pub fn process_single(&mut self, token: String, input: String) -> napi::Result<String> {
        let parsed_token = read_from_text(&token)?;
        Ok(self.inner.process_single(parsed_token, &input)?)
    }
    pub fn process_single_with_debug(
        &mut self,
        token: String,
        input: String
    ) -> napi::Result<String> {
        let parsed_token = read_from_text(&token)?;
        Ok(self.inner.process_single_with_debug(parsed_token, &input)?)
    }

    #[napi]
    pub fn write_to_text_file(&mut self, id: String, path: String) -> napi::Result<()> {
        Ok(self.inner.write_to_text_file(&id, Path::new(&path))?)
    }

    #[napi]
    pub fn read_from_text_file(&mut self, path: String) -> napi::Result<String> {
        let id = self.inner.read_from_text_file(Path::new(&path))?;

        Ok(id)
    }
}

#[cfg(feature = "bytecode")]
#[napi]
impl ATPProcessorJS {
    #[napi]
    pub fn process_all_with_debug(&mut self, id: String, input: String) -> napi::Result<String> {
        Ok(self.inner.process_all_with_debug(&id, &input)?)
    }

    #[napi]
    pub fn write_to_bytecode_file(&mut self, id: String, path: String) -> napi::Result<()> {
        Ok(self.inner.write_to_bytecode_file(&id, Path::new(&path))?)
    }
    #[napi]
    pub fn read_from_bytecode_file(&mut self, path: String) -> napi::Result<String> {
        Ok(self.inner.read_from_bytecode_file(Path::new(&path))?)
    }
    #[napi]
    pub fn process_all_bytecode_with_debug(
        &mut self,
        id: String,
        input: String
    ) -> napi::Result<String> {
        Ok(self.inner.process_all_bytecode_with_debug(&id, &input)?)
    }
    #[napi]
    pub fn process_single_bytecode_with_debug(
        &mut self,
        token: String,
        input: String
    ) -> napi::Result<String> {
        let parsed_token = read_bytecode_from_text(&token)?;
        Ok(self.inner.process_single_bytecode_with_debug(parsed_token, &input)?)
    }
}

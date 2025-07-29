use napi_derive::napi;

use crate::{
    builder::{ atp_builder::AtpBuilder, atp_processor::AtpProcessorMethods },
    ffi::javascript::main::processor::ATPProcessorJS,
};

#[napi]
pub struct AtpBuilderJS {
    inner: AtpBuilder,
}

#[napi]
impl AtpBuilderJS {
    #[napi(constructor)]
    pub fn new() -> Self {
        AtpBuilderJS {
            inner: AtpBuilder::new(),
        }
    }
    #[napi]
    pub fn build(&self) -> napi::Result<(ATPProcessorJS, String)> {
        let (p, i) = self.inner.build();
        let mut processor = ATPProcessorJS::new();

        let text_vec = p.get_text_transform_vec(&i)?;

        let id = processor.add_transform(text_vec)?;

        Ok((processor, id))
    }
}

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

#[napi]
impl AtpBuilderJS {
    #[napi]
    pub fn trim_both(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().trim_both(),
        }
    }
    #[napi]
    pub fn trim_left(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().trim_left(),
        }
    }
    #[napi]
    pub fn trim_right(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().trim_right(),
        }
    }
    #[napi]
    pub fn delete_first(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().delete_first(),
        }
    }
    #[napi]
    pub fn delete_last(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().delete_last(),
        }
    }
    #[napi]
    pub fn split_characters(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().split_characters(),
        }
    }
    #[napi]
    pub fn to_uppercase_all(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().to_uppercase_all(),
        }
    }
    #[napi]
    pub fn to_lowercase_all(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().to_lowercase_all(),
        }
    }
    #[napi]
    pub fn to_url_encoded(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().to_url_encoded(),
        }
    }
    #[napi]
    pub fn to_url_decoded(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().to_url_decoded(),
        }
    }
    #[napi]
    pub fn join_to_camel_case(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().join_to_camel_case(),
        }
    }
    #[napi]
    pub fn join_to_pascal_case(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().join_to_pascal_case(),
        }
    }
    #[napi]
    pub fn join_to_kebab_case(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().join_to_kebab_case(),
        }
    }
    #[napi]
    pub fn join_to_snake_case(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().join_to_snake_case(),
        }
    }
}

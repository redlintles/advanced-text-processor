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
    #[napi]
    pub fn capitalize_first_word(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().capitalize_first_word(),
        }
    }
    #[napi]
    pub fn capitalize_last_word(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().capitalize_last_word(),
        }
    }
    #[napi]
    pub fn to_html_escaped(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().to_html_escaped(),
        }
    }
    #[napi]
    pub fn to_html_unescaped(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().to_html_unescaped(),
        }
    }
    #[napi]
    pub fn to_reverse(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().to_reverse(),
        }
    }
    #[napi]
    pub fn remove_whitespace(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().remove_whitespace(),
        }
    }
    #[napi]
    pub fn to_json_escaped(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().to_json_escaped(),
        }
    }
    #[napi]
    pub fn to_json_unescaped(&mut self) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().to_json_unescaped(),
        }
    }
    #[napi]
    pub fn add_to_beginning(&mut self, text: String) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().add_to_beginning(&text),
        }
    }
    #[napi]
    pub fn add_to_end(&mut self, text: String) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().add_to_end(&text),
        }
    }
    #[napi]
    pub fn capitalize_chunk(&mut self, start_index: u32, end_index: u32) -> napi::Result<Self> {
        Ok(AtpBuilderJS {
            inner: self.inner.clone().capitalize_chunk(start_index as usize, end_index as usize)?,
        })
    }
    #[napi]
    pub fn capitalize_range(&mut self, start_index: u32, end_index: u32) -> napi::Result<Self> {
        Ok(AtpBuilderJS {
            inner: self.inner.clone().capitalize_range(start_index as usize, end_index as usize)?,
        })
    }
    #[napi]
    pub fn capitalize_single_word(&mut self, index: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().capitalize_single_word(index as usize),
        }
    }
    #[napi]
    pub fn delete_after(&mut self, index: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().delete_after(index as usize),
        }
    }
    #[napi]
    pub fn delete_before(&mut self, index: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().delete_before(index as usize),
        }
    }
    #[napi]
    pub fn delete_single(&mut self, index: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().delete_single(index as usize),
        }
    }
    #[napi]
    pub fn to_lowercase_word(&mut self, index: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().to_lowercase_word(index as usize),
        }
    }
    #[napi]
    pub fn to_uppercase_word(&mut self, index: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().to_uppercase_word(index as usize),
        }
    }
    #[napi]
    pub fn to_uppercase_single(&mut self, index: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().to_uppercase_single(index as usize),
        }
    }
    #[napi]
    pub fn to_lowercase_single(&mut self, index: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().to_lowercase_single(index as usize),
        }
    }
    #[napi]
    pub fn insert(&mut self, index: u32, text: String) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().insert(index as usize, &text),
        }
    }
    #[napi]
    pub fn replace_all_with(&mut self, pattern: String, text: String) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().replace_all_with(&pattern, &text),
        }
    }
    #[napi]
    pub fn replace_first_with(&mut self, pattern: String, text: String) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().replace_first_with(&pattern, &text),
        }
    }
    #[napi]
    pub fn replace_last_with(&mut self, pattern: String, text: String) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().replace_last_with(&pattern, &text),
        }
    }
    #[napi]
    pub fn replace_count_with(&mut self, pattern: String, text: String, times: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().replace_count_with(&pattern, &text, times as usize),
        }
    }
    #[napi]
    pub fn replace_nth_with(&mut self, pattern: String, text: String, n: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().replace_nth_with(&pattern, &text, n as usize),
        }
    }
    #[napi]
    pub fn delete_chunk(&mut self, start_index: u32, end_index: u32) -> napi::Result<Self> {
        Ok(AtpBuilderJS {
            inner: self.inner.clone().delete_chunk(start_index as usize, end_index as usize)?,
        })
    }
    #[napi]
    pub fn to_lowercase_chunk(&mut self, start_index: u32, end_index: u32) -> napi::Result<Self> {
        Ok(AtpBuilderJS {
            inner: self.inner.clone().to_lowercase_chunk(start_index as usize, end_index as usize)?,
        })
    }
    #[napi]
    pub fn to_uppercase_chunk(&mut self, start_index: u32, end_index: u32) -> napi::Result<Self> {
        Ok(AtpBuilderJS {
            inner: self.inner.clone().to_uppercase_chunk(start_index as usize, end_index as usize)?,
        })
    }
    #[napi]
    pub fn select(&mut self, start_index: u32, end_index: u32) -> napi::Result<Self> {
        Ok(AtpBuilderJS {
            inner: self.inner.clone().select(start_index as usize, end_index as usize)?,
        })
    }
    #[napi]
    pub fn split_select(&mut self, pattern: String, index: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().split_select(&pattern, index as usize),
        }
    }
    #[napi]
    pub fn repeat(&mut self, times: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().repeat(times as usize),
        }
    }
    #[napi]
    pub fn rotate_left(&mut self, times: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().rotate_left(times as usize),
        }
    }
    #[napi]
    pub fn rotate_right(&mut self, times: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().rotate_right(times as usize),
        }
    }
    #[napi]
    pub fn pad_left(&mut self, text: String, times: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().pad_left(&text, times as usize),
        }
    }
    #[napi]
    pub fn pad_right(&mut self, text: String, times: u32) -> Self {
        AtpBuilderJS {
            inner: self.inner.clone().pad_right(&text, times as usize),
        }
    }
}

#[cfg(test)]
mod ffi_builder_tests {
    use crate::{ ffi::javascript::main::builder::{ AtpBuilderJS } };

    #[test]
    fn simple_test() {
        let (mut p, id) = AtpBuilderJS::new()
            .add_to_beginning("banana".to_string())
            .add_to_end("laranja".to_string())
            .build()
            .unwrap();

        let r = p.process_all(id, "coxinha".to_string()).unwrap();

        assert_eq!(r, "bananacoxinhalaranja".to_string());
    }
}

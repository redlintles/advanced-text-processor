use crate::token_data::{ TokenMethods };

use crate::token_data::token_defs::*;
use crate::utils::errors::AtpError;

use super::atp_processor::{ AtpProcessorMethods, AtpProcessor };
#[cfg(feature = "debug")]
use super::atp_processor::AtpProcessorDebugMethods;
#[cfg(feature = "bytecode")]
use super::atp_processor::AtpProcessorBytecodeMethods;
#[cfg(feature = "bytecode_debug")]
use super::atp_processor::AtpProcessorBytecodeDebugMethods;
#[derive(Default)]
pub struct AtpBuilder {
    tokens: Vec<Box<dyn TokenMethods>>,
}
#[derive(Default)]
pub struct AtpProcessorBuilder {
    tokens: Vec<Box<dyn TokenMethods>>,
}

impl AtpProcessorBuilder {
    pub fn text_processor(self) -> (Box<dyn AtpProcessorMethods>, String) {
        let mut processor: Box<dyn AtpProcessorMethods> = Box::new(AtpProcessor::new());
        let identifier = processor.add_transform(self.tokens);
        (processor, identifier)
    }
    #[cfg(feature = "debug")]
    pub fn text_debug_processor(self) -> (Box<dyn AtpProcessorDebugMethods>, String) {
        let mut processor: Box<dyn AtpProcessorDebugMethods> = Box::new(AtpProcessor::new());
        let identifier = processor.add_transform(self.tokens);
        (processor, identifier)
    }
    #[cfg(feature = "bytecode")]
    pub fn bytecode_processor(self) -> (Box<dyn AtpProcessorBytecodeMethods>, String) {
        let mut processor: Box<dyn AtpProcessorBytecodeMethods> = Box::new(AtpProcessor::new());
        let identifier = processor.add_transform(self.tokens);
        (processor, identifier)
    }
    #[cfg(feature = "bytecode_debug")]
    pub fn bytecode_debug_processor(self) -> (Box<dyn AtpProcessorBytecodeDebugMethods>, String) {
        let mut processor: Box<dyn AtpProcessorBytecodeDebugMethods> = Box::new(
            AtpProcessor::new()
        );
        let identifier = processor.add_transform(self.tokens);
        (processor, identifier)
    }
}

impl AtpBuilder {
    pub fn new() -> AtpBuilder {
        AtpBuilder {
            tokens: Vec::new(),
        }
    }

    pub fn build(self) -> AtpProcessorBuilder {
        AtpProcessorBuilder {
            tokens: self.tokens,
        }
    }
}

impl AtpBuilder {
    pub fn trim_both(mut self) -> Self {
        self.tokens.push(Box::new(tbs::Tbs::default()));
        self
    }

    pub fn trim_left(mut self) -> Self {
        self.tokens.push(Box::new(tls::Tls::default()));
        self
    }
    pub fn trim_right(mut self) -> Self {
        self.tokens.push(Box::new(trs::Trs::default()));
        self
    }
    pub fn add_to_end(mut self, text: &str) -> Self {
        self.tokens.push(Box::new(ate::Ate::params(text)));
        self
    }
    pub fn add_to_beginning(mut self, text: &str) -> Self {
        self.tokens.push(Box::new(atb::Atb::params(text)));
        self
    }
    pub fn delete_first(mut self) -> Self {
        self.tokens.push(Box::new(dlf::Dlf::default()));
        self
    }
    pub fn delete_last(mut self) -> Self {
        self.tokens.push(Box::new(dll::Dll::default()));
        self
    }
    pub fn delete_after(mut self, index: usize) -> Self {
        self.tokens.push(Box::new(dla::Dla::params(index)));
        self
    }
    pub fn delete_before(mut self, index: usize) -> Self {
        self.tokens.push(Box::new(dlb::Dlb::params(index)));
        self
    }
    pub fn delete_chunk(mut self, start_index: usize, end_index: usize) -> Self {
        self.tokens.push(Box::new(dlc::Dlc::params(start_index, end_index)));
        self
    }
    pub fn replace_all_with(mut self, pattern: &str, text_to_replace: &str) -> Self {
        self.tokens.push(
            Box::new(match raw::Raw::params(pattern.to_string(), text_to_replace.to_string()) {
                Ok(x) => x,
                Err(e) => panic!("{}", e),
            })
        );

        self
    }
    pub fn replace_first_with(mut self, pattern: &str, text_to_replace: &str) -> Self {
        self.tokens.push(
            Box::new(match rfw::Rfw::params(pattern.to_string(), text_to_replace.to_string()) {
                Ok(x) => x,
                Err(e) => panic!("{}", e),
            })
        );
        self
    }
    pub fn replace_last_with(mut self, pattern: &str, text_to_replace: &str) -> Self {
        self.tokens.push(
            Box::new(match rlw::Rlw::params(pattern.to_string(), text_to_replace.to_string()) {
                Ok(x) => x,
                Err(e) => panic!("{}", e),
            })
        );
        self
    }
    pub fn replace_nth_with(mut self, pattern: &str, text_to_replace: &str, index: usize) -> Self {
        self.tokens.push(
            Box::new(match
                rnw::Rnw::params(pattern.to_string(), text_to_replace.to_string(), index)
            {
                Ok(x) => x,
                Err(e) => panic!("{}", e),
            })
        );
        self
    }

    pub fn replace_count_with(
        mut self,
        pattern: &str,
        text_to_replace: &str,
        count: usize
    ) -> Self {
        self.tokens.push(
            Box::new(match
                rcw::Rcw::params(pattern.to_string(), text_to_replace.to_string(), count)
            {
                Ok(x) => x,
                Err(e) => panic!("{}", e),
            })
        );

        self
    }
    pub fn rotate_left(mut self, times: usize) -> Self {
        self.tokens.push(Box::new(rtl::Rtl::params(times)));
        self
    }
    pub fn rotate_right(mut self, times: usize) -> Self {
        self.tokens.push(Box::new(rtr::Rtr::params(times)));
        self
    }
    pub fn repeat(mut self, times: usize) -> Self {
        self.tokens.push(Box::new(rpt::Rpt::params(times)));
        self
    }

    pub fn select(mut self, start_index: usize, end_index: usize) -> Self {
        self.tokens.push(Box::new(slt::Slt::params(start_index, end_index)));
        self
    }
    pub fn to_uppercase_all(mut self) -> Self {
        self.tokens.push(Box::new(tua::Tua::default()));
        self
    }
    pub fn to_lowercase_all(mut self) -> Self {
        self.tokens.push(Box::new(tla::Tla::default()));
        self
    }
    pub fn to_uppercase_single(mut self, index: usize) -> Self {
        self.tokens.push(Box::new(tucs::Tucs::params(index)));
        self
    }
    pub fn to_lowercase_single(mut self, index: usize) -> Self {
        self.tokens.push(Box::new(tlcs::Tlcs::params(index)));
        self
    }
    pub fn to_uppercase_chunk(mut self, start_index: usize, end_index: usize) -> Self {
        self.tokens.push(Box::new(tucc::Tucc::params(start_index, end_index)));
        self
    }
    pub fn to_lowercase_chunk(mut self, start_index: usize, end_index: usize) -> Self {
        self.tokens.push(Box::new(tlcc::Tlcc::params(start_index, end_index)));
        self
    }

    pub fn capitalize_first_word(mut self) -> Self {
        self.tokens.push(Box::new(cfw::Cfw::default()));
        self
    }
    pub fn capitalize_last_word(mut self) -> Self {
        self.tokens.push(Box::new(cfw::Cfw::default()));
        self
    }
    pub fn split_select(mut self, pattern: &str, index: usize) -> Self {
        self.tokens.push(
            Box::new(match sslt::Sslt::params(pattern, index) {
                Ok(x) => x,
                Err(e) => panic!("{}", e),
            })
        );
        self
    }
    pub fn capitalize_chunk(
        mut self,
        start_index: usize,
        end_index: usize
    ) -> Result<Self, AtpError> {
        self.tokens.push(Box::new(ctc::Ctc::params(start_index, end_index)?));
        Ok(self)
    }
    pub fn capitalize_range(mut self, start_index: usize, end_index: usize) -> Self {
        self.tokens.push(Box::new(ctr::Ctr::params(start_index, end_index)));
        self
    }
    pub fn capitalize_single_word(mut self, index: usize) -> Self {
        self.tokens.push(Box::new(cts::Cts::params(index)));
        self
    }

    pub fn to_url_encoded(mut self) -> Self {
        self.tokens.push(Box::new(urle::Urle::default()));
        self
    }
    pub fn to_url_decoded(mut self) -> Self {
        self.tokens.push(Box::new(urld::Urld::default()));
        self
    }
    pub fn to_reverse(mut self) -> Self {
        self.tokens.push(Box::new(rev::Rev::default()));
        self
    }
    pub fn split_characters(mut self) -> Self {
        self.tokens.push(Box::new(splc::Splc::default()));
        self
    }

    pub fn to_html_escaped(mut self) -> Self {
        self.tokens.push(Box::new(htmle::Htmle::default()));
        self
    }
    pub fn to_html_unescaped(mut self) -> Self {
        self.tokens.push(Box::new(htmlu::Htmlu::default()));
        self
    }

    pub fn to_json_escaped(mut self) -> Self {
        self.tokens.push(Box::new(jsone::Jsone::default()));
        self
    }
    pub fn to_json_unescaped(mut self) -> Self {
        self.tokens.push(Box::new(jsonu::Jsonu::default()));
        self
    }

    pub fn insert(mut self, index: usize, text_to_insert: &str) -> Self {
        self.tokens.push(Box::new(ins::Ins::params(index, text_to_insert)));
        self
    }

    pub fn to_lowercase_word(mut self, index: usize) -> Self {
        self.tokens.push(Box::new(tlcw::Tlcw::params(index)));
        self
    }
    pub fn to_uppercase_word(mut self, index: usize) -> Self {
        self.tokens.push(Box::new(tucw::Tucw::params(index)));
        self
    }

    pub fn join_to_kebab_case(mut self) -> Self {
        self.tokens.push(Box::new(jkbc::Jkbc::default()));
        self
    }
    pub fn join_to_snake_case(mut self) -> Self {
        self.tokens.push(Box::new(jsnc::Jsnc::default()));
        self
    }
    pub fn join_to_camel_case(mut self) -> Self {
        self.tokens.push(Box::new(jcmc::Jcmc::default()));
        self
    }
    pub fn join_to_pascal_case(mut self) -> Self {
        self.tokens.push(Box::new(jpsc::Jpsc::default()));
        self
    }
    pub fn pad_left(mut self, text: &str, times: usize) -> Self {
        self.tokens.push(Box::new(padl::Padl::params(text, times)));
        self
    }
    pub fn pad_right(mut self, text: &str, times: usize) -> Self {
        self.tokens.push(Box::new(padr::Padr::params(text, times)));
        self
    }
    pub fn remove_whitespace(mut self) -> Self {
        self.tokens.push(Box::new(rmws::Rmws::default()));
        self
    }

    pub fn delete_single(mut self, index: usize) -> Self {
        self.tokens.push(Box::new(dls::Dls::params(index)));
        self
    }
}

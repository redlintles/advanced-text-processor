use crate::{ tokens::{ transforms::*, TokenMethods } };

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::BytecodeTokenMethods;

use std::collections::HashMap;

macro_rules! token_map_string_token_methods {
    ($($key:literal => $val:ty),* $(,)?) => {
        {
        let mut map: HashMap<String, fn() -> Box<dyn TokenMethods>> = HashMap::new();
        $(
            map.insert($key.to_string(), || Box::new(<$val as Default>::default()));
        )*
        map
        }
    };
}

#[cfg(feature = "bytecode")]
macro_rules! token_map_string_bytecode_methods {
    ($($key:literal => $val:ty),* $(,)?) => {
        {
        let mut map: HashMap<String, fn() -> Box<dyn BytecodeTokenMethods>> = HashMap::new();
        $(
            map.insert($key.to_string(), || Box::new(<$val as Default>::default()));
        )*
        map
        }
    };
}
#[cfg(feature = "bytecode")]
macro_rules! token_map_bytecode_token {
    ($($key:literal => $val:ty),* $(,)?) => {
        {
        let mut map: HashMap<u8, fn() -> Box<dyn BytecodeTokenMethods>> = HashMap::new();
        $(
            map.insert($key, || Box::new(<$val as Default>::default()));
        )*
        map
        }
    };
}

// Aqui, não usamos colchetes ou `[]`, apenas expandimos tokens
macro_rules! for_each_token_entry {
    ($macro:ident) => {
        $macro! {
            "atb" => atb::Atb,
            "ate" => ate::Ate,
            "dlc" => dlc::Dlc,
            "dlf" => dlf::Dlf,
            "dll" => dll::Dll,
            "dla" => dla::Dla,
            "dlb" => dlb::Dlb,
            "rfw" => rfw::Rfw,
            "rcw" => rcw::Rcw,
            "raw" => raw::Raw,
            "tbs" => tbs::Tbs,
            "tls" => tls::Tls,
            "trs" => trs::Trs,
            "rpt" => rpt::Rpt,
            "rtr" => rtr::Rtr,
            "rtl" => rtl::Rtl,
            "slt" => slt::Slt,
            "tua" => tua::Tua,
            "tla" => tla::Tla,
            "tucs" => tucs::Tucs,
            "tlcs" => tlcs::Tlcs,
            "tucc" => tucc::Tucc,
            "tlcc" => tlcc::Tlcc,
            "cfw" => cfw::Cfw,
            "clw" => clw::Clw,
            "sslt" => sslt::Sslt,
            "ctc" => ctc::Ctc,
            "ctr" => ctr::Ctr,
            "cts" => cts::Cts,
            "rlw" => rlw::Rlw,
            "rnw" => rnw::Rnw,
            "urle" => urle::Urle,
            "urld" => urld::Urld,
            "rev" => rev::Rev,
            "splc" => splc::Splc,
            "htmle" => htmle::Htmle,            
            "htmlu" => htmlu::Htmlu,            
            "jsone" => jsone::Jsone,
            "jsonu" => jsonu::Jsonu,
            "ins" => ins::Ins,
            "tlcw" => tlcw::Tlcw,
            "tucw" => tucw::Tucw,
            "jkbc" => jkbc::Jkbc,
            "jcmc" => jcmc::Jcmc,
            "jpsc" => jpsc::Jpsc,
            "padl" => padl::Padl,
            "padr" => padr::Padr,
            "rmws" => rmws::Rmws,
            "dls" => dls::Dls,
        }
    };
}
#[cfg(feature = "bytecode")]
macro_rules! for_each_bytecode_entry {
    ($macro:ident) => {
        $macro! {
            0x01 => atb::Atb,
            0x02 => ate::Ate,
            0x03 => dlf::Dlf,
            0x04 => dll::Dll,
            0x05 => tbs::Tbs,
            0x06 => tls::Tls,
            0x07 => trs::Trs,
            0x08 => dlc::Dlc,
            0x09 => dla::Dla,
            0x0a => dlb::Dlb,
            0x0b => raw::Raw,
            0x0c => rfw::Rfw,
            0x0d => rpt::Rpt,
            0x0e => rtl::Rtl,
            0x0f => rtr::Rtr,
            0x10 => rcw::Rcw,
            0x11 => slt::Slt,
            0x12 => tua::Tua,
            0x13 => tla::Tla,
            0x14 => tucs::Tucs,
            0x15 => tlcs::Tlcs,
            0x16 => tucc::Tucc,
            0x17 => tlcc::Tlcc,
            0x18 => cfw::Cfw,
            0x19 => clw::Clw,
            0x1a => sslt::Sslt,
            0x1b => ctc::Ctc,
            0x1c => ctr::Ctr,
            0x1d => cts::Cts,
            0x1e => rlw::Rlw,
            0x1f => rnw::Rnw,
            0x20 => urle::Urle,
            0x21 => urld::Urld,
            0x22 => rev::Rev,
            0x23 => splc::Splc,
            0x24 => htmle::Htmle,
            0x25 => htmlu::Htmlu,
            0x26 => jsone::Jsone,
            0x27 => jsonu::Jsonu,
            0x28 => ins::Ins,
            0x29 => tlcw::Tlcw,
            0x2a => tucw::Tucw,
            0x2b => jkbc::Jkbc,
            0x2c => jsnc::Jsnc,
            0x2d => jcmc::Jcmc,
            0x2e => jpsc::Jpsc,
            0x2f => padl::Padl,
            0x30 => padr::Padr,
            0x31 => rmws::Rmws,
            0x32 => dls::Dls,
        }
    };
}

pub fn get_supported_default_tokens() -> HashMap<String, fn() -> Box<dyn TokenMethods>> {
    for_each_token_entry!(token_map_string_token_methods)
}

#[cfg(feature = "bytecode")]
pub fn get_supported_bytecode_tokens() -> HashMap<String, fn() -> Box<dyn BytecodeTokenMethods>> {
    for_each_token_entry!(token_map_string_bytecode_methods)
}
#[cfg(feature = "bytecode")]
pub fn get_mapping_bytecode_to_token() -> HashMap<u8, fn() -> Box<dyn BytecodeTokenMethods>> {
    for_each_bytecode_entry!(token_map_bytecode_token)
}

use std::{ borrow::Cow, sync::{ Arc, LazyLock } };

use crate::{
    tokens::{ TokenMethods, instructions::*, transforms::* },
    utils::errors::{ AtpError, AtpErrorCode },
};
#[derive(Clone)]
pub enum TokenRef {
    Boxed(Box<dyn TokenMethods>),
    Shared(Arc<dyn TokenMethods>),
}

impl TokenRef {
    fn clone_ref(&self) -> TokenRef {
        match self {
            TokenRef::Boxed(b) => TokenRef::Boxed(b.clone()),
            TokenRef::Shared(a) => TokenRef::Shared(a.clone()),
        }
    }
    fn as_ref(&self) -> &dyn TokenMethods {
        match self {
            TokenRef::Boxed(b) => b.as_ref(),
            TokenRef::Shared(a) => a.as_ref(),
        }
    }
    pub fn into_box(self) -> Box<dyn TokenMethods> {
        match self {
            TokenRef::Boxed(b) => b,
            TokenRef::Shared(a) => a.clone_box(),
        }
    }
}

#[derive(Clone)]
pub struct TokenEntry {
    identifier: Cow<'static, str>,
    code: u32,
    token: TokenRef,
}
#[derive(Clone)]
pub enum TableQuery {
    String(Cow<'static, str>),
    Bytecode(u32),
}

impl TokenEntry {
    fn new(identifier: Cow<'static, str>, code: u32, token: TokenRef) -> TokenEntry {
        TokenEntry { identifier, code, token }
    }
}

impl TokenEntry {
    pub fn get_bytecode(&self) -> u32 {
        self.code
    }
    pub fn get_identifier(&self) -> &str {
        &self.identifier.as_ref()
    }
    pub fn get_token(&self) -> TokenRef {
        self.token.clone_ref()
    }
}

pub trait TokenTableMethods {
    fn find(&self, query: TableQuery) -> Result<&TokenEntry, AtpError>;
}

impl TokenTableMethods for Vec<TokenEntry> {
    fn find(&self, query: TableQuery) -> Result<&TokenEntry, AtpError> {
        for entry in self.iter() {
            match query {
                TableQuery::String(ref x) => {
                    if x.as_ref() == entry.get_identifier() {
                        return Ok(entry);
                    }
                }
                TableQuery::Bytecode(x) => {
                    if x == entry.get_bytecode() {
                        return Ok(entry);
                    }
                }
            }
        }
        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Token Not Found in mapping".into()),
                "TokenTable.find()",
                "query"
            )
        )
    }
}

pub static TOKEN_TABLE: LazyLock<Vec<TokenEntry>> = LazyLock::new(|| {
    // Lista de tokens e bytecodes correspondentes, igual ao legado
    let entries: &[(&str, u32, fn() -> TokenRef)] = &[
        ("atb", 0x01, || TokenRef::Shared(Arc::new(atb::Atb::default()))),
        ("ate", 0x02, || TokenRef::Shared(Arc::new(ate::Ate::default()))),
        ("dlc", 0x08, || TokenRef::Shared(Arc::new(dlc::Dlc::default()))),
        ("dlf", 0x03, || TokenRef::Shared(Arc::new(dlf::Dlf::default()))),
        ("dll", 0x04, || TokenRef::Shared(Arc::new(dll::Dll::default()))),
        ("dla", 0x09, || TokenRef::Shared(Arc::new(dla::Dla::default()))),
        ("dlb", 0x0a, || TokenRef::Shared(Arc::new(dlb::Dlb::default()))),
        ("rfw", 0x0b, || TokenRef::Shared(Arc::new(rfw::Rfw::default()))),
        ("rcw", 0x10, || TokenRef::Shared(Arc::new(rcw::Rcw::default()))),
        ("raw", 0x0c, || TokenRef::Shared(Arc::new(raw::Raw::default()))),
        ("tbs", 0x05, || TokenRef::Shared(Arc::new(tbs::Tbs::default()))),
        ("tls", 0x06, || TokenRef::Shared(Arc::new(tls::Tls::default()))),
        ("trs", 0x07, || TokenRef::Shared(Arc::new(trs::Trs::default()))),
        ("rpt", 0x0d, || TokenRef::Shared(Arc::new(rpt::Rpt::default()))),
        ("rtr", 0x0f, || TokenRef::Shared(Arc::new(rtr::Rtr::default()))),
        ("rtl", 0x0e, || TokenRef::Shared(Arc::new(rtl::Rtl::default()))),
        ("slt", 0x11, || TokenRef::Shared(Arc::new(slt::Slt::default()))),
        ("tua", 0x12, || TokenRef::Shared(Arc::new(tua::Tua::default()))),
        ("tla", 0x13, || TokenRef::Shared(Arc::new(tla::Tla::default()))),
        ("tucs", 0x14, || TokenRef::Shared(Arc::new(tucs::Tucs::default()))),
        ("tlcs", 0x15, || TokenRef::Shared(Arc::new(tlcs::Tlcs::default()))),
        ("tucc", 0x16, || TokenRef::Shared(Arc::new(tucc::Tucc::default()))),
        ("tlcc", 0x17, || TokenRef::Shared(Arc::new(tlcc::Tlcc::default()))),
        ("cfw", 0x18, || TokenRef::Shared(Arc::new(cfw::Cfw::default()))),
        ("clw", 0x19, || TokenRef::Shared(Arc::new(clw::Clw::default()))),
        ("sslt", 0x1a, || TokenRef::Shared(Arc::new(sslt::Sslt::default()))),
        ("ctc", 0x1b, || TokenRef::Shared(Arc::new(ctc::Ctc::default()))),
        ("ctr", 0x1c, || TokenRef::Shared(Arc::new(ctr::Ctr::default()))),
        ("cts", 0x1d, || TokenRef::Shared(Arc::new(cts::Cts::default()))),
        ("rlw", 0x1e, || TokenRef::Shared(Arc::new(rlw::Rlw::default()))),
        ("rnw", 0x1f, || TokenRef::Shared(Arc::new(rnw::Rnw::default()))),
        ("urle", 0x20, || TokenRef::Shared(Arc::new(urle::Urle::default()))),
        ("urld", 0x21, || TokenRef::Shared(Arc::new(urld::Urld::default()))),
        ("rev", 0x22, || TokenRef::Shared(Arc::new(rev::Rev::default()))),
        ("splc", 0x23, || TokenRef::Shared(Arc::new(splc::Splc::default()))),
        ("htmle", 0x24, || TokenRef::Shared(Arc::new(htmle::Htmle::default()))),
        ("htmlu", 0x25, || TokenRef::Shared(Arc::new(htmlu::Htmlu::default()))),
        ("jsone", 0x26, || TokenRef::Shared(Arc::new(jsone::Jsone::default()))),
        ("jsonu", 0x27, || TokenRef::Shared(Arc::new(jsonu::Jsonu::default()))),
        ("ins", 0x28, || TokenRef::Shared(Arc::new(ins::Ins::default()))),
        ("tlcw", 0x29, || TokenRef::Shared(Arc::new(tlcw::Tlcw::default()))),
        ("tucw", 0x2a, || TokenRef::Shared(Arc::new(tucw::Tucw::default()))),
        ("jkbc", 0x2b, || TokenRef::Shared(Arc::new(jkbc::Jkbc::default()))),
        ("jcmc", 0x2d, || TokenRef::Shared(Arc::new(jcmc::Jcmc::default()))),
        ("jpsc", 0x2e, || TokenRef::Shared(Arc::new(jpsc::Jpsc::default()))),
        ("padl", 0x2f, || TokenRef::Shared(Arc::new(padl::Padl::default()))),
        ("padr", 0x30, || TokenRef::Shared(Arc::new(padr::Padr::default()))),
        ("rmws", 0x31, || TokenRef::Shared(Arc::new(rmws::Rmws::default()))),
        ("dls", 0x32, || TokenRef::Shared(Arc::new(dls::Dls::default()))),
        ("ifdc", 0x33, || TokenRef::Shared(Arc::new(ifdc::Ifdc::default()))),
    ];

    entries
        .iter()
        .map(|(id, code, f)| TokenEntry::new(Cow::Borrowed(id), *code, f()))
        .collect()
});

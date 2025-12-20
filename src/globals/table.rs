use std::sync::{ Arc, LazyLock };

use crate::{
    tokens::{ TokenMethods, transforms::*, instructions::* },
    utils::errors::{ AtpError, AtpErrorCode },
};

#[derive(Clone)]
struct TokenEntry {
    identifier: &'static str,
    code: u32,
    token: Arc<dyn TokenMethods>,
}

enum TableQuery {
    String(&'static str),
    Bytecode(u32),
}

impl TokenEntry {
    fn new(identifier: &'static str, code: u32, token: Arc<dyn TokenMethods>) -> TokenEntry {
        TokenEntry { identifier, code, token }
    }
}

impl TokenEntry {
    fn get_bytecode(&self) -> u32 {
        self.code
    }
    fn get_identifier(&self) -> &'static str {
        self.identifier
    }
    fn get_token(&self) -> Arc<dyn TokenMethods> {
        self.token.clone()
    }
}

trait TokenTableMethods {
    fn add_entry(
        &mut self,
        identifier: &'static str,
        code: u32,
        token: Arc<dyn TokenMethods>
    ) -> ();
    fn find(&self, query: TableQuery) -> Result<&TokenEntry, AtpError>;
}

impl TokenTableMethods for Vec<TokenEntry> {
    fn add_entry(
        &mut self,
        identifier: &'static str,
        code: u32,
        token: Arc<dyn TokenMethods>
    ) -> () {
        self.push(TokenEntry::new(identifier, code, token));
        ()
    }
    fn find(&self, query: TableQuery) -> Result<&TokenEntry, AtpError> {
        for entry in self.iter() {
            match query {
                TableQuery::String(x) => {
                    if x == entry.get_identifier() {
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
    let entries: &[(&str, u32, fn() -> Arc<dyn TokenMethods>)] = &[
        ("atb", 0x01, || Arc::new(atb::Atb::default())),
        ("ate", 0x02, || Arc::new(ate::Ate::default())),
        ("dlc", 0x08, || Arc::new(dlc::Dlc::default())),
        ("dlf", 0x03, || Arc::new(dlf::Dlf::default())),
        ("dll", 0x04, || Arc::new(dll::Dll::default())),
        ("dla", 0x09, || Arc::new(dla::Dla::default())),
        ("dlb", 0x0a, || Arc::new(dlb::Dlb::default())),
        ("rfw", 0x0b, || Arc::new(rfw::Rfw::default())),
        ("rcw", 0x10, || Arc::new(rcw::Rcw::default())),
        ("raw", 0x0c, || Arc::new(raw::Raw::default())),
        ("tbs", 0x05, || Arc::new(tbs::Tbs::default())),
        ("tls", 0x06, || Arc::new(tls::Tls::default())),
        ("trs", 0x07, || Arc::new(trs::Trs::default())),
        ("rpt", 0x0d, || Arc::new(rpt::Rpt::default())),
        ("rtr", 0x0f, || Arc::new(rtr::Rtr::default())),
        ("rtl", 0x0e, || Arc::new(rtl::Rtl::default())),
        ("slt", 0x11, || Arc::new(slt::Slt::default())),
        ("tua", 0x12, || Arc::new(tua::Tua::default())),
        ("tla", 0x13, || Arc::new(tla::Tla::default())),
        ("tucs", 0x14, || Arc::new(tucs::Tucs::default())),
        ("tlcs", 0x15, || Arc::new(tlcs::Tlcs::default())),
        ("tucc", 0x16, || Arc::new(tucc::Tucc::default())),
        ("tlcc", 0x17, || Arc::new(tlcc::Tlcc::default())),
        ("cfw", 0x18, || Arc::new(cfw::Cfw::default())),
        ("clw", 0x19, || Arc::new(clw::Clw::default())),
        ("sslt", 0x1a, || Arc::new(sslt::Sslt::default())),
        ("ctc", 0x1b, || Arc::new(ctc::Ctc::default())),
        ("ctr", 0x1c, || Arc::new(ctr::Ctr::default())),
        ("cts", 0x1d, || Arc::new(cts::Cts::default())),
        ("rlw", 0x1e, || Arc::new(rlw::Rlw::default())),
        ("rnw", 0x1f, || Arc::new(rnw::Rnw::default())),
        ("urle", 0x20, || Arc::new(urle::Urle::default())),
        ("urld", 0x21, || Arc::new(urld::Urld::default())),
        ("rev", 0x22, || Arc::new(rev::Rev::default())),
        ("splc", 0x23, || Arc::new(splc::Splc::default())),
        ("htmle", 0x24, || Arc::new(htmle::Htmle::default())),
        ("htmlu", 0x25, || Arc::new(htmlu::Htmlu::default())),
        ("jsone", 0x26, || Arc::new(jsone::Jsone::default())),
        ("jsonu", 0x27, || Arc::new(jsonu::Jsonu::default())),
        ("ins", 0x28, || Arc::new(ins::Ins::default())),
        ("tlcw", 0x29, || Arc::new(tlcw::Tlcw::default())),
        ("tucw", 0x2a, || Arc::new(tucw::Tucw::default())),
        ("jkbc", 0x2b, || Arc::new(jkbc::Jkbc::default())),
        ("jcmc", 0x2d, || Arc::new(jcmc::Jcmc::default())),
        ("jpsc", 0x2e, || Arc::new(jpsc::Jpsc::default())),
        ("padl", 0x2f, || Arc::new(padl::Padl::default())),
        ("padr", 0x30, || Arc::new(padr::Padr::default())),
        ("rmws", 0x31, || Arc::new(rmws::Rmws::default())),
        ("dls", 0x32, || Arc::new(dls::Dls::default())),
        ("ifdc", 0x33, || Arc::new(ifdc::Ifdc::default())),
    ];

    entries
        .iter()
        .map(|(id, code, f)| TokenEntry::new(id, *code, f()))
        .collect()
});

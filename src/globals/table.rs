use std::{ borrow::Cow, collections::HashMap, sync::{ Arc, LazyLock } };

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

impl TokenEntry {
    fn new(identifier: Cow<'static, str>, code: u32, token: TokenRef) -> TokenEntry {
        TokenEntry { identifier, code, token }
    }

    pub fn get_bytecode(&self) -> u32 {
        self.code
    }
    pub fn get_identifier(&self) -> &str {
        self.identifier.as_ref()
    }
    pub fn get_token(&self) -> TokenRef {
        self.token.clone_ref()
    }
}

#[derive(Clone, Debug)]
pub enum QuerySource {
    Identifier(Cow<'static, str>),
    Bytecode(u32),
}

#[derive(Clone, Copy, Debug)]
pub enum QueryTarget {
    Identifier,
    Bytecode,
    Token,
}

#[derive(Clone)]
pub enum TargetValue {
    Identifier(&'static str),
    Bytecode(u32),
    Token(TokenRef),
}

pub struct TokenTable {
    // 4 hashmaps como você descreveu:
    id_to_code: HashMap<&'static str, u32>,
    code_to_id: HashMap<u32, &'static str>,
    id_to_token: HashMap<&'static str, TokenRef>,
    code_to_token: HashMap<u32, TokenRef>,

    // opcional: manter os entries por debug/iterar/listar
    entries: Vec<TokenEntry>,
}

impl TokenTable {
    pub fn find(
        &self,
        (query_source, query_target): (QuerySource, QueryTarget)
    ) -> Result<TargetValue, AtpError> {
        let err = ||
            AtpError::new(
                AtpErrorCode::TokenNotFound("Token Not Found in mapping".into()),
                "TOKEN_TABLE.find()",
                "query"
            );

        match (query_source, query_target) {
            // ✅ CORINGA: QuerySource e QueryTarget "iguais" (eco + valida existência)
            (QuerySource::Identifier(id), QueryTarget::Identifier) => {
                // valida que existe
                if self.id_to_code.contains_key(id.as_ref()) {
                    // devolve o próprio id, mas como &'static str:
                    // como seu mapa usa &'static str, pegamos pelo code_to_id via code:
                    let code = *self.id_to_code.get(id.as_ref()).ok_or_else(err)?;
                    let real_id = *self.code_to_id.get(&code).ok_or_else(err)?;
                    Ok(TargetValue::Identifier(real_id))
                } else {
                    Err(err())
                }
            }
            (QuerySource::Bytecode(code), QueryTarget::Bytecode) => {
                // valida que existe
                if self.code_to_id.contains_key(&code) {
                    Ok(TargetValue::Bytecode(code))
                } else {
                    Err(err())
                }
            }

            // --- demais casos normais ---

            (QuerySource::Identifier(id), QueryTarget::Bytecode) => {
                let code = *self.id_to_code.get(id.as_ref()).ok_or_else(err)?;
                Ok(TargetValue::Bytecode(code))
            }
            (QuerySource::Identifier(id), QueryTarget::Token) => {
                let tok = self.id_to_token.get(id.as_ref()).ok_or_else(err)?;
                Ok(TargetValue::Token(tok.clone_ref()))
            }
            (QuerySource::Bytecode(code), QueryTarget::Identifier) => {
                let id = *self.code_to_id.get(&code).ok_or_else(err)?;
                Ok(TargetValue::Identifier(id))
            }
            (QuerySource::Bytecode(code), QueryTarget::Token) => {
                let tok = self.code_to_token.get(&code).ok_or_else(err)?;
                Ok(TargetValue::Token(tok.clone_ref()))
            }
        }
    }
    // Se você ainda quiser “TokenEntry” por ID/Bytecode:
    pub fn entry_by_id(&self, id: &str) -> Option<&TokenEntry> {
        self.entries.iter().find(|e| e.get_identifier() == id)
    }
    pub fn entry_by_code(&self, code: u32) -> Option<&TokenEntry> {
        self.entries.iter().find(|e| e.get_bytecode() == code)
    }
}

macro_rules! define_token_table {
    ($vis:vis static $name:ident = [$(($id:literal, $code:expr, $ctor:expr)),* $(,)?];) => {
        $vis static $name: LazyLock<TokenTable> = LazyLock::new(|| {
            let mut id_to_code: HashMap<&'static str, u32> = HashMap::new();
            let mut code_to_id: HashMap<u32, &'static str> = HashMap::new();
            let mut id_to_token: HashMap<&'static str, TokenRef> = HashMap::new();
            let mut code_to_token: HashMap<u32, TokenRef> = HashMap::new();

            let mut entries: Vec<TokenEntry> = Vec::new();

            $(
                // constrói o token UMA vez e duplica a referência (Arc) pros 2 maps
                let token: TokenRef = ($ctor)();
                let token_for_code = token.clone_ref();

                // (opcional) checagens simples: duplicatas
                if id_to_code.contains_key($id) {
                    panic!("define_token_table: duplicate identifier: {}", $id);
                }
                if code_to_id.contains_key(&$code) {
                    panic!("define_token_table: duplicate bytecode: 0x{:x} for {}", $code, $id);
                }

                id_to_code.insert($id, $code);
                code_to_id.insert($code, $id);

                id_to_token.insert($id, token.clone_ref());
                code_to_token.insert($code, token_for_code);

                entries.push(TokenEntry::new(Cow::Borrowed($id), $code, token));
            )*

            TokenTable {
                id_to_code,
                code_to_id,
                id_to_token,
                code_to_token,
                entries,
            }
        });
    };
}

// ✅ sua “tabela 3 colunas” vira só isso:
define_token_table! {
    pub static TOKEN_TABLE = [
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
}

use std::{ borrow::Cow, collections::HashMap, sync::{ Arc, LazyLock } };

use crate::{ tokens::InstructionMethods, utils::errors::{ AtpError, AtpErrorCode } };

use crate::tokens::{ instructions::*, transforms::* };

#[derive(Clone)]
pub enum TokenRef {
    Boxed(Box<dyn InstructionMethods>),
    Shared(Arc<dyn InstructionMethods>),
}

impl TokenRef {
    fn clone_ref(&self) -> TokenRef {
        match self {
            TokenRef::Boxed(b) => TokenRef::Boxed(b.clone()),
            TokenRef::Shared(a) => TokenRef::Shared(a.clone()),
        }
    }

    pub fn into_box(self) -> Box<dyn InstructionMethods> {
        match self {
            TokenRef::Boxed(b) => b,
            TokenRef::Shared(a) => a.clone_box(),
        }
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
    Params,
}

#[derive(Clone)]
pub enum TargetValue {
    Identifier(&'static str),
    Bytecode(u32),
    Token(TokenRef),
    Params(Arc<[InstructionParam]>),
}

pub struct TokenTable {
    id_to_code: HashMap<&'static str, u32>,
    code_to_id: HashMap<u32, &'static str>,
    id_to_token: HashMap<&'static str, TokenRef>,
    code_to_token: HashMap<u32, TokenRef>,
    id_to_expected: HashMap<&'static str, Arc<[InstructionParam]>>,
    code_to_expected: HashMap<u32, Arc<[InstructionParam]>>,
}

impl TokenTable {
    pub fn find(
        &self,
        (query_source, query_target): (QuerySource, QueryTarget)
    ) -> Result<TargetValue, AtpError> {
        let err = || {
            AtpError::new(
                AtpErrorCode::TokenNotFound("Token Not Found in mapping".into()),
                "TOKEN_TABLE.find()",
                "query"
            )
        };

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
            (QuerySource::Identifier(id), QueryTarget::Params) => {
                let params = self.id_to_expected.get(id.as_ref()).ok_or_else(err)?;
                Ok(TargetValue::Params(params.clone()))
            }
            (QuerySource::Bytecode(code), QueryTarget::Params) => {
                let params = self.code_to_expected.get(&code).ok_or_else(err)?;
                Ok(TargetValue::Params(params.clone()))
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParamType {
    String,
    Usize,
    Token,
    VarRef,
    BlockRef,
}

pub struct InstructionParam {
    pub param_type: ParamType,
    pub optional: bool,
}

impl InstructionParam {
    pub fn opt(param_type: ParamType) -> Self {
        InstructionParam {
            param_type,
            optional: true,
        }
    }
    pub fn req(param_type: ParamType) -> Self {
        InstructionParam {
            param_type,
            optional: false,
        }
    }
}

macro_rules! define_token_table {
    (
        $vis:vis static $name:ident = [
            $((
                $id:literal,
                $code:expr,
                $ctor:expr,
                $params:expr $(,)? // ✅ aceita vírgula opcional aqui
            )),*
            $(,)?
        ];
    ) => {
        $vis static $name: LazyLock<TokenTable> = LazyLock::new(|| {
            let mut id_to_code: HashMap<&'static str, u32> = HashMap::new();
            let mut code_to_id: HashMap<u32, &'static str> = HashMap::new();
            let mut id_to_token: HashMap<&'static str, TokenRef> = HashMap::new();
            let mut code_to_token: HashMap<u32, TokenRef> = HashMap::new();

            let mut id_to_expected: HashMap<&'static str, Arc<[InstructionParam]>> = HashMap::new();
            let mut code_to_expected: HashMap<u32, Arc<[InstructionParam]>> = HashMap::new();

            $(
                let token: TokenRef = ($ctor)();
                let token_for_code = token.clone_ref();

                if id_to_code.contains_key($id) {
                    panic!("define_token_table: duplicate identifier: {}", $id);
                }
                if code_to_id.contains_key(&$code) {
                    panic!("define_token_table: duplicate bytecode: 0x{:x} for {}", $code, $id);
                }

                // ✅ aqui é o ponto crítico: array -> slice

                id_to_code.insert($id, $code);
                code_to_id.insert($code, $id);

                id_to_token.insert($id, token.clone_ref());
                code_to_token.insert($code, token_for_code);

                let arc_params = Arc::new($params);

                id_to_expected.insert($id, arc_params.clone());
                code_to_expected.insert($code, arc_params);
            )*

            TokenTable {
                id_to_code,
                code_to_id,
                id_to_token,
                code_to_token,
                id_to_expected,
                code_to_expected,
            }
        });
    };
}

// ✅ sua “tabela 3 colunas” vira só isso:
define_token_table! {
    pub static TOKEN_TABLE = [
        (
            "atb",
            0x01,
            || TokenRef::Shared(Arc::new(atb::Atb::default())),
            [InstructionParam::req(ParamType::String)],
        ),
        (
            "ate",
            0x02,
            || TokenRef::Shared(Arc::new(ate::Ate::default())),
            [InstructionParam::req(ParamType::String)],
        ),
        (
            "dlc",
            0x08,
            || TokenRef::Shared(Arc::new(dlc::Dlc::default())),
            [InstructionParam::req(ParamType::Usize), InstructionParam::req(ParamType::Usize)],
        ),
        ("dlf", 0x03, || TokenRef::Shared(Arc::new(dlf::Dlf::default())), []),
        ("dll", 0x04, || TokenRef::Shared(Arc::new(dll::Dll::default())), []),
        (
            "dla",
            0x09,
            || TokenRef::Shared(Arc::new(dla::Dla::default())),
            [InstructionParam::req(ParamType::Usize)],
        ),
        (
            "dlb",
            0x0a,
            || TokenRef::Shared(Arc::new(dlb::Dlb::default())),
            [InstructionParam::req(ParamType::Usize)],
        ),
        (
            "rfw",
            0x0b,
            || TokenRef::Shared(Arc::new(rfw::Rfw::default())),
            [InstructionParam::req(ParamType::String), InstructionParam::req(ParamType::String)],
        ),
        (
            "rcw",
            0x10,
            || TokenRef::Shared(Arc::new(rcw::Rcw::default())),
            [
                InstructionParam::req(ParamType::String),
                InstructionParam::req(ParamType::String),
                InstructionParam::req(ParamType::Usize),
            ],
        ),
        (
            "raw",
            0x0c,
            || TokenRef::Shared(Arc::new(raw::Raw::default())),
            [InstructionParam::req(ParamType::String), InstructionParam::req(ParamType::String)],
        ),
        ("tbs", 0x05, || TokenRef::Shared(Arc::new(tbs::Tbs::default())), []),
        ("tls", 0x06, || TokenRef::Shared(Arc::new(tls::Tls::default())), []),
        ("trs", 0x07, || TokenRef::Shared(Arc::new(trs::Trs::default())), []),
        (
            "rpt",
            0x0d,
            || TokenRef::Shared(Arc::new(rpt::Rpt::default())),
            [InstructionParam::req(ParamType::Usize)],
        ),
        (
            "rtr",
            0x0f,
            || TokenRef::Shared(Arc::new(rtr::Rtr::default())),
            [InstructionParam::req(ParamType::Usize)],
        ),
        (
            "rtl",
            0x0e,
            || TokenRef::Shared(Arc::new(rtl::Rtl::default())),
            [InstructionParam::req(ParamType::Usize)],
        ),
        (
            "slt",
            0x11,
            || TokenRef::Shared(Arc::new(slt::Slt::default())),
            [InstructionParam::req(ParamType::Usize), InstructionParam::req(ParamType::Usize)],
        ),
        ("tua", 0x12, || TokenRef::Shared(Arc::new(tua::Tua::default())), []),
        ("tla", 0x13, || TokenRef::Shared(Arc::new(tla::Tla::default())), []),
        (
            "tucs",
            0x14,
            || TokenRef::Shared(Arc::new(tucs::Tucs::default())),
            [InstructionParam::req(ParamType::Usize)],
        ),
        (
            "tlcs",
            0x15,
            || TokenRef::Shared(Arc::new(tlcs::Tlcs::default())),
            [InstructionParam::req(ParamType::Usize)],
        ),
        (
            "tucc",
            0x16,
            || TokenRef::Shared(Arc::new(tucc::Tucc::default())),
            [InstructionParam::req(ParamType::Usize), InstructionParam::req(ParamType::Usize)],
        ),
        (
            "tlcc",
            0x17,
            || TokenRef::Shared(Arc::new(tlcc::Tlcc::default())),
            [InstructionParam::req(ParamType::Usize), InstructionParam::req(ParamType::Usize)],
        ),
        ("cfw", 0x18, || TokenRef::Shared(Arc::new(cfw::Cfw::default())), []),
        ("clw", 0x19, || TokenRef::Shared(Arc::new(clw::Clw::default())), []),
        (
            "sslt",
            0x1a,
            || TokenRef::Shared(Arc::new(sslt::Sslt::default())),
            [InstructionParam::req(ParamType::String), InstructionParam::req(ParamType::Usize)],
        ),
        (
            "ctc",
            0x1b,
            || TokenRef::Shared(Arc::new(ctc::Ctc::default())),
            [InstructionParam::req(ParamType::Usize), InstructionParam::req(ParamType::Usize)],
        ),
        (
            "ctr",
            0x1c,
            || TokenRef::Shared(Arc::new(ctr::Ctr::default())),
            [InstructionParam::req(ParamType::Usize), InstructionParam::req(ParamType::Usize)],
        ),
        (
            "cts",
            0x1d,
            || TokenRef::Shared(Arc::new(cts::Cts::default())),
            [InstructionParam::req(ParamType::Usize)],
        ),
        (
            "rlw",
            0x1e,
            || TokenRef::Shared(Arc::new(rlw::Rlw::default())),
            [InstructionParam::req(ParamType::String), InstructionParam::req(ParamType::String)],
        ),
        (
            "rnw",
            0x1f,
            || TokenRef::Shared(Arc::new(rnw::Rnw::default())),
            [
                InstructionParam::req(ParamType::String),
                InstructionParam::req(ParamType::String),
                InstructionParam::req(ParamType::Usize),
            ],
        ),
        ("urle", 0x20, || TokenRef::Shared(Arc::new(urle::Urle::default())), []),
        ("urld", 0x21, || TokenRef::Shared(Arc::new(urld::Urld::default())), []),
        ("rev", 0x22, || TokenRef::Shared(Arc::new(rev::Rev::default())), []),
        ("splc", 0x23, || TokenRef::Shared(Arc::new(splc::Splc::default())), []),
        ("htmle", 0x24, || TokenRef::Shared(Arc::new(htmle::Htmle::default())), []),
        ("htmlu", 0x25, || TokenRef::Shared(Arc::new(htmlu::Htmlu::default())), []),
        ("jsone", 0x26, || TokenRef::Shared(Arc::new(jsone::Jsone::default())), []),
        ("jsonu", 0x27, || TokenRef::Shared(Arc::new(jsonu::Jsonu::default())), []),
        (
            "ins",
            0x28,
            || TokenRef::Shared(Arc::new(ins::Ins::default())),
            [InstructionParam::req(ParamType::Usize), InstructionParam::req(ParamType::String)],
        ),
        (
            "tlcw",
            0x29,
            || TokenRef::Shared(Arc::new(tlcw::Tlcw::default())),
            [InstructionParam::req(ParamType::Usize)],
        ),
        (
            "tucw",
            0x2a,
            || TokenRef::Shared(Arc::new(tucw::Tucw::default())),
            [InstructionParam::req(ParamType::Usize)],
        ),
        ("jkbc", 0x2b, || TokenRef::Shared(Arc::new(jkbc::Jkbc::default())), []),
        ("jcmc", 0x2d, || TokenRef::Shared(Arc::new(jcmc::Jcmc::default())), []),
        ("jpsc", 0x2e, || TokenRef::Shared(Arc::new(jpsc::Jpsc::default())), []),
        (
            "padl",
            0x2f,
            || TokenRef::Shared(Arc::new(padl::Padl::default())),
            [InstructionParam::req(ParamType::String), InstructionParam::req(ParamType::Usize)],
        ),
        (
            "padr",
            0x30,
            || TokenRef::Shared(Arc::new(padr::Padr::default())),
            [InstructionParam::req(ParamType::String), InstructionParam::req(ParamType::Usize)],
        ),
        ("rmws", 0x31, || TokenRef::Shared(Arc::new(rmws::Rmws::default())), []),
        (
            "dls",
            0x32,
            || TokenRef::Shared(Arc::new(dls::Dls::default())),
            [InstructionParam::req(ParamType::Usize)],
        ),
        (
            "ifdc",
            0x33,
            || TokenRef::Shared(Arc::new(ifdc::Ifdc::default())),
            [InstructionParam::req(ParamType::String), InstructionParam::req(ParamType::Token)],
        ),
        (
            "blk",
            0x34,
            || TokenRef::Shared(Arc::new(blk::Blk::default())),
            [InstructionParam::req(ParamType::String), InstructionParam::req(ParamType::Token)],
        ),
        (
            "cblk",
            0x35,
            || TokenRef::Shared(Arc::new(blk::Blk::default())),
            [InstructionParam::req(ParamType::String)],
        ),
    ];
}

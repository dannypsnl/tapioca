use ariadne::{Cache, Source};
use enotation::ENotation;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Module {
    pub source: (String, Source<String>),
    pub requires: Vec<Require>,
    pub claim_forms: Vec<ClaimForm>,
    pub define_forms: Vec<DefineForm>,
    pub other_forms: Vec<ENotation>,
}

impl Cache<String> for Module {
    type Storage = String;

    fn fetch(
        &mut self,
        id: &String,
    ) -> Result<&Source<Self::Storage>, Box<dyn std::fmt::Debug + '_>> {
        self.source.fetch(id)
    }

    fn display<'a>(&self, id: &'a String) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.source.display(id)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Require {
    pub module: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClaimForm {
    // (: x : int)
    // claims `x` has type `int`
    pub id: String,
    // TODO: normalize this to internal AST
    pub typ: Typ,
}

// NOTE: (define x : <type> <expr>) will be elaborated to
// (: x : <type>)
// (define x <expr>)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DefineForm {
    // (define x <expr>)
    DefineConstant {
        span: ReportSpan,
        id: String,
        expr: Expr,
    },
    // (define (f x y z ...)
    //   <body_1>
    //   ...
    //   <body_k>)
    DefineFunction {
        span: ReportSpan,
        id: String,
        params: Vec<String>,
        body: Vec<Expr>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Expr {
    Bool(bool),
    Char(char),
    String(String),
    Rational(i64, i64),
    Float(f64),
    Int(i64),

    Identifier(String),
    Symbol(String),

    App(Box<Expr>, Vec<Expr>),

    List(Vec<Expr>),
    Tuple(Vec<Expr>),
    Object(Vec<(String, Expr)>),

    Syntax(Box<Expr>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Typ {
    Bool,
    Char,
    String,
    Symbol,
    Rational,
    Float,
    Int,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Syntax,
    Void,
    Array(Box<Typ>),
    List(Box<Typ>),
    Tuple(Vec<Typ>),
    Record(Vec<(String, Typ)>),
    Func { params: Vec<Typ>, result: Box<Typ> },
}
impl Display for Typ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Typ::Bool => write!(f, "bool"),
            Typ::Char => write!(f, "char"),
            Typ::String => write!(f, "string"),
            Typ::Symbol => write!(f, "symbol"),
            Typ::Rational => write!(f, "rational"),
            Typ::Float => write!(f, "float"),
            Typ::Int => write!(f, "int"),
            Typ::I8 => write!(f, "i8"),
            Typ::I16 => write!(f, "i16"),
            Typ::I32 => write!(f, "i32"),
            Typ::I64 => write!(f, "i64"),
            Typ::U8 => write!(f, "u8"),
            Typ::U16 => write!(f, "u16"),
            Typ::U32 => write!(f, "u32"),
            Typ::U64 => write!(f, "u64"),
            Typ::Syntax => write!(f, "syntax"),
            Typ::Void => write!(f, "void"),
            Typ::Array(typ) => write!(f, "(array {})", typ),
            Typ::List(typ) => write!(f, "(list {})", typ),
            Typ::Tuple(vec) => {
                write!(f, "(tuple")?;
                for (i, typ) in vec.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{}", typ)?;
                    } else {
                        write!(f, " {}", typ)?;
                    }
                }
                write!(f, ")")
            }
            Typ::Record(_vec) => todo!(),
            Typ::Func { params, result } => {
                for (i, typ) in params.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{}", typ)?;
                    } else {
                        write!(f, " {}", typ)?;
                    }
                }
                write!(f, "-> {}", result)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportSpan {
    source: String,
    start_offset: usize,
    end_offset: usize,
}

impl From<enotation::DiagnosticSpan> for ReportSpan {
    fn from(dspan: enotation::DiagnosticSpan) -> Self {
        ReportSpan {
            source: dspan.file.unwrap(),
            start_offset: dspan.start_offset,
            end_offset: dspan.end_offset,
        }
    }
}

impl ariadne::Span for ReportSpan {
    type SourceId = String;
    fn source(&self) -> &Self::SourceId {
        &self.source
    }
    fn start(&self) -> usize {
        self.start_offset
    }
    fn end(&self) -> usize {
        self.end_offset
    }
}

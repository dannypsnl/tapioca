use ariadne::Source;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Module<'a> {
    pub source: Source<&'a str>,
    pub requires: Vec<Require>,
    pub claim_forms: Vec<ClaimForm>,
    pub define_forms: Vec<DefineForm>,
    pub other_forms: Vec<Expr>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Require {
    pub module: String,
}

#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
pub enum DefineForm {
    // (define x <expr>)
    DefineConstant {
        id: String,
        expr: Expr,
    },
    // (define (f x y z ...)
    //   <body_1>
    //   ...
    //   <body_k>)
    DefineFunction {
        id: String,
        params: Vec<String>,
        body: Vec<Expr>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    Bool(bool),
    Char(char),
    String(String),
    Rational(i64, i64),
    Float(f64),
    Int(i64),

    Identifier(String),

    List(Vec<Expr>),
    Tuple(Vec<Expr>),
    Object(Vec<(String, Expr)>),

    Syntax(Box<Expr>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Typ {
    Bool,
    Char,
    String,
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
    Array(Box<Typ>),
    List(Box<Typ>),
    Tuple(Vec<Typ>),
}

pub struct ReportSpan {
    diagspan: enotation::DiagnosticSpan,
}

impl ReportSpan {
    pub fn new(diagspan: enotation::DiagnosticSpan) -> Self {
        ReportSpan { diagspan }
    }
}

impl ariadne::Span for ReportSpan {
    type SourceId = ();
    fn source(&self) -> &Self::SourceId {
        &()
    }
    fn start(&self) -> usize {
        self.diagspan.start_offset
    }
    fn end(&self) -> usize {
        self.diagspan.end_offset
    }
}

use ariadne::{Cache, Source};
use enotation::ENotation;
use serde::{Deserialize, Serialize};

pub mod expr;
pub mod typ;
use expr::Expr;
use typ::Typ;

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
        body: Expr,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

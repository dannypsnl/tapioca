use ariadne::Source;
use enotation::ENotation;

#[derive(Debug)]
pub struct Module<'a> {
    pub source: Source<&'a str>,
    pub requires: Vec<Require>,
    pub claim_forms: Vec<ClaimForm>,
    pub define_forms: Vec<DefineForm>,
    pub other_forms: Vec<ENotation>,
}

#[derive(Debug)]
pub struct Require {
    pub module: String,
}

#[derive(Debug)]
pub struct ClaimForm {
    // (: x : int)
    // claims `x` has type `int`
    pub id: String,
    // TODO: normalize this to internal AST
    pub typ: ENotation,
}

// NOTE: (define x : <type> <expr>) will be elaborated to
// (: x : <type>)
// (define x <expr>)
#[derive(Debug)]
pub enum DefineForm {
    // (define x <expr>)
    DefineConstant {
        id: String,
        expr: ENotation,
    },
    // (define (f x y z ...)
    //   <body_1>
    //   ...
    //   <body_k>)
    DefineFunction {
        id: String,
        params: Vec<String>,
        body: Vec<ENotation>,
    },
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

use ariadne::{Report, ReportKind, Source};
use enotation::{DiagnosticSpan, EFile, ENotation, ENotationParser, Rule};
use from_pest::{ConversionError, FromPest, Void};
use pest::Parser;
use std::{collections::BTreeMap, fs};

mod matcher;

use matcher::{EPattern, ematch};

#[derive(Debug)]
pub struct Module<'a> {
    source: Source<&'a str>,
    requires: Vec<Require>,
    claim_forms: Vec<ClaimForm>,
    define_forms: Vec<DefineForm>,
    other_forms: Vec<ENotation>,
}

#[derive(Debug)]
pub struct Require {
    module: String,
}

#[derive(Debug)]
pub struct ClaimForm {
    // (: x : int)
    // claims `x` has type `int`
    id: String,
    // TODO: normalize this to internal AST
    typ: ENotation,
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

fn expand_module(input: &str) -> Result<Module, Error> {
    let mut module = Module {
        source: Source::from(input),
        claim_forms: vec![],
        define_forms: vec![],
        other_forms: vec![],
        requires: vec![],
    };

    let mut output = ENotationParser::parse(Rule::file, input).unwrap();
    let efile = EFile::from_pest(&mut output)?;
    for notation in efile.notations {
        expand_top_level(&mut module, notation);
    }

    Ok(module)
}

fn expand_top_level(module: &mut Module, notation: ENotation) {
    use EPattern::*;
    let mut binds = BTreeMap::new();
    if ematch(
        &mut binds,
        &notation,
        List(vec![Id("define"), RestHole("rest")]),
    ) {
        expand_defines(module, &notation);
    } else if ematch(
        &mut binds,
        &notation,
        List(vec![Id("require"), Hole("module")]),
    ) {
        let m = binds.get("module").unwrap();
        module.requires.push(Require {
            module: format!("{}", m),
        });
    } else if ematch(
        &mut binds,
        &notation,
        List(vec![Id("require"), RestHole("_")]),
    ) {
        Report::build(ReportKind::Error, ReportSpan::new(notation.span))
            .with_message("")
            .finish()
            .print(module.source.clone())
            .unwrap();
    }
}

fn expand_claims(module: &mut Module, notation: &ENotation) {
    todo!()
}

fn expand_defines(module: &mut Module, notation: &ENotation) {
    todo!()
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("example/hello.ss")?;
    let module = expand_module(input.as_str())?;
    println!("{:?}", module);
    Ok(())
}

struct ReportSpan {
    diagspan: DiagnosticSpan,
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
impl ReportSpan {
    fn new(diagspan: DiagnosticSpan) -> Self {
        ReportSpan { diagspan }
    }
}

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    Parser(ConversionError<Void>),
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}
impl From<ConversionError<Void>> for Error {
    fn from(err: ConversionError<Void>) -> Error {
        Error::Parser(err)
    }
}

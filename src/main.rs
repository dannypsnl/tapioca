use enotation::{EFile, ENotation, ENotationBody, ENotationParser, Rule};
use from_pest::{ConversionError, FromPest, Void};
use pest::Parser;
use std::fs;

mod matcher;

pub struct Module {
    claim_forms: Vec<ClaimForm>,
    define_forms: Vec<DefineForm>,
    other_forms: Vec<ENotation>,
}

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

fn expand_module(notations: Vec<ENotation>) -> Result<Module, Error> {
    let mut module = Module {
        claim_forms: vec![],
        define_forms: vec![],
        other_forms: vec![],
    };

    for notation in notations {
        expand_top_level(&mut module, notation);
    }

    Ok(module)
}

fn expand_top_level(module: &mut Module, notation: ENotation) {
    match &notation.body {
        ENotationBody::Container(container) => match container {
            enotation::container::Container::List(list) => {
                let e = list.elems();
            }

            enotation::container::Container::Set(set) => todo!(),
            enotation::container::Container::UnamedObject(unamed_object) => todo!(),
            enotation::container::Container::Object(object) => todo!(),
        },
        ENotationBody::Literal(literal) => todo!(),
        ENotationBody::Quoting(quoting) => todo!(),
        ENotationBody::Syntaxing(syntaxing) => todo!(),
    }
}

fn expand_requires(module: &mut Module, vec: &Vec<ENotation>) {
    todo!()
}

fn expand_claims(module: &mut Module, vec: &Vec<ENotation>) {
    todo!()
}

fn expand_defines(module: &mut Module, vec: &Vec<ENotation>) {
    todo!()
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("example/hello.ss")?;
    let mut output = ENotationParser::parse(Rule::file, input.as_str()).unwrap();
    let efile = EFile::from_pest(&mut output)?;
    let module = expand_module(efile.notations)?;
    println!("Hello, world!");
    Ok(())
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

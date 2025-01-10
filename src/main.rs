use enotation::{ENotation, ENotationBody};

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

struct ParseError {}

fn expand_module(notations: Vec<ENotation>) -> Result<Module, ParseError> {
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
        ENotationBody::List(vec) => {
            if vec.len() != 0 {
                match &vec[0].body {
                    ENotationBody::Identifier(id) => match id.as_str() {
                        "require" => expand_requires(module, vec),
                        ":" => expand_claims(module, vec),
                        "define" => expand_defines(module, vec),
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            } else {
                // TODO: report an empty application
            }
        }
        // rest are all been treated as script part
        ENotationBody::Boolean(_) => todo!(),
        ENotationBody::Integer(_) => todo!(),
        ENotationBody::Rational(_, _) => todo!(),
        ENotationBody::Float(_) => todo!(),
        ENotationBody::Char(_) => todo!(),
        ENotationBody::Str(_) => todo!(),
        ENotationBody::Identifier(_) => todo!(),
        ENotationBody::Set(vec) => todo!(),
        ENotationBody::UnamedObject(vec) => todo!(),
        ENotationBody::Object(vec) => todo!(),
        ENotationBody::Quote(enotation) => todo!(),
        ENotationBody::QuasiQuote(enotation) => todo!(),
        ENotationBody::Unquote(enotation) => todo!(),
        ENotationBody::UnquoteSplicing(enotation) => todo!(),
        ENotationBody::Syntax(enotation) => todo!(),
        ENotationBody::QuasiSyntax(enotation) => todo!(),
        ENotationBody::Unsyntax(enotation) => todo!(),
        ENotationBody::UnsyntaxSplicing(enotation) => todo!(),
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

fn main() {
    println!("Hello, world!");
}

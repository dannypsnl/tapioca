use chumsky::prelude::*;
use enotation::ENotation;

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

fn main() {
    println!("Hello, world!");
}

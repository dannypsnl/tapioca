use serde::{Deserialize, Serialize};

use super::ReportSpan;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExprBody {
    Bool(bool),
    Char(char),
    String(String),
    Rational(i64, i64),
    Float(f64),
    Int(i64),

    Identifier(Identifier),
    Symbol(String),

    Begin(Vec<Expr>, Box<Expr>),

    Let(Vec<Binding>, Box<Expr>),

    Lambda(Vec<String>, Box<Expr>),
    App(Box<Expr>, Vec<Expr>),

    List(Vec<Expr>),
    Tuple(Vec<Expr>),
    Object(Vec<(String, Expr)>),

    Syntax(Box<Expr>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Identifier {
    Bind {
        origin_name: String,
        lookup_name: String,
    },
    // top-level will not be renamed, and hence just a single
    TopLevel(String),
}

impl Identifier {
    /// Constructs a name that not been renamed, usually they are
    /// 1. top-level definition
    /// 2. parameters in top-level `(define (f ……) ……)` form
    pub fn top_level(name: String) -> Self {
        Identifier::TopLevel(name)
    }

    /// Access to the programmer wrote done name, for example
    ///
    /// (let ([x 1])
    ///   (let ([x x])
    ///     x))
    ///
    /// After expansion process, **enotation** been converted to internal AST, and renaming is complete
    /// 1. The first `let x` will be renamed to, e.g. `x1`
    /// 2. The second `let x` will be renamed to, e.g. `x2`
    ///
    /// Therefore, the renamed AST is
    ///
    /// (let ([x1 1])
    ///   (let ([x2 x1])
    ///     x2))
    ///
    /// But for reporting, shouldn't report cannot find `x2` at all, because no programmers expect the error message is talking internal name from compiler.
    pub fn info_name(&self) -> String {
        match self {
            Identifier::Bind { origin_name, .. } => origin_name.clone(),
            Identifier::TopLevel(origin_name) => origin_name.clone(),
        }
    }
    /// The internal name that use to lookup in type system (after expansion)
    pub fn lookup_name(&self) -> &String {
        match self {
            Identifier::Bind { lookup_name, .. } => lookup_name,
            Identifier::TopLevel(lookup_name) => lookup_name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Binding {
    pub name: String,
    pub expr: Expr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expr {
    pub span: ReportSpan,
    pub body: ExprBody,
}

impl ExprBody {
    pub fn with_span(self, span: ReportSpan) -> Expr {
        Expr { span, body: self }
    }
}

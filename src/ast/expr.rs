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
    // construction
    pub fn top_level(name: String) -> Self {
        Identifier::TopLevel(name)
    }

    // access
    pub fn info_name(&self) -> String {
        match self {
            Identifier::Bind { origin_name, .. } => origin_name.clone(),
            Identifier::TopLevel(origin_name) => origin_name.clone(),
        }
    }
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

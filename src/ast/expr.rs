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
pub struct Identifier {
    pub origin_name: String,
    pub lookup_name: String,
}

impl Identifier {
    pub fn origin(name: String) -> Self {
        Identifier {
            origin_name: name.clone(),
            lookup_name: name,
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

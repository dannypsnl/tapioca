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

    Identifier(String),
    Symbol(String),

    Let(Vec<Binding>, Vec<Expr>),

    App(Box<Expr>, Vec<Expr>),

    List(Vec<Expr>),
    Tuple(Vec<Expr>),
    Object(Vec<(String, Expr)>),

    Syntax(Box<Expr>),
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

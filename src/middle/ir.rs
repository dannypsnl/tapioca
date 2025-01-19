use crate::ast::expr;
use std::collections::BTreeSet;

pub struct Module {}
pub enum Expr {
    Closure(LiftedLambda, BTreeSet<expr::Identifier>),
    Lam(LiftedLambda),
    ClosureEnvGet(usize),
    Identifier(expr::Identifier),
    Begin(Vec<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Vec<Bind>, Box<Expr>),
    App(Box<Expr>, Vec<Expr>),
    List(Vec<Expr>),
    Pair(Box<Expr>, Box<Expr>),
    Object(Vec<(String, Expr)>),
    Syntax(Box<Expr>),
    Bool(bool),
    Char(char),
    String(String),
    Rational(i64, i64),
    Float(f64),
    Int(i64),
    Symbol(String),
}
pub struct Bind {
    pub name: expr::Identifier,
    pub expr: Expr,
}
pub struct LiftedLambda {
    params: Vec<expr::Identifier>,
    expr: Box<Expr>,
}
impl LiftedLambda {
    pub fn new(params: Vec<expr::Identifier>, expr: Box<Expr>) -> Self {
        Self { params, expr }
    }
}

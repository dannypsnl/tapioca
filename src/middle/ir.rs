use crate::ast::expr;
use std::collections::BTreeSet;

pub struct Module {}
pub enum Expr {
    Closure(LiftedLambda, BTreeSet<expr::Identifier>),
    ClosureEnvGet(usize),
    Identifier(expr::Identifier),
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

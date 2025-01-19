use crate::ast::expr;
use std::collections::BTreeSet;

pub mod ir;

fn free_variables(e: &expr::Expr) -> BTreeSet<expr::Identifier> {
    match &e.body {
        expr::ExprBody::Identifier(identifier) => {
            let mut set = BTreeSet::default();
            set.insert(identifier.clone());
            set
        }

        expr::ExprBody::Let(binds, expr) => {
            let mut binding_sets = BTreeSet::default();
            let mut a = BTreeSet::default();
            for bind in binds {
                binding_sets.insert(bind.name.clone());
                a = a.union(&free_variables(&bind.expr)).cloned().collect();
            }
            let b = free_variables(&expr);
            a.union(&b.difference(&binding_sets).cloned().collect())
                .cloned()
                .collect()
        }
        expr::ExprBody::Lambda(params, expr) => {
            let a = free_variables(&expr);
            let params_set: BTreeSet<_> = params.iter().cloned().collect();
            a.difference(&params_set).cloned().collect()
        }

        expr::ExprBody::Begin(exprs, expr) => {
            let sets = exprs.iter().map(free_variables);
            let a = sets.fold(BTreeSet::default(), |acc, vars| {
                acc.union(&vars).cloned().collect()
            });
            let b = free_variables(&expr);
            a.union(&b).cloned().collect()
        }
        expr::ExprBody::App(expr, args) => {
            let f = free_variables(expr);
            let sets = args.iter().map(free_variables);
            let a = sets.fold(f, |acc, vars| acc.union(&vars).cloned().collect());
            a
        }
        expr::ExprBody::List(exprs) => {
            let sets = exprs.iter().map(free_variables);
            let a = sets.fold(BTreeSet::default(), |acc, vars| {
                acc.union(&vars).cloned().collect()
            });
            a
        }
        expr::ExprBody::Pair(expr, expr1) => {
            let a = free_variables(expr);
            let b = free_variables(expr1);
            a.union(&b).cloned().collect()
        }
        expr::ExprBody::Object(fields) => {
            let sets = fields.iter().map(|(_, e)| free_variables(e));
            let a = sets.fold(BTreeSet::default(), |acc, vars| {
                acc.union(&vars).cloned().collect()
            });
            a
        }
        expr::ExprBody::Syntax(expr) => free_variables(&expr),
        // The expression with no sub-expression will not contain any free variables
        expr::ExprBody::Bool(_)
        | expr::ExprBody::Symbol(_)
        | expr::ExprBody::Char(_)
        | expr::ExprBody::String(_)
        | expr::ExprBody::Rational(_, _)
        | expr::ExprBody::Float(_)
        | expr::ExprBody::Int(_) => BTreeSet::default(),
    }
}

use crate::ast::expr;
use std::collections::BTreeSet;

pub mod ir;

pub fn closure_conversion(e: expr::Expr) -> ir::Expr {
    use ir::Expr::*;
    use ir::LiftedLambda;
    match e.body {
        expr::ExprBody::Lambda(params, expr) => {
            let freevars = free_variables(&expr);
            Closure(
                LiftedLambda::new(params, Box::new(replace_free(*expr, &freevars))),
                freevars,
            )
        }
        // dummy recursive
        expr::ExprBody::Begin(vec, expr) => Begin(
            vec.into_iter().map(closure_conversion).collect(),
            Box::new(closure_conversion(*expr)),
        ),
        expr::ExprBody::Let(binds, expr) => {
            let new_binds = binds
                .into_iter()
                .map(|bind| ir::Bind {
                    name: bind.name,
                    expr: closure_conversion(bind.expr),
                })
                .collect();
            Let(new_binds, Box::new(closure_conversion(*expr)))
        }
        expr::ExprBody::App(func, args) => App(
            Box::new(closure_conversion(*func)),
            args.into_iter().map(closure_conversion).collect(),
        ),
        expr::ExprBody::List(exprs) => List(exprs.into_iter().map(closure_conversion).collect()),
        expr::ExprBody::Pair(expr1, expr2) => Pair(
            Box::new(closure_conversion(*expr1)),
            Box::new(closure_conversion(*expr2)),
        ),
        expr::ExprBody::Object(fields) => Object(
            fields
                .into_iter()
                .map(|(name, expr)| (name, closure_conversion(expr)))
                .collect(),
        ),
        expr::ExprBody::Syntax(expr) => Syntax(Box::new(closure_conversion(*expr))),
        expr::ExprBody::Bool(b) => Bool(b),
        expr::ExprBody::Char(c) => Char(c),
        expr::ExprBody::String(s) => String(s),
        expr::ExprBody::Rational(n, d) => Rational(n, d),
        expr::ExprBody::Float(f) => Float(f),
        expr::ExprBody::Int(i) => Int(i),
        expr::ExprBody::Identifier(id) => Identifier(id),
        expr::ExprBody::Symbol(s) => Symbol(s),
    }
}

fn replace_free(e: expr::Expr, freevars: &BTreeSet<expr::Identifier>) -> ir::Expr {
    use ir::Expr::*;
    match e.body {
        expr::ExprBody::Lambda(_, _) => closure_conversion(e),
        expr::ExprBody::Identifier(id) => {
            if freevars.contains(&id) {
                ClosureEnvGet(freevars.iter().position(|r| r == &id).unwrap())
            } else {
                Identifier(id.clone())
            }
        }
        // dummy recursive
        expr::ExprBody::Begin(vec, expr) => Begin(
            vec.into_iter().map(|e| replace_free(e, freevars)).collect(),
            Box::new(replace_free(*expr, freevars)),
        ),
        expr::ExprBody::Let(binds, expr) => {
            let new_binds = binds
                .into_iter()
                .map(|bind| ir::Bind {
                    name: bind.name,
                    expr: replace_free(bind.expr, freevars),
                })
                .collect();
            Let(new_binds, Box::new(replace_free(*expr, freevars)))
        }
        expr::ExprBody::App(func, args) => App(
            Box::new(replace_free(*func, freevars)),
            args.into_iter()
                .map(|e| replace_free(e, freevars))
                .collect(),
        ),
        expr::ExprBody::List(exprs) => List(
            exprs
                .into_iter()
                .map(|e| replace_free(e, freevars))
                .collect(),
        ),
        expr::ExprBody::Pair(expr1, expr2) => Pair(
            Box::new(replace_free(*expr1, freevars)),
            Box::new(replace_free(*expr2, freevars)),
        ),
        expr::ExprBody::Object(fields) => Object(
            fields
                .into_iter()
                .map(|(name, expr)| (name, replace_free(expr, freevars)))
                .collect(),
        ),
        expr::ExprBody::Syntax(expr) => Syntax(Box::new(replace_free(*expr, freevars))),
        expr::ExprBody::Bool(b) => Bool(b),
        expr::ExprBody::Char(c) => Char(c),
        expr::ExprBody::String(s) => String(s),
        expr::ExprBody::Rational(n, d) => Rational(n, d),
        expr::ExprBody::Float(f) => Float(f),
        expr::ExprBody::Int(i) => Int(i),
        expr::ExprBody::Identifier(id) => Identifier(id),
        expr::ExprBody::Symbol(s) => Symbol(s),
    }
}

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

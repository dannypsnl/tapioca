use crate::ast::expr::{self, Binding, ExprBody};
use std::collections::BTreeSet;

pub fn closure_conversion(e: expr::Expr) -> expr::Expr {
    let span = e.span();
    match e.body {
        expr::ExprBody::Lambda(params, expr) => {
            let freevars = free_variables(&expr);
            if freevars.is_empty() {
                ExprBody::Lambda(params, Box::new(closure_conversion(*expr))).with_span(span)
            } else {
                ExprBody::Closure(
                    Box::new(
                        ExprBody::Lambda(params, Box::new(replace_free(*expr, &freevars)))
                            .with_span(span.clone()),
                    ),
                    freevars,
                )
                .with_span(span)
            }
        }
        // dummy recursive
        expr::ExprBody::If(c, t, e) => ExprBody::If(
            Box::new(closure_conversion(*c)),
            Box::new(closure_conversion(*t)),
            Box::new(closure_conversion(*e)),
        )
        .with_span(span),
        expr::ExprBody::Begin(vec, expr) => ExprBody::Begin(
            vec.into_iter().map(closure_conversion).collect(),
            Box::new(closure_conversion(*expr)),
        )
        .with_span(span),
        expr::ExprBody::Let(binds, expr) => {
            let new_binds = binds
                .into_iter()
                .map(|bind| Binding {
                    name: bind.name,
                    typ: bind.typ,
                    expr: closure_conversion(bind.expr),
                })
                .collect();
            ExprBody::Let(new_binds, Box::new(closure_conversion(*expr))).with_span(span)
        }
        expr::ExprBody::App(func, args) => ExprBody::App(
            Box::new(closure_conversion(*func)),
            args.into_iter().map(closure_conversion).collect(),
        )
        .with_span(span),
        expr::ExprBody::List(exprs) => {
            ExprBody::List(exprs.into_iter().map(closure_conversion).collect()).with_span(span)
        }
        expr::ExprBody::Pair(expr1, expr2) => ExprBody::Pair(
            Box::new(closure_conversion(*expr1)),
            Box::new(closure_conversion(*expr2)),
        )
        .with_span(span),
        expr::ExprBody::Object(fields) => ExprBody::Object(
            fields
                .into_iter()
                .map(|(name, expr)| (name, closure_conversion(expr)))
                .collect(),
        )
        .with_span(span),
        expr::ExprBody::Syntax(expr) => {
            ExprBody::Syntax(Box::new(closure_conversion(*expr))).with_span(span)
        }
        _ => e,
    }
}

fn replace_free(e: expr::Expr, freevars: &BTreeSet<expr::Identifier>) -> expr::Expr {
    let span = e.span();
    match e.body {
        ExprBody::Lambda(_, _) => closure_conversion(e),
        ExprBody::Identifier(id) => {
            if freevars.contains(&id) {
                ExprBody::ClosureEnvGet(freevars.iter().position(|r| r == &id).unwrap())
                    .with_span(span)
            } else {
                ExprBody::Identifier(id.clone()).with_span(span)
            }
        }
        // dummy recursive
        expr::ExprBody::If(c, t, e) => ExprBody::If(
            Box::new(replace_free(*c, freevars)),
            Box::new(replace_free(*t, freevars)),
            Box::new(replace_free(*e, freevars)),
        )
        .with_span(span),
        expr::ExprBody::Begin(vec, expr) => ExprBody::Begin(
            vec.into_iter().map(|e| replace_free(e, freevars)).collect(),
            Box::new(replace_free(*expr, freevars)),
        )
        .with_span(span),
        expr::ExprBody::Let(binds, expr) => {
            let new_binds = binds
                .into_iter()
                .map(|bind| Binding {
                    name: bind.name,
                    typ: bind.typ,
                    expr: replace_free(bind.expr, freevars),
                })
                .collect();
            ExprBody::Let(new_binds, Box::new(replace_free(*expr, freevars))).with_span(span)
        }
        expr::ExprBody::App(func, args) => ExprBody::App(
            Box::new(replace_free(*func, freevars)),
            args.into_iter()
                .map(|e| replace_free(e, freevars))
                .collect(),
        )
        .with_span(span),
        expr::ExprBody::List(exprs) => ExprBody::List(
            exprs
                .into_iter()
                .map(|e| replace_free(e, freevars))
                .collect(),
        )
        .with_span(span),
        expr::ExprBody::Pair(expr1, expr2) => ExprBody::Pair(
            Box::new(replace_free(*expr1, freevars)),
            Box::new(replace_free(*expr2, freevars)),
        )
        .with_span(span),
        expr::ExprBody::Object(fields) => ExprBody::Object(
            fields
                .into_iter()
                .map(|(name, expr)| (name, replace_free(expr, freevars)))
                .collect(),
        )
        .with_span(span),
        expr::ExprBody::Syntax(expr) => {
            ExprBody::Syntax(Box::new(replace_free(*expr, freevars))).with_span(span)
        }
        _ => e,
    }
}

fn free_variables(e: &expr::Expr) -> BTreeSet<expr::Identifier> {
    match &e.body {
        expr::ExprBody::Identifier(identifier) => {
            let mut set = BTreeSet::default();
            set.insert(identifier.clone());
            set
        }

        expr::ExprBody::If(c, t, e) => {
            todo!()
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

        _ => panic!(
            "internal error, should need to compute free variables for {:?}",
            e
        ),
    }
}

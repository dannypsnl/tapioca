use crate::{
    ast::{
        DefineForm, Module,
        expr::{Expr, ExprBody, Identifier},
    },
    backend::tinyc::DefineFunc,
};

mod closure_conversion;

pub fn middle_passes(module: &Module) -> Module {
    let mut new_defs = module
        .define_forms
        .iter()
        .cloned()
        .map(|def| match def {
            DefineForm::DefineFunction {
                span,
                id,
                params,
                body,
            } => {
                let new_body = closure_conversion::closure_conversion(body.clone());
                DefineForm::DefineFunction {
                    span,
                    id,
                    params,
                    body: new_body,
                }
            }
            d => d,
        })
        .collect::<Vec<_>>();

    let mut lifted_defs = vec![];
    for def in &new_defs {
        match def {
            DefineForm::DefineFunction { body, .. } => lambda_lifting(&mut lifted_defs, body),
            _ => (),
        }
    }
    lifted_defs.append(&mut new_defs);

    let lifted_defs = lifted_defs
        .iter()
        .cloned()
        .map(|def| match def {
            DefineForm::DefineFunction {
                span,
                id,
                params,
                body,
            } => DefineForm::DefineFunction {
                span,
                id,
                params,
                body: remove_let(body),
            },
            d => d,
        })
        .collect();

    Module {
        source: module.source.clone(),
        requires: module.requires.clone(),
        claim_forms: module.claim_forms.clone(),
        define_forms: lifted_defs,
        other_forms: module.other_forms.clone(),
    }
}

fn remove_let(e: Expr) -> Expr {
    let span = e.span();
    match e.body {
        ExprBody::Begin(vec, expr) => ExprBody::Begin(
            vec.into_iter().map(remove_let).collect(),
            Box::new(remove_let(*expr)),
        )
        .with_span(span),
        ExprBody::Let(vec, expr) => ExprBody::Begin(
            vec.into_iter()
                .map(|bind| ExprBody::Set(bind.name, Box::new(bind.expr)).with_span(expr.span()))
                .collect(),
            Box::new(remove_let(*expr)),
        )
        .with_span(span),
        ExprBody::If(expr, expr1, expr2) => ExprBody::If(
            Box::new(remove_let(*expr)),
            Box::new(remove_let(*expr1)),
            Box::new(remove_let(*expr2)),
        )
        .with_span(span),
        ExprBody::Lambda(vec, expr) => {
            ExprBody::Lambda(vec, Box::new(remove_let(*expr))).with_span(span)
        }
        ExprBody::App(expr, vec) => ExprBody::App(
            Box::new(remove_let(*expr)),
            vec.into_iter().map(remove_let).collect(),
        )
        .with_span(span),
        ExprBody::List(vec) => {
            ExprBody::List(vec.into_iter().map(remove_let).collect()).with_span(span)
        }
        ExprBody::Pair(expr, expr1) => {
            ExprBody::Pair(Box::new(remove_let(*expr)), Box::new(remove_let(*expr1)))
                .with_span(span)
        }
        ExprBody::Object(vec) => ExprBody::Object(
            vec.into_iter()
                .map(|(id, expr)| (id, remove_let(expr)))
                .collect(),
        )
        .with_span(span),
        ExprBody::Syntax(expr) => ExprBody::Syntax(Box::new(remove_let(*expr))).with_span(span),
        ExprBody::Closure(expr, btree_set) => {
            ExprBody::Closure(Box::new(remove_let(*expr)), btree_set).with_span(span)
        }

        _ => e,
    }
}

fn lambda_lifting(defs: &mut Vec<DefineForm>, expr: &Expr) {
    match &expr.body {
        ExprBody::Set(..) => {
            panic!("internal error: lambda lifting should not face set! operation")
        }
        ExprBody::Begin(vec, expr) => {
            for expr in vec {
                lambda_lifting(defs, expr)
            }
            lambda_lifting(defs, expr);
        }
        ExprBody::Let(vec, expr) => {
            for bind in vec {
                lambda_lifting(defs, &bind.expr);
            }
            lambda_lifting(defs, expr);
        }
        ExprBody::If(expr, expr1, expr2) => {
            lambda_lifting(defs, expr);
            lambda_lifting(defs, expr1);
            lambda_lifting(defs, expr2);
        }
        ExprBody::Lambda(params, expr) => {
            defs.push(DefineForm::DefineFunction {
                span: expr.span(),
                id: Identifier::simple("lifted_lambda".to_string()),
                params: params.clone(),
                body: (**expr).clone(),
            });
            lambda_lifting(defs, expr);
        }
        ExprBody::App(expr, vec) => {
            lambda_lifting(defs, expr);
            for expr in vec {
                lambda_lifting(defs, expr)
            }
        }
        ExprBody::List(vec) => {
            for expr in vec {
                lambda_lifting(defs, expr)
            }
        }
        ExprBody::Pair(expr, expr1) => {
            lambda_lifting(defs, expr);
            lambda_lifting(defs, expr1);
        }
        ExprBody::Object(vec) => {
            for (_, expr) in vec {
                lambda_lifting(defs, expr)
            }
        }
        ExprBody::Syntax(expr) => lambda_lifting(defs, expr),
        ExprBody::Syntax(expr) => lambda_lifting(defs, expr),

        ExprBody::Closure(expr, _) => lambda_lifting(defs, expr),

        ExprBody::ClosureEnvGet(_)
        | ExprBody::Bool(_)
        | ExprBody::Char(_)
        | ExprBody::String(_)
        | ExprBody::Rational(_, _)
        | ExprBody::Float(_)
        | ExprBody::Int(_)
        | ExprBody::Identifier(_)
        | ExprBody::Symbol(_) => (),
    }
}

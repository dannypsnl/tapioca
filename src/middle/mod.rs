use crate::{
    ast::{
        DefineForm, Module,
        expr::{Expr, ExprBody},
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

    Module {
        source: module.source.clone(),
        requires: module.requires.clone(),
        claim_forms: module.claim_forms.clone(),
        define_forms: lifted_defs,
        other_forms: module.other_forms.clone(),
    }
}

fn lambda_lifting(defs: &mut Vec<DefineForm>, expr: &Expr) {
    match &expr.body {
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
        ExprBody::Lambda(_, expr) => {
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

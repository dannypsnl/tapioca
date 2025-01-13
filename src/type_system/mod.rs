use crate::ast::{DefineForm, Expr, Module, Typ};
use ariadne::{Report, ReportKind};

mod environment;

fn check_type(typ: &Typ, exp: &Expr) {
    match (typ, exp) {
        _ => todo!(),
    }
}

pub fn check(module: &Module) {
    let mut env = environment::Environment::new(&module.source);
    for claim in &module.claim_forms {
        env.insert(claim.id.clone(), claim.typ.clone());
    }
    for def in &module.define_forms {
        match def {
            DefineForm::DefineConstant { span, id, expr } => {
                check_type(env.lookup(id, span), expr);
            }
            DefineForm::DefineFunction {
                span,
                id,
                params,
                body,
            } => {
                let ty = env.lookup(id, span);
                match ty {
                    Typ::Func {
                        params: typs,
                        result,
                    } => {
                        let mut env = env.derive();
                        for (id, typ) in params.into_iter().zip(typs) {
                            env.insert(id.clone(), typ.clone());
                        }
                        let taken = body.len() - 1;
                        let it = body.iter();
                        for middle_statement in it.clone().take(taken) {
                            check_type(&Typ::Void, middle_statement);
                        }
                        check_type(&*result, it.last().unwrap());
                    }
                    _ => {
                        Report::build(ReportKind::Error, span.clone())
                            .with_code(3)
                            .with_message(format!(
                                "expected `{}` has a function type, but got: `{}`",
                                id.clone(),
                                ty
                            ))
                            .finish()
                            .eprint(module.source.clone())
                            .unwrap();
                    }
                }
            }
        }
    }
}

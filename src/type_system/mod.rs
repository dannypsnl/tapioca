use crate::ast::{DefineForm, Module, typ::TypBody};
use ariadne::{Report, ReportKind};
use environment::Environment;

pub mod environment;

pub fn check(module: &Module) -> Environment {
    let mut env = environment::Environment::new(&module);
    for claim in &module.claim_forms {
        env.insert(claim.id.clone(), claim.typ.clone());
    }
    for def in &module.define_forms {
        match def {
            DefineForm::DefineConstant { span, id, expr } => {
                env.check(span, expr, env.lookup(id, span));
            }
            DefineForm::DefineFunction {
                span,
                id,
                params,
                body,
            } => {
                let ty = env.lookup(id, span);
                match &ty.body {
                    TypBody::Func {
                        params: typs,
                        result,
                    } => {
                        let mut env = env.derive();
                        for (id, typ) in params.into_iter().zip(typs) {
                            env.insert(id.clone(), typ.clone());
                        }
                        env.check(span, body, &*result);
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
                            .eprint(module.clone())
                            .unwrap();
                    }
                }
            }
        }
    }

    env
}

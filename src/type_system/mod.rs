use crate::ast::{DefineForm, Module, Typ};
use ariadne::{Report, ReportKind};

mod environment;

pub fn check(module: &Module) {
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
                            env.check(span, middle_statement, &Typ::Void);
                        }
                        env.check(span, it.last().unwrap(), &*result);
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
}

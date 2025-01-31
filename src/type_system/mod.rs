use crate::ast::{DefineForm, Module, ReportSpan, expr::Identifier, typ::TypBody};
use ariadne::{Report, ReportKind};
use environment::Environment;
use std::collections::BTreeSet;

pub mod environment;

pub fn check(module: &Module) -> Environment {
    let mut top_definitions: BTreeSet<Identifier> = Default::default();

    let mut env = environment::Environment::new(&module);
    for claim in &module.claim_forms {
        env.insert(claim.id.clone(), claim.typ.clone());
    }
    for def in &module.define_forms {
        match def {
            DefineForm::DefineConstant { span, id, expr } => {
                check_redefine(&mut top_definitions, id, span, &module);

                env.check(span, expr, env.lookup(id, span));
            }
            DefineForm::DefineFunction {
                span,
                id,
                params,
                body,
            } => {
                check_redefine(&mut top_definitions, id, span, &module);

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
                                id.info_name(),
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

fn check_redefine(
    top_definitions: &mut BTreeSet<Identifier>,
    id: &Identifier,
    span: &ReportSpan,
    module: &Module,
) {
    if top_definitions.contains(id) {
        Report::build(ReportKind::Error, span.clone())
            .with_code(1)
            .with_message(format!("`{}` is redefined", id.info_name()))
            .finish()
            .eprint(module.clone())
            .unwrap();
        panic!();
    }

    top_definitions.insert(id.clone());
}

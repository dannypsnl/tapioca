use crate::ast::{DefineForm, Module};

mod closure_conversion;

pub fn middle_passes(module: &Module) -> Module {
    let new_defs = module
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

    Module {
        source: module.source.clone(),
        requires: module.requires.clone(),
        claim_forms: module.claim_forms.clone(),
        define_forms: new_defs,
        other_forms: module.other_forms.clone(),
    }
}

use crate::ast::*;
use crate::error;
use crate::matcher::{EPattern::*, ematch};
use ariadne::{Color, Fmt, Report, ReportKind, Source};
use enotation::{EFile, ENotation, ENotationParser, Rule};
use from_pest::FromPest;
use pest::Parser;
use std::collections::BTreeMap;

pub fn expand_module(input: &str) -> Result<Module, error::Error> {
    let mut module = Module {
        source: Source::from(input),
        claim_forms: vec![],
        define_forms: vec![],
        other_forms: vec![],
        requires: vec![],
    };

    let mut output = ENotationParser::parse(Rule::file, input).unwrap();
    let efile = EFile::from_pest(&mut output)?;
    for notation in efile.notations {
        expand_top_level(&mut module, notation);
    }

    Ok(module)
}

fn expand_top_level(module: &mut Module, notation: ENotation) {
    let mut binds = BTreeMap::new();
    if ematch(
        &mut binds,
        &notation,
        List(vec![Id("define"), RestHole("rest")]),
    ) {
        expand_defines(module, &notation);
    } else if ematch(&mut binds, &notation, List(vec![Id(":"), RestHole("rest")])) {
        expand_claims(module, &notation);
    } else if ematch(
        &mut binds,
        &notation,
        List(vec![Id("require"), Hole("module")]),
    ) {
        let m = binds.get("module").unwrap();
        module.requires.push(Require {
            module: format!("{}", m),
        });
    } else if ematch(
        &mut binds,
        &notation,
        List(vec![Id("require"), RestHole("_")]),
    ) {
        let out = Color::Fixed(81);
        Report::build(ReportKind::Error, ReportSpan::new(notation.span))
            .with_code(3)
            .with_message("bad require")
            .with_note(format!("{} form must ……", "match".fg(out)))
            .finish()
            .eprint(module.source.clone())
            .unwrap();
    } else {
        Report::build(ReportKind::Error, ReportSpan::new(notation.span))
            .with_message("unhandled form")
            .finish()
            .print(module.source.clone())
            .unwrap();
    }
}

fn expand_claims(module: &mut Module, notation: &ENotation) {
    let mut binds = BTreeMap::new();
    if ematch(
        &mut binds,
        &notation,
        List(vec![Id(":"), Hole("name"), Id(":"), Hole("typ")]),
    ) {
        module.claim_forms.push(ClaimForm {
            id: binds.get("name").unwrap().to_string(),
            typ: binds.get("typ").unwrap().clone(),
        })
    }
}

fn expand_defines(module: &mut Module, notation: &ENotation) {
    let mut binds = BTreeMap::new();
    if ematch(
        &mut binds,
        &notation,
        List(vec![Id("define"), Hole("name"), Hole("expr")]),
    ) {
        module.define_forms.push(DefineForm::DefineConstant {
            id: binds.get("name").unwrap().to_string(),
            expr: binds.get("expr").unwrap().clone(),
        });
    }
}

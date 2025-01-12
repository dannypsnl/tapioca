use super::EPattern::*;
use super::ematch;
use enotation::{ENotation, ENotationParser, Rule};
use from_pest::FromPest;
use insta::assert_snapshot;
use pest::Parser;
use std::collections::BTreeMap;
use std::fmt::Display;

fn notation(input: &str) -> ENotation {
    let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
    ENotation::from_pest(&mut output).unwrap()
}

struct DisplayVecENotation(Vec<(String, ENotation)>);

impl From<BTreeMap<String, ENotation>> for DisplayVecENotation {
    fn from(map: BTreeMap<String, ENotation>) -> Self {
        DisplayVecENotation(map.into_iter().collect())
    }
}
impl Display for DisplayVecENotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (binder, ele) in &self.0 {
            writeln!(f, "{} = {}", binder, ele)?;
        }
        Ok(())
    }
}

#[test]
fn match_define_form() {
    let mut binds = BTreeMap::new();
    let res = ematch(
        &mut binds,
        notation("(define x 1)"),
        List(vec![Id("define"), Hole("name"), Hole("expr")]),
    );
    assert_eq!(res, true);
    assert_snapshot!(Into::<DisplayVecENotation>::into(binds), @r"
    expr = 1
    name = x
    ");
}

#[test]
fn match_define_form_2() {
    let mut binds = BTreeMap::new();
    let res = ematch(
        &mut binds,
        notation("(define x : i32 1)"),
        List(vec![
            Id("define"),
            Hole("name"),
            Id(":"),
            Hole("type"),
            Hole("expr"),
        ]),
    );
    assert_eq!(res, true);
    assert_snapshot!(Into::<DisplayVecENotation>::into(binds), @r"
    expr = 1
    name = x
    type = i32
    ");
}

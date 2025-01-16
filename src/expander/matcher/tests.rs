use super::EPattern::*;
use super::Matched;
use super::MatchedResult;
use super::ematch;
use enotation::{ENotation, ENotationParser, Rule};
use from_pest::FromPest;
use insta::assert_snapshot;
use pest::Parser;
use std::fmt::Display;

fn notation(input: &str) -> ENotation {
    let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
    ENotation::from_pest(&mut output).unwrap()
}

struct DisplayVecENotation(Vec<(String, Matched)>);

impl From<MatchedResult> for DisplayVecENotation {
    fn from(result: MatchedResult) -> Self {
        DisplayVecENotation(result.binds.into_iter().collect())
    }
}
impl Display for DisplayVecENotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (binder, ele) in &self.0 {
            match ele {
                Matched::One(enotation) => writeln!(f, "{} = {}", binder, enotation)?,
                Matched::Many(vec) => {
                    write!(f, "{} = (", binder)?;
                    for (i, ele) in vec.into_iter().enumerate() {
                        if i == 0 {
                            write!(f, "{}", ele)?;
                        } else {
                            write!(f, " {}", ele)?;
                        }
                    }
                    writeln!(f, ")")?;
                }
            }
        }
        Ok(())
    }
}

#[test]
fn match_define_form() {
    let mut binds = MatchedResult::default();
    let res = ematch(
        &mut binds,
        &notation("(define x 1)"),
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
    let mut binds = MatchedResult::default();
    let res = ematch(
        &mut binds,
        &notation("(define x : i32 1)"),
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

#[test]
fn match_define_form_3() {
    let mut binds = MatchedResult::default();
    let res = ematch(
        &mut binds,
        &notation("(define x : i32 1)"),
        List(vec![Id("define"), RestHole("rest")]),
    );
    assert_eq!(res, true);
    assert_snapshot!(Into::<DisplayVecENotation>::into(binds), @"rest = (x : i32 1)");
}

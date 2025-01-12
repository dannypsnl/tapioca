use super::EPattern::*;
use super::ematch;
use enotation::{ENotation, ENotationParser, Rule};
use from_pest::FromPest;
use insta::assert_debug_snapshot;
use insta::{assert_snapshot, assert_yaml_snapshot};
use pest::Parser;
use std::collections::HashMap;

fn notation(input: &str) -> ENotation {
    let mut output = ENotationParser::parse(Rule::notation, input).unwrap();
    ENotation::from_pest(&mut output).unwrap()
}

#[test]
fn match_define_form() {
    let mut binds = HashMap::new();
    let res = ematch(
        &mut binds,
        notation("(define x 1)"),
        List(vec![Id("define"), Hole("name"), Hole("expr")]),
    );
    assert_eq!(res, true);
    assert_debug_snapshot!(binds.into_iter().collect::<Vec<_>>(), @r#"
    [
        (
            "name",
            ENotation {
                span: DiagnosticSpan {
                    start_line: 1,
                    start_col: 9,
                    start_offset: 8,
                    end_line: 1,
                    end_col: 10,
                    end_offset: 9,
                    span: "x",
                },
                body: Literal(
                    Identifier(
                        Identifier {
                            name: "x",
                        },
                    ),
                ),
            },
        ),
        (
            "expr",
            ENotation {
                span: DiagnosticSpan {
                    start_line: 1,
                    start_col: 11,
                    start_offset: 10,
                    end_line: 1,
                    end_col: 12,
                    end_offset: 11,
                    span: "1",
                },
                body: Literal(
                    Int(
                        Integer {
                            value: 1,
                        },
                    ),
                ),
            },
        ),
    ]
    "#);
}

#[test]
fn match_define_form_2() {
    let mut binds = HashMap::new();
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
    assert_debug_snapshot!(binds.into_iter().collect::<Vec<_>>(), @r#"
    [
        (
            "expr",
            ENotation {
                span: DiagnosticSpan {
                    start_line: 1,
                    start_col: 17,
                    start_offset: 16,
                    end_line: 1,
                    end_col: 18,
                    end_offset: 17,
                    span: "1",
                },
                body: Literal(
                    Int(
                        Integer {
                            value: 1,
                        },
                    ),
                ),
            },
        ),
        (
            "name",
            ENotation {
                span: DiagnosticSpan {
                    start_line: 1,
                    start_col: 9,
                    start_offset: 8,
                    end_line: 1,
                    end_col: 10,
                    end_offset: 9,
                    span: "x",
                },
                body: Literal(
                    Identifier(
                        Identifier {
                            name: "x",
                        },
                    ),
                ),
            },
        ),
        (
            "type",
            ENotation {
                span: DiagnosticSpan {
                    start_line: 1,
                    start_col: 13,
                    start_offset: 12,
                    end_line: 1,
                    end_col: 16,
                    end_offset: 15,
                    span: "i32",
                },
                body: Literal(
                    Identifier(
                        Identifier {
                            name: "i32",
                        },
                    ),
                ),
            },
        ),
    ]
    "#);
}

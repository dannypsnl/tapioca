use crate::ast::*;
use crate::error;
use crate::matcher::{EPattern::*, ematch};
use ariadne::{Color, Fmt, Report, ReportKind, Source};
use enotation::ENotationBody;
use enotation::container::Container;
use enotation::literal::Literal;
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
        module.expand_top_level(notation)?;
    }

    Ok(module)
}

impl<'a> Module<'a> {
    fn expand_top_level(&mut self, notation: ENotation) -> Result<(), error::Error> {
        let mut binds = BTreeMap::new();
        if ematch(
            &mut binds,
            &notation,
            List(vec![Id("define"), RestHole("rest")]),
        ) {
            self.expand_defines(&notation)?;
        } else if ematch(&mut binds, &notation, List(vec![Id(":"), RestHole("rest")])) {
            self.expand_claims(&notation)?;
        } else if ematch(
            &mut binds,
            &notation,
            List(vec![Id("require"), Hole("module")]),
        ) {
            let m = binds.get("module").unwrap();
            self.requires.push(Require {
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
                .eprint(self.source.clone())
                .unwrap();
        } else {
            Report::build(ReportKind::Error, ReportSpan::new(notation.span))
                .with_message("unhandled form")
                .finish()
                .print(self.source.clone())
                .unwrap();
        }

        Ok(())
    }

    fn expand_claims(&mut self, notation: &ENotation) -> Result<(), error::Error> {
        let mut binds = BTreeMap::new();
        if ematch(
            &mut binds,
            &notation,
            List(vec![Id(":"), Hole("name"), Id(":"), Hole("typ")]),
        ) {
            let exp = self.expand_type(binds.get("typ").unwrap())?;
            self.claim_forms.push(ClaimForm {
                id: binds.get("name").unwrap().to_string(),
                typ: exp,
            })
        }

        Ok(())
    }

    fn expand_defines(&mut self, notation: &ENotation) -> Result<(), error::Error> {
        let mut binds = BTreeMap::new();
        if ematch(
            &mut binds,
            &notation,
            List(vec![Id("define"), Hole("name"), Hole("expr")]),
        ) {
            let typ = self.expand_expr(binds.get("expr").unwrap())?;
            self.define_forms.push(DefineForm::DefineConstant {
                id: binds.get("name").unwrap().to_string(),
                expr: typ,
            });
        }

        Ok(())
    }

    fn expand_type(&mut self, notation: &ENotation) -> Result<Typ, error::Error> {
        match &notation.body {
            ENotationBody::Literal(Literal::Identifier(id)) => match id.name.as_str() {
                "boolean" => Ok(Typ::Bool),
                "char" => Ok(Typ::Char),
                "string" => Ok(Typ::String),
                "rational" => Ok(Typ::Rational),
                "float" => Ok(Typ::Float),
                "int" => Ok(Typ::Int),
                "i8" => Ok(Typ::I8),
                "i16" => Ok(Typ::I16),
                "i32" => Ok(Typ::I32),
                "i64" => Ok(Typ::I64),
                "u8" => Ok(Typ::U8),
                "u16" => Ok(Typ::U16),
                "u32" => Ok(Typ::U32),
                "u64" => Ok(Typ::U64),
                "syntax" => Ok(Typ::Syntax),
                // unknown type
                _ => todo!(),
            },
            ENotationBody::Container(Container::List(list)) => {
                let mut ts = list.elems().into_iter();
                let head = ts.next().unwrap();
                if ematch(&mut BTreeMap::default(), head, Id("array")) {
                    let t = self.expand_type(ts.next().unwrap())?;
                    Ok(Typ::Array(t.into()))
                } else if ematch(&mut BTreeMap::default(), head, Id("list")) {
                    let t = self.expand_type(ts.next().unwrap())?;
                    Ok(Typ::Array(t.into()))
                } else if ematch(&mut BTreeMap::default(), head, Id("tuple")) {
                    let mut xs = vec![];
                    for t in ts {
                        xs.push(self.expand_type(t)?);
                    }
                    Ok(Typ::Tuple(xs))
                } else {
                    todo!()
                }
            }
            ENotationBody::Container(Container::Object(obj)) => {
                todo!()
            }
            _ => {
                self.bad_form(notation);
                todo!()
            }
        }
    }

    fn expand_expr(&mut self, notation: &ENotation) -> Result<Expr, error::Error> {
        match &notation.body {
            enotation::ENotationBody::Literal(literal) => match literal {
                Literal::Boolean(boolean) => Ok(Expr::Bool(boolean.value)),
                Literal::Char(c) => Ok(Expr::Char(c.value)),
                Literal::Float(float) => Ok(Expr::Float(float.value)),
                Literal::Rational(rational) => {
                    Ok(Expr::Rational(rational.value.0, rational.value.1))
                }
                Literal::Int(integer) => Ok(Expr::Int(integer.value)),
                Literal::String_(string) => Ok(Expr::String(string.value.clone())),
                Literal::Identifier(identifier) => Ok(Expr::Identifier(identifier.name.clone())),
            },
            enotation::ENotationBody::Container(container) => todo!(),
            enotation::ENotationBody::Quoting(quoting) => todo!(),
            enotation::ENotationBody::Syntaxing(syntaxing) => todo!(),
        }
    }

    fn bad_form(&mut self, notation: &ENotation) {
        let out = Color::Fixed(81);
        Report::build(ReportKind::Error, ReportSpan::new(notation.span.clone()))
            .with_code(3)
            .with_message("bad form")
            .with_note(format!("{} form must ……", "match".fg(out)))
            .finish()
            .eprint(self.source.clone())
            .unwrap();
    }
}

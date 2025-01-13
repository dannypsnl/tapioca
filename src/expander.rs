use crate::ast::{
    expr::{Expr, ExprBody},
    typ::{Typ, TypBody},
    *,
};
use crate::matcher::{EPattern::*, MatchedResult, ematch};
use crate::{error, matcher};
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use enotation::{
    EFile, ENotation, ENotationBody, ENotationParser, Rule, SetDebugFileName, container::Container,
    literal::Literal,
};
use from_pest::FromPest;
use pest::Parser;
use std::path::Path;

pub fn expand_module(filename: &str) -> Result<Module, error::Error> {
    let path: &Path = Path::new(filename);
    let input = std::fs::read_to_string(path).expect("failed to open file");

    let mut output = ENotationParser::parse(Rule::file, input.as_str()).unwrap();
    let mut efile = EFile::from_pest(&mut output)?;
    efile.set_debug_file_name(filename);

    let mut module = Module {
        source: (filename.to_string(), Source::from(input)),
        claim_forms: vec![],
        define_forms: vec![],
        other_forms: vec![],
        requires: vec![],
    };

    for notation in efile.notations {
        module.expand_top_level(notation)?;
    }

    Ok(module)
}

impl Module {
    fn expand_top_level(&mut self, notation: ENotation) -> Result<(), error::Error> {
        let mut binds = MatchedResult::default();
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
            List(vec![Id("require"), RestHole("module")]),
        ) {
            let requires = binds.get_many("module");
            for r in requires {
                if matcher::is_identifier(r) {
                    self.requires.push(Require {
                        module: format!("{}", r),
                    });
                } else {
                    let span: ReportSpan = r.span.clone().into();
                    Report::build(ReportKind::Error, span.clone())
                        .with_code(3)
                        .with_message("bad require")
                        .with_label(Label::new(span.clone()).with_message("Not an identifier"))
                        .finish()
                        .eprint(self.clone())
                        .unwrap();
                }
            }
        } else {
            self.other_forms.push(notation);
        }

        Ok(())
    }

    fn expand_claims(&mut self, notation: &ENotation) -> Result<(), error::Error> {
        let mut binds = MatchedResult::default();
        if ematch(
            &mut binds,
            &notation,
            List(vec![Id(":"), Hole("name"), Id(":"), Hole("typ")]),
        ) {
            let exp = self.expand_type(binds.get_one("typ"))?;
            self.claim_forms.push(ClaimForm {
                id: binds.get_one("name").to_string(),
                typ: exp,
            })
        }

        Ok(())
    }

    fn expand_defines(&mut self, notation: &ENotation) -> Result<(), error::Error> {
        let mut binds = MatchedResult::default();
        if ematch(
            &mut binds,
            &notation,
            List(vec![Id("define"), Hole("name"), Hole("expr")]),
        ) {
            let typ = self.expand_expr(binds.get_one("expr"))?;
            self.define_forms.push(DefineForm::DefineConstant {
                span: notation.span.clone().into(),
                id: binds.get_one("name").to_string(),
                expr: typ,
            });
        }

        Ok(())
    }

    fn expand_type(&mut self, notation: &ENotation) -> Result<Typ, error::Error> {
        let span: ReportSpan = notation.span.clone().into();
        match &notation.body {
            ENotationBody::Literal(Literal::Identifier(id)) => match id.name.as_str() {
                "boolean" => Ok(TypBody::Bool.with_span(span)),
                "char" => Ok(TypBody::Char.with_span(span)),
                "string" => Ok(TypBody::String.with_span(span)),
                "symbol" => Ok(TypBody::Symbol.with_span(span)),
                "rational" => Ok(TypBody::Rational.with_span(span)),
                "float" => Ok(TypBody::Float.with_span(span)),
                "int" => Ok(TypBody::Int.with_span(span)),
                "i8" => Ok(TypBody::I8.with_span(span)),
                "i16" => Ok(TypBody::I16.with_span(span)),
                "i32" => Ok(TypBody::I32.with_span(span)),
                "i64" => Ok(TypBody::I64.with_span(span)),
                "u8" => Ok(TypBody::U8.with_span(span)),
                "u16" => Ok(TypBody::U16.with_span(span)),
                "u32" => Ok(TypBody::U32.with_span(span)),
                "u64" => Ok(TypBody::U64.with_span(span)),
                "syntax" => Ok(TypBody::Syntax.with_span(span)),
                // unknown type
                _ => todo!(),
            },
            ENotationBody::Container(Container::List(list)) => {
                let mut ts = list.elems().into_iter();
                let head = ts.next().unwrap();
                if ematch(&mut MatchedResult::default(), head, Id("array")) {
                    let t = self.expand_type(ts.next().unwrap())?;
                    Ok(TypBody::Array(t.into()).with_span(span))
                } else if ematch(&mut MatchedResult::default(), head, Id("list")) {
                    let t = self.expand_type(ts.next().unwrap())?;
                    Ok(TypBody::Array(t.into()).with_span(span))
                } else if ematch(&mut MatchedResult::default(), head, Id("tuple")) {
                    let mut xs = vec![];
                    for t in ts {
                        xs.push(self.expand_type(t)?);
                    }
                    Ok(TypBody::Tuple(xs).with_span(span))
                } else {
                    todo!()
                }
            }
            ENotationBody::Container(Container::Object(obj)) => {
                let mut fields = vec![];
                for pair in &obj.pairs {
                    fields.push((pair.key.name.clone(), self.expand_type(&pair.value)?))
                }
                Ok(TypBody::Record(fields).with_span(span))
            }
            _ => {
                self.bad_form(notation);
                todo!()
            }
        }
    }

    fn expand_expr(&mut self, notation: &ENotation) -> Result<Expr, error::Error> {
        let span: ReportSpan = notation.span.clone().into();
        match &notation.body {
            enotation::ENotationBody::Literal(literal) => match literal {
                Literal::Boolean(boolean) => Ok(ExprBody::Bool(boolean.value).with_span(span)),
                Literal::Char(c) => Ok(ExprBody::Char(c.value).with_span(span)),
                Literal::Float(float) => Ok(ExprBody::Float(float.value).with_span(span)),
                Literal::Rational(rational) => {
                    Ok(ExprBody::Rational(rational.value.0, rational.value.1).with_span(span))
                }
                Literal::Int(integer) => Ok(ExprBody::Int(integer.value).with_span(span)),
                Literal::String_(string) => {
                    Ok(ExprBody::String(string.value.clone()).with_span(span))
                }
                Literal::Identifier(identifier) => {
                    Ok(ExprBody::Identifier(identifier.name.clone()).with_span(span))
                }
            },
            _ => todo!(),
        }
    }

    fn bad_form(&mut self, notation: &ENotation) {
        let out = Color::Fixed(81);
        let span: ReportSpan = notation.span.clone().into();
        Report::build(ReportKind::Error, span)
            .with_code(3)
            .with_message("bad form")
            .with_note(format!("{} form must ……", "match".fg(out)))
            .finish()
            .eprint(self.clone())
            .unwrap();
    }
}

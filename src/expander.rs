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
use std::{path::Path, vec};

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
            List(vec![Id(":"), Hole("name"), Id(":"), RestHole("typ")]),
        ) {
            let typ = self.expand_type(notation.span.clone().into(), binds.get_many("typ"));
            println!("{}", typ);
            self.claim_forms.push(ClaimForm {
                id: binds.get_one("name").to_string(),
                typ,
            })
        }

        Ok(())
    }

    fn expand_defines(&mut self, notation: &ENotation) -> Result<(), error::Error> {
        let mut binds = MatchedResult::default();
        if ematch(
            &mut binds,
            &notation,
            List(vec![
                Id("define"),
                List(vec![Hole("name"), RestHole("params")]),
                RestHole("body"),
            ]),
        ) {
            let name = binds.get_one("name");

            let mut params = vec![];
            for p in binds.get_many("params") {
                params.push(p.to_string());
            }

            let mut body = vec![];
            for p in binds.get_many("body") {
                body.push(self.expand_expr(p)?);
            }

            self.define_forms.push(DefineForm::DefineFunction {
                span: notation.span.clone().into(),
                id: name.to_string(),
                params,
                body,
            });
        } else if ematch(
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
        } else {
            let span: ReportSpan = notation.span.clone().into();
            Report::build(ReportKind::Error, span.clone())
                .with_code(3)
                .with_message("bad define")
                .with_label(Label::new(span.clone()))
                .finish()
                .eprint(self.clone())
                .unwrap();
        }

        Ok(())
    }

    fn expand_type(&mut self, span: ReportSpan, ns: &Vec<ENotation>) -> Typ {
        if ns.len() == 0 {
            Report::build(ReportKind::Error, span.clone())
                .finish()
                .eprint(self.clone())
                .unwrap();
            panic!()
        } else if ns.len() == 1 {
            self.expand_one_type(&ns[0])
        } else {
            let mut stack = vec![];
            let mut ns = ns.into_iter();
            while let Some(n) = ns.next() {
                if ematch(&mut MatchedResult::default(), n, Id("->")) {
                    return TypBody::Func {
                        params: stack,
                        result: self
                            .expand_type(span.clone(), &ns.cloned().collect())
                            .into(),
                    }
                    .with_span(span.clone());
                } else {
                    stack.push(self.expand_one_type(n));
                }
            }
            unreachable!()
        }
    }

    fn expand_one_type(&mut self, notation: &ENotation) -> Typ {
        let span: ReportSpan = notation.span.clone().into();
        match &notation.body {
            ENotationBody::Literal(Literal::Identifier(id)) => match id.name.as_str() {
                "boolean" => TypBody::Bool.with_span(span),
                "char" => TypBody::Char.with_span(span),
                "string" => TypBody::String.with_span(span),
                "symbol" => TypBody::Symbol.with_span(span),
                "rational" => TypBody::Rational.with_span(span),
                "float" => TypBody::Float.with_span(span),
                "int" => TypBody::Int.with_span(span),
                "i8" => TypBody::I8.with_span(span),
                "i16" => TypBody::I16.with_span(span),
                "i32" => TypBody::I32.with_span(span),
                "i64" => TypBody::I64.with_span(span),
                "u8" => TypBody::U8.with_span(span),
                "u16" => TypBody::U16.with_span(span),
                "u32" => TypBody::U32.with_span(span),
                "u64" => TypBody::U64.with_span(span),
                "syntax" => TypBody::Syntax.with_span(span),
                // unknown type
                _ => todo!(),
            },
            ENotationBody::Container(Container::List(list)) => {
                let mut ts = list.elems().into_iter();
                let head = ts.next().unwrap();
                if ematch(&mut MatchedResult::default(), head, Id("array")) {
                    let t = self.expand_one_type(ts.next().unwrap());
                    TypBody::Array(t.into()).with_span(span)
                } else if ematch(&mut MatchedResult::default(), head, Id("list")) {
                    let t = self.expand_one_type(ts.next().unwrap());
                    TypBody::Array(t.into()).with_span(span)
                } else if ematch(&mut MatchedResult::default(), head, Id("tuple")) {
                    let mut xs = vec![];
                    for t in ts {
                        xs.push(self.expand_one_type(t));
                    }
                    TypBody::Tuple(xs).with_span(span)
                } else {
                    todo!()
                }
            }
            ENotationBody::Container(Container::Object(obj)) => {
                let mut fields = vec![];
                for pair in &obj.pairs {
                    fields.push((pair.key.name.clone(), self.expand_one_type(&pair.value)))
                }
                TypBody::Record(fields).with_span(span)
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

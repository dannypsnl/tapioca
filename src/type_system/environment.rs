use crate::ast::{
    Expr, Module, ReportSpan,
    typ::{Typ, TypBody},
};
use ariadne::{Label, Report, ReportKind};
use std::collections::BTreeMap;

pub struct Environment<'a> {
    source: &'a Module,
    current_scope: BTreeMap<String, Typ>,
    parent: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn unify(&self, span: &ReportSpan, expected: &Typ, actual: &Typ) {
        if expected != actual {
            Report::build(ReportKind::Error, span.clone())
                .with_code(3)
                .with_message("type mismatch")
                .with_label(
                    Label::new(span.clone())
                        .with_message(format!("expected `{}`, found `{}`", expected, actual)),
                )
                .finish()
                .eprint(self.source.clone())
                .unwrap();
        }
    }

    pub fn check(&self, span: &ReportSpan, exp: &Expr, typ: &Typ) {
        use TypBody::*;
        match (&typ.body, exp) {
            (Int, Expr::Int(_)) => (),
            (I8, Expr::Int(_)) => (),
            (I16, Expr::Int(_)) => (),
            (I32, Expr::Int(_)) => (),
            (I64, Expr::Int(_)) => (),
            (U8, Expr::Int(_)) => (),
            (U16, Expr::Int(_)) => (),
            (U32, Expr::Int(_)) => (),
            (U64, Expr::Int(_)) => (),

            (Rational, Expr::Rational(_, _)) => (),
            (Float, Expr::Float(_)) => (),

            (Bool, Expr::Bool(_)) => (),

            (Char, Expr::Char(_)) => (),
            (String, Expr::String(_)) => (),

            (_, Expr::Identifier(id)) => {
                let actual = self.lookup(id, span);
                self.unify(span, typ, actual);
            }

            (_, exp) => {
                let actual = self.infer(span, exp);
                self.unify(span, typ, &actual);
            }
        }
    }

    pub fn infer(&self, span: &ReportSpan, exp: &Expr) -> Typ {
        use TypBody::*;
        // FIXME: The correct span here is the span of Expr, but currently I do that wrong so don't have
        match exp {
            Expr::Bool(_) => Bool.with_span(span.clone()),
            Expr::Char(_) => Char.with_span(span.clone()),
            Expr::String(_) => String.with_span(span.clone()),
            Expr::Rational(_, _) => Rational.with_span(span.clone()),
            Expr::Float(_) => Float.with_span(span.clone()),
            Expr::Int(_) => Int.with_span(span.clone()),
            Expr::Symbol(_) => Symbol.with_span(span.clone()),
            Expr::Syntax(_) => Syntax.with_span(span.clone()),
            Expr::Identifier(id) => self.lookup(id, span).clone(),
            Expr::Tuple(vec) => {
                Tuple(vec.iter().map(|e| self.infer(span, e)).collect()).with_span(span.clone())
            }
            _ => todo!(),
        }
    }
}

impl<'a> Environment<'a> {
    pub fn derive(&self) -> Environment {
        let mut derived = Environment::new(self.source);
        derived.parent = Some(self);
        derived
    }
    pub fn insert(&mut self, id: String, typ: Typ) {
        self.current_scope.insert(id, typ);
    }
    pub fn lookup(&self, id: &String, span: &ReportSpan) -> &Typ {
        match self.current_scope.get(id) {
            Some(ty) => ty,
            None => match self.parent {
                Some(parent) => parent.lookup(id, span),
                None => {
                    Report::build(ReportKind::Error, span.clone())
                        .with_code(3)
                        .with_message(format!("`{}` has no type", id.clone()))
                        .finish()
                        .eprint(self.source.clone())
                        .unwrap();
                    unreachable!()
                }
            },
        }
    }

    pub fn new(source: &'a Module) -> Self {
        Self {
            source,
            current_scope: BTreeMap::new(),
            parent: None,
        }
    }
}

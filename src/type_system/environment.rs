use crate::ast::{
    Module, ReportSpan,
    expr::{Expr, ExprBody},
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
                .with_label(Label::new(expected.span.clone()).with_message("expected type"))
                .with_label(Label::new(actual.span.clone()).with_message("actual type"))
                .with_note(format!("expected `{}`, found `{}`", expected, actual))
                .finish()
                .eprint(self.source.clone())
                .unwrap();
        }
    }

    pub fn check(&self, span: &ReportSpan, exp: &Expr, typ: &Typ) {
        use TypBody::*;
        match (&typ.body, &exp.body) {
            (Int, ExprBody::Int(_)) => (),
            (I8, ExprBody::Int(_)) => (),
            (I16, ExprBody::Int(_)) => (),
            (I32, ExprBody::Int(_)) => (),
            (I64, ExprBody::Int(_)) => (),
            (U8, ExprBody::Int(_)) => (),
            (U16, ExprBody::Int(_)) => (),
            (U32, ExprBody::Int(_)) => (),
            (U64, ExprBody::Int(_)) => (),

            (Rational, ExprBody::Rational(_, _)) => (),
            (Float, ExprBody::Float(_)) => (),

            (Bool, ExprBody::Bool(_)) => (),

            (Char, ExprBody::Char(_)) => (),
            (String, ExprBody::String(_)) => (),

            (_, ExprBody::Identifier(id)) => {
                let actual = self.lookup(id, span);
                self.unify(span, typ, actual);
            }

            (_, _) => {
                let actual = self.infer(span, exp);
                self.unify(span, typ, &actual);
            }
        }
    }

    pub fn infer(&self, span: &ReportSpan, exp: &Expr) -> Typ {
        use TypBody::*;
        let typ_span = exp.span.clone();
        match &exp.body {
            ExprBody::Bool(_) => Bool.with_span(typ_span.clone()),
            ExprBody::Char(_) => Char.with_span(typ_span.clone()),
            ExprBody::String(_) => String.with_span(typ_span.clone()),
            ExprBody::Rational(_, _) => Rational.with_span(typ_span.clone()),
            ExprBody::Float(_) => Float.with_span(typ_span.clone()),
            ExprBody::Int(_) => Int.with_span(typ_span.clone()),
            ExprBody::Symbol(_) => Symbol.with_span(typ_span.clone()),
            ExprBody::Syntax(_) => Syntax.with_span(typ_span.clone()),
            ExprBody::Identifier(id) => self.lookup(id, span).clone(),
            ExprBody::Tuple(vec) => {
                Tuple(vec.iter().map(|e| self.infer(span, e)).collect()).with_span(typ_span.clone())
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

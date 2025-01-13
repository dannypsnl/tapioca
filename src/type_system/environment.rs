use crate::ast::{Expr, Module, ReportSpan, Typ};
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
        match (typ, exp) {
            (Typ::Int, Expr::Int(_)) => (),
            (Typ::I8, Expr::Int(_)) => (),
            (Typ::I16, Expr::Int(_)) => (),
            (Typ::I32, Expr::Int(_)) => (),
            (Typ::I64, Expr::Int(_)) => (),
            (Typ::U8, Expr::Int(_)) => (),
            (Typ::U16, Expr::Int(_)) => (),
            (Typ::U32, Expr::Int(_)) => (),
            (Typ::U64, Expr::Int(_)) => (),

            (Typ::Rational, Expr::Rational(_, _)) => (),
            (Typ::Float, Expr::Float(_)) => (),

            (Typ::Bool, Expr::Bool(_)) => (),

            (Typ::Char, Expr::Char(_)) => (),
            (Typ::String, Expr::String(_)) => (),

            (ty, Expr::Identifier(id)) => {
                let actual = self.lookup(id, span);
                self.unify(span, ty, actual);
            }

            (typ, exp) => {
                let actual = self.infer(span, exp);
                self.unify(span, typ, &actual);
            }
        }
    }

    pub fn infer(&self, span: &ReportSpan, exp: &Expr) -> Typ {
        match exp {
            Expr::Bool(_) => Typ::Bool,
            Expr::Char(_) => Typ::Char,
            Expr::String(_) => Typ::String,
            Expr::Rational(_, _) => Typ::Rational,
            Expr::Float(_) => Typ::Float,
            Expr::Int(_) => Typ::Int,
            Expr::Symbol(_) => Typ::Symbol,
            Expr::Syntax(_) => Typ::Syntax,
            Expr::Identifier(id) => self.lookup(id, span).clone(),
            Expr::Tuple(vec) => Typ::Tuple(vec.iter().map(|e| self.infer(span, e)).collect()),
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

use crate::ast::{
    Module, ReportSpan,
    expr::{self, Expr, ExprBody},
    typ::{Typ, TypBody},
};
use ariadne::{Label, Report, ReportKind};
use std::collections::BTreeMap;
use std::fmt;

pub struct Environment<'a> {
    source: &'a Module,
    current_scope: BTreeMap<expr::Identifier, Typ>,
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

            (_, ExprBody::Begin(mids, returned)) => {
                for middle_statement in mids {
                    self.check(
                        span,
                        middle_statement,
                        &TypBody::Void.with_span(span.clone()),
                    );
                }
                self.check(span, returned, typ);
            }

            (_, ExprBody::Let(bindings, body)) => {
                let mut env = self.derive();
                for bind in bindings {
                    env.insert(bind.name.clone(), self.infer(span, &bind.expr));
                }
                env.check(span, &body, typ);
            }

            (_, _) => {
                let actual = self.infer(span, exp);
                self.unify(span, typ, &actual);
            }
        }
    }

    pub fn infer(&self, span: &ReportSpan, exp: &Expr) -> Typ {
        use TypBody::*;
        let typ_span = exp.span();
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
            ExprBody::Pair(a, b) => Pair(
                Box::new(self.infer(&a.span(), a)),
                Box::new(self.infer(&b.span(), b)),
            )
            .with_span(typ_span.clone()),
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
    pub fn insert(&mut self, id: expr::Identifier, typ: Typ) {
        self.current_scope.insert(id, typ);
    }
    pub fn lookup(&self, id: &expr::Identifier, span: &ReportSpan) -> &Typ {
        match self.current_scope.get(id) {
            Some(ty) => ty,
            None => match self.parent {
                Some(parent) => parent.lookup(id, span),
                None => {
                    Report::build(ReportKind::Error, span.clone())
                        .with_code(3)
                        .with_message(format!("`{}` has no type", id.info_name()))
                        .finish()
                        .eprint(self.source.clone())
                        .unwrap();
                    unreachable!("{:?}", self.current_scope)
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

/// NOTE: This Display is used to provide export json file for each module.
/// Therefore, do not rewrite it for debugging purpose
impl<'a> fmt::Display for Environment<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self.current_scope).expect("failed to serialize")
        )
    }
}

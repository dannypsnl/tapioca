use ariadne::{Report, ReportKind, Source};

use crate::ast::{ReportSpan, Typ};
use std::collections::BTreeMap;

pub struct Environment<'a> {
    source: &'a Source<&'a str>,
    current_scope: BTreeMap<String, Typ>,
    parent: Option<&'a Environment<'a>>,
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
}

impl<'a> Environment<'a> {
    pub fn new(source: &'a Source<&'a str>) -> Self {
        Self {
            source,
            current_scope: BTreeMap::new(),
            parent: None,
        }
    }
}

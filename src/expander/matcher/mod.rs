use enotation::{ENotation, ENotationBody, container::*, literal::Literal};
use std::collections::BTreeMap;

#[cfg(test)]
mod tests;

pub enum EPattern<'a> {
    List(Vec<EPattern<'a>>),
    /// `Id` will match an exact same string
    Id(&'a str),
    /// The name of a hole will bind to an `ENotaion` as the result of `ematch`
    Hole(&'a str),
    /// The rest hole will match the rest of many notations
    RestHole(&'a str),
}
use EPattern::*;

pub enum Matched {
    One(ENotation),
    Many(Vec<ENotation>),
}

impl From<ENotation> for Matched {
    fn from(value: ENotation) -> Self {
        Matched::One(value)
    }
}
impl From<Vec<ENotation>> for Matched {
    fn from(value: Vec<ENotation>) -> Self {
        Matched::Many(value)
    }
}

pub fn is_identifier(n: &ENotation) -> bool {
    match &n.body {
        ENotationBody::Literal(literal) => match literal {
            Literal::Identifier(_) => true,
            _ => false,
        },
        _ => false,
    }
}

pub struct Matcher {
    binds: BTreeMap<String, Matched>,
}

impl Matcher {
    pub fn ematch(&mut self, notation: &ENotation, pattern: EPattern) -> bool {
        match (&notation.body, pattern) {
            (_, Hole(name)) => {
                self.insert(name.to_string(), notation.clone().into());
                true
            }
            (enotation::ENotationBody::Container(container), pattern) => {
                self.ematch_container(container, pattern)
            }
            (enotation::ENotationBody::Literal(literal), pattern) => {
                self.ematch_literal(literal, pattern)
            }
            _ => todo!(),
        }
    }

    fn ematch_container(&mut self, container: &Container, pattern: EPattern) -> bool {
        match (container, pattern) {
            (Container::List(list), EPattern::List(patterns)) => {
                let mut notations = list.elems().into_iter().peekable();

                for pat in patterns {
                    match pat {
                        RestHole(name) => {
                            let rest = notations.map(|n| n.clone()).collect::<Vec<ENotation>>();
                            self.insert(name.to_string(), rest.into());
                            return true;
                        }
                        pat => {
                            if let Some(notation) = notations.next() {
                                if !self.ematch(notation, pat) {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        }
                    }
                }

                if notations.peek().is_some() {
                    return false;
                }

                true
            }
            _ => false,
        }
    }

    fn ematch_literal(&self, literal: &Literal, pattern: EPattern) -> bool {
        match (literal, pattern) {
            (Literal::Identifier(actual), Id(expected)) => actual.name == expected,
            _ => false,
        }
    }
}

impl Matcher {
    fn insert(&mut self, name: String, v: Matched) {
        self.binds.insert(name, v);
    }

    fn get(&self, name: &String) -> &Matched {
        match self.binds.get(name) {
            Some(v) => v,
            None => panic!("no matched result {} be found", name),
        }
    }
    pub fn get_one(&self, name: &str) -> &ENotation {
        match self.get(&name.to_string()) {
            Matched::One(enotation) => enotation,
            Matched::Many(_) => panic!("wrong extraction"),
        }
    }
    pub fn get_many(&self, name: &str) -> &Vec<ENotation> {
        match self.get(&name.to_string()) {
            Matched::One(_) => panic!("wrong extraction"),
            Matched::Many(es) => es,
        }
    }
}

impl Default for Matcher {
    fn default() -> Self {
        Matcher {
            binds: BTreeMap::default(),
        }
    }
}

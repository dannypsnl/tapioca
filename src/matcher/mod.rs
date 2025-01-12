use enotation::{ENotation, ENotationBody, container::*, literal::Literal};
use std::collections::BTreeMap;

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

pub fn ematch(
    binds: &mut BTreeMap<String, ENotation>,
    notation: &ENotation,
    pattern: EPattern,
) -> bool {
    match (&notation.body, pattern) {
        (_, Hole(name)) => {
            binds.insert(name.to_string(), notation.clone());
            true
        }
        (enotation::ENotationBody::Container(container), pattern) => {
            ematch_container(binds, container, pattern)
        }
        (enotation::ENotationBody::Literal(literal), pattern) => {
            ematch_literal(binds, literal, pattern)
        }
        (enotation::ENotationBody::Quoting(quoting), pattern) => todo!(),
        (enotation::ENotationBody::Syntaxing(syntaxing), pattern) => todo!(),
    }
}

fn ematch_container(
    binds: &mut BTreeMap<String, ENotation>,
    container: &Container,
    pattern: EPattern,
) -> bool {
    match (container, pattern) {
        (Container::List(list), EPattern::List(patterns)) => {
            let mut notations = list.elems().into_iter().peekable();

            for pat in patterns {
                match pat {
                    RestHole(name) => {
                        let rest = notations.map(|n| n.clone()).collect::<Vec<ENotation>>();
                        let v = ENotation {
                            body: ENotationBody::Container(Container::List(list::List::PL(
                                list::PList { elems: rest },
                            ))),
                            span: Default::default(), // or any valid span value
                        };
                        binds.insert(name.to_string(), v);
                        return true;
                    }
                    pat => {
                        if let Some(notation) = notations.next() {
                            if !ematch(binds, notation, pat) {
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
        (Container::Set(set), _) => todo!(),
        (Container::UnamedObject(unamed_object), _) => todo!(),
        (Container::Object(object), _) => todo!(),
        _ => false,
    }
}

fn ematch_literal(
    _binds: &mut BTreeMap<String, ENotation>,
    literal: &Literal,
    pattern: EPattern,
) -> bool {
    match (literal, pattern) {
        (Literal::Identifier(actual), Id(expected)) => actual.name == expected,
        _ => false,
    }
}

#[cfg(test)]
mod tests;

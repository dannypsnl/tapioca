use enotation::{ENotation, container::*, literal::Literal};
use std::collections::BTreeMap;

pub enum EPattern<'a> {
    List(Vec<EPattern<'a>>),
    /// `Id` will match an exact same string
    Id(&'a str),
    /// The name of a hole will bind to an `ENotaion` as the result of `ematch`
    Hole(&'a str),
}

use EPattern::*;

fn ematch(binds: &mut BTreeMap<String, ENotation>, notation: ENotation, pattern: EPattern) -> bool {
    let no = notation.clone();
    match (notation.body, pattern) {
        (_, Hole(name)) => {
            binds.insert(name.to_string(), no);
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
    container: Container,
    pattern: EPattern,
) -> bool {
    match (container, pattern) {
        (Container::List(list), EPattern::List(patterns)) => {
            for (notation, pat) in list.elems().iter().zip(patterns) {
                if !ematch(binds, notation.clone(), pat) {
                    return false;
                }
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
    binds: &mut BTreeMap<String, ENotation>,
    literal: Literal,
    pattern: EPattern,
) -> bool {
    match (literal, pattern) {
        (Literal::Identifier(actual), Id(expected)) => actual.name == expected,
        _ => false,
    }
}

#[cfg(test)]
mod tests;

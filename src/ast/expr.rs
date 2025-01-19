use std::{collections::HashSet, fmt::Display};

use serde::{Deserialize, Serialize, Serializer};

use crate::expander::scope::Scope;

use super::{ReportSpan, typ::Typ};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExprBody {
    Bool(bool),
    Char(char),
    String(String),
    Rational(i64, i64),
    Float(f64),
    Int(i64),

    Identifier(Identifier),
    Symbol(String),

    Begin(Vec<Expr>, Box<Expr>),
    Let(Vec<Binding>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),

    Lambda(Vec<Identifier>, Box<Expr>),
    App(Box<Expr>, Vec<Expr>),

    List(Vec<Expr>),
    Pair(Box<Expr>, Box<Expr>),
    Object(Vec<(String, Expr)>),

    Syntax(Box<Expr>),
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Identifier {
    // simple identifiers are those with module scope, which is no need to have complex structure
    // e.g.
    // 1. top-level
    // 2. parameters of top-level function
    Simple(String),
    Normal {
        written_name: String,
        scopes: HashSet<Scope>,
    },
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.lookup_name().partial_cmp(&other.lookup_name())
    }
}
impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Identifier {
    /// Constructs a name that not been renamed, usually they are
    /// 1. top-level definition
    /// 2. parameters in top-level `(define (f ……) ……)` form
    pub fn simple(name: String) -> Self {
        Identifier::Simple(name)
    }
    pub fn normal(written_name: String, scopes: HashSet<Scope>) -> Self {
        Identifier::Normal {
            written_name,
            scopes,
        }
    }

    /// Access to the programmer wrote done name, for example
    ///
    /// (let ([x 1])
    ///   (let ([x x])
    ///     x))
    ///
    /// After expansion process, **enotation** been converted to internal AST, and renaming is complete
    /// 1. The first `let x` will be renamed to, e.g. `x1`
    /// 2. The second `let x` will be renamed to, e.g. `x2`
    ///
    /// Therefore, the renamed AST is
    ///
    /// (let ([x1 1])
    ///   (let ([x2 x1])
    ///     x2))
    ///
    /// But for reporting, shouldn't report cannot find `x2` at all, because no programmers expect the error message is talking internal name from compiler.
    pub fn info_name(&self) -> String {
        match self {
            Identifier::Simple(origin_name) => origin_name.clone(),
            Identifier::Normal { written_name, .. } => written_name.clone(),
        }
    }
    /// The internal name that use to lookup in type system (after expansion)
    pub fn lookup_name(&self) -> String {
        match self {
            Identifier::Simple(lookup_name) => lookup_name.clone(),
            Identifier::Normal {
                written_name,
                scopes,
            } => {
                if scopes.is_empty() {
                    written_name.clone()
                } else {
                    // add an uninterned prefix `#:`
                    let mut res = "#:".to_string();
                    for s in scopes {
                        res.push_str(format!("{}", s).as_str());
                    }
                    res.push_str(format!("-{}", written_name).as_str());
                    res
                }
            }
        }
    }
}

// https://stackoverflow.com/questions/63846516/using-serde-json-to-serialise-maps-with-non-string-keys
impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self.lookup_name()))
    }
}
impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::Simple(name) => write!(f, "{}", name),
            Identifier::Normal {
                written_name,
                scopes,
            } => {
                if scopes.is_empty() {
                    write!(f, "{}", written_name)
                } else {
                    write!(f, "{}{{", written_name)?;
                    for (i, s) in scopes.iter().enumerate() {
                        if i == 0 {
                            write!(f, "{}", s)?;
                        } else {
                            write!(f, ", {}", s)?;
                        }
                    }
                    write!(f, "}}")
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Binding {
    pub name: Identifier,
    pub typ: Typ,
    pub expr: Expr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expr {
    span: ReportSpan,
    pub body: ExprBody,
}

impl Expr {
    pub fn span(&self) -> ReportSpan {
        self.span.clone()
    }
}

impl ExprBody {
    pub fn with_span(self, span: ReportSpan) -> Expr {
        Expr { span, body: self }
    }
}

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub enum Scope {
    Module(String),
    Let(u64),
    Lambda(u64),
    /// when a macro get expansion, this will be introduced to scopes set for that expanded fragment
    Intro(String),
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scope::Module(m) => write!(f, "mod_{}", m),
            Scope::Let(c) => write!(f, "let_{}", c),
            Scope::Lambda(c) => write!(f, "lam_{}", c),
            Scope::Intro(c) => write!(f, "intro_{}", c),
        }
    }
}

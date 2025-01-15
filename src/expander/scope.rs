use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub enum Scope {
    Module(String),
    Let(u64),
    Lambda(u64),
    /// when a macro get expansion, this will be introduced to scopes set for that expanded fragment
    Intro(String),
}

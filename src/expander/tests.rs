use super::{Expander, Module, expr::ExprBody, scope::Scope};
use ariadne::Source;
use insta::assert_snapshot;
use std::{collections::HashSet, fmt::Display};

impl Display for ExprBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprBody::Identifier(identifier) => {
                write!(
                    f,
                    "{{origin: {}, bind: {}}}",
                    identifier.origin_name, identifier.lookup_name
                )
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn expander() {
    let source: (String, Source<String>) = ("test.ss".to_string(), Source::from("".to_string()));
    let mut module = Module::new(source);

    let mut ex = Expander {
        source: module.source.clone(),
        module: &mut module,
        rename_mapping: Default::default(),
        let_count: 0,
        lambda_count: 0,
    };
    let mut scopes = HashSet::new();
    ex.insert(&"b".to_string(), scopes.clone(), "b1".to_string());
    let outer_scopes = scopes.clone();
    scopes.insert(Scope::Let(0));
    let name = "a".to_string();
    ex.insert(&name, scopes.clone(), "a1".to_string());
    scopes.insert(Scope::Let(1));
    ex.insert(&name, scopes.clone(), "a2".to_string());
    assert_snapshot!(ex.lookup_newname(&name, scopes.clone()), @"{origin: a, bind: a2}");
    assert_snapshot!(ex.lookup_newname(&"b".to_string(), outer_scopes.clone()), @"{origin: b, bind: b1}");
}

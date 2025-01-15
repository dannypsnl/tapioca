use crate::ast::{
    expr::{Expr, ExprBody},
    typ::{Typ, TypBody},
    *,
};
use crate::matcher::{EPattern::*, MatchedResult, ematch};
use crate::{error, matcher};
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use enotation::{
    EFile, ENotation, ENotationBody, ENotationParser, Rule, SetDebugFileName,
    container::{self, Container},
    literal::{self, Literal},
};
use expr::Binding;
use from_pest::FromPest;
use pest::Parser;
use scope::Scope;
use std::{collections::HashSet, path::Path, vec};

pub mod scope;

pub fn expand_module(root_path: &Path, filename: &str) -> Result<Module, error::Error> {
    let path: &Path = Path::new(filename);
    let module_path = path.strip_prefix(root_path).unwrap();
    let module_name: String = module_path
        .file_prefix()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let input = std::fs::read_to_string(path).expect("failed to open file");

    let mut output = ENotationParser::parse(Rule::file, input.as_str()).unwrap();
    let mut efile = EFile::from_pest(&mut output)?;
    efile.set_debug_file_name(filename);

    let mut module = Module {
        source: (filename.to_string(), Source::from(input)),
        claim_forms: vec![],
        define_forms: vec![],
        other_forms: vec![],
        requires: vec![],
    };
    let mut expander = Expander {
        module: &mut module,
        rename_mapping: vec![],
    };

    let stack = ScopeStack::module(module_name);
    for notation in efile.notations {
        expander.expand_top_level(&stack, notation)?;
    }

    Ok(module)
}

pub struct ScopeStack<'a> {
    parent: Option<&'a ScopeStack<'a>>,
    current_scope: Scope,
    let_count: u64,
    lambda_count: u64,
}
impl<'a> ScopeStack<'a> {
    fn module(module_name: String) -> Self {
        Self {
            parent: None,
            current_scope: Scope::Module(module_name),
            let_count: 0,
            lambda_count: 0,
        }
    }
    fn extend_let(&'a self) -> Self {
        Self {
            parent: Some(self),
            current_scope: Scope::Let(self.let_count),
            let_count: self.let_count + 1,
            lambda_count: self.lambda_count,
        }
    }
    fn as_set(&self) -> HashSet<Scope> {
        let mut set = HashSet::new();
        set.insert(self.current_scope.clone());
        if let Some(p) = self.parent {
            set.extend(p.as_set());
        }
        set
    }
}

struct Expander<'a> {
    module: &'a mut Module,
    /// 1. first one is the concrete name: e.g. x
    /// 2. second one is the scopes set, e.g. { let1, lambda1 }
    /// 3. thire one is the new name, e.g. #:x-let1
    rename_mapping: Vec<(String, HashSet<Scope>, String)>,
}

impl Expander<'_> {
    fn expand_top_level(
        &mut self,
        stack: &ScopeStack,
        notation: ENotation,
    ) -> Result<(), error::Error> {
        let mut binds = MatchedResult::default();
        if ematch(
            &mut binds,
            &notation,
            List(vec![Id("define"), RestHole("rest")]),
        ) {
            self.expand_defines(stack, &notation)?;
        } else if ematch(&mut binds, &notation, List(vec![Id(":"), RestHole("rest")])) {
            self.expand_claims(&notation)?;
        } else if ematch(
            &mut binds,
            &notation,
            List(vec![Id("require"), RestHole("module")]),
        ) {
            let requires = binds.get_many("module");
            for r in requires {
                if matcher::is_identifier(r) {
                    self.module.requires.push(Require {
                        module: format!("{}", r),
                    });
                } else {
                    let span: ReportSpan = r.span.clone().into();
                    Report::build(ReportKind::Error, span.clone())
                        .with_code(3)
                        .with_message("bad require")
                        .with_label(Label::new(span.clone()).with_message("Not an identifier"))
                        .finish()
                        .eprint(self.module.clone())
                        .unwrap();
                }
            }
        } else {
            self.module.other_forms.push(notation);
        }

        Ok(())
    }

    fn expand_claims(&mut self, notation: &ENotation) -> Result<(), error::Error> {
        let mut binds = MatchedResult::default();
        if ematch(
            &mut binds,
            &notation,
            List(vec![Id(":"), Hole("name"), Id(":"), RestHole("typ")]),
        ) {
            let typ = self.expand_type(notation.span.clone().into(), binds.get_many("typ"));
            self.module.claim_forms.push(ClaimForm {
                id: binds.get_one("name").to_string(),
                typ,
            })
        }

        Ok(())
    }

    fn expand_defines(
        &mut self,
        stack: &ScopeStack,
        notation: &ENotation,
    ) -> Result<(), error::Error> {
        let mut binds = MatchedResult::default();
        if ematch(
            &mut binds,
            &notation,
            List(vec![
                Id("define"),
                List(vec![Hole("name"), RestHole("params")]),
                RestHole("body"),
            ]),
        ) {
            let name = binds.get_one("name");

            let mut params = vec![];
            for p in binds.get_many("params") {
                let name = p.to_string();
                self.insert(&name, stack.as_set(), name.clone());
                params.push(name);
            }

            let mut body = vec![];
            for p in binds.get_many("body") {
                body.push(self.expand_expr(stack, p));
            }
            if let Some((returned, body)) = body.split_last() {
                self.module.define_forms.push(DefineForm::DefineFunction {
                    span: notation.span.clone().into(),
                    id: name.to_string(),
                    params,
                    body: body.into(),
                    returned: returned.clone(),
                });
            }
        } else if ematch(
            &mut binds,
            &notation,
            List(vec![Id("define"), Hole("name"), Hole("expr")]),
        ) {
            let typ = self.expand_expr(stack, binds.get_one("expr"));
            self.module.define_forms.push(DefineForm::DefineConstant {
                span: notation.span.clone().into(),
                id: binds.get_one("name").to_string(),
                expr: typ,
            });
        } else {
            let span: ReportSpan = notation.span.clone().into();
            Report::build(ReportKind::Error, span.clone())
                .with_code(3)
                .with_message("bad define")
                .with_label(Label::new(span.clone()))
                .finish()
                .eprint(self.module.clone())
                .unwrap();
            panic!("")
        }

        Ok(())
    }

    fn expand_type(&mut self, span: ReportSpan, ns: &Vec<ENotation>) -> Typ {
        if ns.len() == 0 {
            Report::build(ReportKind::Error, span.clone())
                .finish()
                .eprint(self.module.clone())
                .unwrap();
            panic!("")
        } else if ns.len() == 1 {
            self.expand_one_type(&ns[0])
        } else {
            let mut stack = vec![];
            let mut ns = ns.into_iter();
            while let Some(n) = ns.next() {
                if ematch(&mut MatchedResult::default(), n, Id("->")) {
                    return TypBody::Func {
                        params: stack,
                        result: self
                            .expand_type(span.clone(), &ns.cloned().collect())
                            .into(),
                    }
                    .with_span(span.clone());
                } else {
                    stack.push(self.expand_one_type(n));
                }
            }
            unreachable!()
        }
    }

    fn expand_one_type(&mut self, notation: &ENotation) -> Typ {
        let span: ReportSpan = notation.span.clone().into();
        match &notation.body {
            ENotationBody::Literal(Literal::Identifier(id)) => match id.name.as_str() {
                "boolean" => TypBody::Bool.with_span(span),
                "char" => TypBody::Char.with_span(span),
                "string" => TypBody::String.with_span(span),
                "symbol" => TypBody::Symbol.with_span(span),
                "rational" => TypBody::Rational.with_span(span),
                "float" => TypBody::Float.with_span(span),
                "int" => TypBody::Int.with_span(span),
                "i8" => TypBody::I8.with_span(span),
                "i16" => TypBody::I16.with_span(span),
                "i32" => TypBody::I32.with_span(span),
                "i64" => TypBody::I64.with_span(span),
                "u8" => TypBody::U8.with_span(span),
                "u16" => TypBody::U16.with_span(span),
                "u32" => TypBody::U32.with_span(span),
                "u64" => TypBody::U64.with_span(span),
                "syntax" => TypBody::Syntax.with_span(span),
                // unknown type
                _ => todo!(),
            },
            ENotationBody::Container(Container::List(list)) => {
                let mut ts = list.elems().into_iter();
                let head = ts.next().unwrap();
                if ematch(&mut MatchedResult::default(), head, Id("array")) {
                    let t = self.expand_one_type(ts.next().unwrap());
                    TypBody::Array(t.into()).with_span(span)
                } else if ematch(&mut MatchedResult::default(), head, Id("list")) {
                    let t = self.expand_one_type(ts.next().unwrap());
                    TypBody::Array(t.into()).with_span(span)
                } else if ematch(&mut MatchedResult::default(), head, Id("tuple")) {
                    let mut xs = vec![];
                    for t in ts {
                        xs.push(self.expand_one_type(t));
                    }
                    TypBody::Tuple(xs).with_span(span)
                } else {
                    todo!()
                }
            }
            ENotationBody::Container(Container::Object(obj)) => {
                let mut fields = vec![];
                for pair in &obj.pairs {
                    fields.push((pair.key.name.clone(), self.expand_one_type(&pair.value)))
                }
                TypBody::Record(fields).with_span(span)
            }
            _ => {
                self.bad_form(notation);
                todo!()
            }
        }
    }

    fn expand_expr(&mut self, stack: &ScopeStack, notation: &ENotation) -> Expr {
        use enotation::ENotationBody::*;

        let span: ReportSpan = notation.span.clone().into();
        match &notation.body {
            Literal(literal) => match literal {
                literal::Literal::Boolean(boolean) => ExprBody::Bool(boolean.value).with_span(span),
                literal::Literal::Char(c) => ExprBody::Char(c.value).with_span(span),
                literal::Literal::Float(float) => ExprBody::Float(float.value).with_span(span),
                literal::Literal::Rational(rational) => {
                    ExprBody::Rational(rational.value.0, rational.value.1).with_span(span)
                }
                literal::Literal::Int(integer) => ExprBody::Int(integer.value).with_span(span),
                literal::Literal::String_(string) => {
                    ExprBody::String(string.value.clone()).with_span(span)
                }
                // every identifier should resolve the scopes set immediately, once it converted to expression
                literal::Literal::Identifier(identifier) => self
                    .lookup_newname(&identifier.to_string(), stack.as_set())
                    .with_span(span),
            },

            Container(container) => match container {
                container::Container::List(_) => self.expand_expr_form(stack, notation),
                container::Container::Set(_)
                | container::Container::UnamedObject(_)
                | container::Container::Object(_) => todo!(),
            },

            Quoting(_) | Syntaxing(_) => todo!(),
        }
    }

    fn expand_many_expr(&mut self, stack: &ScopeStack, notations: &Vec<ENotation>) -> Vec<Expr> {
        notations
            .iter()
            .map(|n| self.expand_expr(stack, n))
            .collect()
    }

    fn expand_expr_form(&mut self, stack: &ScopeStack, list: &ENotation) -> Expr {
        let mut binds = MatchedResult::default();

        if ematch(
            &mut binds,
            list,
            List(vec![
                Id("let"),
                List(vec![RestHole("bindings")]),
                RestHole("body"),
            ]),
        ) {
            let stack = stack.extend_let();
            let bs = self.expand_bindings(&stack, binds.get_many("bindings"));
            let body = self.expand_many_expr(&stack, binds.get_many("body"));
            ExprBody::Let(bs, body).with_span(list.span.clone().into())
        } else {
            todo!()
        }
    }

    fn expand_bindings(&mut self, stack: &ScopeStack, notations: &Vec<ENotation>) -> Vec<Binding> {
        notations
            .iter()
            .map(|n| self.expand_binding(stack, n))
            .collect()
    }
    fn expand_binding(&mut self, stack: &ScopeStack, notation: &ENotation) -> Binding {
        let mut binds = MatchedResult::default();
        if ematch(&mut binds, notation, List(vec![Hole("name"), Hole("expr")])) {
            let name = binds.get_one("name").to_string();
            self.insert(&name, stack.as_set(), self.unique_name(&name));
            let expr = self.expand_expr(stack.parent.unwrap(), binds.get_one("expr"));
            Binding { name, expr }
        } else {
            self.bad_form(notation);
            panic!("bad form")
        }
    }

    fn bad_form(&mut self, notation: &ENotation) {
        let out = Color::Fixed(81);
        let span: ReportSpan = notation.span.clone().into();
        Report::build(ReportKind::Error, span)
            .with_code(3)
            .with_message("bad form")
            .with_note(format!("{} form must ……", "match".fg(out)))
            .finish()
            .eprint(self.module.clone())
            .unwrap();
    }

    fn insert(&mut self, name: &String, scopes: HashSet<Scope>, new_name: String) {
        self.rename_mapping.push((name.clone(), scopes, new_name));
    }

    fn unique_name(&self, name: &String) -> String {
        // add an uninterned prefix
        format!("#:{}", name)
    }

    fn lookup_newname(&self, refname: &String, scopes: HashSet<Scope>) -> ExprBody {
        // notice that `rev` is required, because the insert order is up to down
        // e.g.
        // (define (f a b)
        //   (let ([b 1])
        //     b))
        // though we want `b` refers to **first match**, but this first match is from down to up view, not insert order, and hence the rename mapping should be lookup in reversed order.
        for (name, bind_scopes, new_name) in self.rename_mapping.iter().rev() {
            if name == refname && scopes.is_superset(bind_scopes) {
                return ExprBody::Identifier(new_name.clone());
            }
        }
        panic!("failed to find any proper name {}", refname)
    }
}

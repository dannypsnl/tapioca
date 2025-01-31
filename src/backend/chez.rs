use std::{
    fmt::Display,
    fs::File,
    io::{self, BufWriter, Write},
};

use crate::ast::{
    DefineForm,
    expr::{self, Expr},
};

pub fn schemify(f: &mut BufWriter<File>, def: &DefineForm) -> io::Result<()> {
    match def {
        DefineForm::DefineConstant { id, expr, .. } => {
            write!(f, "(define {} {})", id.info_name(), expr)?;
        }
        DefineForm::DefineFunction {
            span,
            id,
            params,
            body,
        } => {
            write!(f, "(define ({} ", id)?;
            for p in params {
                write!(f, "{} ", p.info_name())?;
            }
            write!(f, ") {})", body)?;
        }
    }
    Ok(())
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.body {
            expr::ExprBody::Bool(t) => {
                if *t {
                    write!(f, "#t")
                } else {
                    write!(f, "#f")
                }
            }
            expr::ExprBody::Char(c) => match c {
                ' ' => write!(f, "#\\space"),
                '\t' => write!(f, "#\\tab"),
                '\r' => write!(f, "#\\return"),
                '\n' => write!(f, "#\\newline"),
                c => write!(f, "#\\{}", c),
            },
            expr::ExprBody::String(s) => write!(f, "\"{}\"", s),
            expr::ExprBody::Rational(p, q) => write!(f, "{}/{}", p, q),
            expr::ExprBody::Float(v) => write!(f, "{}", v),
            expr::ExprBody::Int(i) => write!(f, "{}", i),
            expr::ExprBody::Identifier(id) => write!(f, "{}", id.info_name()),
            expr::ExprBody::Symbol(x) => write!(f, "{}", x),
            expr::ExprBody::Begin(bodys, body) => {
                for b in bodys {
                    write!(f, "{}", b)?;
                }
                write!(f, "{}", body)
            }
            expr::ExprBody::Let(bindings, body) => {
                write!(f, "(let (")?;
                for bind in bindings {
                    write!(f, "[{} {}]", bind.name.info_name(), bind.expr)?;
                }
                write!(f, ") {})", body)
            }
            expr::ExprBody::If(c, t, e) => {
                write!(f, "(if {} {} {})", c, t, e)
            }
            expr::ExprBody::Lambda(params, body) => {
                write!(f, "(lambda (")?;
                for p in params {
                    write!(f, "{} ", p.info_name())?;
                }
                write!(f, ")")?;
                write!(f, "{})", body)
            }
            expr::ExprBody::App(func, args) => {
                write!(f, "({}", func)?;
                for a in args {
                    write!(f, " {}", a)?;
                }
                write!(f, ")")
            }
            expr::ExprBody::List(args) => {
                write!(f, "(list")?;
                for a in args {
                    write!(f, " {}", a)?;
                }
                write!(f, ")")
            }
            expr::ExprBody::Pair(a, b) => write!(f, "(cons {} {})", a, b),
            expr::ExprBody::Syntax(expr) => write!(f, "#'{}", expr),
            expr::ExprBody::Object(_) => todo!(),
        }
    }
}

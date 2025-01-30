use std::{
    fmt::Display,
    fs::File,
    io::{self, BufWriter, Write},
};

use crate::ast::{DefineForm, expr::Expr};

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
            crate::ast::expr::ExprBody::Bool(t) => {
                if *t {
                    write!(f, "#t")
                } else {
                    write!(f, "#f")
                }
            }
            crate::ast::expr::ExprBody::Char(c) => match c {
                ' ' => write!(f, "#\\space"),
                '\t' => write!(f, "#\\tab"),
                '\r' => write!(f, "#\\return"),
                '\n' => write!(f, "#\\newline"),
                c => write!(f, "#\\{}", c),
            },
            crate::ast::expr::ExprBody::String(s) => write!(f, "\"{}\"", s),
            crate::ast::expr::ExprBody::Rational(p, q) => write!(f, "{}/{}", p, q),
            crate::ast::expr::ExprBody::Float(v) => write!(f, "{}", v),
            crate::ast::expr::ExprBody::Int(i) => write!(f, "{}", i),
            crate::ast::expr::ExprBody::Identifier(id) => write!(f, "{}", id.info_name()),
            crate::ast::expr::ExprBody::Symbol(x) => write!(f, "{}", x),
            crate::ast::expr::ExprBody::Begin(bodys, body) => {
                for b in bodys {
                    write!(f, "{}", b)?;
                }
                write!(f, "{}", body)
            }
            crate::ast::expr::ExprBody::Let(bindings, body) => {
                write!(f, "(let (")?;
                for bind in bindings {
                    write!(f, "[{} {}]", bind.name.info_name(), bind.expr)?;
                }
                write!(f, ") {})", body)
            }
            crate::ast::expr::ExprBody::If(expr, expr1, expr2) => todo!(),
            crate::ast::expr::ExprBody::Lambda(params, body) => {
                write!(f, "(lambda (")?;
                for p in params {
                    write!(f, "{} ", p.info_name())?;
                }
                write!(f, ")")?;
                write!(f, "{})", body)
            }
            crate::ast::expr::ExprBody::App(expr, exprs) => todo!(),
            crate::ast::expr::ExprBody::List(exprs) => todo!(),
            crate::ast::expr::ExprBody::Pair(expr, expr1) => todo!(),
            crate::ast::expr::ExprBody::Object(items) => todo!(),
            crate::ast::expr::ExprBody::Syntax(expr) => todo!(),
        }
    }
}

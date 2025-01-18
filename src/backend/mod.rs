use crate::ast::expr::ExprBody;
use crate::ast::typ::{self, TypBody};
use crate::ast::{self, DefineForm};
use crate::type_system::environment::Environment;
use s2c::c_cg::c_stmt;
use s2c::c_cg::c_type::CType;
use s2c::c_cg::c_value::{CLiteral, CValue, IntegerSuffix};
use std::fs::File;
use std::io::Write;
use std::path::Path;

struct Driver<'a> {
    module_path: &'a Path,
    module_name: String,
    c_ctx: c_stmt::Context,
}

pub fn compile(root_path: &Path, env: &Environment<'_>, module: &ast::Module) {
    let raw_path: &Path = Path::new(&module.source.0);
    let module_path = raw_path.strip_prefix(root_path).unwrap();
    let module_name: String = module_path
        .file_prefix()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let mut driver = Driver {
        c_ctx: c_stmt::Context::standard(module_name.clone()),
        module_path,
        module_name,
    };

    for def in &module.define_forms {
        driver.compile_definition(env, def);
    }

    let cpath = driver.module_path.with_extension("c");
    let symbols = driver.module_path.with_extension("json");
    let output = Path::new("_build");
    std::fs::create_dir_all(output).expect("failed to create output directory");
    let mut f = File::create_buffered(output.join(cpath)).expect("failed to open output file");
    write!(&mut f, "{}", driver.c_ctx.current_source.lock().unwrap()).expect("failed to write");

    let mut f = File::create_buffered(output.join(symbols)).expect("failed to open symbols file");
    write!(&mut f, "{}", env).expect("failed to write");
}

impl<'a> Driver<'a> {
    fn compile_definition(&mut self, env: &Environment<'_>, def: &DefineForm) {
        match def {
            DefineForm::DefineConstant { span, id, expr } => {
                let ty = env.lookup(id, span);
                let name = self.mangle_name(&id.lookup_name());

                self.c_ctx
                    .set(self.convert_type(ty), name, self.convert_expr(expr));
            }
            DefineForm::DefineFunction {
                span,
                id,
                params,
                body,
            } => {
                if let TypBody::Func {
                    params: ptys,
                    result,
                } = &env.lookup(id, span).body
                {
                    self.c_ctx.def(
                        id.lookup_name(),
                        self.convert_type(&result),
                        ptys.iter()
                            .map(|t| self.convert_type(t))
                            .zip(params.iter().map(|p| p.lookup_name()))
                            .collect(),
                        |ctx| {
                            match &body.body {
                                ExprBody::Begin(mid, last) => {
                                    for m in mid {
                                        self.convert_expr(m);
                                    }
                                    self.convert_expr(&last);
                                }
                                ExprBody::Let(bindings, expr) => {
                                    for bind in bindings {
                                        ctx.set(
                                            self.convert_type(env.lookup(&bind.name, span)),
                                            bind.name.lookup_name(),
                                            self.convert_expr(&bind.expr),
                                        );
                                    }
                                    self.convert_expr(expr);
                                }
                                _ => {
                                    todo!()
                                }
                            }
                            ctx
                        },
                    );
                } else {
                    panic!("internal error: a function has no function type")
                }
            }
        }
    }

    fn mangle_name(&self, name: &String) -> String {
        format!("{}__{}", self.module_name, name)
    }

    fn convert_type(&self, ty: &typ::Typ) -> CType {
        match &ty.body {
            // stdbool.h
            TypBody::Bool => CType::I8,
            TypBody::Char => CType::U8,

            // TODO: these need runtime supports
            TypBody::String => todo!(),
            TypBody::Int => todo!(),

            // rely s2c
            TypBody::I8 => CType::I8,
            TypBody::I16 => CType::I16,
            TypBody::I32 => CType::I32,
            TypBody::I64 => CType::I64,
            TypBody::U8 => CType::U8,
            TypBody::U16 => CType::U16,
            TypBody::U32 => CType::U32,
            TypBody::U64 => CType::U64,
            TypBody::Void => CType::Void,

            TypBody::Symbol => todo!(),
            TypBody::Rational => todo!(),
            TypBody::Float => todo!(),
            TypBody::Syntax => todo!(),
            TypBody::Vector(_typ) => todo!(),
            TypBody::List(_typ) => todo!(),
            TypBody::Pair(_a, _b) => todo!(),
            TypBody::Record(_vec) => todo!(),
            TypBody::Func {
                params: _,
                result: _,
            } => todo!(),
        }
    }

    fn convert_expr(&self, expr: &ast::expr::Expr) -> CValue {
        match &expr.body {
            ast::expr::ExprBody::Bool(_) => todo!(),
            ast::expr::ExprBody::Char(_) => todo!(),
            ast::expr::ExprBody::String(_) => todo!(),
            ast::expr::ExprBody::Rational(_, _) => todo!(),
            ast::expr::ExprBody::Float(_) => todo!(),
            ast::expr::ExprBody::Int(i) => {
                CValue::Literal(CLiteral::Int(*i as usize, IntegerSuffix::I64))
            }
            ast::expr::ExprBody::Identifier(id) => CValue::Variable(id.lookup_name().clone()),
            ast::expr::ExprBody::Symbol(_) => todo!(),
            _ => todo!(),
        }
    }
}

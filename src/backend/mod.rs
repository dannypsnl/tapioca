use crate::ast::expr::Identifier;
use crate::ast::typ::{self, TypBody};
use crate::ast::{self, DefineForm};
use crate::type_system::environment::Environment;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tinyc::{CExpr, CFile, CStmt, Declare, DefineFunc};

pub mod tinyc;

struct Driver<'a> {
    module_path: &'a Path,
    module_name: String,
    init_list: Vec<(String, CExpr)>,
    cfile: CFile,
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
        module_path,
        module_name,
        init_list: vec![],
        cfile: Default::default(),
    };

    for def in &module.define_forms {
        driver.compile_definition(env, def);
    }

    let cpath = driver.module_path.with_extension("c");
    let symbols = driver.module_path.with_extension("json");
    let output = Path::new("_build");
    std::fs::create_dir_all(output).expect("failed to create output directory");
    let mut f = File::create_buffered(output.join(cpath)).expect("failed to open output file");
    write!(&mut f, "{}", driver.cfile).expect("failed to write");

    let mut f = File::create_buffered(output.join(symbols)).expect("failed to open symbols file");
    write!(&mut f, "{}", env).expect("failed to write");
}

impl<'a> Driver<'a> {
    fn compile_definition(&mut self, env: &Environment<'_>, def: &DefineForm) {
        match def {
            DefineForm::DefineConstant { span, id, expr } => {
                let ty = env.lookup(&Identifier::top_level(id.to_string()), span);
                let name = self.mangle_name(id);
                self.cfile.declares.push(Declare {
                    name: name.clone(),
                    typ: self.convert_type(ty),
                });
                self.init_list.push((name, self.convert_expr(expr)));
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
                } = &env
                    .lookup(&Identifier::top_level(id.to_string()), span)
                    .body
                {
                    self.cfile.funcs.push(DefineFunc {
                        name: self.mangle_name(id),
                        params: params
                            .iter()
                            .cloned()
                            .zip(ptys.iter().map(|t| self.convert_type(t)))
                            .collect(),
                        result: self.convert_type(&result),
                        statement: self.convert_cstmt(body),
                    });
                } else {
                    panic!("internal error: a function has no function type")
                }
            }
        }
    }

    fn mangle_name(&self, name: &String) -> String {
        format!("{}__{}", self.module_name, name)
    }

    fn convert_cstmt(&self, expr: &ast::expr::Expr) -> tinyc::CStmt {
        match &expr.body {
            ast::expr::ExprBody::Begin(es, expr) => {
                let mut next = self.convert_cstmt(expr);
                let mut eit = es.iter().rev();
                while let Some(e) = eit.next() {
                    next = CStmt::Seq {
                        cur: self.convert_expr(e),
                        next: Box::new(next),
                    }
                }
                next
            }
            ast::expr::ExprBody::Let(binds, expr) => {
                let mut next = self.convert_cstmt(expr);
                let mut bit = binds.iter().rev();
                while let Some(b) = bit.next() {
                    next = CStmt::Assign {
                        name: b.name.clone(),
                        expr: self.convert_expr(&b.expr),
                        next: Box::new(next),
                    }
                }
                next
            }

            _ => CStmt::Return(self.convert_expr(expr)),
        }
    }

    fn convert_type(&self, ty: &typ::Typ) -> tinyc::CTyp {
        match &ty.body {
            // stdbool.h
            TypBody::Bool => "bool".into(),
            TypBody::Char => "char".into(),
            TypBody::String => "tapi_string".into(),

            TypBody::Int => "int".into(),
            // stdint.h
            TypBody::I8 => "int8_t".into(),
            TypBody::I16 => "int16_t".into(),
            TypBody::I32 => "int32_t".into(),
            TypBody::I64 => "int64_t".into(),
            TypBody::U8 => "uint8_t".into(),
            TypBody::U16 => "uint16_t".into(),
            TypBody::U32 => "uint32_t".into(),
            TypBody::U64 => "uint64_t".into(),
            TypBody::Void => "void".into(),

            TypBody::Symbol => todo!(),
            TypBody::Rational => todo!(),
            TypBody::Float => todo!(),
            TypBody::Syntax => todo!(),
            TypBody::Array(_typ) => todo!(),
            TypBody::List(_typ) => todo!(),
            TypBody::Tuple(_vec) => todo!(),
            TypBody::Record(_vec) => todo!(),
            TypBody::Func {
                params: _,
                result: _,
            } => todo!(),
        }
    }

    fn convert_expr(&self, expr: &ast::expr::Expr) -> CExpr {
        match &expr.body {
            ast::expr::ExprBody::Bool(_) => todo!(),
            ast::expr::ExprBody::Char(_) => todo!(),
            ast::expr::ExprBody::String(_) => todo!(),
            ast::expr::ExprBody::Rational(_, _) => todo!(),
            ast::expr::ExprBody::Float(_) => todo!(),
            ast::expr::ExprBody::Int(i) => CExpr::CInt(*i),
            ast::expr::ExprBody::Identifier(id) => CExpr::Id(id.lookup_name().clone()),
            ast::expr::ExprBody::Symbol(_) => todo!(),
            _ => todo!(),
        }
    }
}

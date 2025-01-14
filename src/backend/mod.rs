use crate::ast::expr::Expr;
use crate::ast::typ::TypBody;
use crate::ast::{self, DefineForm};
use crate::type_system::environment::Environment;
use std::path::Path;
use std::vec;

pub mod tinyc;

struct Driver<'a> {
    module_path: &'a Path,
    module_name: String,
    init_list: Vec<(String, Expr)>,
}

pub fn compile(root_path: &Path, env: &Environment<'_>, module: &ast::Module) {
    let raw_path: &Path = Path::new(&module.source.0);
    let module_path = raw_path.strip_prefix(root_path).unwrap();
    let module_name: String = module_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let mut driver = Driver {
        module_path,
        module_name,
        init_list: vec![],
    };

    for def in &module.define_forms {
        driver.compile_definition(env, def);
    }
}

impl<'a> Driver<'a> {
    fn compile_definition(&mut self, env: &Environment<'_>, def: &DefineForm) {
        match def {
            DefineForm::DefineConstant { span, id, expr } => {
                let ty = env.lookup(id, span);
                println!("{} {} {:?}", ty, id, expr);
            }
            DefineForm::DefineFunction {
                span,
                id,
                params,
                body,
                returned,
            } => {
                if let TypBody::Func {
                    params: ptys,
                    result,
                } = &env.lookup(id, span).body
                {
                    let pp = params.into_iter().zip(ptys);
                    print!("{} {}(", result, id,);
                    for (i, (id, ty)) in pp.enumerate() {
                        if i == 0 {
                            print!("{} {}", ty, id);
                        } else {
                            print!(", {} {}", ty, id);
                        }
                    }
                    println!(") {{");
                    for ele in body {
                        println!("  {:?};", ele);
                    }
                    println!("  return {:?};", returned);
                    println!("}}");
                }
            }
        }
    }
}

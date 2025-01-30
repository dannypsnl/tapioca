use crate::ast::expr::ExprBody;
use crate::ast::typ::{self, TypBody};
use crate::ast::{self, DefineForm};
use crate::type_system::environment::Environment;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub mod chez;

struct Driver<'a> {
    module_path: &'a Path,
    module_name: String,
}

pub enum Mode {
    Program,
    Library,
}

pub fn compile(root_path: &Path, env: &Environment<'_>, module: &ast::Module, mode: Mode) {
    let raw_path: &Path = Path::new(&module.source.0);
    let module_path = raw_path.strip_prefix(root_path).unwrap();
    let module_name: String = module_path
        .file_prefix()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let driver = Driver {
        module_path,
        module_name,
    };

    let ss_path = driver.module_path.with_extension("ss");
    let output = Path::new("_build");
    std::fs::create_dir_all(output).expect("failed to create output directory");
    let mut f = File::create_buffered(output.join(ss_path)).expect("failed to open output file");

    match mode {
        Mode::Program => {
            for def in &module.define_forms {
                chez::schemify(&mut f, def).expect("failed to produce scheme file");
            }
        }
        Mode::Library => {
            write!(&mut f, "(library {}", driver.module_name).expect("msg");
            for def in &module.define_forms {
                chez::schemify(&mut f, def).expect("failed to produce scheme file");
            }
            write!(&mut f, ")").expect("msg");
        }
    }

    let symbols = driver.module_path.with_extension("json");
    let mut f = File::create_buffered(output.join(symbols)).expect("failed to open symbols file");
    write!(&mut f, "{}", env).expect("failed to write");
}

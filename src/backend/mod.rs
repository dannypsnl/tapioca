use crate::ast;
use crate::type_system::environment::Environment;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub mod chez;

pub enum Mode {
    Program,
    Library,
}

pub fn compile(
    root_path: &Path,
    env: &Environment<'_>,
    module: &ast::Module,
    mode: Mode,
) -> io::Result<()> {
    let raw_path: &Path = Path::new(&module.source.0);
    let module_path = raw_path.strip_prefix(root_path).unwrap();
    let module_name: String = module_path
        .file_prefix()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let ss_path = module_path.with_extension("ss");
    let output = Path::new("_build");
    std::fs::create_dir_all(output).expect("failed to create output directory");
    let mut f = File::create_buffered(output.join(ss_path)).expect("failed to open output file");

    match mode {
        Mode::Program => {
            for def in &module.define_forms {
                chez::schemify(&mut f, def)?;
            }
        }
        Mode::Library => {
            write!(&mut f, "(library ({})", module_name)?;
            write!(&mut f, "(export )")?;
            write!(&mut f, "(import (chezscheme))")?;
            for def in &module.define_forms {
                chez::schemify(&mut f, def)?;
            }
            write!(&mut f, ")")?;
        }
    }

    let symbols = module_path.with_extension("json");
    let mut f = File::create_buffered(output.join(symbols)).expect("failed to open symbols file");
    write!(&mut f, "{}", env)?;

    Ok(())
}

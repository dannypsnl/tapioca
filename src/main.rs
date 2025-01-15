#![feature(path_file_prefix)]
#![feature(file_buffered)]
mod ast;
mod backend;
mod error;
mod expander;
mod matcher;
mod type_system;
use expander::expand_module;

fn main() {
    let root = std::path::Path::new("example");
    let module = expand_module(root, "example/hello.ss").expect("expanding failed");
    let env = type_system::check(&module);
    backend::compile(root, &env, &module);
}

#![feature(path_file_prefix)]
mod ast;
mod backend;
mod error;
mod expander;
mod matcher;
mod type_system;
use expander::expand_module;

fn main() {
    let module = expand_module("example/hello.ss").expect("expanding failed");
    let env = type_system::check(&module);
    backend::compile(std::path::Path::new("example"), &env, &module);
}

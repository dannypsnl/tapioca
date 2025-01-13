mod ast;
mod backend;
mod error;
mod expander;
mod matcher;
mod type_system;
use expander::expand_module;

fn main() {
    let module = expand_module("example/hello.ss").expect("expanding failed");
    type_system::check(&module);
    backend::compile(std::path::Path::new("example"), module);
}

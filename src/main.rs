#![feature(path_file_prefix)]
#![feature(file_buffered)]
#![feature(exact_size_is_empty)]
// They are common module
mod ast;
mod error;
// The first step is expanding
mod expander;
// type checking
mod type_system;
// backend is the final stage
mod backend;

fn main() {
    let root = std::path::Path::new("example");
    let module = expander::expand_module(root, "example/hello.ss").expect("expanding failed");
    let env = type_system::check(&module);
    backend::compile(root, &env, &module, backend::Mode::Program);
}

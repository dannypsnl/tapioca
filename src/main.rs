mod ast;
mod backend;
mod error;
mod expander;
mod matcher;
mod type_system;
use expander::expand_module;

fn main() -> Result<(), error::Error> {
    let input = std::fs::read_to_string("example/hello.ss")?;
    let module = expand_module(input.as_str())?;
    println!("\ndebug\n{:?}", module);
    type_system::check(&module);
    backend::compile(std::path::Path::new("example"), module);
    Ok(())
}

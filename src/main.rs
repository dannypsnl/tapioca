mod ast;
mod error;
mod expander;
mod matcher;
use expander::expand_module;

fn main() -> Result<(), error::Error> {
    let input = std::fs::read_to_string("example/hello.ss")?;
    let module = expand_module(input.as_str())?;
    println!("\ndebug\n{:?}", module);
    Ok(())
}

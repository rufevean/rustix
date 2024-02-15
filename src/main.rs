use std::env;
use std::fs;

mod lexer;
mod parser;
use parser::parse;
fn main() {
    let args: Vec<String> = env::args().collect();
    let name = &args[1];
    let contents = fs::read_to_string(name);
    match contents {
        Ok(contents) => parse::parse(&contents),
        Err(e) => println!("Error reading {}: {}", name, e),
    }

}

use std::env;
use std::fs;
mod minify;
mod lexer;
mod parser;
use parser::parse;
fn main() {
    let args: Vec<String> = env::args().collect();
    let name = &args[1];
    let contents = fs::read_to_string(name);
    let minified_contents = minify::minify(&contents.unwrap());
    parse::parse(&minified_contents);
}

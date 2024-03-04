use std::env;
use std::fs;
mod minify;
mod lexer;
mod parser;
use parser::parse;
mod gen;
use gen::decl::get_outfile;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    
    let name = &args[1];
    match fs::read_to_string(name) {
        Ok(contents) => {
            parse::parse(&contents);
        }
        Err(_) => {
            println!("Error occurred while reading the file");
        }
    } 
    //todo -> creating a output file 
}


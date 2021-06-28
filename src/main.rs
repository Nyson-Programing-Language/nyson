mod lexer;
mod parser;
mod run;

use std::env;
use std::fs;

fn main() {
    let maybe_file = env::args().nth(1);
    let file = if let Some(f) = maybe_file {
        f
    } else {
        panic!("Expected file");
    };
    let maybe_contents = fs::read_to_string(file);
    let contents = if maybe_contents.is_ok() {
        maybe_contents.unwrap()
    } else {
        panic!("Could not open file for reading.");
    };
    println!("{:?}", contents);
    let to_parse = lexer::lexer(contents);
    run::run(to_parse);
    
}

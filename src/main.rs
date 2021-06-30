mod lexer;
mod run;

use std::env;
use std::fs;

fn main() {
    let maybe_file = env::args().nth(1);
    let mut args= env::args();
    let mut dev = false;
    for arg in args {
        if arg == "-dev" {
            dev = true;
        }
    }
    let file = if let Some(f) = maybe_file {
        f
    } else {
        panic!("Expected file");
    };
    let maybe_contents = fs::read_to_string(file);
    let mut contents = if maybe_contents.is_ok() {
        maybe_contents.unwrap()
    } else {
        panic!("Could not open file for reading.");
    };
    let mut space: String = " ".parse().unwrap();
    space.push_str(contents.as_str());
    contents = space;
    if dev {
        println!("{:?}", contents);
    }
    let to_parse = lexer::lexer(contents, dev);
    run::run(to_parse, dev, Vec::new(), Vec::new(), Vec::new());
}
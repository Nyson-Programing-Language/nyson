mod lexer;
mod run;

use std::env;
use std::fs;

#[allow(unused_variables)]
#[allow(unused)]

fn main() {
    let maybe_file = env::args().nth(1);
    let args= env::args();
    let mut dev = false;
    for arg in args {
        if arg == "-dev" {
            dev = true;
        }
    }
    let file = if let Some(f) = maybe_file {
        f
    } else {
        loop {
            println!(">> (type quit to quit)");
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line = line.trim().parse().unwrap();
            if line == "exit" || line == "quit" {
                std::process::exit(1);
            }
            else {
                let mut space: String = " ".parse().unwrap();
                space.push_str(&line);
                let contents = space;
                if dev {
                    println!("contents: {:?}", contents);
                }
                let to_parse = lexer::lexer(contents, dev);
                run::run(to_parse, dev, Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
            }
        }
    };
    let maybe_contents = fs::read_to_string(file);
    let mut contents = if maybe_contents.is_ok() {
        maybe_contents.unwrap()
    } else {
        panic!("Could not open file for reading.");
    };
    let mut space: String = " imp(\"https://raw.githubusercontent.com/Nyson-Programing-Language/nyson/main/src/main.nys\");".parse().unwrap();
    space.push_str(contents.as_str());
    contents = space;
    if dev {
        println!("contents: {:?}", contents);
    }
    let to_parse = lexer::lexer(contents, dev);
    run::run(to_parse, dev, Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
}
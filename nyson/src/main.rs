mod lexer;
mod run;

use std::env;
use std::fs;
use std::process::Command;

#[allow(unused_variables)]
#[allow(unused)]

fn main() {
    let mut dev = false;
    let to_parse = lexer::lexer(" imp(\"https://raw.githubusercontent.com/Nyson-Programing-Language/nyson/main/src/main.nyn\");dec str number: \"-1\";
dec str loading: \"\";
loop(7) {
    loading: \"\";
    loop(number) {
        loading: loading \"#\";
    }
    loop(5-number) {
        loading: loading \"-\";
    }
    log(loading);
    sleep(1000);
    number = math(number+1);
}".to_string(), dev);
        run::run(to_parse, dev, Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    }
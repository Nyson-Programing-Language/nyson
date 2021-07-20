#![allow(warnings, unused)]

mod lexer;
mod run;

use std::env;
use std::fs;
use std::fs::File;
use std::io::{Bytes, Write};
use std::process::Command;
extern crate pbr;

use pbr::ProgressBar;
use std::path::Path;
use std::thread;

#[allow(unused_variables)]
#[allow(unused)]

fn main() {
    let maybe_file = env::args().nth(1);
    let args = env::args();
    let mut dev = false;
    let mut compile = false;
    let mut hard = false;
    for arg in args {
        if arg == "-dev" {
            dev = true;
        } else if arg == "-compile" {
            compile = true;
        } else if arg == "-hard" {
            hard = true;
        } else if arg == "-help" {
            println!("                            HELP                            ");
            println!("------------------------------------------------------------");
            println!("| -help      - shows you help                              |");
            println!("| -dev       - shows you some dev stuff so if you make a   |");
            println!("|              issue on github we will need you to do this |");
            println!("| -compile   - compiles the code into a binary or exe      |");
            println!("| -hard      - (need -compile b4) but compiles with imp aka|");
            println!("|              apps offline if you use a imp from the web  |");
            println!("------------------------------------------------------------");
            std::process::exit(1);
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
            } else {
                let mut space: String = " ".parse().unwrap();
                space.push_str(&line);
                let contents = space;
                if dev {
                    println!("contents: {:?}", contents);
                }
                let to_parse = lexer::lexer(contents, dev);
                run::run(
                    to_parse,
                    dev,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                );
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
    if compile == false {
        let to_parse = lexer::lexer(contents, dev);
        run::run(
            to_parse,
            dev,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new()
        );
    } else {
        let mut pb;
        if hard == true {
            pb = ProgressBar::new(7);
        }
        else {
            pb = ProgressBar::new(6);
        }
        pb.inc();
        Command::new("git")
            .args([
                "clone",
                "https://github.com/Nyson-Programing-Language/nyson.git",
            ])
            .output()
            .expect("failed to execute process");
        pb.inc();
        if hard == true {
            contents = run::hard(
                lexer::lexer(contents.clone(), dev),
                dev,
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new()
            );
            pb.inc();
        }
        set_cont(
            "nyson/src/main.rs".to_string(),
            get_new_code(contents.clone()),
        );
        pb.inc();
        Command::new("cargo")
            .args([
                "build",
                "--release",
                "--manifest-path",
                "./nyson/Cargo.toml",
            ])
            .output()
            .expect("failed to execute process");
        pb.inc();
        copy(
            "nyson/target/release/nyson.exe".to_string(),
            "nysonProgram.exe".to_string(),
        );
        copy(
            "nyson/target/release/nyson".to_string(),
            "nysonProgram".to_string(),
        );
        pb.inc();
        delete("nyson".to_string());
        pb.inc();
        pb.finish_print("done");
    }
}

fn get_new_code(content: String) -> String {
    let mut ruturns = "mod lexer;
mod run;

use std::env;
use std::fs;
use std::process::Command;

#[allow(unused_variables)]
#[allow(unused)]

fn main() {
    let mut dev = false;
    let to_parse = lexer::lexer(\""
        .to_string();
    ruturns.push_str(content.replace("\"", "\\\"").as_str());
    ruturns.push_str("\".to_string(), dev);
        run::run(to_parse, dev, Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    }");
    return ruturns;
}

fn set_cont(loc: String, cont: String) -> std::io::Result<()> {
    let mut file = File::create(loc)?;
    file.write_all(cont.as_bytes())?;
    Ok(())
}

fn copy(path1: String, path2: String) -> std::io::Result<()> {
    fs::copy(path1, path2)?; // Copy foo.txt to bar.txt
    Ok(())
}

fn delete(Path: String) -> std::io::Result<()> {
    fs::remove_dir_all(Path)?;
    Ok(())
}
#![allow(warnings, unused)]

mod lexer;
mod run;

use std::env;
use std::fs;
use std::fs::File;
use std::io::{Bytes, Write};
use std::process::Command;
extern crate pbr;
extern crate serde_json;
use std::collections::HashMap;

use git2::Repository;
use pbr::ProgressBar;
use serde_json::{Map, Value};
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
        } else if arg == "-install" {
            fs::remove_dir_all("dep");
            fs::create_dir("dep");
            let json: serde_json::Value = serde_json::from_str(
                String::from_utf8_lossy((&fs::read_to_string("Nyson.json").unwrap()).as_ref())
                    .to_string()
                    .as_str(),
            )
            .expect("file should be proper JSON");
            let json: HashMap<String, String> =
                serde_json::from_str(json["dep"].to_string().as_str()).unwrap();
            for item in json {
                let mut new_path = "dep/".to_string();
                new_path.push_str(item.0.as_str());
                let url = item.1;
                let repo = match Repository::clone(url.as_str(), new_path.clone()) {
                    Ok(repo) => repo,
                    Err(e) => panic!("failed to clone: {}", e),
                };
            }
            std::process::exit(1);
        } else if arg == "-init" {
            set_cont("Nyson.json".to_string(), "{\n\t\"name\": \"My Program\",\n\t\"dep\": {\n\t\t\"Example\": \"https://github.com/Nyson-Programing-Language/example-dep.git\"\n\t}\n}".to_string());
            fs::create_dir("src");
            set_cont(
                "src/main.nys".to_string(),
                "log(\"Hello World\");".to_string(),
            );
            compile = true;
            std::process::exit(1);
        } else if arg == "-help" {
            println!("                            HELP                            ");
            println!("------------------------------------------------------------");
            println!("| -help      - shows you help                              |");
            println!("| -dev       - shows you some dev stuff so if you make a   |");
            println!("|              issue on github we will need you to do this |");
            println!("| -compile   - compiles the code into a binary or exe      |");
            println!("| -hard      - (need -compile b4) but compiles with imp aka|");
            println!("|              apps offline if you use a imp from the web  |");
            println!("| -init      - makes the init files                        |");
            println!("| -install   - install all the dependencies                |");
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
            Vec::new(),
        );
    } else {
        let mut pb;
        if hard == true {
            pb = ProgressBar::new(9);
        } else {
            pb = ProgressBar::new(8);
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
        let mut contents = lexer::lexer(contents.clone(), dev);
        pb.inc();
        if hard == true {
            contents = run::hard(
                contents,
                dev,
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            pb.inc();
        }
        for item in 0..contents.len() {
            contents[item] = contents[item].replace("\"", "\\\"");
        }
        pb.inc();
        set_cont(
            "nyson/src/main.rs".to_string(),
            get_new_code(contents.clone()),
        );
        pb.inc();
        let cargo = Command::new("cargo")
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

fn get_new_code(content: Vec<String>) -> String {
    let mut ruturns = "mod lexer;
mod run;

use std::env;
use std::fs;
use std::process::Command;

#[allow(unused_variables)]
#[allow(unused)]

fn main() {
    let mut dev = false;
    run::run([\""
        .to_string();
    ruturns.push_str(content.join("\", \"").as_str());
    ruturns.push_str("\"].to_vec().iter().map(|s| s.to_string()).collect(), dev, Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    }");
    return ruturns;
}

fn set_cont(loc: String, cont: String) -> std::io::Result<()> {
    let mut file = File::create(loc)?;
    file.write_all(cont.as_bytes())?;
    Ok(())
}

fn copy(path1: String, path2: String) -> std::io::Result<()> {
    fs::copy(path1, path2)?;
    Ok(())
}

fn delete(Path: String) -> std::io::Result<()> {
    fs::remove_dir_all(Path)?;
    Ok(())
}

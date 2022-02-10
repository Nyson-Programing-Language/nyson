mod lexer;
mod run;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;

extern crate pbr;
extern crate serde_json;
use clap::*;
use std::collections::HashMap;

use git2::Repository;
use pbr::ProgressBar;
use std::path::Path;

fn main() {
    if !Path::new("dep").exists() {
        let r = fs::create_dir("dep");
        if r.is_err() {
            run::error("Could not create dir.".to_string());
        }
    }
    let uses: Vec<String> = vec!["false".to_string(), "false".to_string()];
    let matches = App::new("Nyson")
        .version(env!("CARGO_PKG_VERSION"))
        .about("a programing language made in rust")
        .arg(
            Arg::with_name("INPUT")
                .help("the file to run")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("dev")
                .short("v")
                .long("verbose")
                .help("Gives you dev debug tools")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("compile")
                .short("c")
                .long("compile")
                .help("Compiles your program")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("hard")
                .short("h")
                .long("hard")
                .help("compiles the language to offline mode")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("run")
                .short("r")
                .long("run")
                .help("Runs the program")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("install")
                .short("i")
                .long("install")
                .help("installs all the dependencies")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("init")
                .long("init")
                .help("makes a new project")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .help("sets the output file name")
                .takes_value(true),
        )
        .get_matches();
    let compile = matches.is_present("compile");
    let _hard = matches.is_present("hard");
    let run = matches.is_present("run");
    let dev = matches.is_present("dev");
    let file;
    if matches.is_present("INPUT") || run {
        file = matches.value_of("INPUT").unwrap().to_string();
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
                let _output = run::run(
                    to_parse,
                    dev,
                    uses.clone(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                );
                //run and delete
            }
        }
    };
    let maybe_contents;
    if run {
        maybe_contents = fs::read_to_string("src/main.nys")
    } else {
        maybe_contents = fs::read_to_string(file)
    }
    let mut contents = if maybe_contents.is_ok() {
        maybe_contents.unwrap()
    } else {
        run::error("Could not open file for reading.".to_string());
        "".to_string()
    };
    let mut space: String = " ".to_string();
    space.push_str(contents.as_str());
    contents = space;
    if dev {
        println!("contents: {:?}", contents);
    }
    if !compile {
        let to_parse = lexer::lexer(contents, dev);
        run_code(
            run::run(
                to_parse,
                dev,
                uses,
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
            dev,
        )
        .unwrap();
        // run and delete
    } else {
        let to_parse = lexer::lexer(contents, dev);
        run_code(
            run::run(
                to_parse,
                dev,
                uses,
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
            dev,
        )
        .unwrap();
        if matches.is_present("output") {
            if Path::new("./nyson").exists() {
                fs::rename("nyson", matches.value_of("output").unwrap().to_string()).unwrap();
            } else if Path::new("./nyson.exe").exists() {
                fs::rename("nyson.exe", matches.value_of("output").unwrap().to_string()).unwrap();
            }
        }
    }
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

fn delete(path: String) -> std::io::Result<()> {
    fs::remove_dir_all(path)?;
    Ok(())
}

fn loop_throught_dir(dir: &Path) {
    if dir.is_dir() {
        let folder = fs::read_dir(dir).unwrap();
        let mut pb = ProgressBar::new(fs::read_dir(dir).unwrap().count() as u64);
        for item in folder {
            let path = item.unwrap().path();
            if path.is_dir() {
                make_path(path.as_path().to_string_lossy().to_string());
                pb.inc();
            }
        }
    }
}

fn make_path(path: String) {
    let mut path_made = path.clone();
    if !path.is_empty() {
        path_made.push_str("/dep");
    } else {
        path_made.push_str("dep")
    }
    let r = fs::remove_dir_all(path_made.clone());
    if r.is_err() {
        run::error("Could not delete dir.".to_string());
    }
    let r = fs::create_dir(path_made);
    if r.is_err() {
        run::error("Could not create dir.".to_string());
    }
    let mut path_made = path.clone();
    if !path.is_empty() {
        path_made.push_str("/Nyson.json");
    } else {
        path_made.push_str("Nyson.json")
    }
    let json: serde_json::Value = serde_json::from_str(
        String::from_utf8_lossy(fs::read_to_string(path_made).unwrap().as_ref())
            .to_string()
            .as_str(),
    )
    .expect("file should be proper JSON");
    let json: HashMap<String, String> =
        serde_json::from_str(json["dep"].to_string().as_str()).unwrap();
    let mut pb = ProgressBar::new(json.len() as u64);
    for item in json {
        let mut new_path = path.clone();
        if !path.is_empty() {
            new_path.push_str("/dep/");
        } else {
            new_path.push_str("dep/");
        }
        new_path.push_str(item.0.as_str());
        let url = item.1;
        let _repo = match Repository::clone(url.as_str(), new_path.clone()) {
            Ok(repo) => repo,
            Err(e) => {
                run::error(["failed to clone: ", e.to_string().as_str()].join(""));
                Repository::clone(url.as_str(), new_path.clone()).unwrap()
            }
        };
        pb.inc();
    }
    let mut new_path = path.clone();
    if !path.is_empty() {
        new_path.push_str("/dep/");
    } else {
        new_path.push_str("dep/");
    }
    loop_throught_dir(new_path.as_ref());
}

fn run_code(input: String, dev: bool) -> std::io::Result<()> {
    let mut nyson_rs = std::env::temp_dir();
    nyson_rs.push("nyson.rs");
    let mut file = File::create(nyson_rs.to_str().unwrap())?;
    file.write_all(input.as_bytes())?;
    let output = Command::new("rustc")
        .arg("-O")
        .arg("-A")
        .arg("dead_code")
        .arg(nyson_rs.to_str().unwrap())
        .output()
        .expect("command failed to start");
    if !output.status.success() || dev {
        println!("code: {}", input);
        println!("version: {}", env!("CARGO_PKG_VERSION"));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    if !output.status.success() {
        println!("\x1b[0;31mI GOT AN ERROR (please make an issue on the github page with this output :) its https://github.com/Nyson-Programing-Language/nyson)\x1b[0m");
    }
    fs::remove_file(nyson_rs.to_str().unwrap())?;
    Ok(())
}

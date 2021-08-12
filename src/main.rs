mod lexer;
mod run;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;
extern crate pbr;
extern crate serde_json;
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
    let maybe_file = env::args().nth(1);
    let args = env::args();
    let mut dev = false;
    let mut compile = false;
    let mut hard = false;
    let mut run = false;
    for arg in args {
        if arg == "--dev" {
            dev = true;
        } else if arg == "--compile" {
            compile = true;
        } else if arg == "--hard" || arg == "-h" {
            hard = true;
        } else if arg == "run" {
            run = true;
        } else if arg == "--install" || arg == "-i" {
            if Path::new(".gitignore").exists() {
                let mut cont = fs::read_to_string(".gitignore").unwrap();
                cont.push_str("\n\n# Nyson\ndep");
                let _r = set_cont(".gitignore".to_string(), cont);
            } else {
                let r = set_cont(".gitignore".to_string(), "# Nyson\ndep".to_string());
                if r.is_err() {
                    run::error("Could not set file contents.".to_string());
                }
            }
            make_path("".to_string());
            std::process::exit(1);
        } else if arg == "--init" {
            let r = set_cont("Nyson.json".to_string(), "{\n\t\"name\": \"My Program\",\n\t\"dep\": {\n\t\t\"Example\": \"https://github.com/Nyson-Programing-Language/example-dep.git\"\n\t}\n}".to_string());
            if r.is_err() {
                run::error("Could not set file contents.".to_string());
            }
            let r = fs::create_dir("src");
            if r.is_err() {
                run::error("Could not create dir.".to_string());
            }
            let r = set_cont(
                "src/main.nys".to_string(),
                "log(\"Hello World\");".to_string(),
            );
            if r.is_err() {
                run::error("Could not set file contents.".to_string());
            }
            std::process::exit(1);
        } else if arg == "--help" || arg == "-h" {
            println!("                            HELP                            ");
            println!("-------------------------------------------------------------");
            println!("| --help      - shows you help                              |");
            println!("| --dev       - shows you some dev stuff so if you make a   |");
            println!("|              issue on github we will need you to do this  |");
            println!("| --compile   - compiles the code into a binary or exe      |");
            println!("| --hard      - (need -compile b4) but compiles with imp aka|");
            println!("|              apps offline if you use a imp from the web   |");
            println!("| --init      - makes the init files                        |");
            println!("| --install   - install all the dependencies                |");
            println!("| run        - runs the main.nys                            |");
            println!("-------------------------------------------------------------");
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
                let _output = run::run(
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
        let _output = run::run(
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
        if hard {
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
        let mut contents = lexer::lexer(contents, dev);
        pb.inc();
        if hard {
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
        let r = set_cont("nyson/src/main.rs".to_string(), get_new_code(contents));
        if r.is_err() {
            run::error("Could not set file contents.".to_string());
        }
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
        let _r = copy(
            "nyson/target/release/nyson.exe".to_string(),
            "nysonProgram.exe".to_string(),
        );
        let _r = copy(
            "nyson/target/release/nyson".to_string(),
            "nysonProgram".to_string(),
        );
        pb.inc();
        let r = delete("nyson".to_string());
        if r.is_err() {
            run::error("Could not delete file.".to_string());
        }
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
    let dev = false;
    let output = run::run([r\""
        .to_string();
    ruturns.push_str(content.join("\", r\"").as_str());
    ruturns.push_str("\"].to_vec().iter().map(|s| s.to_string()).collect(), dev, Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    }");
    ruturns.to_string()
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
        String::from_utf8_lossy((&fs::read_to_string(path_made).unwrap()).as_ref())
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
            Err(e) => {run::error(["failed to clone: ", e.to_string().as_str()].join("")); Repository::clone(url.as_str(), new_path.clone()).unwrap()},
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

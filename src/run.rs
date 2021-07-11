#![allow(warnings, unused)]
use std::ops::{Add, Sub, Mul, Div};
use rand::Rng;
use crate::lexer;
use std::{fs, env};
use std::thread;
use std::fs::File;
use std::io::{Write, stdout, Read, BufReader};
use std::str::{SplitWhitespace, Split};
use std::process::Command;
use curl::easy::Easy;
use std::io::{stdin};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;

#[allow(unused)]

pub fn run(mut contents: Vec<String>, dev: bool, mut memory_names: Vec<String>, mut memory_values: Vec<String>, mut memory_types: Vec<String>, mut func_names: Vec<String>, mut func_par: Vec<String>, mut func_code: Vec<String>) {
    if dev {
        println!("contents: {:?}", contents);
    }
    let mut quotes = 0;
    let mut squigle = 0;
    let mut readfrom = 0;
    let mut skiperwiper = false;
    let mut read = true;
    let mut threads = Vec::new();
    let mut group_memory: Vec<String> = Vec::new();
    while read {
        read = false;
        skiperwiper = false;
        for mut x in readfrom..contents.len() {
            if skiperwiper == false {
                if (contents[x] == "\"" || contents[x] == "\'" || contents[x] == r"\`") && contents[x-1] != "\\" {
                    quotes = quotes + 1;
                }
                if (contents[x] == "{" || contents[x] == "[") && quotes%2 == 0 {
                    squigle = squigle + 1;
                }
                if (contents[x] == "}" || contents[x] == "]") && quotes%2 == 0 {
                    squigle = squigle - 1;
                }
                if quotes%2 == 0 && squigle == 0 {
                    if contents[x] == "log" {
                        log(x, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev);
                    }
                    else if contents[x] == "audio" {
                        let contents_save = contents.clone();
                        let x_save = x.clone();
                        let memory_types_save = memory_types.clone();
                        let memory_values_save = memory_values.clone();
                        let memory_names_save = memory_names.clone();
                        let dev_save = dev.clone();
                        let handle = thread::spawn(move || {
                            let mut vec:Vec<String> = Vec::new();
                            let mut skip = false;
                            let mut n = 0;
                            for y in x+1..contents_save.len() {
                                if skip == false {
                                    if contents_save[x+1] != "(" {
                                        println!("You have to put a parentheses after a log");
                                        std::process::exit(1);
                                    }
                                    if contents_save[y] == "(" {
                                        n = n +1;
                                    }
                                    else if contents_save[y] == ")" {
                                        n = n-1;
                                    }
                                    if n == 0 {
                                        skip = true;
                                        for z in x+1..y+1 {
                                            vec.push((&contents_save[z]).parse().unwrap());
                                        }
                                    }
                                }
                            }
                            if dev {
                                println!("vec: {:?}",  vec);
                            }
                            let mut z = 0;
                            for y in vec.to_vec() {
                                if y == "(" || y == ")" {
                                    z = z + 1;
                                }
                            }
                            skip = false;
                            let mut string: String = "".to_string();
                            let mut n = 0;
                            let mut n1 = 1;
                            let mut skips = 0;
                            for y in 1..vec.len() {
                                if skips == 0 {
                                    if skip == false {
                                        if (vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`") && vec[y-1] != "\\" {
                                            n = n + 1;
                                        }else if vec[y] == "(" && n % 2 == 0 {
                                            n1 = n1 + 1;
                                        }
                                        else if vec[y] == ")" && n % 2 == 0 {
                                            n1 = n1 - 1;
                                        }else if n % 2 == 1 {
                                            string.push_str(vec[y].as_str());
                                        } else if vec[y] == "math" {
                                            string.push_str(math(y, vec.to_vec(), memory_names_save.clone(), memory_values_save.clone(), memory_types_save.clone(), dev_save).to_string().as_str());
                                            let mut leng = 0;
                                            let mut n2 = 0;
                                            let mut skip1 = false;
                                            for f in y+1..vec.len() {
                                                if skip1 == false {
                                                    if vec[y+1] != "(" {
                                                        println!("You have to put a parentheses after a log");
                                                        std::process::exit(1);
                                                    }
                                                    if vec[f] == "(" {
                                                        n2 = n2 +1;
                                                    }
                                                    else if vec[f] == ")" {
                                                        n2 = n2-1;
                                                    }
                                                    if n2 == 0 {
                                                        skip1 = true;
                                                        for z in y+1..f+1 {
                                                            leng = leng + 1;
                                                        }
                                                    }
                                                }
                                            }
                                            skips = leng;
                                        } else if vec[y] == "round" {
                                            string.push_str(round(y, vec.to_vec(), memory_names_save.clone(), memory_values_save.clone(), memory_types_save.clone(), dev_save).to_string().as_str());
                                            let mut leng = 0;
                                            let mut n2 = 0;
                                            let mut skip1 = false;
                                            for f in y+1..vec.len() {
                                                if skip1 == false {
                                                    if vec[y+1] != "(" {
                                                        println!("You have to put a parentheses after a log");
                                                        std::process::exit(1);
                                                    }
                                                    if contents_save[f] == "(" {
                                                        n2 = n2 +1;
                                                    }
                                                    else if contents_save[f] == ")" {
                                                        n2 = n2-1;
                                                    }
                                                    if n2 == 0 {
                                                        skip1 = true;
                                                        for z in y+1..f+1 {
                                                            leng = leng + 1;
                                                        }
                                                    }
                                                }
                                            }
                                            skips = leng;
                                        } else if vec[y] == "GET" {
                                            string.push_str(get_request(y, vec.to_vec(), memory_names_save.clone(), memory_values_save.clone(), memory_types_save.clone(), dev_save).to_string().as_str());
                                            let mut leng = 0;
                                            let mut n2 = 0;
                                            let mut skip1 = false;
                                            for f in y+1..vec.len() {
                                                if skip1 == false {
                                                    if vec[y+1] != "(" {
                                                        println!("You have to put a parentheses after a log");
                                                        std::process::exit(1);
                                                    }
                                                    if contents_save[f] == "(" {
                                                        n2 = n2 +1;
                                                    }
                                                    else if contents_save[f] == ")" {
                                                        n2 = n2-1;
                                                    }
                                                    if n2 == 0 {
                                                        skip1 = true;
                                                        for z in y+1..f+1 {
                                                            leng = leng + 1;
                                                        }
                                                    }
                                                }
                                            }
                                            skips = leng;
                                        } else if vec[y] == "replace" {
                                            string.push_str(replace(y, vec.to_vec(), memory_names_save.clone(), memory_values_save.clone(), memory_types_save.clone(), dev_save).to_string().as_str());
                                            let mut leng = 0;
                                            let mut n2 = 0;
                                            let mut skip1 = false;
                                            for f in y+1..vec.len() {
                                                if skip1 == false {
                                                    if vec[y+1] != "(" {
                                                        println!("You have to put a parentheses after a log");
                                                        std::process::exit(1);
                                                    }
                                                    if vec[f] == "(" {
                                                        n2 = n2 +1;
                                                    }
                                                    else if vec[f] == ")" {
                                                        n2 = n2-1;
                                                    }
                                                    if n2 == 0 {
                                                        skip1 = true;
                                                        for z in y+1..f+1 {
                                                            leng = leng + 1;
                                                        }
                                                    }
                                                }
                                            }
                                            skips = leng;
                                        } else if vec[y] == "input" {
                                            string.push_str(input(y, vec.to_vec(), memory_names_save.clone(), memory_values_save.clone(), memory_types_save.clone(), dev_save).to_string().as_str());
                                            let mut leng = 0;
                                            let mut n2 = 0;
                                            let mut skip1 = false;
                                            for f in y+1..vec.len() {
                                                if skip1 == false {
                                                    if vec[y+1] != "(" {
                                                        println!("You have to put a parentheses after a log");
                                                        std::process::exit(1);
                                                    }
                                                    if vec[f] == "(" {
                                                        n2 = n2 +1;
                                                    }
                                                    else if vec[f] == ")" {
                                                        n2 = n2-1;
                                                    }
                                                    if n2 == 0 {
                                                        skip1 = true;
                                                        for z in y+1..f+1 {
                                                            leng = leng + 1;
                                                        }
                                                    }
                                                }
                                            }
                                            skips = leng;
                                        } else if vec[y] == "exec" {
                                            string.push_str(exec(y, vec.to_vec(), memory_names_save.clone(), memory_values_save.clone(), memory_types_save.clone(), dev_save).to_string().as_str());
                                            let mut leng = 0;
                                            let mut n2 = 0;
                                            let mut skip1 = false;
                                            for f in y+1..vec.len() {
                                                if skip1 == false {
                                                    if vec[y+1] != "(" {
                                                        println!("You have to put a parentheses after a log");
                                                        std::process::exit(1);
                                                    }
                                                    if vec[f] == "(" {
                                                        n2 = n2 +1;
                                                    }
                                                    else if vec[f] == ")" {
                                                        n2 = n2-1;
                                                    }
                                                    if n2 == 0 {
                                                        skip1 = true;
                                                        for z in y+1..f+1 {
                                                            leng = leng + 1;
                                                        }
                                                    }
                                                }
                                            }
                                            skips = leng;
                                        } else if vec[y] == "trim" {
                                            string.push_str(trim(y, vec.to_vec(), memory_names_save.clone(), memory_values_save.clone(), memory_types_save.clone(), dev).to_string().as_str());
                                            let mut leng = 0;
                                            let mut n2 = 0;
                                            let mut skip1 = false;
                                            for f in y+1..vec.len() {
                                                if skip1 == false {
                                                    if vec[y+1] != "(" {
                                                        println!("You have to put a parentheses after a log");
                                                        std::process::exit(1);
                                                    }
                                                    if vec[f] == "(" {
                                                        n2 = n2 +1;
                                                    }
                                                    else if vec[f] == ")" {
                                                        n2 = n2-1;
                                                    }
                                                    if n2 == 0 {
                                                        skip1 = true;
                                                        for z in y+1..f+1 {
                                                            leng = leng + 1;
                                                        }
                                                    }
                                                }
                                            }
                                            skips = leng;
                                        } else if vec[y] == "timeh" {
                                            string.push_str(time_readable(y, vec.to_vec(), memory_names_save.clone(), memory_values_save.clone(), memory_types_save.clone(), dev).to_string().as_str());
                                            let mut leng = 0;
                                            let mut n2 = 0;
                                            let mut skip1 = false;
                                            for f in y+1..vec.len() {
                                                if skip1 == false {
                                                    if vec[y+1] != "(" {
                                                        println!("You have to put a parentheses after a log");
                                                        std::process::exit(1);
                                                    }
                                                    if vec[f] == "(" {
                                                        n2 = n2 +1;
                                                    }
                                                    else if vec[f] == ")" {
                                                        n2 = n2-1;
                                                    }
                                                    if n2 == 0 {
                                                        skip1 = true;
                                                        for z in y+1..f+1 {
                                                            leng = leng + 1;
                                                        }
                                                    }
                                                }
                                            }
                                            skips = leng;
                                        } else if vec[y] == "time" {
                                            string.push_str(time(y, vec.to_vec(), memory_names_save.clone(), memory_values_save.clone(), memory_types_save.clone(), dev).to_string().as_str());
                                            let mut leng = 0;
                                            let mut n2 = 0;
                                            let mut skip1 = false;
                                            for f in y+1..vec.len() {
                                                if skip1 == false {
                                                    if vec[y+1] != "(" {
                                                        println!("You have to put a parentheses after a log");
                                                        std::process::exit(1);
                                                    }
                                                    if vec[f] == "(" {
                                                        n2 = n2 +1;
                                                    }
                                                    else if vec[f] == ")" {
                                                        n2 = n2-1;
                                                    }
                                                    if n2 == 0 {
                                                        skip1 = true;
                                                        for z in y+1..f+1 {
                                                            leng = leng + 1;
                                                        }
                                                    }
                                                }
                                            }
                                            skips = leng;
                                        } else if vec[y] == "getcont" {
                                            string.push_str(get_contents(y, vec.to_vec(), memory_names_save.clone(), memory_values_save.clone(), memory_types_save.clone(), dev_save).to_string().as_str());
                                            let mut leng = 0;
                                            let mut n2 = 0;
                                            let mut skip1 = false;
                                            for f in y+1..vec.len() {
                                                if skip1 == false {
                                                    if vec[y+1] != "(" {
                                                        println!("You have to put a parentheses after a log");
                                                        std::process::exit(1);
                                                    }
                                                    if vec[f] == "(" {
                                                        n2 = n2 +1;
                                                    }
                                                    else if vec[f] == ")" {
                                                        n2 = n2-1;
                                                    }
                                                    if n2 == 0 {
                                                        skip1 = true;
                                                        for z in y+1..f+1 {
                                                            leng = leng + 1;
                                                        }
                                                    }
                                                }
                                            }
                                            skips = leng;
                                        } else {
                                            let mut postion = memory_names_save.len();
                                            let mut skip1 = false;
                                            for pos in 0..memory_names_save.len() {
                                                if skip1 == false {
                                                    if memory_names_save[pos].to_string() == vec[y].to_string() {
                                                        postion = pos;
                                                        skip1 = true;
                                                    }
                                                }
                                            }
                                            if postion != memory_names_save.len() {
                                                string.push_str(&*memory_values_save[postion].to_string());
                                            }
                                        }
                                    }
                                }
                                else {
                                    skips = skips -1;
                                }
                            }
                            let stringreturn = string;
                            let mut vecs = stringreturn.replace("\n", " ");
                            vecs = vecs.replace("\t", " ");
                            let mut endvec: Vec<&str> = vecs.split(" ").collect();
                            use std::env;
                            if env::consts::OS == "linux" {
                                let mut vecs = stringreturn.replace("\n", " ");
                                vecs = vecs.replace("\t", " ");
                                let mut endvec: Vec<&str> = vecs.split(" ").collect();
                                Command::new("cvlc")
                                    .args(endvec)
                                    .output()
                                    .expect("failed to execute process");
                            }
                            else if env::consts::OS == "windows" {
                                let mut endvec: Vec<&str> = Vec::new();
                                endvec.push("/C");
                                let mut endstirng: String = r"'%PROGRAMFILES%\VideoLAN\VLC\vlc.exe' -I dummy --dummy-quiet ".to_string();
                                endstirng.push_str(&stringreturn);
                                println!("{:?}", endstirng);
                                endvec.push(&endstirng);
                                endvec.push("-I");
                                endvec.push("dummy");
                                endvec.push("--dummy-quiet");
                                endvec.push(&stringreturn);
                                println!("{:?}", endvec);
                                Command::new("cmd")
                                    .args(endvec)
                                    .output()
                                    .expect("failed to execute process");
                            }
                            else if env::consts::OS == "macos" {
                                let mut vecs = stringreturn.replace("\n", " ");
                                vecs = vecs.replace("\t", " ");
                                let mut endvec: Vec<&str> = Vec::new();
                                endvec.push("-I");
                                endvec.push("rc");
                                for q in vecs.split(" ") {
                                    endvec.push(q);
                                }
                                Command::new("/Applications/VLC.app/Contents/MacOS/VLC")
                                    .args(endvec)
                                    .output()
                                    .expect("failed to execute process");
                            }
                        });
                        threads.push(handle);
                    }
                    else if contents[x] == "loop" {
                        readfrom = x+1;
                        skiperwiper = true;
                        read = true;
                        let mut vec:Vec<String> = Vec::new();
                        let mut skip = false;
                        let number_of_times = math(x, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev);
                        if number_of_times > 0 as f32 {
                            let mut n = 0;
                            let mut reached = false;
                            let mut loc1 = 0;
                            let mut loc2 = 0;
                            for y in x+1..contents.len() {
                                if skip == false {
                                    if contents[y] == "{" {
                                        n = n +1;
                                        reached = true;
                                        loc1 = y;
                                    }
                                    else if contents[y] == "}" {
                                        n = n-1;
                                    }
                                    if n > 0 {
                                        vec.push((&contents[y]).parse().unwrap());
                                    }
                                    else if reached == true {
                                        skip = true;
                                        loc2 = y;
                                    }
                                }
                            }
                            vec.remove(0);
                            let mut newvec = Vec::new();
                            for t in 0..contents.clone().len() {
                                if t == loc2 {
                                    for q in 1..number_of_times.round() as i32 {
                                        for y in vec.clone() {
                                            newvec.push(y);
                                        }
                                    }
                                }
                                else {
                                    newvec.push(contents[t].clone());
                                }
                            }
                            newvec.remove(loc1);
                            if dev {
                                println!("newvec: {:?}", newvec);
                            }
                            contents = newvec;
                        }
                    }
                    else if contents[x] == "while" {
                        readfrom = x;
                        skiperwiper = true;
                        read = true;
                        let mut vec:Vec<String> = Vec::new();
                        let mut skip = false;
                        let mut n = 0;
                        let mut reached = false;
                        let mut loc1 = 0;
                        let mut loc2 = 0;
                        for y in x+1..contents.len() {
                            if skip == false {
                                if contents[y] == "{" {
                                    n = n +1;
                                    reached = true;
                                    loc1 = y;
                                }
                                else if contents[y] == "}" {
                                    n = n-1;
                                }
                                if n > 0 {
                                    vec.push((&contents[y]).parse().unwrap());
                                }
                                else if reached == true {
                                    skip = true;
                                    loc2 = y;
                                }
                            }
                        }
                        let mut newvec = Vec::new();
                        for t in 0..contents.clone().len() {
                            if t == x {
                                newvec.push("if".to_string())
                            }
                            else if t == loc2 {
                                newvec.push(contents[loc2].clone());
                                for q in x..loc2+1 {
                                    newvec.push(contents[q].clone());
                                }
                            }
                            else {
                                newvec.push(contents[t].clone());
                            }
                        }
                        if dev {
                            println!("newvec: {:?}", newvec);
                        }
                        contents = newvec;
                    }
                    else if contents[x] == "sleep" {
                        let number_of_times = math(x, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev);
                        thread::sleep_ms(number_of_times as u32);
                    }else if contents[x] == "exec" {
                        let stringreturn = exec(x, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev);
                    }
                    else if contents[x] == "setcont" {
                        set_contents(x, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev);
                    }
                    else if contents[x] == "POST" {
                        post_request(x, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev);
                    }
                    else if contents[x] == "func" {
                        let vec:Vec<String> = Vec::new();
                        let mut skip = false;
                        let mut n = 1;
                        let mut reached = false;
                        let mut name:String = "".parse().unwrap();
                        for y in x+2..contents.len() {
                            if skip == false {
                                if contents[y] == "(" {
                                    n = n -1;
                                    reached = true;
                                }
                                else if contents[y] == ")" {
                                    n = n-1;
                                }
                                if n > 0 {
                                    name.push_str(&contents[y]);
                                }
                                else if reached == true {
                                    skip = true;
                                }
                            }
                        }
                        let mut code:String = "".parse().unwrap();
                        skip = false;
                        n = 0;
                        reached = false;
                        for y in x+1..contents.len() {
                            if skip == false {
                                if contents[y] == "}" {
                                    n = n-1;
                                }
                                if n > 0 {
                                    code.push_str(&contents[y]);
                                }
                                else if reached == true {
                                    skip = true;
                                }
                                if contents[y] == "{" {
                                    n = n +1;
                                    reached = true;
                                }
                            }
                        }
                        let mut par:String = "".parse().unwrap();
                        skip = false;
                        n = 0;
                        reached = false;
                        for y in x+2..contents.len() {
                            if skip == false {
                                if contents[y] == ")" {
                                    n = n-1;
                                }
                                if n > 0 {
                                    par.push_str(&contents[y]);
                                }
                                else if reached == true {
                                    skip = true;
                                }
                                if contents[y] == "(" {
                                    n = n +1;
                                    reached = true;
                                }
                            }
                        }
                        if dev {
                            println!("par: {}", par);
                            println!("code: {}", code);
                            println!("name: {}", name);
                        }
                        func_par.push(par);
                        func_code.push(code);
                        func_names.push(name);
                        if dev {
                            println!("func_par: {:?}", func_par);
                            println!("func_code: {:?}", func_code);
                            println!("func_names: {:?}", func_names);
                        }
                    }
                    else if contents[x] == "imp" {
                        let imp = imp(x, contents.clone(), dev);
                        readfrom = x;
                        skiperwiper = true;
                        read = true;
                        let mut delete = Vec::new();
                        let mut deleted = 0;
                        let mut skirt = false;
                        let mut n3 = 0;
                        delete.push(x);
                        for y1 in x+1..contents.len() {
                            if skirt == false {
                                if contents[y1] == "(" {
                                    n3 = n3 + 1;
                                }
                                if n3 == 0 {
                                    skirt = true;
                                }
                                if contents[y1] == ")" {
                                    n3 = n3 - 1;
                                }
                                delete.push(y1);
                            }
                        }
                        for item in delete {
                            contents.remove(item - deleted);
                            deleted = deleted + 1 ;
                        }
                        let mut newVec = Vec::new();
                        for itom in 0..contents.len() {
                            if itom == x {
                                for item in imp.clone() {
                                    newVec.push(item);
                                }
                            }
                            newVec.push(contents[itom].clone());
                        }
                        contents = newVec;
                    }
                    else if contents[x] == "dec" {
                        let memory_names_save = memory_names.clone();
                        let memory_types_save = memory_types.clone();
                        let memory_values_save = memory_values.clone();
                        let mut types = false;
                        let mut position = x+1;
                        let mut group = false;
                        let mut square_brackets = 0;
                        if contents[position] == "int" {
                            memory_types.push(String::from("int"));
                            memory_names.push(String::from(contents[position+1].clone()));
                            position = position + 1;
                        } else if contents[position] == "str"  {
                            memory_types.push(String::from("str"));
                            memory_names.push(String::from(contents[position+1].clone()));
                            position = position + 1;

                        } else if contents[position] == "array"  {
                            memory_types.push(String::from("array"));
                            memory_names.push(String::from(contents[position+1].clone()));
                            position = position + 1;

                        } 
                        else if contents[position] == "grp"  {
                            memory_types.push(String::from("grp"));
                            memory_names.push(String::from(contents[position+1].clone()));
                            position = position + 1;
                        }
                        else if contents[position] == "inf"  {
                            memory_types.push(String::from("inf"));
                            memory_names.push(String::from(contents[position+1].clone()));
                            position = position + 1;
                        }
                        else if contents[position] == "anon"  {
                            memory_types.push(String::from("anon"));
                            types = true;
                        }
                        let mut clone_class = String::from("");
                        let mut value = String::new();
                        let mut value_array = Vec::new();
                        let mut value_group = Vec::new();
                        let mut n = 0;
                        let mut quote = 0;
                        let mut squig = 0;
                        position = position+2;
                        let mut group = false;
                        loop {
                            if contents[position] == "[" {
                                value_array = array_fn(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev);
                                break;
                            }
                            else if contents[position] == "{" {
                                value_group = group_fn(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev);
                                group = true;
                                squig = squig+1;
                            }
                            else if contents[position] == "}" {
                                squig = squig-1;
                                if group == true && squig == 0 && contents[position+1] == "," {
                                    clone_class = contents[position+2].clone().to_string();
                                }
                            }
                            else{
                                if square_brackets == 0 {
                                    if contents[position] == ";" {
                                        if dev {
                                            println!("contents[x+move_up+move_up+move_up_up+move_final]: {:?}", contents[position]);
                                        }
                                        break;
                                    }
                                    else if group == false {
                                        if (contents[position] == "\"" || contents[position] == "\'" || contents[position] == r"\`") && contents[position-1] != "\\" {
                                            quote = quote + 1;
                                        }

                                        else {
                                            if contents[position] == "math" {
                                                value.push_str(math(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                                                n = 1;
                                            }
                                            else if contents[position] == "round" {
                                                value.push_str(round(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                n = 1;
                                            }
                                            else if contents[position] == "replace" {
                                                value.push_str(replace(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                n = 1;
                                            }
                                            else if contents[position] == "input" {
                                                value.push_str(input(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                n = 1;
                                            }
                                            else if contents[position] == "exec" {
                                                value.push_str(exec(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                n = 1;
                                            }
                                            else if contents[position] == "trim" {
                                                value.push_str(trim(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                n = 1;
                                            }
                                            else if contents[position] == "timeh" {
                                                value.push_str(time_readable(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                n = 1;
                                            }
                                            else if contents[position] == "time" {
                                                value.push_str(time(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                n = 1;
                                            }
                                            else if contents[position] == "getcont" {
                                                value.push_str(get_contents(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                n = 1;
                                            }
                                            else {
                                                if n == 0 {
                                                    if quote%2 == 1 {
                                                        value.push_str(contents[position].as_str());
                                                    }
                                                    else {
                                                        let mut positions = memory_names_save.len();
                                                        let mut skip = false;
                                                        for pos in 0..memory_names_save.len() {
                                                            if skip == false {
                                                                if memory_names_save[pos].to_string() == contents[position].to_string() {
                                                                    positions = pos;
                                                                    skip = true;
                                                                }
                                                            }
                                                        }
                                                        if positions != memory_names_save.len() && (contents[x+1].trim() == ":" || contents[x+1].trim() == "=") {
                                                            value.push_str(memory_values_save[positions].to_string().as_str());
                                                        }
                                                        else {
                                                            value.push_str(contents[position].as_str());
                                                        }
                                                    }
                                                }
                                            }
                                            if n >= 1 && contents[position] == "(" {
                                                n = n + 1
                                            }
                                            else if n >= 1 && contents[position] == ")" {
                                                n = n - 1;
                                                if n == 1 {
                                                    n = 0;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            position = position+1;
                            if dev {
                                println!("position: {:?}", position);
                            }
                        }
                        if value_array.join("") != "" {
                            memory_values.push(value_array.join("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v").clone());
                        } else if value_group.join("") != ""{
                            value_group.push(clone_class.clone());
                            memory_values.push(value_group.join("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v").clone());
                            let name_of_item = memory_names[memory_names.len()-1].clone();
                            for d in 0..value_group.len()-1 {
                                let mut name:String = name_of_item.to_string();
                                name.push_str(".");
                                let mut location = 0;
                                for items in 0..group_memory.len() {
                                    if items < group_memory.len()-1 {
                                        if group_memory[items+1].parse::<i32>().is_ok() && group_memory[items] == clone_class.clone() {
                                            location = items + (d*2) + 3;
                                        }
                                    }
                                }
                                name.push_str(&*group_memory[location]);
                                memory_names.push(name.clone());
                                memory_values.push(value_group[d].clone());
                                memory_types.push("str".parse().unwrap());
                                
                            }
                        }
                        else {
                            memory_values.push(value.clone());
                        }
                        
                        if types {
                            memory_names.push(value.clone());
                        }
                        if dev {
                            println!("memory_names: {:?}", memory_names);
                            println!("memory_types: {:?}", memory_types);
                            println!("memory_values: {:?}", memory_values);
                        }
                    }
                    else if contents[x] == "group" {
                        let build_name = String::from(contents[x+1].clone());
                        let mut end_pos: usize = 0;
                        let mut objects: Vec<String> = Vec::new();
                        for j in x+2..contents.len() {
                            if contents[j] == "}" {
                                break;
                            }
                            objects.push(String::from(contents[j].clone()))
                        }
                        let mut objects_object: Vec<String> = Vec::new();
                        for y in 0..objects.len() {
                            if objects[y] == "," {

                            } else if objects[y] == " " {

                            } else if objects[y] == "\r" {

                            } else if objects[y] == "\n" {

                            } else if objects[y] == "\"" {

                            }
                            else if objects[y] == "{" {

                            }
                            else if objects[y] == "}" {

                            }
                            else {
                                objects_object.push(objects[y].clone().to_string())
                            }
                        }
                        let mut classifier = String::new();
                        group_memory.push(build_name.clone());
                        group_memory.push(objects_object.len().to_string());
                        for d in 0..objects_object.len() {
                            group_memory.push(build_name.clone());
                            group_memory.push(objects_object[d].clone());
                        }
                    }
                    else if contents[x] == "if" {
                        let mut loc1 = 0;
                        let mut loc2 = 0;
                        let mut vec:Vec<String> = Vec::new();
                        let mut skip = false;
                        let mut n = 0;
                        let mut reached = false;
                        for y in x+1..contents.len() {
                            if skip == false {
                                if contents[y] == "{" {
                                    n = n +1;
                                    reached = true;
                                    loc1 = y;
                                }
                                else if contents[y] == "}" {
                                    n = n-1;
                                }
                                if n > 0 {
                                    vec.push((&contents[y]).parse().unwrap());
                                }
                                else if reached == true {
                                    skip = true;
                                    loc2 = y;
                                }
                            }
                        }
                        vec.remove(0);
                        let code = vec.clone();
                        let mut vec:Vec<String> = Vec::new();
                        let mut skip = false;
                        let mut n = 0;
                        for y in x+1..contents.len() {
                            if skip == false {
                                if contents[x+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if contents[y] == "(" {
                                    n = n +1;
                                }
                                else if contents[y] == ")" {
                                    n = n-1;
                                }
                                if n == 0 {
                                    skip = true;
                                    for z in x+1..y+1 {
                                        vec.push((&contents[z]).parse().unwrap());
                                    }
                                }
                            }
                        }
                        if dev {
                            println!("vec: {:?}",  vec);
                        }
                        let mut z = 0;
                        for y in vec.to_vec() {
                            if y == "(" || y == ")" {
                                z = z + 1;
                            }
                        }
                        skip = false;
                        let mut string: String = "".to_string();
                        let mut n = 0;
                        let mut n1 = 1;
                        let mut skips = 0;
                        for y in 1..vec.len() {
                            if skips == 0 {
                                if skip == false {
                                    if (vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`") && vec[y-1] != "\\" {
                                        n = n + 1;
                                    }else if vec[y] == "(" && n % 2 == 0 {
                                        n1 = n1 + 1;
                                    }
                                    else if vec[y] == ")" && n % 2 == 0 {
                                        n1 = n1 - 1;
                                    }else if n % 2 == 1 {
                                        string.push_str(vec[y].as_str());
                                    } else if vec[y] == "math" {
                                        string.push_str(math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                                        let mut leng = 0;
                                        let mut n2 = 0;
                                        let mut skip1 = false;
                                        for f in y+1..vec.len() {
                                            if skip1 == false {
                                                if vec[y+1] != "(" {
                                                    println!("You have to put a parentheses after a log");
                                                    std::process::exit(1);
                                                }
                                                if vec[f] == "(" {
                                                    n2 = n2 +1;
                                                }
                                                else if vec[f] == ")" {
                                                    n2 = n2-1;
                                                }
                                                if n2 == 0 {
                                                    skip1 = true;
                                                    for z in y+1..f+1 {
                                                        leng = leng + 1;
                                                    }
                                                }
                                            }
                                        }
                                        skips = leng;
                                    } else if vec[y] == "round" {
                                        string.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                                        let mut leng = 0;
                                        let mut n2 = 0;
                                        let mut skip1 = false;
                                        for f in y+1..vec.len() {
                                            if skip1 == false {
                                                if vec[y+1] != "(" {
                                                    println!("You have to put a parentheses after a log");
                                                    std::process::exit(1);
                                                }
                                                if contents[f] == "(" {
                                                    n2 = n2 +1;
                                                }
                                                else if contents[f] == ")" {
                                                    n2 = n2-1;
                                                }
                                                if n2 == 0 {
                                                    skip1 = true;
                                                    for z in y+1..f+1 {
                                                        leng = leng + 1;
                                                    }
                                                }
                                            }
                                        }
                                        skips = leng;
                                    } else if vec[y] == "GET" {
                                        string.push_str(get_request(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                                        let mut leng = 0;
                                        let mut n2 = 0;
                                        let mut skip1 = false;
                                        for f in y+1..vec.len() {
                                            if skip1 == false {
                                                if vec[y+1] != "(" {
                                                    println!("You have to put a parentheses after a log");
                                                    std::process::exit(1);
                                                }
                                                if contents[f] == "(" {
                                                    n2 = n2 +1;
                                                }
                                                else if contents[f] == ")" {
                                                    n2 = n2-1;
                                                }
                                                if n2 == 0 {
                                                    skip1 = true;
                                                    for z in y+1..f+1 {
                                                        leng = leng + 1;
                                                    }
                                                }
                                            }
                                        }
                                        skips = leng;
                                    } else if vec[y] == "replace" {
                                        string.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                                        let mut leng = 0;
                                        let mut n2 = 0;
                                        let mut skip1 = false;
                                        for f in y+1..vec.len() {
                                            if skip1 == false {
                                                if vec[y+1] != "(" {
                                                    println!("You have to put a parentheses after a log");
                                                    std::process::exit(1);
                                                }
                                                if vec[f] == "(" {
                                                    n2 = n2 +1;
                                                }
                                                else if vec[f] == ")" {
                                                    n2 = n2-1;
                                                }
                                                if n2 == 0 {
                                                    skip1 = true;
                                                    for z in y+1..f+1 {
                                                        leng = leng + 1;
                                                    }
                                                }
                                            }
                                        }
                                        skips = leng;
                                    } else if vec[y] == "input" {
                                        string.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                                        let mut leng = 0;
                                        let mut n2 = 0;
                                        let mut skip1 = false;
                                        for f in y+1..vec.len() {
                                            if skip1 == false {
                                                if vec[y+1] != "(" {
                                                    println!("You have to put a parentheses after a log");
                                                    std::process::exit(1);
                                                }
                                                if vec[f] == "(" {
                                                    n2 = n2 +1;
                                                }
                                                else if vec[f] == ")" {
                                                    n2 = n2-1;
                                                }
                                                if n2 == 0 {
                                                    skip1 = true;
                                                    for z in y+1..f+1 {
                                                        leng = leng + 1;
                                                    }
                                                }
                                            }
                                        }
                                        skips = leng;
                                    } else if vec[y] == "exec" {
                                        string.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                                        let mut leng = 0;
                                        let mut n2 = 0;
                                        let mut skip1 = false;
                                        for f in y+1..vec.len() {
                                            if skip1 == false {
                                                if vec[y+1] != "(" {
                                                    println!("You have to put a parentheses after a log");
                                                    std::process::exit(1);
                                                }
                                                if vec[f] == "(" {
                                                    n2 = n2 +1;
                                                }
                                                else if vec[f] == ")" {
                                                    n2 = n2-1;
                                                }
                                                if n2 == 0 {
                                                    skip1 = true;
                                                    for z in y+1..f+1 {
                                                        leng = leng + 1;
                                                    }
                                                }
                                            }
                                        }
                                        skips = leng;
                                    } else if vec[y] == "trim" {
                                        string.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                                        let mut leng = 0;
                                        let mut n2 = 0;
                                        let mut skip1 = false;
                                        for f in y+1..vec.len() {
                                            if skip1 == false {
                                                if vec[y+1] != "(" {
                                                    println!("You have to put a parentheses after a log");
                                                    std::process::exit(1);
                                                }
                                                if vec[f] == "(" {
                                                    n2 = n2 +1;
                                                }
                                                else if vec[f] == ")" {
                                                    n2 = n2-1;
                                                }
                                                if n2 == 0 {
                                                    skip1 = true;
                                                    for z in y+1..f+1 {
                                                        leng = leng + 1;
                                                    }
                                                }
                                            }
                                        }
                                        skips = leng;
                                    } else if vec[y] == "timeh" {
                                        string.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                                        let mut leng = 0;
                                        let mut n2 = 0;
                                        let mut skip1 = false;
                                        for f in y+1..vec.len() {
                                            if skip1 == false {
                                                if vec[y+1] != "(" {
                                                    println!("You have to put a parentheses after a log");
                                                    std::process::exit(1);
                                                }
                                                if vec[f] == "(" {
                                                    n2 = n2 +1;
                                                }
                                                else if vec[f] == ")" {
                                                    n2 = n2-1;
                                                }
                                                if n2 == 0 {
                                                    skip1 = true;
                                                    for z in y+1..f+1 {
                                                        leng = leng + 1;
                                                    }
                                                }
                                            }
                                        }
                                        skips = leng;
                                    } else if vec[y] == "time" {
                                        string.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                                        let mut leng = 0;
                                        let mut n2 = 0;
                                        let mut skip1 = false;
                                        for f in y+1..vec.len() {
                                            if skip1 == false {
                                                if vec[y+1] != "(" {
                                                    println!("You have to put a parentheses after a log");
                                                    std::process::exit(1);
                                                }
                                                if vec[f] == "(" {
                                                    n2 = n2 +1;
                                                }
                                                else if vec[f] == ")" {
                                                    n2 = n2-1;
                                                }
                                                if n2 == 0 {
                                                    skip1 = true;
                                                    for z in y+1..f+1 {
                                                        leng = leng + 1;
                                                    }
                                                }
                                            }
                                        }
                                        skips = leng;
                                    } else if vec[y] == "getcont" {
                                        string.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                                        let mut leng = 0;
                                        let mut n2 = 0;
                                        let mut skip1 = false;
                                        for f in y+1..vec.len() {
                                            if skip1 == false {
                                                if vec[y+1] != "(" {
                                                    println!("You have to put a parentheses after a log");
                                                    std::process::exit(1);
                                                }
                                                if vec[f] == "(" {
                                                    n2 = n2 +1;
                                                }
                                                else if vec[f] == ")" {
                                                    n2 = n2-1;
                                                }
                                                if n2 == 0 {
                                                    skip1 = true;
                                                    for z in y+1..f+1 {
                                                        leng = leng + 1;
                                                    }
                                                }
                                            }
                                        }
                                        skips = leng;
                                    } else if vec[y] == "=" || vec[y] == "!" || vec[y] == ">" || vec[y] == "<" {
                                        string.push(vec[y].parse().unwrap());
                                    } else {
                                        let mut postion = memory_names.len();
                                        let mut skip1 = false;
                                        for pos in 0..memory_names.len() {
                                            if skip1 == false {
                                                if memory_names[pos].to_string() == vec[y].to_string() {
                                                    postion = pos;
                                                    skip1 = true;
                                                }
                                            }
                                        }
                                        if postion != memory_names.len() {
                                            if vec[y+1] == "(" {
                                                let number_of_item = math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                                                string.push_str(&*memory_values[postion].split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v").nth(number_of_item.parse().unwrap()).unwrap().to_string());
                                            }
                                            else {
                                                string.push_str(&*memory_values[postion].to_string());
                                            }
                                        }
                                    }
                                }
                            }
                            else {
                                skips = skips -1;
                            }
                        }
                        let mut result:Vec<String> = Vec::new();
                        let mut last = 0;
                        for (index, matched) in string.match_indices(|c: char| c == "=".chars().nth(0).unwrap() || c == "!".chars().nth(0).unwrap() || c == ">".chars().nth(0).unwrap() || c == "<".chars().nth(0).unwrap() || c == "|".chars().nth(0).unwrap()) {
                            if last != index {
                                result.push((&string[last..index]).parse().unwrap());
                            }
                            result.push(matched.parse().unwrap());
                            last = index + matched.len();
                        }
                        if last < string.len() {
                            result.push((&string[last..]).parse().unwrap());
                        }
                        let mut output = Vec::new();
                        for item in 0..result.len() {
                            let mut next = &result[item];
                            if 0 < item {
                                next = &result[item-1];
                            }
                            if result[item] == "=" && 0 < item {
                                if result[item-1] == "=" || result[item-1] == "!" || result[item-1] == ">" || result[item-1] == "<" {
                                    output.push(result[item - 1].to_owned() + &*"=".to_string());
                                }
                            }
                        
                            else if result[item] == "|" && 0 < item {
                                if result[item+1] == "|" {
                                    output.push("||".parse().unwrap());
                                }
                            }
                            else if (result[item] == ">" || result[item] == "<") && 0 < item {
                                if result[item+1] != "=" {
                                    output.push(result[item].to_owned());
                                }
                            }
                            else if result[item] != "!" && result[item] != "<" && result[item] != ">" {
                                output.push(result[item].parse().unwrap());
                            }
                        }
                        let mut outcome = false;
                        for item in 0..output.len() {
                            let if_number = output[item].chars();
                            let mut if_number_bool = true;
                            for c in if_number {
                                if (char::is_numeric(c) || c == '.') && if_number_bool == true {
                                    if_number_bool = true;
                                }
                                else {
                                    if_number_bool = false;
                                }
                            }
                            if !if_number_bool {
                                let mut postion1 = memory_names.len();
                                let mut skip = false;
                                for pos in 0..memory_names.len() {
                                    if skip == false {
                                        if memory_names[pos].to_string() == output[item].to_string() {
                                            postion1 = pos;
                                            skip = true;
                                        }
                                    }
                                }
                                if postion1 != memory_names.len() {
                                    output[item] = memory_values[postion1].to_string();
                                }
                            }
                        }
                        for item in 0..output.len() {
                            if output[item] == "==" && output[item-1] == output[item+1] {
                                outcome = true;
                            }
                            else if output[item] == "!=" && output[item-1] != output[item+1] {
                                outcome = true;
                            }
                            else if output[item] == ">=" && output[item-1].parse::<i32>().unwrap() >= output[item+1].parse::<i32>().unwrap() {
                                outcome = true;
                            }
                            else if output[item] == "<=" && output[item-1].parse::<i32>().unwrap() <= output[item+1].parse::<i32>().unwrap() {
                                outcome = true;
                            }
                            else if output[item] == "<" && output[item-1].parse::<i32>().unwrap() < output[item+1].parse::<i32>().unwrap() {
                                outcome = true;
                            }
                            else if output[item] == ">" && output[item-1].parse::<i32>().unwrap() > output[item+1].parse::<i32>().unwrap() {
                                outcome = true;
                            }
                        }
                        if outcome == true {
                            contents[loc1] = " ".parse().unwrap();
                            contents[loc2] = " ".parse().unwrap();
                            readfrom = loc1;
                            skiperwiper = true;
                            read = true;
                        }
                        else {
                            if contents[loc2+1] == "while" {
                                contents[loc2+1] = " ".parse().unwrap();
                            }
                            if contents[loc2+2] == "while" {
                                contents[loc2+2] = " ".parse().unwrap();
                            }
                        }
                        if dev {
                            println!("output: {:?}", output);
                            println!("outcome: {:?}", outcome);
                            println!("code: {:?}", code);
                            println!("contents[loc1]: {:?}", contents[loc1]);
                            println!("contents[loc2]: {:?}", contents[loc2]);
                            println!("contents: {:?}", contents);
                        }
                    }
                    else {
                        if x > 2 {
                            if contents[x-2] != "func" {
                                let mut postion = func_names.len();
                                let mut skip = false;
                                for pos in 0..func_names.len() {
                                    if skip == false {
                                        if func_names[pos].to_string() == contents[x].to_string() {
                                            postion = pos;
                                            skip = true;
                                        }
                                    }
                                }
                                if postion != func_names.len() {
                                    let mut space: String = " ".parse().unwrap();
                                    space.push_str(func_code[postion].as_str());
                                    let mut to_to_parse = space;
                                    if dev {
                                        println!("contents: {:?}", to_to_parse);
                                    }
                                    let to_parse = lexer::lexer(to_to_parse, dev);
                                    readfrom = x;
                                    skiperwiper = true;
                                    read = true;
                                    let mut delete = Vec::new();
                                    let mut deleted = 0;
                                    let mut skirt = false;
                                    let mut n3 = 0;
                                    delete.push(x);
                                    for y1 in x+1..contents.len() {
                                        if skirt == false {
                                            if contents[y1] == "(" {
                                                n3 = n3 + 1;
                                            }
                                            if n3 == 0 {
                                                skirt = true;
                                            }
                                            if contents[y1] == ")" {
                                                n3 = n3 - 1;
                                            }
                                            delete.push(y1);
                                        }
                                    }
                                    for item in delete {
                                        contents.remove(item - deleted);
                                        deleted = deleted + 1 ;
                                    }
                                    let mut newVec = Vec::new();
                                    for itom in 0..contents.len() {
                                        if itom == x {
                                            for item in to_parse.clone() {
                                                newVec.push(item);
                                            }
                                        }
                                        newVec.push(contents[itom].clone());
                                    }
                                    contents = newVec;
                                }
                                else {
                                    let mut postion = memory_names.len();
                                    let mut skip = false;
                                    for pos in 0..memory_names.len() {
                                        if skip == false {
                                            if memory_names[pos].to_string() == contents[x].to_string() {
                                                postion = pos;
                                                skip = true;
                                            }
                                        }
                                    }
                                    if postion != memory_names.len() && (contents[x+1].trim() == ":" || contents[x+1].trim() == "=") && contents[x-2].trim() != "dec" {
                                        let mut position = x+2;
                                        let mut value = String::new();
                                        let mut n = 0;
                                        let mut quote = 0;
                                        let memory_names_save = memory_names.clone();
                                        let mut memory_values_save = memory_values.clone();
                                        let memmory_types_save = memory_types.clone();
                                        loop {
                                            if contents[position] == ";" {
                                                if dev {
                                                    println!("contents[x+move_up+move_up+move_up_up+move_final]: {:?}", contents[position]);
                                                }
                                                break;
                                            }
                                            else {
                                                if (contents[position] == "\"" || contents[position] == "\'" || contents[position] == r"\`") && contents[position-1] != "\\" {
                                                    quote = quote + 1;
                                                }
                                                else {
                                                    if contents[position] == "math" {
                                                        value.push_str(math(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                                                        n = 1;
                                                    }
                                                    else if contents[position] == "round" {
                                                        value.push_str(round(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                        n = 1;
                                                    }
                                                    else if contents[position] == "GET" {
                                                        value.push_str(get_request(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                        n = 1;
                                                    }
                                                    else if contents[position] == "replace" {
                                                        value.push_str(replace(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                        n = 1;
                                                    }
                                                    else if contents[position] == "input" {
                                                        value.push_str(input(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                        n = 1;
                                                    }
                                                    else if contents[position] == "exec" {
                                                        value.push_str(exec(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                        n = 1;
                                                    }
                                                    else if contents[position] == "trim" {
                                                        value.push_str(trim(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                        n = 1;
                                                    }

                                                    else if contents[position] == "timeh" {
                                                        value.push_str(time_readable(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                        n = 1;
                                                    }
                                                    else if contents[position] == "time" {
                                                        value.push_str(time(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                        n = 1;
                                                    }
                                                    else if contents[position] == "getcont" {
                                                        value.push_str(get_contents(position, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                                        n = 1;
                                                    }
                                                    else {
                                                        if n == 0 {
                                                            if quote%2 == 1 {
                                                                value.push_str(contents[position].as_str());
                                                            }
                                                            else {
                                                                let mut positions = memory_names_save.len();
                                                                let mut skip = false;
                                                                for pos in 0..memory_names_save.len() {
                                                                    if skip == false {
                                                                        if memory_names_save[pos].to_string() == contents[position].to_string() {
                                                                            positions = pos;
                                                                            skip = true;
                                                                        }
                                                                    }
                                                                }
                                                                if positions != memory_names_save.len() {
                                                                    value.push_str(memory_values_save[positions].to_string().as_str());
                                                                }
                                                                else {
                                                                    value.push_str(contents[position].as_str());
                                                                }
                                                            }
                                                        }
                                                    }
                                                    if n >= 1 && contents[position] == "(" {
                                                        n = n + 1
                                                    }
                                                    else if n >= 1 && contents[position] == ")" {
                                                        n = n - 1;
                                                        if n == 1 {
                                                            n = 0;
                                                        }
                                                    }
                                                }
                                            }
                                            position = position+1;
                                            if dev {
                                                println!("position: {:?}", position);
                                            }
                                        }
                                        memory_values[postion] = value;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for i in threads {
        i.join().unwrap();
    }
}

pub fn find_greatest(list_of_numbers: &[i32]) -> &i32 {
    let mut largest = &list_of_numbers[0];
    for number in list_of_numbers {
        if number > largest {
            largest = number
        }
    }
    return largest;
}

pub fn log(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x+1..contents.len() {
        if skip == false {
            if contents[x+1] != "(" {
                println!("You have to put a parentheses after a log");
                std::process::exit(1);
            }
            if contents[y] == "(" {
                n = n +1;
            }
            else if contents[y] == ")" {
                n = n-1;
            }
            if n == 0 {
                skip = true;
                for z in x+1..y+1 {
                    vec.push((&contents[z]).parse().unwrap());
                }
            }
        }
    }
    if dev {
        println!("vec: {:?}",  vec);
    }
    let mut z = 0;
    for y in vec.to_vec() {
        if y == "(" || y == ")" {
            z = z + 1;
        }
    }
    skip = false;
    let mut string: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    for y in 1..vec.len() {
        if skips == 0 {
            if skip == false {
                if (vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`") && vec[y-1] != "\\" {
                    n = n + 1;
                }else if vec[y] == "(" && n % 2 == 0 {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" && n % 2 == 0 {
                    n1 = n1 - 1;
                }else if n % 2 == 1 {
                    string.push_str(vec[y].as_str());
                } else if vec[y] == "math" {
                    string.push_str(math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "round" {
                    string.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "GET" {
                    string.push_str(get_request(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "replace" {
                    string.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "input" {
                    string.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "exec" {
                    string.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "trim" {
                    string.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "timeh" {
                    string.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "time" {
                    string.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "getcont" {
                    string.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else {
                    let mut postion = memory_names.len();
                    let mut skip1 = false;
                    for pos in 0..memory_names.len() {
                        if skip1 == false {
                            if memory_names[pos].to_string() == vec[y].to_string() {
                                postion = pos;
                                skip1 = true;
                            }
                        }
                    }
                    if postion != memory_names.len() {
                        if vec[y+1] == "(" {
                            let number_of_item = math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                            println!("{:?}", memory_values);
                            string.push_str(&*memory_values[postion].split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v").nth(number_of_item.parse().unwrap()).unwrap().to_string());
                        }
                        else {
                            string.push_str(&*memory_values[postion].to_string());
                        }
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    println!("{}", string);
}

pub fn exec(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) -> String {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x+1..contents.len() {
        if skip == false {
            if contents[x+1] != "(" {
                println!("You have to put a parentheses after a log");
                std::process::exit(1);
            }
            if contents[y] == "(" {
                n = n +1;
            }
            else if contents[y] == ")" {
                n = n-1;
            }
            if n == 0 {
                skip = true;
                for z in x+1..y+1 {
                    vec.push((&contents[z]).parse().unwrap());
                }
            }
        }
    }
    if dev {
        println!("vec: {:?}",  vec);
    }
    let mut z = 0;
    for y in vec.to_vec() {
        if y == "(" || y == ")" {
            z = z + 1;
        }
    }
    skip = false;
    let mut string: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    for y in 1..vec.len() {
        if skips == 0 {
            if skip == false {
                if (vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`") && vec[y-1] != "\\" {
                    n = n + 1;
                }else if vec[y] == "(" && n % 2 == 0 {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" && n % 2 == 0 {
                    n1 = n1 - 1;
                }else if n % 2 == 1 {
                    string.push_str(vec[y].as_str());
                } else if vec[y] == "math" {
                    string.push_str(math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "round" {
                    string.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "GET" {
                    string.push_str(get_request(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "replace" {
                    string.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "input" {
                    string.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "exec" {
                    string.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "trim" {
                    string.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "timeh" {
                    string.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "time" {
                    string.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "getcont" {
                    string.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else {
                    let mut postion = memory_names.len();
                    let mut skip1 = false;
                    for pos in 0..memory_names.len() {
                        if skip1 == false {
                            if memory_names[pos].to_string() == vec[y].to_string() {
                                postion = pos;
                                skip1 = true;
                            }
                        }
                    }
                    if postion != memory_names.len() {
                        if vec[y+1] == "(" {
                            let number_of_item = math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                            string.push_str(&*memory_values[postion].split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v").nth(number_of_item.parse().unwrap()).unwrap().to_string());
                        }
                        else {
                            string.push_str(&*memory_values[postion].to_string());
                        }
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    let stringreturn = string;
    let mut vecs = stringreturn.replace("\n", " ");
    vecs = vecs.replace("\t", " ");
    if env::consts::OS == "windows" {
        let mut endvec: Vec<&str> = Vec::new();
        endvec.push("/C");
        endvec.push(&stringreturn);
        if dev {
            println!("Command args: {:?}", endvec);
        }
        let output = Command::new("cmd")
            .args(endvec)
            .output()
            .expect("failed to execute process");
        return String::from_utf8_lossy(&output.stdout).to_string();
    }
    else {
        let mut endvec: Vec<&str> = vecs.split(" ").collect();
        let commandname = endvec[0];
        endvec.remove(0);
        if dev {
            println!("Command Name: {}", commandname);
            println!("Command args: {:?}", endvec);
        }
        let output = Command::new(commandname)
            .args(endvec)
            .output()
            .expect("failed to execute process");
        return String::from_utf8_lossy(&output.stdout).to_string();
    }
}

pub fn round(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) -> i32 {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x+1..contents.len() {
        if skip == false {
            if contents[x+1] != "(" {
                println!("You have to put a parentheses after the function on line {}", get_line(x, contents.clone()));
                std::process::exit(1);
            }
            if contents[y] == "(" {
                n = n +1;
            }
            else if contents[y] == ")" {
                n = n-1;
            }
            if n == 0 {
                skip = true;
                for z in x+1..y+1 {
                    vec.push(contents[z].to_string());
                }
            }
        }
    }
    let mut n = 0;
    let mut what_to_do_first = Vec::new();
    if vec[0] == "\"" || vec[0] == "\'" || vec[0] == r"\`"{
        vec.remove(0);
        vec.remove(vec.len()-1);
    }
    for y in 0..vec.len() {
        if vec[y] == "(" && vec[y-1] != "\\" {
            n = n +1;
        }
        else if vec[y] == ")" && vec[y-1] != "\\" {
            n = n-1;
        }
        what_to_do_first.push(n);
    }
    let mut keep_going = true;
    while keep_going {
        let mut skip =  false;
        for y in 0..vec.len() {
            if skip == false {
                if vec[y] == "math" {
                    vec[y] = math(y, vec.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                    let mut skip1 = false;
                    let mut t = 0;
                    while vec.len() > 1 {
                        for z in 1..vec.len() {
                            if skip1 == false {
                                if contents[z] == "(" {
                                    t = t +1;
                                }
                                else if contents[z] == ")" {
                                    t = t-1;
                                }
                                if t%2 == 1 {
                                    vec.remove(z);
                                    skip1 = true;
                                }
                            }
                        }
                        skip1 = false;
                    }

                    skip = true;
                }
                else if vec[y] == "round" {
                    vec[y] = round(y, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                    let skip1 = false;
                    let mut t = 0;
                    for y in x+1..vec.len() {
                        if skip1 == false {
                            if contents[y] == "(" {
                                t = t +1;
                            }
                            else if contents[y] == ")" {
                                t = t-1;
                            }
                            if t%2 == 0 {
                                vec.remove(y);
                            }
                        }
                    }
                    skip = true;
                }
                else if vec[y] == "GET" {
                    vec[y] = get_request(y, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                    let skip1 = false;
                    let mut t = 0;
                    for y in x+1..vec.len() {
                        if skip1 == false {
                            if contents[y] == "(" {
                                t = t +1;
                            }
                            else if contents[y] == ")" {
                                t = t-1;
                            }
                            if t%2 == 0 {
                                vec.remove(y);
                            }
                        }
                    }
                    skip = true;
                } else if vec[y] == "replace" {
                    vec[y] = replace(y, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                    let skip1 = false;
                    let mut t = 0;
                    for y in x+1..vec.len() {
                        if skip1 == false {
                            if contents[y] == "(" {
                                t = t +1;
                            }
                            else if contents[y] == ")" {
                                t = t-1;
                            }
                            if t%2 == 0 {
                                vec.remove(y);
                            }
                        }
                    }
                    skip = true;
                } else if vec[y] == "input" {
                    vec[y] = input(y, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                    let skip1 = false;
                    let mut t = 0;
                    for y in x+1..vec.len() {
                        if skip1 == false {
                            if contents[y] == "(" {
                                t = t +1;
                            }
                            else if contents[y] == ")" {
                                t = t-1;
                            }
                            if t%2 == 0 {
                                vec.remove(y);
                            }
                        }
                    }
                    skip = true;
                } else if vec[y] == "exec" {
                    vec[y] = exec(y, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                    let skip1 = false;
                    let mut t = 0;
                    for y in x+1..vec.len() {
                        if skip1 == false {
                            if contents[y] == "(" {
                                t = t +1;
                            }
                            else if contents[y] == ")" {
                                t = t-1;
                            }
                            if t%2 == 0 {
                                vec.remove(y);
                            }
                        }
                    }
                    skip = true;
                } else if vec[y] == "trim" {
                    vec[y] = trim(y, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                    let skip1 = false;
                    let mut t = 0;
                    for y in x+1..vec.len() {
                        if skip1 == false {
                            if contents[y] == "(" {
                                t = t +1;
                            }
                            else if contents[y] == ")" {
                                t = t-1;
                            }
                            if t%2 == 0 {
                                vec.remove(y);
                            }
                        }
                    }
                    skip = true;
                } else if vec[y] == "timeh" {
                    vec[y] = time_readable(y, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                    let skip1 = false;
                    let mut t = 0;
                    for y in x+1..vec.len() {
                        if skip1 == false {
                            if contents[y] == "(" {
                                t = t +1;
                            }
                            else if contents[y] == ")" {
                                t = t-1;
                            }
                            if t%2 == 0 {
                                vec.remove(y);
                            }
                        }
                    }
                    skip = true;
                } else if vec[y] == "time" {
                    vec[y] = time(y, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                    let skip1 = false;
                    let mut t = 0;
                    for y in x+1..vec.len() {
                        if skip1 == false {
                            if contents[y] == "(" {
                                t = t +1;
                            }
                            else if contents[y] == ")" {
                                t = t-1;
                            }
                            if t%2 == 0 {
                                vec.remove(y);
                            }
                        }
                    }
                    skip = true;
                } else if vec[y] == "getcont" {
                    vec[y] = get_contents(y, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                    let skip1 = false;
                    let mut t = 0;
                    for y in x+1..vec.len() {
                        if skip1 == false {
                            if contents[y] == "(" {
                                t = t +1;
                            }
                            else if contents[y] == ")" {
                                t = t-1;
                            }
                            if t%2 == 0 {
                                vec.remove(y);
                            }
                        }
                    }
                    skip = true;
                }
                else {
                    let mut postion = memory_names.len();
                    let mut skip = false;
                    for pos in 0..memory_names.len() {
                        if skip == false {
                            if memory_names[pos].to_string() == vec[y].to_string() {
                                postion = pos;
                                skip = true;
                            }
                        }
                    }
                    if postion != memory_names.len() {
                        vec[y] = memory_values[postion].to_string();
                    }
                }
            }
        }
        if vec.len() == 1 {
            keep_going = false;
        }
    }
    //let returns:i32 = vec[0].parse().unwrap();
    return vec[0].parse::<f32>().unwrap().round() as i32;
}

pub fn set_contents(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) -> std::io::Result<()> {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x+1..contents.len() {
        if skip == false {
            if contents[x+1] != "(" {
                println!("You have to put a parentheses after the function on line {}", get_line(x, contents.clone()));
                std::process::exit(1);
            }
            if contents[y] == "(" {
                n = n +1;
            }
            else if contents[y] == ")" {
                n = n-1;
            }
            if n == 0 {
                skip = true;
                for z in x+1..y+1 {
                    vec.push(contents[z].to_string());
                }
            }
        }
    }
    let mut n = 0;
    vec.remove(0);
    vec.remove(vec.len()-1);
    let mut file = Vec::new();
    let mut number_of_seperators = 0;
    for number in 0..vec.len() {
        if vec[number] == "," {
            number_of_seperators = number_of_seperators + 1;
        }
        else if number_of_seperators == 0 {
            file.push(vec[number].clone());
        }
    }
    let mut text = Vec::new();
    let mut number_of_seperators = 0;
    for number in 0..vec.len() {
        if vec[number] == "," {
            number_of_seperators = number_of_seperators + 1;
        }
        else if number_of_seperators == 1 {
            text.push(vec[number].clone());
        }
    }
    let mut skip = false;
    let mut file_s: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    for y in 0..file.len() {
        if skips == 0 {
            if skip == false {
                if (vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`") && vec[y-1] != "\\" {
                    n = n + 1;
                }else if vec[y] == "(" && n % 2 == 0  {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" && n % 2 == 0 {
                    n1 = n1 - 1;
                }else if n % 2 == 1 {
                    file_s.push_str(vec[y].as_str());
                } else if vec[y] == "math" {
                    file_s.push_str(math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "round" {
                    file_s.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "GET" {
                    file_s.push_str(get_request(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "replace" {
                    file_s.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "input" {
                    file_s.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "exec" {
                    file_s.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "trim" {
                    file_s.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "timeh" {
                    file_s.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "time" {
                    file_s.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "getcont" {
                    file_s.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else {
                    let mut postion = memory_names.len();
                    let mut skip1 = false;
                    for pos in 0..memory_names.len() {
                        if skip1 == false {
                            if memory_names[pos].to_string() == vec[y].to_string() {
                                postion = pos;
                                skip1 = true;
                            }
                        }
                    }
                    if postion != memory_names.len() {
                        file_s.push_str(&*memory_values[postion].to_string());
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    let mut skip = false;
    let mut text_s: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    for y in 0..text.len() {
        if skips == 0 {
            if skip == false {
                if (text[y] == "\"" || text[y] == "\'" || text[y] == r"\`") && text[y-1] != "\\" {
                    n = n + 1;
                }else if text[y] == "(" && n % 2 == 0 {
                    n1 = n1 + 1;
                }
                else if text[y] == ")" && n % 2 == 0 {
                    n1 = n1 - 1;
                }else if n % 2 == 1 {
                    text_s.push_str(text[y].as_str());
                } else if text[y] == "math" {
                    text_s.push_str(math(y, text.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..text.len() {
                        if skip1 == false {
                            if text[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if text[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if text[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if text[y] == "round" {
                    text_s.push_str(round(y, text.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..text.len() {
                        if skip1 == false {
                            if text[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if text[y] == "GET" {
                    text_s.push_str(get_request(y, text.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..text.len() {
                        if skip1 == false {
                            if text[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "replace" {
                    text_s.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "input" {
                    text_s.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "exec" {
                    text_s.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "trim" {
                    text_s.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "timeh" {
                    file_s.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "time" {
                    text_s.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "getcont" {
                    text_s.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else {
                    let mut postion = memory_names.len();
                    let mut skip1 = false;
                    for pos in 0..memory_names.len() {
                        if skip1 == false {
                            if memory_names[pos].to_string() == text[y].to_string() {
                                postion = pos;
                                skip1 = true;
                            }
                        }
                    }
                    if postion != memory_names.len() {
                        text_s.push_str(&*memory_values[postion].to_string());
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    if dev {
        println!("vec: {:?}", vec);
        println!("file: {:?}", file);
        println!("text: {:?}", text);
        println!("file_s: {}", file_s);
        println!("text_s: {}", text_s);
    }
    let mut file = File::create(file_s)?;
    file.write_all(text_s.as_ref())?;
    Ok(())
}

pub fn input(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

pub fn get_contents(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) -> String {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x+1..contents.len() {
        if skip == false {
            if contents[x+1] != "(" {
                println!("You have to put a parentheses after a log");
                std::process::exit(1);
            }
            if contents[y] == "(" {
                n = n +1;
            }
            else if contents[y] == ")" {
                n = n-1;
            }
            if n == 0 {
                skip = true;
                for z in x+1..y+1 {
                    vec.push((&contents[z]).parse().unwrap());
                }
            }
        }
    }
    if dev {
        println!("vec: {:?}",  vec);
    }
    let mut z = 0;
    for y in vec.to_vec() {
        if y == "(" || y == ")" {
            z = z + 1;
        }
    }
    skip = false;
    let mut string: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    for y in 1..vec.len() {
        if skips == 0 {
            if skip == false {
                if (vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`") && vec[y-1] != "\\" {
                    n = n + 1;
                }else if vec[y] == "(" && n % 2 == 0 {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" && n % 2 == 0 {
                    n1 = n1 - 1;
                }else if n % 2 == 1 {
                    string.push_str(vec[y].as_str());
                } else if vec[y] == "math" {
                    string.push_str(math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "round" {
                    string.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "GET" {
                    string.push_str(get_request(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "replace" {
                    string.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "input" {
                    string.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "exec" {
                    string.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "trim" {
                    string.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "timeh" {
                    string.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "time" {
                    string.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "getcont" {
                    string.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else {
                    let mut postion = memory_names.len();
                    let mut skip1 = false;
                    for pos in 0..memory_names.len() {
                        if skip1 == false {
                            if memory_names[pos].to_string() == vec[y].to_string() {
                                postion = pos;
                                skip1 = true;
                            }
                        }
                    }
                    if postion != memory_names.len() {
                        if vec[y+1] == "(" {
                            let number_of_item = math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                            string.push_str(&*memory_values[postion].split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v").nth(number_of_item.parse().unwrap()).unwrap().to_string());
                        }
                        else {
                            string.push_str(&*memory_values[postion].to_string());
                        }
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    let maybe_contents = fs::read_to_string(string);
    let mut contents = if maybe_contents.is_ok() {
        maybe_contents.unwrap()
    } else {
        panic!("Could not open file for reading.");
    };
    return contents;
}

pub fn replace(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) -> String {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x+1..contents.len() {
        if skip == false {
            if contents[x+1] != "(" {
                println!("You have to put a parentheses after the function on line {}", get_line(x, contents.clone()));
                std::process::exit(1);
            }
            if contents[y] == "(" {
                n = n +1;
            }
            else if contents[y] == ")" {
                n = n-1;
            }
            if n == 0 {
                skip = true;
                for z in x+1..y+1 {
                    vec.push(contents[z].to_string());
                }
            }
        }
    }
    let mut n = 0;
    vec.remove(0);
    vec.remove(vec.len()-1);
    let mut imput = Vec::new();
    let mut number_of_seperators = 0;
    for number in 0..vec.len() {
        if vec[number] == "," {
            number_of_seperators = number_of_seperators + 1;
        }
        else if number_of_seperators == 0 {
            imput.push(vec[number].clone());
        }
    }
    let mut find = Vec::new();
    let mut number_of_seperators = 0;
    for number in 0..vec.len() {
        if vec[number] == "," {
            number_of_seperators = number_of_seperators + 1;
        }
        else if number_of_seperators == 1 {
            find.push(vec[number].clone());
        }
    }
    let mut replacer = Vec::new();
    let mut number_of_seperators = 0;
    for number in 0..vec.len() {
        if vec[number] == "," {
            number_of_seperators = number_of_seperators + 1;
        }
        else if number_of_seperators == 2 {
            replacer.push(vec[number].clone());
        }
    }
    let mut skip = false;
    let mut imput_s: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    for y in 0..imput.len() {
        if skips == 0 {
            if skip == false {
                if vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`" {
                    n = n + 1;
                }else if vec[y] == "(" && n % 2 == 0 {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" && n % 2 == 0 {
                    n1 = n1 - 1;
                }else if n % 2 == 1 {
                    imput_s.push_str(vec[y].as_str());
                } else if vec[y] == "math" {
                    imput_s.push_str(math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "round" {
                    imput_s.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "GET" {
                    imput_s.push_str(get_request(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "replace" {
                    imput_s.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "input" {
                    imput_s.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "exec" {
                    imput_s.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "trim" {
                    imput_s.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "timeh" {
                    imput_s.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "time" {
                    imput_s.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "getcont" {
                    imput_s.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else {
                    let mut postion = memory_names.len();
                    let mut skip1 = false;
                    for pos in 0..memory_names.len() {
                        if skip1 == false {
                            if memory_names[pos].to_string() == vec[y].to_string() {
                                postion = pos;
                                skip1 = true;
                            }
                        }
                    }
                    if postion != memory_names.len() {
                        imput_s.push_str(&*memory_values[postion].to_string());
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    let mut skip = false;
    let mut find_s: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    for y in 0..find.len() {
        if skips == 0 {
            if skip == false {
                if find[y] == "\"" || find[y] == "\'" || find[y] == r"\`" {
                    n = n + 1;
                }else if find[y] == "(" && n % 2 == 0 {
                    n1 = n1 + 1;
                }
                else if find[y] == ")" && n % 2 == 0 {
                    n1 = n1 - 1;
                }else if n % 2 == 1 {
                    find_s.push_str(find[y].as_str());
                } else if find[y] == "math" {
                    find_s.push_str(math(y, find.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..find.len() {
                        if skip1 == false {
                            if find[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if find[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if find[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if find[y] == "round" {
                    find_s.push_str(round(y, find.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..find.len() {
                        if skip1 == false {
                            if find[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if find[y] == "GET" {
                    find_s.push_str(get_request(y, find.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..find.len() {
                        if skip1 == false {
                            if find[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "replace" {
                    find_s.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "input" {
                    find_s.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "exec" {
                    find_s.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "trim" {
                    find_s.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "timeh" {
                    find_s.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "time" {
                    find_s.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "getcont" {
                    find_s.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else {
                    let mut postion = memory_names.len();
                    let mut skip1 = false;
                    for pos in 0..memory_names.len() {
                        if skip1 == false {
                            if memory_names[pos].to_string() == find[y].to_string() {
                                postion = pos;
                                skip1 = true;
                            }
                        }
                    }
                    if postion != memory_names.len() {
                        find_s.push_str(&*memory_values[postion].to_string());
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    let mut skip = false;
    let mut replacer_s: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    for y in 0..replacer.len() {
        if skips == 0 {
            if skip == false {
                if replacer[y] == "\"" || replacer[y] == "\'" || replacer[y] == r"\`" {
                    n = n + 1;
                }else if replacer[y] == "(" && n % 2 == 0 {
                    n1 = n1 + 1;
                }
                else if replacer[y] == ")" && n % 2 == 0 {
                    n1 = n1 - 1;
                }else if n % 2 == 1 {
                    replacer_s.push_str(replacer[y].as_str());
                } else if replacer[y] == "math" {
                    replacer_s.push_str(math(y, replacer.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..replacer.len() {
                        if skip1 == false {
                            if replacer[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if replacer[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if replacer[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if replacer[y] == "round" {
                    replacer_s.push_str(round(y, replacer.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..replacer.len() {
                        if skip1 == false {
                            if replacer[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if replacer[y] == "GET" {
                    replacer_s.push_str(get_request(y, replacer.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..replacer.len() {
                        if skip1 == false {
                            if replacer[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                    let mut postion = memory_names.len();
                    let mut skip1 = false;
                    for pos in 0..memory_names.len() {
                        if skip1 == false {
                            if memory_names[pos].to_string() == replacer[y].to_string() {
                                postion = pos;
                                skip1 = true;
                            }
                        }
                    }
                    if postion != memory_names.len() {
                        replacer_s.push_str(&*memory_values[postion].to_string());
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    if dev {
        println!("vec: {:?}", vec);
        println!("imput: {:?}", imput);
        println!("find: {:?}", find);
        println!("replacer: {:?}", replacer);
        println!("imput_s: {}", imput_s);
        println!("find_s: {}", find_s);
        println!("replacer_s: {}", replacer_s);
    }
    return imput_s.replace(&*find_s, &*replacer_s);
}

pub fn imp(x:usize, contents: Vec<String>, dev: bool) -> Vec<String> {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x+1..contents.len() {
        if skip == false {
            if contents[x+1] != "(" {
                println!("You have to put a parentheses after the function on line {}", get_line(x, contents.clone()));
                std::process::exit(1);
            }
            if contents[y] == "(" {
                n = n +1;
            }
            else if contents[y] == ")" {
                n = n-1;
            }
            if n == 0 {
                skip = true;
                for z in x+1..y+1 {
                    vec.push(contents[z].to_string());
                }
            }
        }
    }
    vec.remove(0);
    vec.remove(vec.len()-1);
    vec.remove(0);
    vec.remove(vec.len()-1);
    let string:String = vec.join("").to_string();
    if dev {
        println!("string: {}", string);
    }
    let mut contents:String = "".to_string();
    if string.starts_with("https://") || string.starts_with("http://") {
        let mut dst = Vec::new();
        let mut easy = Easy::new();
        easy.url(&*string).unwrap();

        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
        drop(transfer);

        contents =  dst.iter().map(|&c| c as char).collect::<String>();
    }
    else {
        let maybe_contents = fs::read_to_string(string);
        contents = if maybe_contents.is_ok() {
            maybe_contents.unwrap()
        } else {
            panic!("Could not open file for reading.");
        };
    }
    let mut space: String = " ".parse().unwrap();
    space.push_str(contents.as_str());
    contents = space;
    if dev {
        println!("contents: {:?}", contents);
    }
    let to_parse = lexer::lexer(contents, dev);
    if dev {
        println!("to_parse: {:?}", to_parse);
    }
    return to_parse;
}

pub fn math(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) -> f32 {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x+1..contents.len() {
        if skip == false {
            if contents[x+1] != "(" {
                println!("You have to put a parentheses after the function on line {}", get_line(x, contents.clone()));
                std::process::exit(1);
            }
            if contents[y] == "(" {
                n = n +1;
            }
            else if contents[y] == ")" {
                n = n-1;
            }
            if n%2 == 0 {
                skip = true;
                for z in x+1..y+1 {
                    vec.push(contents[z].to_string());
                }
            }
        }
    }
    let mut n = 0;
    let mut what_to_do_first = Vec::new();
    vec.remove(0);
    vec.remove(vec.len()-1);
    for y in 0..vec.len() {
        if vec[y] == "(" {
            n = n +1;
        }
        else if vec[y] == ")" {
            n = n-1;
        }
        what_to_do_first.push(n);
    }
    if find_greatest(&*what_to_do_first) > &0 {
        // has parenties
    }
    else {
        let mut keep_going = true;
        while keep_going {
            let mut skip =  false;
            for y in 0..vec.len() {
                if skip == false {
                    let mut rng = rand::thread_rng();
                    let if_number = vec[y].chars();
                    let mut if_number_bool = true;
                    for c in if_number {
                        if (char::is_numeric(c) || c == '.') && if_number_bool == true {
                            if_number_bool = true;
                        }
                        else {
                            if_number_bool = false;
                        }
                    }
                    if !if_number_bool {
                        let mut postion1 = memory_names.len();
                        let mut skip = false;
                        for pos in 0..memory_names.len() {
                            if skip == false {
                                if memory_names[pos].to_string() == vec[y].to_string() {
                                    postion1 = pos;
                                    skip = true;
                                }
                            }
                        }
                        if postion1 != memory_names.len() {
                            vec[y] = memory_values[postion1].to_string();
                        }

                    }
                    if vec[y].to_lowercase() == "random" {
                        vec[y] = rng.gen::<f32>().to_string();
                        skip = true;
                    }
                    else if vec[y] == "+" {
                        if vec[y + 1].to_lowercase() == "random" {
                            vec[y+1] = rng.gen::<f32>().to_string();
                        }
                        let if_number = vec[y+1].chars();
                        let mut if_number_bool = true;
                        for c in if_number {
                            if (char::is_numeric(c) || c == '.') && if_number_bool == true {
                                if_number_bool = true;
                            }
                            else {
                                if_number_bool = false;
                            }
                        }
                        if !if_number_bool {
                            let mut postion1 = memory_names.len();
                            let mut skip = false;
                            for pos in 0..memory_names.len() {
                                if skip == false {
                                    if memory_names[pos].to_string() == vec[y+1].to_string() {
                                        postion1 = pos;
                                        skip = true;
                                    }
                                }
                            }
                            if postion1 != memory_names.len() {
                                vec[y+1] = memory_values[postion1].to_string();
                            }
                        }
                        vec[y - 1] = vec[y - 1].parse::<f32>().unwrap().add(vec[y + 1].parse::<f32>().unwrap()).to_string();
                        vec.remove(y);
                        vec.remove(y);
                        skip = true;
                    }
                    else if vec[y] == "-" {
                        if vec[y + 1].to_lowercase() == "random" {
                            vec[y+1] = rng.gen::<f32>().to_string();
                        }
                        let if_number = vec[y+1].chars();
                        let mut if_number_bool = true;
                        for c in if_number {
                            if (char::is_numeric(c) || c == '.') && if_number_bool == true {
                                if_number_bool = true;
                            }
                            else {
                                if_number_bool = false;
                            }
                        }
                        if !if_number_bool {
                            let mut postion1 = memory_names.len();
                            let mut skip = false;
                            for pos in 0..memory_names.len() {
                                if skip == false {
                                    if memory_names[pos].to_string() == vec[y+1].to_string() {
                                        postion1 = pos;
                                        skip = true;
                                    }
                                }
                            }
                            if postion1 != memory_names.len() {
                                vec[y+1] = memory_values[postion1].to_string();
                            }
                        }
                        vec[y - 1] = vec[y - 1].parse::<f32>().unwrap().sub(vec[y + 1].parse::<f32>().unwrap()).to_string();
                        vec.remove(y);
                        vec.remove(y);
                        skip = true;
                    }
                    else if vec[y] == "*" {
                        if vec[y + 1].to_lowercase() == "random" {
                            vec[y+1] = rng.gen::<f32>().to_string();
                        }
                        let if_number = vec[y+1].chars();
                        let mut if_number_bool = true;
                        for c in if_number {
                            if (char::is_numeric(c) || c == '.') && if_number_bool == true {
                                if_number_bool = true;
                            }
                            else {
                                if_number_bool = false;
                            }
                        }
                        if !if_number_bool {
                            let mut postion1 = memory_names.len();
                            let mut skip = false;
                            for pos in 0..memory_names.len() {
                                if skip == false {
                                    if memory_names[pos].to_string() == vec[y+1].to_string() {
                                        postion1 = pos;
                                        skip = true;
                                    }
                                }
                            }
                            if postion1 != memory_names.len() {
                                vec[y+1] = memory_values[postion1].to_string();
                            }
                        }
                        vec[y - 1] = vec[y - 1].parse::<f32>().unwrap().mul(vec[y + 1].parse::<f32>().unwrap()).to_string();
                        vec.remove(y);
                        vec.remove(y);
                        skip = true;
                    }
                    else if vec[y] == "/" {
                        if vec[y + 1].to_lowercase() == "random" {
                            vec[y+1] = rng.gen::<f32>().to_string();
                        }
                        let if_number = vec[y+1].chars();
                        let mut if_number_bool = true;
                        for c in if_number {
                            if (char::is_numeric(c) || c == '.') && if_number_bool == true {
                                if_number_bool = true;
                            }
                            else {
                                if_number_bool = false;
                            }
                        }
                        if !if_number_bool {
                            let mut postion1 = memory_names.len();
                            let mut skip = false;
                            for pos in 0..memory_names.len() {
                                if skip == false {
                                    if memory_names[pos].to_string() == vec[y+1].to_string() {
                                        postion1 = pos;
                                        skip = true;
                                    }
                                }
                            }
                            if postion1 != memory_names.len() {
                                vec[y+1] = memory_values[postion1].to_string();
                            }
                        }
                        vec[y - 1] = vec[y - 1].parse::<f32>().unwrap().div(vec[y + 1].parse::<f32>().unwrap()).to_string();
                        vec.remove(y);
                        vec.remove(y);
                        skip = true;
                    }
                    else if vec[y] == "^" {
                        if vec[y + 1].to_lowercase() == "random" {
                            vec[y+1] = rng.gen::<f32>().to_string();
                        }
                        let if_number = vec[y+1].chars();
                        let mut if_number_bool = true;
                        for c in if_number {
                            if (char::is_numeric(c) || c == '.') && if_number_bool == true {
                                if_number_bool = true;
                            }
                            else {
                                if_number_bool = false;
                            }
                        }
                        if !if_number_bool {
                            let mut postion1 = memory_names.len();
                            let mut skip = false;
                            for pos in 0..memory_names.len() {
                                if skip == false {
                                    if memory_names[pos].to_string() == vec[y+1].to_string() {
                                        postion1 = pos;
                                        skip = true;
                                    }
                                }
                            }
                            if postion1 != memory_names.len() {
                                vec[y+1] = memory_values[postion1].to_string();
                            }
                        }
                        vec[y - 1] = vec[y - 1].parse::<f32>().unwrap().powf(vec[y + 1].parse::<f32>().unwrap()).to_string();
                        vec.remove(y);
                        vec.remove(y);
                        skip = true;
                    }
                    else {
                        let mut postion = memory_names.len();
                        let mut skip = false;
                        for pos in 0..memory_names.len() {
                            if skip == false {
                                if memory_names[pos].to_string() == vec[y].to_string() {
                                    postion = pos;
                                    skip = true;
                                }
                            }
                        }
                        if postion != memory_names.len() {
                            vec[y] = memory_values[postion].to_string();
                        }
                    }
                }
            }
            if vec.len() == 1 {
                keep_going = false;
            }
        }
    }
    return vec[0].parse().unwrap();
}

pub fn trim(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) -> String {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x+1..contents.len() {
        if skip == false {
            if contents[x+1] != "(" {
                println!("You have to put a parentheses after the function on line {}", get_line(x, contents.clone()));
                std::process::exit(1);
            }
            if contents[y] == "(" {
                n = n +1;
            }
            else if contents[y] == ")" {
                n = n-1;
            }
            if n%2 == 0 {
                skip = true;
                for z in x+1..y+1 {
                    vec.push(contents[z].to_string());
                }
            }
        }
    }
    vec.remove(0);
    vec.remove(vec.len()-1);
    let mut skip = false;
    let mut imput_s: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    for y in 0..vec.len() {
        if skips == 0 {
            if skip == false {
                if (vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`") && vec[y-1] != "\\" {
                    n = n + 1;
                }else if vec[y] == "(" && n % 2 == 0 {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" && n % 2 == 0 {
                    n1 = n1 - 1;
                }else if n % 2 == 1 {
                    imput_s.push_str(vec[y].as_str());
                } else if vec[y] == "math" {
                    imput_s.push_str(math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "round" {
                    imput_s.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "GET" {
                    imput_s.push_str(get_request(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "replace" {
                    imput_s.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "input" {
                    imput_s.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "exec" {
                    imput_s.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "trim" {
                    imput_s.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "timeh" {
                    imput_s.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "time" {
                    imput_s.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "getcont" {
                    imput_s.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else {
                    let mut postion = memory_names.len();
                    let mut skip1 = false;
                    for pos in 0..memory_names.len() {
                        if skip1 == false {
                            if memory_names[pos].to_string() == vec[y].to_string() {
                                postion = pos;
                                skip1 = true;
                            }
                        }
                    }
                    if postion != memory_names.len() {
                        imput_s.push_str(&*memory_values[postion].to_string());
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    return imput_s.trim().to_string();
}

pub fn time_readable(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) -> String {
    let time = time(x, contents, memory_names, memory_values, memory_types, dev);
    let d = UNIX_EPOCH + Duration::from_millis(time as u64);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    return timestamp_str;
}

pub fn array_fn(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) -> Vec<String> {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x..contents.len() {
        if skip == false {
            if contents[y] == "[" {
                n = n +1;
            }
            else if contents[y] == "]" {
                n = n-1;
            }
            if n%2 == 0 {
                skip = true;
                for z in x..y+1 {
                    vec.push(contents[z].to_string());
                }
            }
        }
    }
    vec.remove(0);
    vec.remove(vec.len()-1);
    let mut skip = false;
    let mut imput_s: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    let mut output_array = Vec::new();
    for y in 0..vec.len() {
        if skips == 0 {
            if skip == false {
                if n % 2 == 0 && vec[y] == "," {
                    output_array.push(imput_s);
                    imput_s = "".to_string();
                }
                else if y < 1 {
                    if vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`" {
                        n = n + 1;
                    }else if vec[y] == "[" && n % 2 == 0 {
                        n1 = n1 + 1;
                    }
                    else if vec[y] == "]" && n % 2 == 0 {
                        n1 = n1 - 1;
                    }else if n % 2 == 1 {
                        imput_s.push_str(vec[y].as_str());
                    } else if vec[y] == "math" {
                        imput_s.push_str(math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "round" {
                        imput_s.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if contents[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if contents[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "GET" {
                        imput_s.push_str(get_request(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if contents[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if contents[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "replace" {
                        imput_s.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "input" {
                        imput_s.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "exec" {
                        imput_s.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "trim" {
                        imput_s.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "timeh" {
                        imput_s.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "time" {
                        imput_s.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "getcont" {
                        imput_s.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else {
                        let mut postion = memory_names.len();
                        let mut skip1 = false;
                        for pos in 0..memory_names.len() {
                            if skip1 == false {
                                if memory_names[pos].to_string() == vec[y].to_string() {
                                    postion = pos;
                                    skip1 = true;
                                }
                            }
                        }
                        if postion != memory_names.len() {
                            imput_s.push_str(&*memory_values[postion].to_string());
                        }
                    }
                }
                else {
                    if (vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`") && vec[y-1] != "\\" {
                        n = n + 1;
                    }else if vec[y] == "[" && n % 2 == 0 {
                        n1 = n1 + 1;
                    }
                    else if vec[y] == "]" && n % 2 == 0 {
                        n1 = n1 - 1;
                    }else if n % 2 == 1 {
                        imput_s.push_str(vec[y].as_str());
                    } else if vec[y] == "math" {
                        imput_s.push_str(math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "round" {
                        imput_s.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if contents[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if contents[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "GET" {
                        imput_s.push_str(get_request(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if contents[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if contents[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "replace" {
                        imput_s.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "input" {
                        imput_s.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "exec" {
                        imput_s.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "trim" {
                        imput_s.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "timeh" {
                        imput_s.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "time" {
                        imput_s.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "getcont" {
                        imput_s.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else {
                        let mut postion = memory_names.len();
                        let mut skip1 = false;
                        for pos in 0..memory_names.len() {
                            if skip1 == false {
                                if memory_names[pos].to_string() == vec[y].to_string() {
                                    postion = pos;
                                    skip1 = true;
                                }
                            }
                        }
                        if postion != memory_names.len() {
                            imput_s.push_str(&*memory_values[postion].to_string());
                        }
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    output_array.push(imput_s);
    return output_array;
}

pub fn get_line(x:usize, contents: Vec<String>) -> i32 {
    let mut line = 1;
    for n in 0..x {
        if contents[n] == "\n" {
            line = line + 1;
        }
    }
    return line;
}

pub fn get_request(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) -> String {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x+1..contents.len() {
        if skip == false {
            if contents[x+1] != "(" {
                println!("You have to put a parentheses after a log");
                std::process::exit(1);
            }
            if contents[y] == "(" {
                n = n +1;
            }
            else if contents[y] == ")" {
                n = n-1;
            }
            if n == 0 {
                skip = true;
                for z in x+1..y+1 {
                    vec.push((&contents[z]).parse().unwrap());
                }
            }
        }
    }
    if dev {
        println!("vec: {:?}",  vec);
    }
    let mut z = 0;
    for y in vec.to_vec() {
        if y == "(" || y == ")" {
            z = z + 1;
        }
    }
    skip = false;
    let mut string: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    for y in 1..vec.len() {
        if skips == 0 {
            if skip == false {
                if (vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`") && vec[y-1] != "\\" {
                    n = n + 1;
                }else if vec[y] == "(" && n % 2 == 0 {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" && n % 2 == 0 {
                    n1 = n1 - 1;
                }else if n % 2 == 1 {
                    string.push_str(vec[y].as_str());
                } else if vec[y] == "math" {
                    string.push_str(math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "round" {
                    string.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "GET" {
                    string.push_str(get_request(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "replace" {
                    string.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "input" {
                    string.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "exec" {
                    string.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "trim" {
                    string.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "timeh" {
                    string.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "time" {
                    string.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "getcont" {
                    string.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else {
                    let mut postion = memory_names.len();
                    let mut skip1 = false;
                    for pos in 0..memory_names.len() {
                        if skip1 == false {
                            if memory_names[pos].to_string() == vec[y].to_string() {
                                postion = pos;
                                skip1 = true;
                            }
                        }
                    }
                    if postion != memory_names.len() {
                        if vec[y+1] == "(" {
                            let number_of_item = math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string();
                            string.push_str(&*memory_values[postion].split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v").nth(number_of_item.parse().unwrap()).unwrap().to_string());
                        }
                        else {
                            string.push_str(&*memory_values[postion].to_string());
                        }
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url(&*string).unwrap();

    let mut transfer = easy.transfer();
    transfer
        .write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        })
        .unwrap();
    transfer.perform().unwrap();
    drop(transfer);

    return dst.iter().map(|&c| c as char).collect::<String>();
}

pub fn post_request(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x+1..contents.len() {
        if skip == false {
            if contents[x+1] != "(" {
                println!("You have to put a parentheses after the function on line {}", get_line(x, contents.clone()));
                std::process::exit(1);
            }
            if contents[y] == "(" {
                n = n +1;
            }
            else if contents[y] == ")" {
                n = n-1;
            }
            if n == 0 {
                skip = true;
                for z in x+1..y+1 {
                    vec.push(contents[z].to_string());
                }
            }
        }
    }
    let mut n = 0;
    vec.remove(0);
    vec.remove(vec.len()-1);
    let mut imput = Vec::new();
    let mut number_of_seperators = 0;
    for number in 0..vec.len() {
        if vec[number] == "," {
            number_of_seperators = number_of_seperators + 1;
        }
        else if number_of_seperators == 0 {
            imput.push(vec[number].clone());
        }
    }
    let mut find = Vec::new();
    let mut number_of_seperators = 0;
    for number in 0..vec.len() {
        if vec[number] == "," {
            number_of_seperators = number_of_seperators + 1;
        }
        else if number_of_seperators == 1 {
            find.push(vec[number].clone());
        }
    }
    let mut skip = false;
    let mut imput_s: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    for y in 0..imput.len() {
        if skips == 0 {
            if skip == false {
                if vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`" {
                    n = n + 1;
                }else if vec[y] == "(" && n % 2 == 0 {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" && n % 2 == 0 {
                    n1 = n1 - 1;
                }else if n % 2 == 1 {
                    imput_s.push_str(vec[y].as_str());
                } else if vec[y] == "math" {
                    imput_s.push_str(math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if vec[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "round" {
                    imput_s.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "GET" {
                    imput_s.push_str(get_request(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "round" {
                    imput_s.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else {
                    let mut postion = memory_names.len();
                    let mut skip1 = false;
                    for pos in 0..memory_names.len() {
                        if skip1 == false {
                            if memory_names[pos].to_string() == vec[y].to_string() {
                                postion = pos;
                                skip1 = true;
                            }
                        }
                    }
                    if postion != memory_names.len() {
                        imput_s.push_str(&*memory_values[postion].to_string());
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    let mut skip = false;
    let mut find_s: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    for y in 0..find.len() {
        if skips == 0 {
            if skip == false {
                if find[y] == "\"" || find[y] == "\'" || find[y] == r"\`" {
                    n = n + 1;
                }else if find[y] == "(" && n % 2 == 0 {
                    n1 = n1 + 1;
                }
                else if find[y] == ")" && n % 2 == 0 {
                    n1 = n1 - 1;
                }else if n % 2 == 1 {
                    find_s.push_str(find[y].as_str());
                } else if find[y] == "math" {
                    find_s.push_str(math(y, find.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..find.len() {
                        if skip1 == false {
                            if find[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if find[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if find[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if find[y] == "round" {
                    find_s.push_str(round(y, find.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..find.len() {
                        if skip1 == false {
                            if find[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if find[y] == "GET" {
                    find_s.push_str(get_request(y, find.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..find.len() {
                        if skip1 == false {
                            if find[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "round" {
                    find_s.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y+1..vec.len() {
                        if skip1 == false {
                            if vec[y+1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 = n2 +1;
                            }
                            else if contents[f] == ")" {
                                n2 = n2-1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for z in y+1..f+1 {
                                    leng = leng + 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else {
                    let mut postion = memory_names.len();
                    let mut skip1 = false;
                    for pos in 0..memory_names.len() {
                        if skip1 == false {
                            if memory_names[pos].to_string() == find[y].to_string() {
                                postion = pos;
                                skip1 = true;
                            }
                        }
                    }
                    if postion != memory_names.len() {
                        find_s.push_str(&*memory_values[postion].to_string());
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    if dev {
        println!("vec: {:?}", vec);
        println!("imput: {:?}", imput);
        println!("find: {:?}", find);
        println!("imput_s: {}", imput_s);
        println!("find_s: {}", find_s);
    }
    let mut data = find_s.as_bytes();

    let mut easy = Easy::new();
    easy.url(&*imput_s).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().unwrap();
}

pub fn time(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_millis() as f64;
}

pub fn group_fn(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool) -> Vec<String> {
    // creates vector, bool, and int
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x..contents.len() {
        if skip == false {
            if contents[y] == "{" {
                n = n +1;
            }
            else if contents[y] == "}" {
                n = n-1;
            }
            if n%2 == 0 {
                skip = true;
                for z in x..y+1 {
                    vec.push(contents[z].to_string());
                }
            }
        }
    }
    vec.remove(0);
    vec.remove(vec.len()-1);
    let mut skip = false;
    let mut imput_s: String = "".to_string();
    let mut n = 0;
    let mut n1 = 1;
    let mut skips = 0;
    let mut output_array = Vec::new();
    for y in 0..vec.len() {
        if skips == 0 {
            if skip == false {
                if n % 2 == 0 && vec[y] == "," {
                    output_array.push(imput_s);
                    imput_s = "".to_string();
                }
                else if y < 1 {
                    if vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`" {
                        n = n + 1;
                    }else if vec[y] == "{" && n % 2 == 0 {
                        n1 = n1 + 1;
                    }
                    else if vec[y] == "}" && n % 2 == 0 {
                        n1 = n1 - 1;
                    }else if n % 2 == 1 {
                        imput_s.push_str(vec[y].as_str());
                    } else if vec[y] == "math" {
                        imput_s.push_str(math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "round" {
                        imput_s.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if contents[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if contents[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "GET" {
                        imput_s.push_str(get_request(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if contents[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if contents[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "replace" {
                        imput_s.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "input" {
                        imput_s.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "exec" {
                        imput_s.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "trim" {
                        imput_s.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "timeh" {
                        imput_s.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "time" {
                        imput_s.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "getcont" {
                        imput_s.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else {
                        let mut postion = memory_names.len();
                        let mut skip1 = false;
                        for pos in 0..memory_names.len() {
                            if skip1 == false {
                                if memory_names[pos].to_string() == vec[y].to_string() {
                                    postion = pos;
                                    skip1 = true;
                                }
                            }
                        }
                        if postion != memory_names.len() {
                            imput_s.push_str(&*memory_values[postion].to_string());
                        }
                    }
                }
                else {
                    if (vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`") && vec[y-1] != "\\" {
                        n = n + 1;
                    }else if vec[y] == "[" && n % 2 == 0 {
                        n1 = n1 + 1;
                    }
                    else if vec[y] == "]" && n % 2 == 0 {
                        n1 = n1 - 1;
                    }else if n % 2 == 1 {
                        imput_s.push_str(vec[y].as_str());
                    } else if vec[y] == "math" {
                        imput_s.push_str(math(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "round" {
                        imput_s.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if contents[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if contents[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "GET" {
                        imput_s.push_str(get_request(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if contents[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if contents[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "replace" {
                        imput_s.push_str(replace(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "input" {
                        imput_s.push_str(input(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "exec" {
                        imput_s.push_str(exec(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "trim" {
                        imput_s.push_str(trim(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "timeh" {
                        imput_s.push_str(time_readable(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "time" {
                        imput_s.push_str(time(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else if vec[y] == "getcont" {
                        imput_s.push_str(get_contents(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                        let mut leng = 0;
                        let mut n2 = 0;
                        let mut skip1 = false;
                        for f in y+1..vec.len() {
                            if skip1 == false {
                                if vec[y+1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if vec[f] == "(" {
                                    n2 = n2 +1;
                                }
                                else if vec[f] == ")" {
                                    n2 = n2-1;
                                }
                                if n2 == 0 {
                                    skip1 = true;
                                    for z in y+1..f+1 {
                                        leng = leng + 1;
                                    }
                                }
                            }
                        }
                        skips = leng;
                    } else {
                        let mut postion = memory_names.len();
                        let mut skip1 = false;
                        for pos in 0..memory_names.len() {
                            if skip1 == false {
                                if memory_names[pos].to_string() == vec[y].to_string() {
                                    postion = pos;
                                    skip1 = true;
                                }
                            }
                        }
                        if postion != memory_names.len() {
                            imput_s.push_str(&*memory_values[postion].to_string());
                        }
                    }
                }
            }
        }
        else {
            skips = skips -1;
        }
    }
    output_array.push(imput_s);
    return output_array;
    
}

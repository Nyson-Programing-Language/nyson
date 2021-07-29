use crate::{lexer, run};
use curl::easy::Easy;
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::{Add, Div, Mul, Sub};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{env, fs};
extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;
use eval::eval;

pub fn find_greatest(list_of_numbers: &[i32]) -> &i32 {
    let mut largest = &list_of_numbers[0];
    for number in list_of_numbers {
        if number > largest {
            largest = number
        }
    }
    largest
}

pub fn log(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
) {
    let string = getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        0,
    )
    .first()
    .unwrap()
    .to_string();
    println!("{}", string);
}

pub fn getstring(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
    int: i32,
) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x + 1..contents.len() {
        if !skip {
            if int == 0 || int == 2 {
                if contents[y] == "(" {
                    n += 1;
                } else if contents[y] == ")" {
                    n -= 1;
                }
            } else if int == 1 {
                if contents[y] == "[" {
                    n += 1;
                } else if contents[y] == "]" {
                    n -= 1;
                }
            }
            if n == 0 {
                skip = true;
                for z in x..y + 1 {
                    vec.push(contents[z].to_string());
                }
            }
        }
    }
    vec.remove(0);
    vec.remove(vec.len() - 1);
    if int == 0 || int == 2 {
        vec.remove(0);
    }
    let skip = false;
    let mut imput_s: String = "".to_string();
    let mut n = 0;
    let mut skips = 0;
    let mut output_array = Vec::new();
    for y in 0..vec.len() {
        if skips == 0 {
            if !skip {
                let mut continues = true;
                if n % 2 == 0 && vec[y] == "," {
                    output_array.push(imput_s);
                    imput_s = "".to_string();
                } else if y < 1 {
                    if vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`" {
                        n += 1;
                        continues = false;
                    }
                } else if (vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`")
                    && vec[y - 1] != "\\"
                {
                    n += 1;
                    continues = false;
                }
                if !continues {
                } else if vec[y] == "(" && n % 2 == 0 {
                } else if vec[y] == ")" && n % 2 == 0 {
                } else if n % 2 == 1 {
                    imput_s.push_str(vec[y].as_str());
                } else if vec[y] == "math" {
                    imput_s.push_str(
                        math(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                        )
                        .to_string()
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 += 1;
                            } else if vec[f] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..f + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "round" {
                    imput_s.push_str(
                        round(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                        )
                        .to_string()
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 += 1;
                            } else if contents[f] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..f + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "GET" {
                    imput_s.push_str(
                        get_request(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                        )
                        .to_string()
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[f] == "(" {
                                n2 += 1;
                            } else if contents[f] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..f + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "replace" {
                    imput_s.push_str(
                        replace(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                        )
                        .to_string()
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 += 1;
                            } else if vec[f] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..f + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "input" {
                    imput_s.push_str(input().to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 += 1;
                            } else if vec[f] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..f + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "exec" {
                    imput_s.push_str(
                        exec(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                        )
                        .to_string()
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 += 1;
                            } else if vec[f] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..f + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "trim" {
                    imput_s.push_str(
                        trim(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                        )
                        .to_string()
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 += 1;
                            } else if vec[f] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..f + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "timeh" {
                    imput_s.push_str(time_readable().to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 += 1;
                            } else if vec[f] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..f + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "time" {
                    imput_s.push_str(time().to_string().as_str());
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 += 1;
                            } else if vec[f] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..f + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if vec[y] == "getcont" {
                    imput_s.push_str(
                        get_contents(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                        )
                        .to_string()
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for f in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if vec[f] == "(" {
                                n2 += 1;
                            } else if vec[f] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..f + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                } else if int == 2
                    && (vec[y] == "="
                        || vec[y] == ">"
                        || vec[y] == "<"
                        || vec[y] == "!"
                        || vec[y] == "|"
                        || vec[y] == "&")
                {
                    imput_s.push_str(vec[y].as_str());
                } else {
                    let mut postion = memory_names.len();
                    let mut skip1 = false;
                    for pos in 0..memory_names.len() {
                        if !skip1 && memory_names[pos] == vec[y] {
                            postion = pos;
                            skip1 = true;
                        }
                    }
                    if postion != memory_names.len() {
                        if y + 1 < vec.len() {
                            if vec[y + 1] == "(" {
                                let number_of_item = math(
                                    y,
                                    vec.to_vec(),
                                    memory_names.clone(),
                                    memory_values.clone(),
                                    func_names.clone(),
                                    func_par.clone(),
                                    func_code.clone(),
                                )
                                .to_string();
                                imput_s.push_str(
                                    &*memory_values[postion]
                                        .split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                                        .nth(number_of_item.parse().unwrap())
                                        .unwrap()
                                        .to_string(),
                                );
                            } else {
                                imput_s.push_str(&*memory_values[postion].to_string());
                            }
                        } else {
                            imput_s.push_str(&*memory_values[postion].to_string());
                        }
                    } else {
                        let mut postion = func_names.len();
                        let mut skip = false;
                        for pos in 0..func_names.len() {
                            if !skip && func_names[pos] == vec[y] {
                                postion = pos;
                                skip = true;
                            }
                        }
                        let mut contetntstr: Vec<String> = Vec::new();
                        if postion != func_names.len() {
                            for x in func_code[postion].split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                            {
                                contetntstr.push(x.to_string());
                            }
                            imput_s.push_str(
                                run::run(
                                    contetntstr,
                                    dev,
                                    memory_names.clone(),
                                    memory_values.clone(),
                                    memory_types.clone(),
                                    func_names.clone(),
                                    func_par.clone(),
                                    func_code.clone(),
                                )
                                .as_str(),
                            );
                        }
                    }
                }
            }
        } else {
            skips -= 1;
        }
    }
    output_array.push(imput_s);
    output_array
}

pub fn eval_eval(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
) -> Vec<String> {
    return lexer::lexer(
        getstring(
            x,
            contents,
            memory_names,
            memory_values,
            memory_types,
            func_names,
            func_par,
            func_code,
            dev,
            0,
        )
        .first()
        .unwrap()
        .to_string(),
        dev,
    );
}

pub fn exec(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
) -> String {
    let string = getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        0,
    )
    .first()
    .unwrap()
    .to_string();
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
    } else {
        let mut endvec: Vec<&str> = vecs.split(' ').collect();
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

pub fn round(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
) -> i32 {
    getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        0,
    )[0]
    .parse::<f32>()
    .unwrap()
    .round() as i32
}

pub fn set_contents(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
) -> std::io::Result<()> {
    let vec = getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        0,
    );
    let file_s = vec[0].to_string();
    let text_s = vec[1].to_string();
    if dev {
        println!("vec: {:?}", vec);
        println!("file_s: {}", file_s);
        println!("text_s: {}", text_s);
    }
    let mut file = File::create(file_s)?;
    file.write_all(text_s.as_ref())?;
    Ok(())
}

pub fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

pub fn get_contents(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
) -> String {
    let string = getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        0,
    )
    .first()
    .unwrap()
    .to_string();
    let maybe_contents = fs::read_to_string(string);

    if maybe_contents.is_ok() {
        maybe_contents.unwrap()
    } else {
        panic!("Could not open file for reading.");
    }
}

pub fn replace(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
) -> String {
    let vec = getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        0,
    );
    let imput_s = vec[0].to_string();
    let find_s = vec[1].to_string();
    let replacer_s = vec[2].to_string();
    if dev {
        println!("vec: {:?}", vec);
        println!("imput_s: {}", imput_s);
        println!("find_s: {}", find_s);
        println!("replacer_s: {}", replacer_s);
    }
    imput_s.replace(&*find_s, &*replacer_s)
}

pub fn imp(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
    begining: String,
) -> Vec<String> {
    let mut string = begining.clone();
    if !begining.is_empty() {
        string.push('/');
    }
    string.push_str(
        getstring(
            x,
            contents,
            memory_names.clone(),
            memory_values.clone(),
            memory_types.clone(),
            func_names.clone(),
            func_par.clone(),
            func_code.clone(),
            dev,
            0,
        )
        .first()
        .unwrap()
        .to_string()
        .as_str(),
    );
    if dev {
        println!("string: {}", string);
    }
    let mut contents;
    let mut came_from_imp = false;
    if string.clone().starts_with("https://") || string.starts_with("http://") {
        let mut dst = Vec::new();
        let mut easy = Easy::new();
        easy.url(&*string.clone()).unwrap();

        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
        drop(transfer);

        contents = dst.iter().map(|&c| c as char).collect::<String>();
    } else if string.clone().ends_with(".nys") {
        let maybe_contents = fs::read_to_string(string.clone());
        contents = if maybe_contents.is_ok() {
            maybe_contents.unwrap()
        } else {
            panic!("Could not open file for reading.");
        };
    } else {
        came_from_imp = true;
        let mut newstring = begining.clone();
        if begining.is_empty() {
            newstring.push_str("dep/");
        } else {
            newstring.push_str("/dep/");
        }
        newstring.push_str(string.clone().as_str());
        newstring.push_str("/src/main.nys");
        println!("{}", newstring);
        let maybe_contents = fs::read_to_string(newstring);
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
    let mut to_parse = lexer::lexer(contents, dev);
    if dev {
        println!("to_parse: {:?}", to_parse);
    }
    if came_from_imp {
        let mut quotes = 0;
        let mut squigle = 0;
        let mut readfrom = 0;
        let mut read = true;
        while read {
            read = false;
            let mut skiperwiper = false;
            for x in readfrom..to_parse.len() {
                if !skiperwiper {
                    if dev {
                        println!("contents[x]: {}", to_parse[x]);
                        println!("x: {}", x);
                        println!("quotes: {}", quotes);
                        println!("squigle: {}", squigle);
                    }
                    if (to_parse[x] == "\"" || to_parse[x] == "\'" || to_parse[x] == r"\`")
                        && to_parse[x - 1] != "\\"
                    {
                        quotes += 1;
                    }
                    if (to_parse[x] == "{" || to_parse[x] == "[") && quotes % 2 == 0 {
                        squigle += 1;
                    }
                    if (to_parse[x] == "}" || to_parse[x] == "]") && quotes % 2 == 0 {
                        squigle -= 1;
                    }
                    if quotes % 2 == 0 && squigle == 0 && to_parse[x] == "imp" {
                        let mut new_loc = begining.clone();
                        if begining.clone().is_empty() {
                            new_loc.push_str("dep/");
                        } else {
                            new_loc.push_str("/dep/");
                        }
                        new_loc.push_str(string.clone().as_str());
                        let imp = imp(
                            x,
                            to_parse.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            new_loc,
                        );
                        readfrom = x;
                        skiperwiper = true;
                        read = true;
                        let mut delete = Vec::new();
                        let mut deleted = 0;
                        let mut skirt = false;
                        let mut n3 = 0;
                        delete.push(x);
                        for y1 in x + 1..to_parse.len() {
                            if !skirt {
                                if to_parse[y1] == "(" {
                                    n3 += 1;
                                }
                                if n3 == 0 {
                                    skirt = true;
                                }
                                if to_parse[y1] == ")" {
                                    n3 -= 1;
                                }
                                delete.push(y1);
                            }
                        }
                        for item in delete {
                            to_parse.remove(item - deleted);
                            deleted += 1;
                        }
                        let mut new_vec = Vec::new();
                        for itom in 0..to_parse.len() {
                            if itom == x {
                                for item in imp.clone() {
                                    new_vec.push(item);
                                }
                            }
                            new_vec.push(to_parse[itom].clone());
                        }
                        to_parse = new_vec;
                    }
                }
            }
        }
    }
    to_parse
}

pub fn math(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    _func_names: Vec<String>,
    _func_par: Vec<String>,
    _func_code: Vec<String>,
) -> f32 {
    let mut vec: Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    for y in x + 1..contents.len() {
        if !skip {
            if contents[x + 1] != "(" {
                println!(
                    "You have to put a parentheses after the function on line {}",
                    get_line(x, contents)
                );
                std::process::exit(1);
            }
            if contents[y] == "(" {
                n += 1;
            } else if contents[y] == ")" {
                n -= 1;
            }
            if n % 2 == 0 {
                skip = true;
                for z in x + 1..y + 1 {
                    vec.push(contents[z].to_string());
                }
            }
        }
    }
    let mut n = 0;
    let mut what_to_do_first = Vec::new();
    vec.remove(0);
    vec.remove(vec.len() - 1);
    for y in 0..vec.len() {
        if vec[y] == "(" {
            n += 1;
        } else if vec[y] == ")" {
            n -= 1;
        }
        what_to_do_first.push(n);
    }
    if find_greatest(&*what_to_do_first) > &0 {
    } else {
        let mut keep_going = true;
        while keep_going {
            let mut skip = false;
            for y in 0..vec.len() {
                if !skip {
                    let mut rng = rand::thread_rng();
                    let if_number = vec[y].chars();
                    let mut if_number_bool = true;
                    for c in if_number {
                        if (char::is_numeric(c) || c == '.') && if_number_bool {
                            if_number_bool = true;
                        } else {
                            if_number_bool = false;
                        }
                    }
                    if !if_number_bool {
                        let mut postion1 = memory_names.len();
                        let mut skip = false;
                        for pos in 0..memory_names.len() {
                            if !skip && memory_names[pos] == vec[y] {
                                postion1 = pos;
                                skip = true;
                            }
                        }
                        if postion1 != memory_names.len() {
                            vec[y] = memory_values[postion1 - 1].to_string();
                        }
                    }
                    if vec[y].to_lowercase() == "random" {
                        vec[y] = rng.gen::<f32>().to_string();

                        skip = true;
                    } else if vec[y] == "+" {
                        if vec[y + 1].to_lowercase() == "random" {
                            vec[y + 1] = rng.gen::<f32>().to_string();
                        }
                        let if_number = vec[y + 1].chars();
                        let mut if_number_bool = true;
                        for c in if_number {
                            if (char::is_numeric(c) || c == '.') && if_number_bool {
                                if_number_bool = true;
                            } else {
                                if_number_bool = false;
                            }
                        }
                        if !if_number_bool {
                            let mut postion1 = memory_names.len();
                            let mut skip = false;
                            for pos in 0..memory_names.len() {
                                if !skip && memory_names[pos] == vec[y + 1] {
                                    postion1 = pos;
                                    skip = true;
                                }
                            }
                            if postion1 != memory_names.len() {
                                vec[y + 1] = memory_values[postion1].to_string();
                            }
                        }
                        vec[y - 1] = vec[y - 1]
                            .parse::<f32>()
                            .unwrap()
                            .add(vec[y + 1].parse::<f32>().unwrap())
                            .to_string();
                        vec.remove(y);
                        vec.remove(y);
                        skip = true;
                    } else if vec[y] == "-" {
                        if vec[y + 1].to_lowercase() == "random" {
                            vec[y + 1] = rng.gen::<f32>().to_string();
                        }
                        let if_number = vec[y + 1].chars();
                        let mut if_number_bool = true;
                        for c in if_number {
                            if (char::is_numeric(c) || c == '.') && if_number_bool {
                                if_number_bool = true;
                            } else {
                                if_number_bool = false;
                            }
                        }
                        if !if_number_bool {
                            let mut postion1 = memory_names.len();
                            let mut skip = false;
                            for pos in 0..memory_names.len() {
                                if !skip && memory_names[pos] == vec[y + 1] {
                                    postion1 = pos;
                                    skip = true;
                                }
                            }
                            if postion1 != memory_names.len() {
                                vec[y + 1] = memory_values[postion1].to_string();
                            }
                        }
                        vec[y - 1] = vec[y - 1]
                            .parse::<f32>()
                            .unwrap()
                            .sub(vec[y + 1].parse::<f32>().unwrap())
                            .to_string();
                        vec.remove(y);
                        vec.remove(y);
                        skip = true;
                    } else if vec[y] == "*" {
                        if vec[y + 1].to_lowercase() == "random" {
                            vec[y + 1] = rng.gen::<f32>().to_string();
                        }
                        let if_number = vec[y + 1].chars();
                        let mut if_number_bool = true;
                        for c in if_number {
                            if (char::is_numeric(c) || c == '.') && if_number_bool {
                                if_number_bool = true;
                            } else {
                                if_number_bool = false;
                            }
                        }
                        if !if_number_bool {
                            let mut postion1 = memory_names.len();
                            let mut skip = false;
                            for pos in 0..memory_names.len() {
                                if !skip && memory_names[pos] == vec[y + 1] {
                                    postion1 = pos;
                                    skip = true;
                                }
                            }
                            if postion1 != memory_names.len() {
                                vec[y + 1] = memory_values[postion1].to_string();
                            }
                        }
                        vec[y - 1] = vec[y - 1]
                            .parse::<f32>()
                            .unwrap()
                            .mul(vec[y + 1].parse::<f32>().unwrap())
                            .to_string();
                        vec.remove(y);
                        vec.remove(y);
                        skip = true;
                    } else if vec[y] == "/" {
                        if vec[y + 1].to_lowercase() == "random" {
                            vec[y + 1] = rng.gen::<f32>().to_string();
                        }
                        let if_number = vec[y + 1].chars();
                        let mut if_number_bool = true;
                        for c in if_number {
                            if (char::is_numeric(c) || c == '.') && if_number_bool {
                                if_number_bool = true;
                            } else {
                                if_number_bool = false;
                            }
                        }
                        if !if_number_bool {
                            let mut postion1 = memory_names.len();
                            let mut skip = false;
                            for pos in 0..memory_names.len() {
                                if !skip && memory_names[pos] == vec[y + 1] {
                                    postion1 = pos;
                                    skip = true;
                                }
                            }
                            if postion1 != memory_names.len() {
                                vec[y + 1] = memory_values[postion1].to_string();
                            }
                        }
                        vec[y - 1] = vec[y - 1]
                            .parse::<f32>()
                            .unwrap()
                            .div(vec[y + 1].parse::<f32>().unwrap())
                            .to_string();
                        vec.remove(y);
                        vec.remove(y);
                        skip = true;
                    } else if vec[y] == "^" {
                        if vec[y + 1].to_lowercase() == "random" {
                            vec[y + 1] = rng.gen::<f32>().to_string();
                        }
                        let if_number = vec[y + 1].chars();
                        let mut if_number_bool = true;
                        for c in if_number {
                            if (char::is_numeric(c) || c == '.') && if_number_bool {
                                if_number_bool = true;
                            } else {
                                if_number_bool = false;
                            }
                        }
                        if !if_number_bool {
                            let mut postion1 = memory_names.len();
                            let mut skip = false;
                            for pos in 0..memory_names.len() {
                                if !skip && memory_names[pos] == vec[y + 1] {
                                    postion1 = pos;
                                    skip = true;
                                }
                            }
                            if postion1 != memory_names.len() {
                                vec[y + 1] = memory_values[postion1].to_string();
                            }
                        }
                        vec[y - 1] = vec[y - 1]
                            .parse::<f32>()
                            .unwrap()
                            .powf(vec[y + 1].parse::<f32>().unwrap())
                            .to_string();
                        vec.remove(y);
                        vec.remove(y);
                        skip = true;
                    } else {
                        let mut postion = memory_names.len();
                        let mut skip = false;
                        for pos in 0..memory_names.len() {
                            if !skip && memory_names[pos] == vec[y] {
                                postion = pos;
                                skip = true;
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
    // println!("vec: {:?}", vec);
    vec[0].parse().unwrap()
}

pub fn trim(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
) -> String {
    return getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        0,
    )
    .first()
    .unwrap()
    .to_string()
    .trim()
    .to_string();
}

pub fn time_readable() -> String {
    let time = time();
    let d = UNIX_EPOCH + Duration::from_millis(time as u64);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    timestamp_str
}

pub fn array_fn(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
) -> Vec<String> {
    getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        1,
    )
}

pub fn get_line(x: usize, contents: Vec<String>) -> i32 {
    let mut line = 1;
    for n in 0..x {
        if contents[n] == "\n" {
            line += 1;
        }
    }
    line
}

pub fn get_request(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
) -> String {
    let string = getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        0,
    )
    .first()
    .unwrap()
    .to_string();
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

pub fn post_request(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
) {
    let reply = getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        0,
    );
    let imput_s = reply[0].to_string();
    let find_s = reply[1].to_string();
    if dev {
        println!("vec: {:?}", reply);
        println!("imput_s: {}", imput_s);
        println!("find_s: {}", find_s);
    }
    let mut data = find_s.as_bytes();

    let mut easy = Easy::new();
    easy.url(&*imput_s).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer
        .read_function(|buf| Ok(data.read(buf).unwrap_or(0)))
        .unwrap();
    transfer.perform().unwrap();
}

pub fn time() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis() as f64
}

pub fn group_fn(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
) -> Vec<String> {
    getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        1,
    )
}

pub fn declare(
    mut memory_names: Vec<String>,
    mut memory_values: Vec<String>,
    mut memory_types: Vec<String>,
    mut func_names: Vec<String>,
    mut func_par: Vec<String>,
    mut func_code: Vec<String>,
    mut contents: Vec<String>,
    mut group_memory: Vec<String>,
    mut x: usize,
    mut dev: bool,
) {
    let memory_names1 = memory_names.clone();
    let memory_values1 = memory_values.clone();
    let memory_types1 = memory_types.clone();
    let func_names1 = func_names.clone();
    let func_par1 = func_par.clone();
    let func_code1 = func_code.clone();
    let memory_names_save = memory_names.clone();
    let memory_values_save = memory_values.clone();
    let mut types = false;
    let mut position = x + 1;
    let square_brackets = 0;

    // get type
    if contents[position] == "int" {
        memory_types.push(String::from("int"));
        memory_names.push(contents[position + 1].clone());
        position += 1;
    } else if contents[position] == "str" {
        memory_types.push(String::from("str"));
        memory_names.push(contents[position + 1].clone());
        position += 1;
    } else if contents[position] == "arr" {
        memory_types.push(String::from("arr"));
        memory_names.push(contents[position + 1].clone());
        position += 1;
    } else if contents[position] == "grp" {
        memory_types.push(String::from("grp"));
        memory_names.push(contents[position + 1].clone());
        position += 1;
    } else if contents[position] == "inf" {
        memory_types.push(String::from("inf"));
        memory_names.push(contents[position + 1].clone());
        position += 1;
    } else if contents[position] == "anon" {
        memory_types.push(String::from("anon"));
        types = true;
    }

    //more vars
    let mut clone_class = String::from("");
    let mut value = String::new();
    let mut value_array = Vec::new();
    let mut value_group = Vec::new();

    //more vars
    let mut n = 0;
    let mut quote = 0;
    let mut squig = 0; // brace/squiggly bracket checker
    let mut brackets = 0;

    //more vars
    position += 2;
    let mut group = false;

    loop {
        if contents[position] == "[" {
            // if bracket run fn array
            value_array = array_fn(
                position - 1,
                contents.clone(),
                memory_names.clone(),
                memory_values.clone(),
                memory_types.clone(),
                func_names.clone(),
                func_par.clone(),
                func_code.clone(),
                dev,
            );
            break;
        } else if contents[position] == "{" {
            // if group run
            value_group = group_fn(
                position - 1,
                contents.clone(),
                memory_names.clone(),
                memory_values.clone(),
                memory_types.clone(),
                func_names.clone(),
                func_par.clone(),
                func_code.clone(),
                dev,
            );
            group = true;
            squig += 1;
        } else if contents[position] == "}" {
            squig -= 1;
            if group && squig == 0 && contents[position + 1] == "," {
                clone_class = contents[position + 2].clone().to_string();
            }
        } else if contents[position] == "(" {
            brackets += 1;
        } else if contents[position] == ")" {
            brackets -= 1;
        } else if square_brackets == 0 {
            if contents[position] == ";" {
                if dev {
                    println!(
                        "contents[x+move_up+move_up+move_up_up+move_final]: {:?}",
                        contents[position]
                    );
                }
                break;
            } else if !group {
                if (contents[position] == "\""
                    || contents[position] == "\'"
                    || contents[position] == r"\`")
                    && contents[position - 1] != "\\"
                {
                    quote += 1;
                } else if brackets == 0 {
                    if contents[position] == "math" {
                        value.push_str(
                            math(
                                position,
                                contents.clone(),
                                memory_names.clone(),
                                memory_values.clone(),
                                func_names.clone(),
                                func_par.clone(),
                                func_code.clone(),
                            )
                            .to_string()
                            .as_str(),
                        );
                        n = 1;
                    } else if contents[position] == "round" {
                        value.push_str(
                            round(
                                position,
                                contents.clone(),
                                memory_names.clone(),
                                memory_values.clone(),
                                memory_types.clone(),
                                func_names.clone(),
                                func_par.clone(),
                                func_code.clone(),
                                dev,
                            )
                            .to_string()
                            .as_str(),
                        );
                        n = 1;
                    } else if contents[position] == "replace" {
                        value.push_str(
                            replace(
                                position,
                                contents.clone(),
                                memory_names.clone(),
                                memory_values.clone(),
                                memory_types.clone(),
                                func_names.clone(),
                                func_par.clone(),
                                func_code.clone(),
                                dev,
                            )
                            .to_string()
                            .as_str(),
                        );
                        n = 1;
                    } else if contents[position] == "input" {
                        value.push_str(input().to_string().as_str());
                        n = 1;
                    } else if contents[position] == "exec" {
                        value.push_str(
                            exec(
                                position,
                                contents.clone(),
                                memory_names.clone(),
                                memory_values.clone(),
                                memory_types.clone(),
                                func_names.clone(),
                                func_par.clone(),
                                func_code.clone(),
                                dev,
                            )
                            .to_string()
                            .as_str(),
                        );
                        n = 1;
                    } else if contents[position] == "trim" {
                        value.push_str(
                            trim(
                                position,
                                contents.clone(),
                                memory_names.clone(),
                                memory_values.clone(),
                                memory_types.clone(),
                                func_names.clone(),
                                func_par.clone(),
                                func_code.clone(),
                                dev,
                            )
                            .to_string()
                            .as_str(),
                        );
                        n = 1;
                    } else if contents[position] == "timeh" {
                        value.push_str(time_readable().to_string().as_str());
                        n = 1;
                    } else if contents[position] == "time" {
                        value.push_str(time().to_string().as_str());
                        n = 1;
                    } else if contents[position] == "getcont" {
                        value.push_str(
                            get_contents(
                                position,
                                contents.clone(),
                                memory_names.clone(),
                                memory_values.clone(),
                                memory_types.clone(),
                                func_names.clone(),
                                func_par.clone(),
                                func_code.clone(),
                                dev,
                            )
                            .to_string()
                            .as_str(),
                        );
                        n = 1;
                    } else if n == 0 {
                        if quote % 2 == 1 {
                            value.push_str(contents[position].as_str());
                        } else {
                            let mut positions = memory_names_save.len();
                            let mut skip = false;
                            for pos in 0..memory_names_save.len() {
                                if !skip && memory_names_save[pos] == contents[position] {
                                    positions = pos;
                                    skip = true;
                                }
                            }
                            if positions != memory_names_save.len()
                                && (contents[x + 1].trim() == ":" || contents[x + 1].trim() == "=")
                            {
                                value.push_str(memory_values_save[positions].to_string().as_str());
                            } else {
                                let mut postion = func_names.len();
                                let mut skip = false;
                                for pos in 0..func_names.len() {
                                    if !skip && func_names[pos] == contents[position] {
                                        postion = pos;
                                        skip = true;
                                    }
                                }
                                if postion != func_names.len() {
                                    let mut contetntstr: Vec<String> = Vec::new();
                                    for x in func_code[postion]
                                        .split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                                    {
                                        contetntstr.push(x.to_string());
                                    }
                                    value.push_str(
                                        run::run(
                                            contetntstr,
                                            dev,
                                            memory_names1.clone(),
                                            memory_values1.clone(),
                                            memory_types1.clone(),
                                            func_names1.clone(),
                                            func_par1.clone(),
                                            func_code1.clone(),
                                        )
                                        .as_str(),
                                    );
                                } else {
                                    value.push_str(contents[position].as_str());
                                }
                            }
                        }
                    }
                    if n >= 1 && contents[position] == "(" {
                        n += 1
                    } else if n >= 1 && contents[position] == ")" {
                        n -= 1;
                        if n == 1 {
                            n = 0;
                        }
                    }
                }
            }
        }
        position += 1;
        if dev {
            println!("position: {:?}", position);
        }
    }
    if value_array.join("") != "" {
        memory_values.push(
            value_array
                .join("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                .clone(),
        );
    } else if value_group.join("") != "" {
        value_group.push(clone_class.clone());
        memory_values.push(
            value_group
                .join("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                .clone(),
        );
        let name_of_item = memory_names[memory_names.len() - 1].clone();
        for d in 0..value_group.len() - 1 {
            let mut name: String = name_of_item.to_string();
            name.push('.');
            let mut location = 0;
            for items in 0..group_memory.len() {
                if items < group_memory.len() - 1
                    && group_memory[items + 1].parse::<i32>().is_ok()
                    && group_memory[items] == clone_class.clone()
                {
                    location = items + (d * 2) + 3;
                }
            }
            name.push_str(&*group_memory[location]);
            memory_names.push(name.clone());
            memory_values.push(value_group[d].clone());
            memory_types.push("str".parse().unwrap());
        }
    }
    // } else if value.contains("+") && memory_types[memory_types.len() - 1] != "str"{
    //     for names in 0..memory_names.len() {
    //         value = str::replace(value.as_str(), memory_names[names].as_str(), memory_values[names].as_str());
    //     }
    let mut var_rep: Vec<String> = Vec::new();
    for mut item in x + 4..contents.len() {
        if contents[item] == ";" {
            break;
        } else if contents[item] == "\""
            || contents[item] == "\'"
            || contents[item] == "\r"
            || contents[item] == "\n"
        {
            // || contents[item] == "\r" || contents[item] == "\n"
            while true {
                // println!("{:?}", contents[item]);
                if contents[item] == "\""
                    || contents[item] == "\'"
                    || contents[item] == "+"
                    || contents[item] == "-"
                    || contents[item] == "/"
                    || contents[item] == "*"
                {
                    // println!{"broken!"}
                    break;
                } else {
                    item = item + 1;
                }
            }
        } else {
            if contents[item] == "\r" || contents[item] == "\n" {
            } else {
                var_rep.push(contents[item].clone());
            }
        }
    }
    // println!("{:?}", var_rep);
    // // println!("{:?} success!", var_rep);
    // println!("{:?}", var_rep.len());
    if var_rep.len() == 0 {
    } else {
        for name in 0..memory_names.len() {
            for item in 0..var_rep.len() {
                let tempname = "123456789".to_string() + var_rep[item].clone().as_str();
                // println!("{:?}, {:?}", tempname, memory_names[name]);
                if memory_names[name] == tempname {
                    value = value.to_string().replace(
                        &memory_names[name].replace("123456789", ""),
                        memory_values[name].as_str(),
                    );
                }
            }
        }
    }
    if memory_types[memory_types.len() - 1] == "int" {
        let number = eval(value.clone().as_str());
        if number.is_ok() {
            memory_values.push(number.unwrap().to_string());
        } else {
            memory_values.push(value.clone());
        }
    } else {
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

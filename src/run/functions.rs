use crate::{lexer, run};
use curl::easy::Easy;
use rand::Rng;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{env, fs};
extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;
use rustc_serialize::json::Json;
use sysinfo::{ProcessorExt, SystemExt};

extern crate meval;

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
    uses: Vec<String>,
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
        uses,
        0,
    )
    .first()
    .unwrap()
    .to_string();
    format!("println!(\"{{}}\", {});", string)
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
    uses: Vec<String>,
    int: i64,
) -> Vec<String> {
    if dev {
        println!("memory_names: {:?}", memory_names);
        println!("memory_values: {:?}", memory_values);
        println!("memory_types: {:?}", memory_types);
        println!("func_names: {:?}", func_names);
        println!("func_par: {:?}", func_par);
        println!("func_code: {:?}", func_code);
    }
    let mut vec: Vec<String> = Vec::new();
    let mut skip = false;
    let mut n = 0;
    if int == 0 || int == 2 || int == 3 {
        if contents[x + 1] != "(" {
            run::error(
                [
                    "You need to have a \"(\" after the function on line ",
                    get_line(x, contents.clone()).to_string().as_str(),
                ]
                .join(""),
            );
        }
    } else if (int == 1 || int == 4) && contents[x + 1] == "[" {
        run::error(
            [
                "You need to have a \"[\" after the function on line ",
                get_line(x, contents.clone()).to_string().as_str(),
            ]
            .join(""),
        );
    }
    for y in x + 1..contents.len() {
        if !skip {
            if int == 0 || int == 2 || int == 3 {
                if contents[y] == "(" {
                    n += 1;
                } else if contents[y] == ")" {
                    n -= 1;
                }
            } else if int == 1 || int == 4 {
                if contents[y] == "[" {
                    n += 1;
                } else if contents[y] == "]" {
                    n -= 1;
                }
            }
            if n == 0 {
                skip = true;
                for elem in x..y + 1 {
                    vec.push(contents[elem].to_string());
                }
            }
        }
    }
    vec.remove(0);
    vec.pop();
    match int {
        0 => {
            vec.remove(0);
        }
        2 => {
            vec.remove(0);
        }
        3 => {
            vec.remove(0);
        }
        _ => {}
    }
    skip = false;
    let mut imput_s: String = "".to_string();
    let mut n = 0;
    let mut skips = 0;
    let mut parent = 0;
    let mut output_array = Vec::new();
    if dev {
        println!("vec: {:?}", vec);
    }
    for y in 0..vec.len() {
        if skips == 0 {
            if !skip {
                let mut continues = true;
                if (n % 2 == 0 || int == 3) && vec[y] == "," {
                    let mut number_of_items = 1;
                    let mut number_of_quotes = 0;
                    let mut number_of_brackets = 0;
                    let mut last_item_was_backslash = false;
                    for f in imput_s.trim_matches(',').trim().chars() {
                        if f == '\\' {
                            last_item_was_backslash = true;
                        } else {
                            if (f == '\"' || f == '\'' || f == '`') && !last_item_was_backslash {
                                number_of_quotes += 1;
                            } else if number_of_quotes % 2 == 0
                                && f == '('
                                && !last_item_was_backslash
                            {
                                number_of_brackets += 1;
                            } else if number_of_quotes % 2 == 0
                                && f == ')'
                                && !last_item_was_backslash
                            {
                                number_of_brackets -= 1;
                            } else if number_of_quotes % 2 == 0
                                && f == ','
                                && number_of_brackets == 0
                            {
                                number_of_items += 1;
                            }
                            last_item_was_backslash = false;
                        }
                    }
                    if imput_s.trim() != "" {
                        output_array.push(format!(
                            "format!(\"{}\",{})",
                            "{}".repeat(number_of_items),
                            imput_s.trim_matches(',').trim()
                        ));
                    }
                    imput_s = "".to_string();
                } else if y < 1 {
                    if vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`" {
                        imput_s.push_str(&vec[y]);
                        n += 1;
                        continues = false;
                    }
                } else if (vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`")
                    && vec[y - 1] != "\\"
                {
                    n += 1;
                    imput_s.push_str(&vec[y]);
                    continues = false;
                    if n % 2 == 0 {
                        imput_s.push(',');
                    }
                } else if y + 1 < vec.len()
                    && (vec[y + 1] == "\"" || vec[y + 1] == "\'" || vec[y + 1] == r"\`")
                    && vec[y] == "\\"
                {
                    continues = false;
                    imput_s.push('\\');
                }
                if !continues {
                } else if vec[y] == "(" && n % 2 == 0 {
                    parent += 1;
                } else if vec[y] == ")" && n % 2 == 0 {
                    parent -= 1;
                } else if parent != 0 || (int == 3 && vec[y] == ",") {
                } else if n % 2 == 1 || vec[y].parse::<f64>().is_ok() {
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
                            uses.clone(),
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
                    imput_s.push(',');
                } else if vec[y] == "arg" {
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
                    imput_s.push(',');
                    if leng == 2 {
                        imput_s.push_str("env::args()");
                    } else {
                        imput_s.push_str(
                            format!(
                                "env::args()
                                .nth({} as usize)
                                .unwrap()
                                .as_str()",
                                math(
                                    y,
                                    vec.to_vec(),
                                    memory_names.clone(),
                                    memory_values.clone(),
                                    func_names.clone(),
                                    func_par.clone(),
                                    func_code.clone(),
                                    uses.clone(),
                                )
                            )
                            .as_str(),
                        );
                    }
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
                            uses.clone(),
                        )
                        .to_string()
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for elem in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[elem] == "(" {
                                n2 += 1;
                            } else if contents[elem] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..elem + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                    imput_s.push(',');
                } else if vec[y] == "split" {
                    imput_s.push_str(
                        split(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            uses.clone(),
                        )
                        .to_string()
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for elem in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[elem] == "(" {
                                n2 += 1;
                            } else if contents[elem] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..elem + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                    imput_s.push(',');
                } else if vec[y] == "join" {
                    imput_s.push_str(
                        join(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            uses.clone(),
                        )
                        .to_string()
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for elem in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[elem] == "(" {
                                n2 += 1;
                            } else if contents[elem] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..elem + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                    imput_s.push(',');
                } else if vec[y] == "split_k" {
                    imput_s.push_str(
                        split_k(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            uses.clone(),
                        )
                        .to_string()
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for elem in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[elem] == "(" {
                                n2 += 1;
                            } else if contents[elem] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..elem + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                    imput_s.push(',');
                } else if vec[y] == "length" {
                    imput_s.push_str(
                        length(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            uses.clone(),
                        )
                        .to_string()
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for elem in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[elem] == "(" {
                                n2 += 1;
                            } else if contents[elem] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..elem + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                    imput_s.push(',');
                } else if vec[y] == "first" {
                    imput_s.push_str(
                        first(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            uses.clone(),
                        )
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for elem in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[elem] == "(" {
                                n2 += 1;
                            } else if contents[elem] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..elem + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                    imput_s.push(',');
                } else if vec[y] == "last" {
                    imput_s.push_str(
                        last(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            uses.clone(),
                        )
                        .as_str(),
                    );
                    let mut leng = 0;
                    let mut n2 = 0;
                    let mut skip1 = false;
                    for elem in y + 1..vec.len() {
                        if !skip1 {
                            if vec[y + 1] != "(" {
                                println!("You have to put a parentheses after a log");
                                std::process::exit(1);
                            }
                            if contents[elem] == "(" {
                                n2 += 1;
                            } else if contents[elem] == ")" {
                                n2 -= 1;
                            }
                            if n2 == 0 {
                                skip1 = true;
                                for _z in y + 1..elem + 1 {
                                    leng += 1;
                                }
                            }
                        }
                    }
                    skips = leng;
                    imput_s.push(',');
                } else if vec[y] == "random" {
                    imput_s.push_str(rand::thread_rng().gen::<f64>().to_string().as_str());
                } else if vec[y] == "request" {
                    imput_s.push_str(
                        request(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            uses.clone(),
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
                    imput_s.push(',');
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
                            uses.clone(),
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
                    imput_s.push(',');
                } else if vec[y] == "numeric" {
                    imput_s.push_str(
                        is_number(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            uses.clone(),
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
                    imput_s.push(',');
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
                    imput_s.push(',');
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
                            uses.clone(),
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
                    imput_s.push(',');
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
                            uses.clone(),
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
                    imput_s.push(',');
                } else if vec[y] == "os.total_memory" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.total_memory().to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "os.uptime" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.uptime().to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "os.name" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.name().unwrap().to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "os.kernel_version" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.kernel_version().unwrap().to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "os.os_version" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.os_version().unwrap().to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "os.host_name" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.host_name().unwrap().to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "os.used_memory" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.used_memory().to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "os.total_swap" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.total_swap().to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "os.used_swap" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.used_swap().to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "os.load_average.one" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.load_average().one.to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "os.cpu_usage" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.global_processor_info().cpu_usage().to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "os.cpu_name" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.global_processor_info().name().to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "os.load_average.five" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.load_average().five.to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "os.load_average.fifteen" && uses[0] == *"true" {
                    use sysinfo::System;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    imput_s.push_str(sys.load_average().fifteen.to_string().as_str());
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
                    imput_s.push(',');
                } else if vec[y] == "timeh" {
                    imput_s.push_str(
                        time_readable(
                            y,
                            vec.to_vec(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            uses.clone(),
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
                    imput_s.push(',');
                } else if vec[y] == "os" && uses[0] == *"true" {
                    imput_s.push_str(env::consts::OS);
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
                    imput_s.push(',');
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
                    imput_s.push(',');
                } else if vec[y] == "internet_time" {
                    imput_s.push_str(internet_time().to_string().as_str());
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
                    imput_s.push(',');
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
                            uses.clone(),
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
                    imput_s.push(',');
                } else if int == 2
                    && (vec[y] == "="
                        || vec[y] == ">"
                        || vec[y] == "<"
                        || vec[y] == "!"
                        || vec[y] == "|"
                        || vec[y] == "&"
                        || vec[y] == "true"
                        || vec[y] == "false")
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
                                    uses.clone(),
                                )
                                .to_string();
                                imput_s.push_str(
                                    memory_values[postion]
                                        .split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                                        .nth(number_of_item.parse().unwrap())
                                        .unwrap(),
                                );
                            } else if vec[y + 1] == "[" {
                                let json = Json::from_str(memory_values[postion].trim()).unwrap();
                                let original = getstring(
                                    y,
                                    vec.clone(),
                                    memory_names.clone(),
                                    memory_values.clone(),
                                    memory_types.clone(),
                                    func_names.clone(),
                                    func_par.clone(),
                                    func_code.clone(),
                                    dev,
                                    uses.clone(),
                                    4,
                                );
                                let slices: Vec<&str> =
                                    original.iter().map(AsRef::as_ref).collect();
                                imput_s.push_str(
                                    json.find_path(&slices).unwrap().to_string().as_str(),
                                );
                            } else {
                                imput_s.push_str(&memory_values[postion].to_string());
                            }
                        } else {
                            imput_s.push_str(&memory_values[postion].to_string());
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
                        if postion != func_names.len() {
                            let mut contetntstr: Vec<String> = Vec::new();
                            for x in func_code[postion].split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                            {
                                contetntstr.push(x.to_string());
                            }
                            let mut contetntstr1: Vec<String> = Vec::new();
                            for x in func_par[postion].split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                            {
                                contetntstr1.push(x.to_string());
                            }
                            let contetntstr2: Vec<String> = getstring(
                                y,
                                vec.clone(),
                                memory_names.clone(),
                                memory_values.clone(),
                                memory_types.clone(),
                                func_names.clone(),
                                func_par.clone(),
                                func_code.clone(),
                                dev,
                                uses.clone(),
                                0,
                            );
                            for x in func_par[postion].split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                            {
                                contetntstr1.push(x.to_string());
                            }
                            imput_s.push_str(
                                run::run(
                                    contetntstr,
                                    dev,
                                    uses.clone(),
                                    contetntstr1.clone(),
                                    contetntstr2.clone(),
                                    memory_types.clone(),
                                    func_names.clone(),
                                    func_par.clone(),
                                    func_code.clone(),
                                )
                                .as_str(),
                            );
                        } else if int == 3 {
                            imput_s.push_str(vec[y].as_str());
                        } else if !all_the_allowed_stuff().contains(&vec[y]) {
                            imput_s.push_str(&vec[y]);
                            imput_s.push(',');
                        }
                    }
                }
            }
        } else {
            skips -= 1;
        }
    }
    let mut number_of_items = 1;
    let mut number_of_quotes = 0;
    let mut number_of_brackets = 0;
    let mut last_item_was_backslash = false;
    for f in imput_s.trim_matches(',').trim().chars() {
        if f == '\\' {
            last_item_was_backslash = true;
        } else {
            if (f == '\"' || f == '\'' || f == '`') && !last_item_was_backslash {
                number_of_quotes += 1;
            } else if number_of_quotes % 2 == 0 && f == '(' && !last_item_was_backslash {
                number_of_brackets += 1;
            } else if number_of_quotes % 2 == 0 && f == ')' && !last_item_was_backslash {
                number_of_brackets -= 1;
            } else if number_of_quotes % 2 == 0 && f == ',' && number_of_brackets == 0 {
                number_of_items += 1;
            }
            last_item_was_backslash = false;
        }
    }
    if imput_s.trim() != "" {
        output_array.push(format!(
            "format!(\"{}\",{})",
            "{}".repeat(number_of_items),
            imput_s.trim_matches(',').trim()
        ));
    }
    if dev {
        println!("output_array: {:?}", output_array);
    }
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
    uses: Vec<String>,
) -> Vec<String> {
    let mut stringz = " ".to_string();
    stringz.push_str(
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
            uses,
            0,
        )
        .first()
        .unwrap()
        .as_str(),
    );
    lexer::lexer(stringz, dev)
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
    uses: Vec<String>,
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
        uses,
        0,
    )
    .first()
    .unwrap()
    .to_string();
    format!("exec({})", string)
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
    uses: Vec<String>,
) -> String {
    format!(
        "{}.parse::<f64>().unwrap().round() as i64",
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
            uses,
            0,
        )[0]
    )
}

pub fn split(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
    uses: Vec<String>,
) -> String {
    let mut items = getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        uses,
        0,
    );
    let replacer: String = items.last().unwrap().to_string();
    items.pop();
    format!("{}.split(\"{}\")", items.first().unwrap(), replacer)
}

pub fn split_k(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
    uses: Vec<String>,
) -> String {
    let mut items = getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        uses,
        0,
    );
    let replacer: String = items.last().unwrap().to_string();
    items.pop();
    format!("split_k({}, {})", items.first().unwrap(), replacer)
}

pub fn length(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
    uses: Vec<String>,
) -> String {
    format!(
        "{}.count()",
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
            uses,
            0,
        )
        .first()
        .unwrap()
    )
}

pub fn first(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
    uses: Vec<String>,
) -> String {
    format!(
        "{}.first().unwrap()",
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
            uses,
            0,
        )
        .first()
        .unwrap()
    )
}

pub fn last(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
    uses: Vec<String>,
) -> String {
    format!(
        "{}.last().unwrap()",
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
            uses,
            0,
        )
        .first()
        .unwrap()
    )
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
    uses: Vec<String>,
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
        uses,
        0,
    );
    format!("set_contents({},{});", vec[0], vec[1])
}

pub fn input() -> String {
    "input()".to_string()
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
    uses: Vec<String>,
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
        uses,
        0,
    )
    .first()
    .unwrap()
    .to_string();
    format!(
        "fs::read_to_string({}).expect(\"Unable to read file\")",
        string
    )
}

pub fn is_number(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
    uses: Vec<String>,
) -> String {
    format!(
        "{}.parse::<f64>().is_ok()",
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
            uses,
            0,
        )
        .first()
        .unwrap()
    )
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
    uses: Vec<String>,
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
        uses,
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
    format!("{}.replace({},{})", imput_s, find_s, replacer_s)
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
    uses: Vec<String>,
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
            uses.clone(),
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
    if string.starts_with("https://") || string.starts_with("http://") {
        let mut dst = Vec::new();
        let mut easy = Easy::new();
        easy.url(&string).unwrap();

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
    } else if string.ends_with(".nys") {
        let maybe_contents = fs::read_to_string(string.clone());
        contents = if maybe_contents.is_ok() {
            maybe_contents.unwrap()
        } else {
            run::error("Could not open file for reading.".to_string());
            "".to_string()
        };
    } else {
        came_from_imp = true;
        let mut newstring = begining.clone();
        if begining.is_empty() {
            newstring.push_str("dep/");
        } else {
            newstring.push_str("/dep/");
        }
        newstring.push_str(string.as_str());
        newstring.push_str("/src/main.nys");
        println!("{}", newstring);
        let maybe_contents = fs::read_to_string(newstring);
        contents = if maybe_contents.is_ok() {
            maybe_contents.unwrap()
        } else {
            run::error("Could not open file for reading.".to_string());
            "".to_string()
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
                            uses.clone(),
                            new_loc,
                        );
                        readfrom = x;
                        skiperwiper = true;
                        // read = true;
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
    uses: Vec<String>,
) -> f64 {
    meval::eval_str(
        getstring(
            x,
            contents,
            memory_names.clone(),
            memory_values,
            memory_names,
            _func_names,
            _func_par,
            _func_code,
            false,
            uses,
            3,
        )
        .first()
        .unwrap()
        .to_string()
        .as_str(),
    )
    .unwrap()
    .to_string()
    .parse::<f64>()
    .unwrap()
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
    uses: Vec<String>,
) -> String {
    let getstirng = getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        uses,
        0,
    );
    format!("{}.trim()", getstirng.first().unwrap())
}

pub fn time_readable(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
    uses: Vec<String>,
) -> String {
    let getstirng = getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        uses,
        0,
    );
    if !getstirng.is_empty() {
        let time: f64 = getstirng.get(0).unwrap().parse().unwrap();
        if getstirng.first().unwrap() == "true" {
            let d: SystemTime = UNIX_EPOCH + Duration::from_millis(time as u64);
            // Create DateTime from SystemTime
            let datetime = DateTime::<Utc>::from(d);
            // Formats the combined date and time with the specified format string.
            let timestamp_str = datetime.format(getstirng.get(1).unwrap()).to_string();
            timestamp_str
        } else {
            use humantime::format_duration;
            let d = Duration::from_millis(time as u64);
            format_duration(d).to_string()
        }
    } else {
        let time = time();
        let d = UNIX_EPOCH + Duration::from_millis(time.parse::<u64>().unwrap());
        // Create DateTime from SystemTime
        let datetime = DateTime::<Utc>::from(d);
        // Formats the combined date and time with the specified format string.
        let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
        timestamp_str
    }
}

pub fn get_line(x: usize, contents: Vec<String>) -> i64 {
    let mut line = 1;
    for n in 0..x {
        if contents[n] == "\n" {
            line += 1;
        }
    }
    if line >= run::code_to_add().matches('\n').count() {
        return (line - run::code_to_add().matches('\n').count() + 1) as i64;
    } else {
        (line + 1) as i64
    }
}

pub fn request(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
    uses: Vec<String>,
) -> String {
    let getstring_response = getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        uses,
        0,
    );
    format!("request(vec![{}])", getstring_response.join(","))
}

pub fn join(
    x: usize,
    contents: Vec<String>,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
    dev: bool,
    uses: Vec<String>,
) -> String {
    let getstring_response = getstring(
        x,
        contents,
        memory_names,
        memory_values,
        memory_types,
        func_names,
        func_par,
        func_code,
        dev,
        uses,
        0,
    );
    let last = getstring_response.last().unwrap();
    format!(
        "vec![{}].join({})",
        getstring_response[..getstring_response.len() - 1].join(","),
        last
    )
}

pub fn time() -> String {
    "time()".to_string()
}

pub fn internet_time() -> String {
    "internet_time()".to_string()
}

// pub fn group_fn(
//     x: usize,
//     contents: Vec<String>,
//     memory_names: Vec<String>,
//     memory_values: Vec<String>,
//     memory_types: Vec<String>,
//     func_names: Vec<String>,
//     func_par: Vec<String>,
//     func_code: Vec<String>,
//     dev: bool, uses: Vec<String>,
// ) -> Vec<String> {
//     getstring(
//         x,
//         contents,
//         memory_names,
//         memory_values,
//         memory_types,
//         func_names,
//         func_par,
//         func_code,
//         dev, uses.clone(),
//         1,
//     )
// }

pub fn all_the_allowed_stuff() -> Vec<String> {
    vec![
        ")".to_string(),
        "}".to_string(),
        "]".to_string(),
        "(".to_string(),
        "{".to_string(),
        "[".to_string(),
        ",".to_string(),
    ]
}

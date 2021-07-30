mod functions;
use crate::lexer;
use std::env;
use std::process::Command;
use std::{thread, time};
extern crate chrono;
extern crate eval;
use eval::eval;

pub fn run(
    mut contents: Vec<String>,
    dev: bool,
    mut memory_names: Vec<String>,
    mut memory_values: Vec<String>,
    mut memory_types: Vec<String>,
    mut func_names: Vec<String>,
    mut func_par: Vec<String>,
    mut func_code: Vec<String>,
) -> String {
    if dev {
        println!("contents: {:?}", contents);
    }
    let mut quotes = 0;
    let mut squigle = 0;
    let mut readfrom = 0;
    let mut read = true;
    let mut threads = Vec::new();
    let mut group_memory: Vec<String> = Vec::new();
    while read {
        read = false;
        let mut skiperwiper = false;
        for x in readfrom..contents.len() {
            if !skiperwiper {
                if dev {
                    println!("contents[x]: {}", contents[x]);
                    println!("x: {}", x);
                    println!("quotes: {}", quotes);
                    println!("squigle: {}", squigle);
                }
                if (contents[x] == "\"" || contents[x] == "\'" || contents[x] == r"\`")
                    && contents[x - 1] != "\\"
                {
                    quotes += 1;
                }
                if (contents[x] == "{" || contents[x] == "[" || contents[x] == "(")
                    && quotes % 2 == 0
                {
                    squigle += 1;
                }
                if (contents[x] == "}" || contents[x] == "]" || contents[x] == ")")
                    && quotes % 2 == 0
                {
                    squigle -= 1;
                }
                if quotes % 2 == 0 && squigle == 0 {
                    if contents[x] == "log" {
                        functions::log(
                            x,
                            contents.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                        );
                    } else if contents[x] == "ret" {
                        return functions::getstring(
                            x,
                            contents.clone(),
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
                        .trim()
                        .to_string();
                    } else if contents[x] == "exit" {
                        std::process::exit(1);
                    } else if contents[x] == "audio" {
                        let contents_save = contents.clone();
                        let memory_types_save = memory_types.clone();
                        let memory_values_save = memory_values.clone();
                        let memory_names_save = memory_names.clone();
                        let func_names_save = func_names.clone();
                        let func_par_save = func_par.clone();
                        let func_code_save = func_code.clone();
                        let handle = thread::spawn(move || {
                            let stringreturn = functions::getstring(
                                x,
                                contents_save.clone(),
                                memory_names_save.clone(),
                                memory_values_save.clone(),
                                memory_types_save.clone(),
                                func_names_save.clone(),
                                func_par_save.clone(),
                                func_code_save.clone(),
                                dev,
                                0,
                            )
                            .first()
                            .unwrap()
                            .to_string();
                            if env::consts::OS == "linux" {
                                let mut vecs = stringreturn.replace("\n", " ");
                                vecs = vecs.replace("\t", " ");
                                let endvec: Vec<&str> = vecs.split(' ').collect();
                                Command::new("cvlc")
                                    .args(endvec)
                                    .output()
                                    .expect("failed to execute process");
                            } else if env::consts::OS == "windows" {
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
                            } else if env::consts::OS == "macos" {
                                let mut vecs = stringreturn.replace("\n", " ");
                                vecs = vecs.replace("\t", " ");
                                let mut endvec: Vec<&str> = Vec::new();
                                endvec.push("-I");
                                endvec.push("rc");
                                for q in vecs.split(' ') {
                                    endvec.push(q);
                                }
                                Command::new("/Applications/VLC.app/Contents/MacOS/VLC")
                                    .args(endvec)
                                    .output()
                                    .expect("failed to execute process");
                            }
                        });
                        threads.push(handle);
                    } else if contents[x] == "loop" {
                        readfrom = x + 1;
                        skiperwiper = true;
                        read = true;
                        let mut vec: Vec<String> = Vec::new();
                        let mut skip = false;
                        let number_of_times = functions::math(
                            x,
                            contents.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                        );
                        if number_of_times > 0 as f32 {
                            let mut n = 0;
                            let mut reached = false;
                            let mut loc1 = 0;
                            let mut loc2 = 0;
                            for y in x + 1..contents.len() {
                                if !skip {
                                    if contents[y] == "{" {
                                        n += 1;
                                        reached = true;
                                        loc1 = y;
                                    } else if contents[y] == "}" {
                                        n -= 1;
                                    }
                                    if n > 0 {
                                        vec.push((&contents[y]).parse().unwrap());
                                    } else if reached {
                                        skip = true;
                                        loc2 = y;
                                    }
                                }
                            }
                            vec.remove(0);
                            let mut new_vec = Vec::new();
                            for t in 0..contents.clone().len() {
                                if t == loc2 {
                                    for _q in 1..number_of_times.round() as i32 {
                                        for y in vec.clone() {
                                            new_vec.push(y);
                                        }
                                    }
                                } else {
                                    new_vec.push(contents[t].clone());
                                }
                            }
                            new_vec.remove(loc1);
                            if dev {
                                println!("new_vec: {:?}", new_vec);
                            }
                            contents = new_vec;
                        }
                    } else if contents[x] == "while" {
                        readfrom = x;
                        skiperwiper = true;
                        read = true;
                        let mut vec: Vec<String> = Vec::new();
                        let mut skip = false;
                        let mut n = 0;
                        let mut reached = false;
                        let mut loc2 = 0;
                        for y in x + 1..contents.len() {
                            if !skip {
                                if contents[y] == "{" {
                                    n += 1;
                                    reached = true;
                                } else if contents[y] == "}" {
                                    n -= 1;
                                }
                                if n > 0 {
                                    vec.push((&contents[y]).parse().unwrap());
                                } else if reached {
                                    skip = true;
                                    loc2 = y;
                                }
                            }
                        }
                        let mut new_vec = Vec::new();
                        for t in 0..contents.clone().len() {
                            if t == x {
                                new_vec.push("if".to_string())
                            } else if t == loc2 {
                                new_vec.push(contents[loc2].clone());
                                for q in x..loc2 + 1 {
                                    new_vec.push(contents[q].clone());
                                }
                            } else {
                                new_vec.push(contents[t].clone());
                            }
                        }
                        if dev {
                            println!("new_vec: {:?}", new_vec);
                        }
                        contents = new_vec;
                    } else if contents[x] == "sleep" {
                        let number_of_times = functions::math(
                            x,
                            contents.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                        );
                        thread::sleep(time::Duration::from_millis(number_of_times as u64));
                    } else if contents[x] == "exec" {
                        functions::exec(
                            x,
                            contents.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                        );
                    } else if contents[x] == "setcont" {
                        let r = functions::set_contents(
                            x,
                            contents.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                        );
                        if r.is_err() {
                            panic!("Could not set file contents.");
                        }
                    } else if contents[x] == "POST" {
                        functions::post_request(
                            x,
                            contents.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                        );
                    } else if contents[x] == "func" {
                        let mut skip = false;
                        let mut n = 1;
                        let mut reached = false;
                        let mut name: String = "".parse().unwrap();
                        for y in x + 2..contents.len() {
                            if !skip {
                                if contents[y] == "(" {
                                    n -= 1;
                                    reached = true;
                                } else if contents[y] == ")" {
                                    n -= 1;
                                }
                                if n > 0 {
                                    name.push_str(&contents[y]);
                                } else if reached {
                                    skip = true;
                                }
                            }
                        }
                        let mut code = Vec::new();
                        skip = false;
                        n = 0;
                        reached = false;
                        for y in x + 1..contents.len() {
                            if !skip {
                                if contents[y] == "}" {
                                    n -= 1;
                                }
                                if n > 0 {
                                    code.push(&contents[y]);
                                } else if reached {
                                    skip = true;
                                }
                                if contents[y] == "{" {
                                    n += 1;
                                    reached = true;
                                }
                            }
                        }
                        let mut par = functions::getstring(
                            x + 2,
                            contents.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            3,
                        )
                        .join("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v");
                        if dev {
                            println!("par: {}", par);
                            println!("code: {:?}", code);
                            println!("name: {}", name);
                        }
                        let mut strinogeuroheu = "".to_string();
                        for x in 0..code.len() {
                            strinogeuroheu.push_str(code[x]);
                            if x != code.len() {
                                strinogeuroheu.push_str("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v");
                            }
                        }
                        func_par.push(par);
                        func_code.push(strinogeuroheu);
                        func_names.push(name);
                        if dev {
                            println!("func_par: {:?}", func_par);
                            println!("func_code: {:?}", func_code);
                            println!("func_names: {:?}", func_names);
                        }
                    } else if contents[x] == "eval" {
                        let imp = functions::eval(
                            x,
                            contents.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                        );
                        readfrom = x;
                        skiperwiper = true;
                        read = true;
                        let mut delete = Vec::new();
                        let mut deleted = 0;
                        let mut pass = false;
                        let mut n3 = 0;
                        delete.push(x);
                        for y1 in x + 1..contents.len() {
                            if !pass {
                                if contents[y1] == "(" {
                                    n3 += 1;
                                }
                                if n3 == 0 {
                                    pass = true;
                                }
                                if contents[y1] == ")" {
                                    n3 -= 1;
                                }
                                delete.push(y1);
                            }
                        }
                        for item in delete {
                            contents.remove(item - deleted);
                            deleted += 1;
                        }
                        let mut new_vec = Vec::new();
                        for itom in 0..contents.len() {
                            if itom == x {
                                for item in imp.clone() {
                                    new_vec.push(item);
                                }
                            }
                            new_vec.push(contents[itom].clone());
                        }
                        contents = new_vec;
                    } else if contents[x] == "imp" {
                        let imp = functions::imp(
                            x,
                            contents.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            "".to_string(),
                        );
                        readfrom = x;
                        skiperwiper = true;
                        read = true;
                        let mut delete = Vec::new();
                        let mut deleted = 0;
                        let mut pass = false;
                        let mut n3 = 0;
                        delete.push(x);
                        for y1 in x + 1..contents.len() {
                            if !pass {
                                if contents[y1] == "(" {
                                    n3 += 1;
                                }
                                if n3 == 0 {
                                    pass = true;
                                }
                                if contents[y1] == ")" {
                                    n3 -= 1;
                                }
                                delete.push(y1);
                            }
                        }
                        for item in delete {
                            contents.remove(item - deleted);
                            deleted += 1;
                        }
                        let mut new_vec = Vec::new();
                        for itom in 0..contents.len() {
                            if itom == x {
                                for item in imp.clone() {
                                    new_vec.push(item);
                                }
                            }
                            new_vec.push(contents[itom].clone());
                        }
                        contents = new_vec;
                    } else if contents[x] == "dec" {
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
                                value_array = functions::array_fn(
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
                                value_group = functions::group_fn(
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
                                        println!("contents[x+move_up+move_up+move_up_up+move_final]: {:?}", contents[position]);
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
                                                functions::math(
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
                                                functions::round(
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
                                                functions::replace(
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
                                            value.push_str(functions::input().to_string().as_str());
                                            n = 1;
                                        } else if contents[position] == "exec" {
                                            value.push_str(
                                                functions::exec(
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
                                                functions::trim(
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
                                            value.push_str(
                                                functions::time_readable().to_string().as_str(),
                                            );
                                            n = 1;
                                        } else if contents[position] == "time" {
                                            value.push_str(functions::time().to_string().as_str());
                                            n = 1;
                                        } else if contents[position] == "getcont" {
                                            value.push_str(
                                                functions::get_contents(
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
                                                    if !skip
                                                        && memory_names_save[pos]
                                                            == contents[position]
                                                    {
                                                        positions = pos;
                                                        skip = true;
                                                    }
                                                }
                                                if positions != memory_names_save.len()
                                                    && (contents[x + 1].trim() == ":"
                                                        || contents[x + 1].trim() == "=")
                                                {
                                                    value.push_str(
                                                        memory_values_save[positions]
                                                            .to_string()
                                                            .as_str(),
                                                    );
                                                } else {
                                                    let mut postion = func_names.len();
                                                    let mut skip = false;
                                                    for pos in 0..func_names.len() {
                                                        if !skip
                                                            && func_names[pos] == contents[position]
                                                        {
                                                            postion = pos;
                                                            skip = true;
                                                        }
                                                    }
                                                    if postion != func_names.len() {
                                                        let mut contetntstr: Vec<String> =
                                                            Vec::new();
                                                        for t in func_code[postion].split(
                                                            "zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v",
                                                        ) {
                                                            contetntstr.push(t.to_string());
                                                        }
                                                        let mut contetntstr1: Vec<String> =
                                                            Vec::new();
                                                        for t in func_par[postion].split(
                                                            "zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v",
                                                        ) {
                                                            contetntstr1.push(t.to_string());
                                                        }
                                                        let mut contetntstr2: Vec<String> =
                                                            functions::getstring(
                                                                x,
                                                                contents.clone(),
                                                                memory_names_save.clone(),
                                                                memory_values_save.clone(),
                                                                memory_types.clone(),
                                                                func_names.clone(),
                                                                func_par.clone(),
                                                                func_code.clone(),
                                                                dev,
                                                                0,
                                                            );
                                                        for t in func_par[postion].split(
                                                            "zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v",
                                                        ) {
                                                            contetntstr1.push(t.to_string());
                                                        }
                                                        value.push_str(
                                                            run(
                                                                contetntstr,
                                                                dev,
                                                                contetntstr1.clone(),
                                                                contetntstr2.clone(),
                                                                memory_types.clone(),
                                                                func_names.clone(),
                                                                func_par.clone(),
                                                                func_code.clone(),
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
                        } else if memory_types[memory_types.len() - 1] == "int" {
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
                    } else if contents[x] == "group" {
                        let build_name = contents[x + 1].clone();
                        let mut objects: Vec<String> = Vec::new();
                        for j in x + 2..contents.len() {
                            if contents[j] == "}" {
                                break;
                            }
                            objects.push(contents[j].clone())
                        }
                        let mut objects_object: Vec<String> = Vec::new();
                        for y in 0..objects.len() {
                            if objects[y] != ","
                                && objects[y] != " "
                                && objects[y] != "\r"
                                && objects[y] != "\n"
                                && objects[y] != "\""
                                && objects[y] != "{"
                                && objects[y] != "}"
                            {
                                objects_object.push(objects[y].clone().to_string())
                            }
                        }
                        String::new();
                        group_memory.push(build_name.clone());
                        group_memory.push(objects_object.len().to_string());
                        for d in 0..objects_object.len() {
                            group_memory.push(build_name.clone());
                            group_memory.push(objects_object[d].clone());
                        }
                    } else if contents[x] == "append" {
                        let mut params: Vec<String> = Vec::new();
                        for item in x..contents.len() {
                            if contents[item].is_empty()
                                || contents[item] == ","
                                || contents[item] == "("
                                || contents[item] == "append"
                                || contents[item] == "\""
                            {
                            } else if contents[item] == ")" {
                                break;
                            } else {
                                params.push(contents[item].clone());
                            }
                        }
                        for object in 0..memory_names.len() {
                            if memory_names[object] == params[0] {
                                // zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v
                                memory_values[object] = memory_values[object].clone()
                                    + "zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v"
                                    + params[1].as_str();
                                break;
                            }
                        }
                    } else if contents[x] == "cut" {
                        let mut parameters: Vec<String> = Vec::new();
                        for item in x..contents.len() {
                            if contents[item].is_empty()
                                || contents[item] == ","
                                || contents[item] == "("
                                || contents[item] == "cut"
                                || contents[item] == "\""
                            {
                            } else if contents[item] == ")" {
                                break;
                            } else {
                                parameters.push(contents[item].clone());
                            }
                        }
                        let mut change = String::new();
                        let mut count = 0;
                        for object in 0..memory_names.len() {
                            if memory_names[object] == parameters[0] {
                                //    identify.replace("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v", "");
                                let identify_split = memory_values[object]
                                    .split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v");
                                let id_vec: Vec<&str> = identify_split.collect();
                                let id_save = id_vec;
                                let mut id_save_string: Vec<String> = Vec::new();
                                for thing in 0..id_save.len() {
                                    id_save_string.push(id_save[thing].to_string());
                                }
                                id_save_string.remove(parameters[1].parse().unwrap());
                                let mut temp = String::new();
                                for elem in 0..id_save_string.len() {
                                    temp.push_str(id_save_string[elem].as_str());
                                    temp.push_str("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v");
                                }
                                change = temp;
                                count = object;
                            }
                        }
                        memory_values[count] = change;
                    } else if contents[x] == "if" {
                        let mut loc1 = 0;
                        let mut loc2 = 0;
                        let mut vec: Vec<String> = Vec::new();
                        let mut skip = false;
                        let mut n = 0;
                        for y in x + 1..contents.len() {
                            if !skip {
                                if contents[y] == "{" {
                                    if n == 0 {
                                        loc1 = y;
                                    }
                                    n += 1;
                                } else if contents[y] == "}" {
                                    n -= 1;
                                    if n == 0 {
                                        skip = true;
                                        loc2 = y;
                                    }
                                }
                                if n > 0 {
                                    vec.push((&contents[y]).parse().unwrap());
                                }
                            }
                        }
                        vec.remove(0);
                        let code = vec.clone();
                        let mut vec: Vec<String> = Vec::new();
                        let mut skip = false;
                        let mut n = 0;
                        for y in x + 1..contents.len() {
                            if !skip {
                                if contents[x + 1] != "(" {
                                    println!("You have to put a parentheses after a log");
                                    std::process::exit(1);
                                }
                                if contents[y] == "(" {
                                    n += 1;
                                } else if contents[y] == ")" {
                                    n -= 1;
                                }
                                if n == 0 {
                                    skip = true;
                                    for z in x + 1..y + 1 {
                                        vec.push((&contents[z]).parse().unwrap());
                                    }
                                }
                            }
                        }
                        if dev {
                            println!("vec: {:?}", vec);
                        }
                        let string: String = functions::getstring(
                            x,
                            contents.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            2,
                        )
                        .first()
                        .unwrap()
                        .to_string();
                        let mut result: Vec<String> = Vec::new();
                        let mut last = 0;
                        for (index, matched) in string.match_indices(|c: char| {
                            c == "=".chars().next().unwrap()
                                || c == "!".chars().next().unwrap()
                                || c == ">".chars().next().unwrap()
                                || c == "<".chars().next().unwrap()
                                || c == "|".chars().next().unwrap()
                                || c == "&".chars().next().unwrap()
                        }) {
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
                            if result[item] == "=" && 0 < item {
                                if result[item - 1] == "="
                                    || result[item - 1] == "!"
                                    || result[item - 1] == ">"
                                    || result[item - 1] == "<"
                                {
                                    output.push(result[item - 1].to_owned() + &*"=".to_string());
                                }
                            } else if result[item] == "|" && 0 < item {
                                if result[item + 1] == "|" {
                                    output.push("||".parse().unwrap());
                                }
                            } else if result[item] == "&" && 0 < item {
                                if result[item + 1] == "&" {
                                    output.push("&&".parse().unwrap());
                                }
                            } else if (result[item] == ">" || result[item] == "<") && 0 < item {
                                if result[item + 1] != "=" {
                                    output.push(result[item].to_owned());
                                }
                            } else if result[item] != "!"
                                && result[item] != "<"
                                && result[item] != ">"
                            {
                                output.push(result[item].parse().unwrap());
                            }
                        }
                        for item in 0..output.len() {
                            let if_number = output[item].chars();
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
                                    if !skip && memory_names[pos] == output[item] {
                                        postion1 = pos;
                                        skip = true;
                                    }
                                }
                                if postion1 != memory_names.len() {
                                    output[item] = memory_values[postion1].to_string();
                                }
                            }
                        }
                        for item in 0..output.len() {
                            if (output[item] == "==" && output[item - 1] == output[item + 1])
                                || (output[item] == "!=" && output[item - 1] != output[item + 1])
                                || (output[item] == ">="
                                    && output[item - 1].parse::<i32>().unwrap()
                                        >= output[item + 1].parse::<i32>().unwrap())
                                || (output[item] == "<="
                                    && output[item - 1].parse::<i32>().unwrap()
                                        <= output[item + 1].parse::<i32>().unwrap())
                                || (output[item] == "<"
                                    && output[item - 1].parse::<i32>().unwrap()
                                        < output[item + 1].parse::<i32>().unwrap())
                                || (output[item] == ">"
                                    && output[item - 1].parse::<i32>().unwrap()
                                        > output[item + 1].parse::<i32>().unwrap())
                            {
                                output[item] = "true".to_string();
                                output[item - 1] = "".to_string();
                                output[item + 1] = "".to_string();
                            } else if (output[item] == "=="
                                && !(output[item - 1] == output[item + 1]))
                                || (output[item] == "!=" && !(output[item - 1] != output[item + 1]))
                                || (output[item] == ">="
                                    && !(output[item - 1].parse::<i32>().unwrap()
                                        >= output[item + 1].parse::<i32>().unwrap()))
                                || (output[item] == "<="
                                    && !(output[item - 1].parse::<i32>().unwrap()
                                        <= output[item + 1].parse::<i32>().unwrap()))
                                || (output[item] == "<"
                                    && !(output[item - 1].parse::<i32>().unwrap()
                                        < output[item + 1].parse::<i32>().unwrap()))
                                || (output[item] == ">"
                                    && !(output[item - 1].parse::<i32>().unwrap()
                                        > output[item + 1].parse::<i32>().unwrap()))
                            {
                                output[item] = "false".to_string();
                                output[item - 1] = "".to_string();
                                output[item + 1] = "".to_string();
                            }
                        }
                        output = lexer::no_extra_whitespace(output, dev);
                        let mut new_out = Vec::new();
                        for item in 0..output.len() {
                            if !output[item].is_empty() {
                                new_out.push(output[item].clone());
                            }
                        }
                        output = new_out;
                        while output.len() > 1 {
                            for item in 0..output.len() {
                                if item > 0 && item < output.len() {
                                    if (output[item] == "&&"
                                        && output[item - 1] == "true"
                                        && output[item + 1] == "true")
                                        || (output[item] == "||"
                                            && (output[item - 1] == "true"
                                                || output[item + 1] == "true"))
                                    {
                                        output[item] = "true".to_string();
                                        output[item - 1] = "".to_string();
                                        output[item + 1] = "".to_string();
                                    } else if output[item] == "&&" || output[item] == "||" {
                                        for _i in 0..output.len() {
                                            output.pop();
                                        }
                                        output.push("false".to_string());
                                    }
                                }
                                output = lexer::no_extra_whitespace(output, dev);
                                let mut new_out = Vec::new();
                                for item in 0..output.len() {
                                    if !output[item].is_empty() {
                                        new_out.push(output[item].clone());
                                    }
                                }
                                output = new_out;
                            }
                        }
                        if output[0] == "true" {
                            contents[loc1] = " ".parse().unwrap();
                            contents[loc2] = " ".parse().unwrap();
                            readfrom = loc1;
                            skiperwiper = true;
                            read = true;
                        } else if loc2 + 2 < contents.len() {
                            if contents[loc2 + 1] == "while" {
                                contents[loc2 + 1] = " ".parse().unwrap();
                            } else if contents[loc2 + 2] == "while" {
                                contents[loc2 + 2] = " ".parse().unwrap();
                            } else if contents[loc2 + 1] == "else" || contents[loc2 + 2] == "else" {
                                let mut skip = false;
                                let mut n = 0;
                                for y in loc2 + 1..contents.len() {
                                    if !skip {
                                        if contents[y] == "{" {
                                            if n == 0 {
                                                contents[y] = "".to_string();
                                            }
                                            n += 1;
                                        } else if contents[y] == "}" {
                                            n -= 1;
                                            if n == 0 {
                                                skip = true;
                                                contents[y] = "".to_string();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if dev {
                            println!("output: {:?}", output);
                            println!("code: {:?}", code);
                            println!("contents[loc1]: {:?}", contents[loc1]);
                            println!("contents[loc2]: {:?}", contents[loc2]);
                            println!("contents: {:?}", contents);
                        }
                    } else if x > 2 && contents[x - 2] != "func" {
                        let mut postion = func_names.len();
                        let mut skip = false;
                        for pos in 0..func_names.len() {
                            if !skip && func_names[pos] == contents[x] {
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
                            let _outputs = run(
                                contetntstr,
                                dev,
                                memory_names.clone(),
                                memory_values.clone(),
                                memory_types.clone(),
                                func_names.clone(),
                                func_par.clone(),
                                func_code.clone(),
                            );
                        } else {
                            let mut postion = memory_names.len();
                            let mut skip = false;
                            for pos in 0..memory_names.len() {
                                if !skip && memory_names[pos] == contents[x] {
                                    postion = pos;
                                    skip = true;
                                }
                            }
                            if postion != memory_names.len()
                                && (contents[x + 1].trim() == ":" || contents[x + 1].trim() == "=")
                                && contents[x - 2].trim() != "dec"
                            {
                                let mut position = x + 2;
                                let mut value = String::new();
                                let mut n = 0;
                                let mut quote = 0;
                                let memory_names_save = memory_names.clone();
                                let memory_values_save = memory_values.clone();
                                let memmory_types_save = memory_types.clone();
                                let func_names_save = func_names.clone();
                                let func_code_save = func_code.clone();
                                let func_par_save = func_par.clone();
                                loop {
                                    if dev {
                                        println!("contents[x+move_up+move_up+move_up_up+move_final]: {:?}", contents[position]);
                                    }
                                    if contents[position] == ";" {
                                        break;
                                    } else if (contents[position] == "\""
                                        || contents[position] == "\'"
                                        || contents[position] == r"\`")
                                        && contents[position - 1] != "\\"
                                    {
                                        quote += 1;
                                    } else {
                                        if contents[position] == "math" {
                                            value.push_str(
                                                functions::math(
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
                                                functions::round(
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
                                        } else if contents[position] == "GET" {
                                            value.push_str(
                                                functions::get_request(
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
                                                functions::replace(
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
                                            value.push_str(functions::input().to_string().as_str());
                                            n = 1;
                                        } else if contents[position] == "exec" {
                                            value.push_str(
                                                functions::exec(
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
                                                functions::trim(
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
                                            value.push_str(
                                                functions::time_readable().to_string().as_str(),
                                            );
                                            n = 1;
                                        } else if contents[position] == "time" {
                                            value.push_str(functions::time().to_string().as_str());
                                            n = 1;
                                        } else if contents[position] == "getcont" {
                                            value.push_str(
                                                functions::get_contents(
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
                                                    if !skip
                                                        && memory_names_save[pos]
                                                            == contents[position]
                                                    {
                                                        positions = pos;
                                                        skip = true;
                                                    }
                                                }
                                                if positions != memory_names_save.len() {
                                                    value.push_str(
                                                        memory_values_save[positions]
                                                            .to_string()
                                                            .as_str(),
                                                    );
                                                } else {
                                                    let mut postion = func_names_save.len();
                                                    let mut skip = false;
                                                    for pos in 0..func_names_save.len() {
                                                        if !skip
                                                            && func_names_save[pos]
                                                                == contents[position]
                                                        {
                                                            postion = pos;
                                                            skip = true;
                                                        }
                                                    }
                                                    if postion != func_names.len() {
                                                        let mut contetntstr: Vec<String> =
                                                            Vec::new();
                                                        for t in func_code_save[postion].split(
                                                            "zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v",
                                                        ) {
                                                            contetntstr.push(t.to_string());
                                                        }
                                                        let mut contetntstr1: Vec<String> =
                                                            Vec::new();
                                                        for t in func_par_save[postion].split(
                                                            "zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v",
                                                        ) {
                                                            contetntstr1.push(t.to_string());
                                                        }
                                                        let mut contetntstr2: Vec<String> =
                                                            functions::getstring(
                                                                x,
                                                                contents.clone(),
                                                                memory_names_save.clone(),
                                                                memory_values_save.clone(),
                                                                memory_types.clone(),
                                                                func_names_save.clone(),
                                                                func_par_save.clone(),
                                                                func_code_save.clone(),
                                                                dev,
                                                                0,
                                                            );
                                                        for t in func_par_save[postion].split(
                                                            "zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v",
                                                        ) {
                                                            contetntstr1.push(t.to_string());
                                                        }
                                                        value.push_str(
                                                            run(
                                                                contetntstr,
                                                                dev,
                                                                contetntstr1.clone(),
                                                                contetntstr2.clone(),
                                                                memory_types.clone(),
                                                                func_names_save.clone(),
                                                                func_par_save.clone(),
                                                                func_code_save.clone(),
                                                            )
                                                            .as_str(),
                                                        );
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
                                    position += 1;
                                    if dev {
                                        println!("position: {:?}", position);
                                    }
                                }
                                memory_values[postion] = value;
                            } else if postion != memory_names.len()
                                && contents[x + 1].trim() == "("
                                && contents[x - 2].trim() != "dec"
                            {
                                let id = functions::math(
                                    x,
                                    contents.clone(),
                                    memory_names.clone(),
                                    memory_values.clone(),
                                    func_names.clone(),
                                    func_par.clone(),
                                    func_code.clone(),
                                );
                                let mut skipz = false;
                                let mut nigro = 0;
                                let mut pos = x;
                                for nx in x + 1..contents.len() {
                                    if !skipz {
                                        if contents[nx] == "(" {
                                            nigro += 1;
                                        } else if contents[nx] == ")" {
                                            nigro -= 1;
                                        }
                                        if nigro == 0 {
                                            pos = nx;
                                            skipz = true;
                                        }
                                    }
                                }
                                let mut position = pos + 2;
                                let mut value = String::new();
                                let mut n = 0;
                                let mut quote = 0;
                                let memory_names_save = memory_names.clone();
                                let memory_values_save = memory_values.clone();
                                let memmory_types_save = memory_types.clone();
                                let func_names_save = func_names.clone();
                                let func_code_save = func_code.clone();
                                let func_par_save = func_par.clone();
                                loop {
                                    if contents[position] == ";" {
                                        if dev {
                                            println!("contents[x+move_up+move_up+move_up_up+move_final]: {:?}", contents[position]);
                                        }
                                        break;
                                    } else if (contents[position] == "\""
                                        || contents[position] == "\'"
                                        || contents[position] == r"\`")
                                        && contents[position - 1] != "\\"
                                    {
                                        quote += 1;
                                    } else {
                                        if contents[position] == "math" {
                                            value.push_str(
                                                functions::math(
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
                                                functions::round(
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
                                        } else if contents[position] == "GET" {
                                            value.push_str(
                                                functions::get_request(
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
                                                functions::replace(
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
                                            value.push_str(functions::input().to_string().as_str());
                                            n = 1;
                                        } else if contents[position] == "exec" {
                                            value.push_str(
                                                functions::exec(
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
                                                functions::trim(
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
                                            value.push_str(
                                                functions::time_readable().to_string().as_str(),
                                            );
                                            n = 1;
                                        } else if contents[position] == "time" {
                                            value.push_str(functions::time().to_string().as_str());
                                            n = 1;
                                        } else if contents[position] == "getcont" {
                                            value.push_str(
                                                functions::get_contents(
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
                                                    if !skip
                                                        && memory_names_save[pos]
                                                            == contents[position]
                                                    {
                                                        positions = pos;
                                                        skip = true;
                                                    }
                                                }
                                                if positions != memory_names_save.len() {
                                                    value.push_str(
                                                        memory_values_save[positions]
                                                            .to_string()
                                                            .as_str(),
                                                    );
                                                } else {
                                                    let mut postion = func_names.len();
                                                    let mut skip = false;
                                                    for pos in 0..func_names.len() {
                                                        if !skip
                                                            && func_names[pos] == contents[position]
                                                        {
                                                            postion = pos;
                                                            skip = true;
                                                        }
                                                    }
                                                    if postion != func_names.len() {
                                                        let mut contetntstr: Vec<String> =
                                                            Vec::new();
                                                        for t in func_code_save[postion].split(
                                                            "zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v",
                                                        ) {
                                                            contetntstr.push(t.to_string());
                                                        }
                                                        let mut contetntstr1: Vec<String> =
                                                            Vec::new();
                                                        for t in func_par_save[postion].split(
                                                            "zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v",
                                                        ) {
                                                            contetntstr1.push(t.to_string());
                                                        }
                                                        let mut contetntstr2: Vec<String> =
                                                            functions::getstring(
                                                                x,
                                                                contents.clone(),
                                                                memory_names_save.clone(),
                                                                memory_values_save.clone(),
                                                                memory_types.clone(),
                                                                func_names_save.clone(),
                                                                func_par_save.clone(),
                                                                func_code_save.clone(),
                                                                dev,
                                                                0,
                                                            );
                                                        for t in func_par_save[postion].split(
                                                            "zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v",
                                                        ) {
                                                            contetntstr1.push(t.to_string());
                                                        }
                                                        value.push_str(
                                                            run(
                                                                contetntstr,
                                                                dev,
                                                                contetntstr1.clone(),
                                                                contetntstr2.clone(),
                                                                memory_types.clone(),
                                                                func_names_save.clone(),
                                                                func_par_save.clone(),
                                                                func_code_save.clone(),
                                                            )
                                                            .as_str(),
                                                        );
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
                                    position += 1;
                                    if dev {
                                        println!("position: {:?}", position);
                                    }
                                }
                                let mut new_value = memory_values[postion]
                                    .split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                                    .collect::<Vec<&str>>();
                                new_value[id as usize] = &value;
                                memory_values[postion] =
                                    new_value.join("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v");
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
    "".to_string()
}

pub(crate) fn hard(
    mut contents: Vec<String>,
    dev: bool,
    memory_names: Vec<String>,
    memory_values: Vec<String>,
    memory_types: Vec<String>,
    func_names: Vec<String>,
    func_par: Vec<String>,
    func_code: Vec<String>,
) -> Vec<String> {
    if dev {
        println!("contents: {:?}", contents);
    }
    let mut quotes = 0;
    let mut squigle = 0;
    let mut readfrom = 0;
    let mut read = true;
    while read {
        read = false;
        let mut skiperwiper = false;
        for x in readfrom..contents.len() {
            if !skiperwiper {
                if dev {
                    println!("contents[x]: {}", contents[x]);
                    println!("x: {}", x);
                    println!("quotes: {}", quotes);
                    println!("squigle: {}", squigle);
                }
                if (contents[x] == "\"" || contents[x] == "\'" || contents[x] == r"\`")
                    && contents[x - 1] != "\\"
                {
                    quotes += 1;
                }
                if (contents[x] == "{" || contents[x] == "[") && quotes % 2 == 0 {
                    squigle += 1;
                }
                if (contents[x] == "}" || contents[x] == "]") && quotes % 2 == 0 {
                    squigle -= 1;
                }
                if quotes % 2 == 0 && squigle == 0 && contents[x] == "imp" {
                    let imp = functions::imp(
                        x,
                        contents.clone(),
                        memory_names.clone(),
                        memory_values.clone(),
                        memory_types.clone(),
                        func_names.clone(),
                        func_par.clone(),
                        func_code.clone(),
                        dev,
                        "".to_string(),
                    );
                    readfrom = x;
                    skiperwiper = true;
                    read = true;
                    let mut delete = Vec::new();
                    let mut deleted = 0;
                    let mut skirt = false;
                    let mut n3 = 0;
                    delete.push(x);
                    for y1 in x + 1..contents.len() {
                        if !skirt {
                            if contents[y1] == "(" {
                                n3 += 1;
                            }
                            if n3 == 0 {
                                skirt = true;
                            }
                            if contents[y1] == ")" {
                                n3 -= 1;
                            }
                            delete.push(y1);
                        }
                    }
                    for item in delete {
                        contents.remove(item - deleted);
                        deleted += 1;
                    }
                    let mut new_vec = Vec::new();
                    for itom in 0..contents.len() {
                        if itom == x {
                            for item in imp.clone() {
                                new_vec.push(item);
                            }
                        }
                        new_vec.push(contents[itom].clone());
                    }
                    contents = new_vec;
                }
            }
        }
    }
    contents
}

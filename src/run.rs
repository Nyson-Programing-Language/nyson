mod functions;
use crate::lexer;
use std::env;
use std::process::Command;
use std::thread;

extern crate chrono;
extern crate meval;

pub fn run(
    mut contents: Vec<String>,
    dev: bool,
    mut uses: Vec<String>,
    mut memory_names: Vec<String>,
    mut memory_values: Vec<String>,
    mut memory_types: Vec<String>,
    mut func_names: Vec<String>,
    mut func_par: Vec<String>,
    mut func_code: Vec<String>,
) -> String {
    let mut returns = "
    use std::io::Read;
    use std::net::TcpStream;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::fs;
    use std::io::Write;
    fn internet_time() -> f64 {
        let mut stream = TcpStream::connect(\"time.nist.gov:13\").unwrap();
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();
        buffer = buffer.trim().to_string();
        let days = buffer.split(\" \").nth(0).unwrap().parse::<usize>().unwrap() - 40587;
        let hours = buffer
            .split(\" \")
            .nth(2)
            .unwrap()
            .split(\":\")
            .nth(0)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mins = buffer
            .split(\" \")
            .nth(2)
            .unwrap()
            .split(\":\")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let seconds = buffer
            .split(\" \")
            .nth(2)
            .unwrap()
            .split(\":\")
            .nth(2)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let milliseconds = buffer
            .replace(\"  \", \" \")
            .replace(\"  \", \" \") // for if there is a single digit millisecond
            .split(\" \")
            .nth(6)
            .unwrap()
            .parse::<f32>()
            .unwrap()
            .floor() as usize;
        let unix_time_stamp = days * 24 * 60 * 60 * 1000
            + hours * 60 * 60 * 1000
            + mins * 60 * 1000
            + seconds * 1000
            + milliseconds;
        return unix_time_stamp as f64;
    }
    fn time() -> f64 {
        let start = SystemTime::now();
        start
            .duration_since(UNIX_EPOCH)
            .expect(\"Time went backwards\")
            .as_millis() as f64
    }
    fn input() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        return line.trim().to_string();
    }
    fn request(input:Vec<String>) -> String {
        let url = input.get(1).unwrap().trim().to_string();
        let mut tcpstream_url = \"\";
        let mut tcpstream_url_path = \"\";
        let mut split = url.split('/').collect::<Vec<&str>>();
        let mut right_part;
        let mut right_part1;
        if url.starts_with(\"http\") {
            tcpstream_url = url.split('/').nth(2).unwrap();
            if url.split('/').count() > 3 {
                right_part = &split[3..];
                right_part1 = right_part.join(\"/\");
                tcpstream_url_path = &right_part1;
            }
        }
        else {
            tcpstream_url = url.split('/').nth(0).unwrap();
            if url.split('/').count() > 1 {
                right_part = &split[1..];
                right_part1 = right_part.join(\"/\");
                tcpstream_url_path = &right_part1;
            }
        }
        let mut stream = TcpStream::connect(format!(\"{}:80\", tcpstream_url.trim())).unwrap();
        let mut headers = \"\".to_string();
        if input.len() > 3 {
            headers = format!(\"\\r\\nHost: {}\\r\\nAccept: */*\\r\\n{}\", tcpstream_url.trim(), input[3..].join(\"\\r\\n\"));
        }
        else {
            headers = format!(\"\\r\\nHost: {}\\r\\nAccept: */*\\r\\n\", tcpstream_url.trim());
        }
        let mut cont = \"\";
        if input.len() > 2 {
            cont = input.get(2).unwrap();
        }
        stream.write(format!(\"{} /{} HTTP/1.1{}\\r\\n\\r\\n{}\", input.get(0).unwrap().trim().to_string(), tcpstream_url_path, headers, cont).as_bytes());
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();
        buffer.trim().to_string().split(\"\\r\\n\").nth(1).unwrap().to_string()
    }
    fn main() {"
        .to_string();
    let mut newcont: Vec<String> = vec![" ".to_string()];
    for i in lexer::lexer(code_to_add(), dev) {
        newcont.push(i);
    }
    for i in contents {
        newcont.push(i);
    }
    contents = newcont.clone();
    if dev {
        println!("contents: {:?}", contents);
    }
    let mut quotes = 0;
    let mut squigle = 0;
    let mut readfrom = 0;
    let mut read = true;
    let mut group_memory: Vec<String> = Vec::new();
    while read {
        read = false;
        let mut skiperwiper = false;
        for x in readfrom..contents.len() {
            if !skiperwiper {
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

                if dev {
                    println!("contents[x]: {}", contents[x]);
                    println!("x: {}", x);
                    println!("quotes: {}", quotes);
                    println!("squigle: {}", squigle);
                }

                if quotes % 2 == 0 && squigle == 0 {
                    if "log" == contents[x].as_str() {
                        returns = format!(
                            "{}{}",
                            returns,
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
                                uses.clone(),
                            )
                        );
                    } else if "ret" == contents[x].as_str() {
                        returns = format!(
                            "{}return {};",
                            returns,
                            functions::getstring(
                                x,
                                contents.clone(),
                                memory_names.clone(),
                                memory_values.clone(),
                                memory_types.clone(),
                                func_names.clone(),
                                func_par.clone(),
                                func_code.clone(),
                                dev,
                                uses.clone(),
                                0
                            )
                            .first()
                            .unwrap()
                            .to_string()
                        );
                    } else if "use" == contents[x].as_str() {
                        if contents[x + 1].as_str() == "os" {
                            uses[0] = "true".to_string();
                        } else if contents[x + 1].as_str() == "audio" {
                            uses[1] = "true".to_string();
                        }
                    } else if "request" == contents[x].as_str() {
                        returns = format!(
                            "{}{};",
                            returns,
                            functions::request(
                                x,
                                contents.clone(),
                                memory_names.clone(),
                                memory_values.clone(),
                                memory_types.clone(),
                                func_names.clone(),
                                func_par.clone(),
                                func_code.clone(),
                                dev,
                                uses.clone(),
                            )
                        );
                    } else if "exit" == contents[x].as_str() {
                        returns = format!("{}std::process::exit(1);", returns);
                    } else if "audio" == contents[x].as_str()
                        && uses[1] == *"true"
                        && "use" != contents[x - 1].as_str()
                    {
                        let contents_save = contents.clone();
                        let memory_types_save = memory_types.clone();
                        let memory_values_save = memory_values.clone();
                        let memory_names_save = memory_names.clone();
                        let func_names_save = func_names.clone();
                        let func_par_save = func_par.clone();
                        let func_code_save = func_code.clone();
                        let uses_save = uses.clone();
                        let _handle = thread::spawn(move || {
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
                                uses_save,
                                0,
                            )
                            .first()
                            .unwrap()
                            .to_string();
                            if env::consts::OS == "linux" {
                                let mut vecs = stringreturn.replace('\n', " ");
                                vecs = vecs.replace('\t', " ");
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
                                let mut vecs = stringreturn.replace('\n', " ");
                                vecs = vecs.replace('\t', " ");
                                let mut endvec: Vec<&str> = vec!["-I", "rc"];
                                for q in vecs.split(' ') {
                                    endvec.push(q);
                                }
                                Command::new("/Applications/VLC.app/Contents/MacOS/VLC")
                                    .args(endvec)
                                    .output()
                                    .expect("failed to execute process");
                            }
                        });
                        //threads.push(handle);
                    } else if "loop" == contents[x].as_str() {
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
                            uses.clone(),
                        );
                        if number_of_times > 0 as f64 {
                            let mut n = 0;
                            let mut reached = false;
                            let mut loc1 = 0;
                            let mut loc2 = 0;
                            for y in x + 1..contents.len() {
                                if !skip {
                                    if contents[y] == "{" {
                                        n += 1;
                                        reached = true;
                                        if n == 1 {
                                            loc1 = y;
                                        }
                                    } else if contents[y] == "}" {
                                        n -= 1;
                                    }
                                    if n > 0 {
                                        vec.push(contents[y].parse().unwrap());
                                    } else if reached {
                                        skip = true;
                                        loc2 = y;
                                    }
                                }
                            }
                            vec.remove(0);
                            returns = format!(
                                "{}for _ in 0..{}{{{}}};",
                                number_of_times,
                                returns,
                                run(
                                    vec,
                                    dev,
                                    uses.clone(),
                                    Vec::new(),
                                    Vec::new(),
                                    Vec::new(),
                                    Vec::new(),
                                    Vec::new(),
                                    Vec::new(),
                                )
                            );
                        }
                    } else if "while" == contents[x].as_str() {
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
                                    vec.push(contents[y].parse().unwrap());
                                } else if reached {
                                    skip = true;
                                    loc2 = y;
                                }
                            }
                        }
                        returns = format!(
                            "{}while {{{}}}",
                            returns,
                            run(
                                vec,
                                dev,
                                uses.clone(),
                                Vec::new(),
                                Vec::new(),
                                Vec::new(),
                                Vec::new(),
                                Vec::new(),
                                Vec::new(),
                            )
                        );
                    } else if "sleep" == contents[x].as_str() {
                        let number_of_times = functions::math(
                            x,
                            contents.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            uses.clone(),
                        );
                        returns = format!(
                            "{}thread::sleep(time::Duration::from_millis({}));",
                            returns, number_of_times as u64
                        );
                    } else if "exec" == contents[x].as_str() {
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
                            uses.clone(),
                        );
                    } else if "setcont" == contents[x].as_str() {
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
                            uses.clone(),
                        );
                        if r.is_err() {
                            error("Could not set file contents.".to_string());
                        }
                    } else if "eval" == contents[x].as_str() {
                        let imp = functions::eval_eval(
                            x,
                            contents.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            uses.clone(),
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
                            if itom == x - 1 {
                                new_vec.push(contents[itom].clone());
                                for item in imp.clone() {
                                    new_vec.push(item);
                                }
                            } else {
                                new_vec.push(contents[itom].clone());
                            }
                        }
                        contents = new_vec;
                    } else if "imp" == contents[x].as_str() {
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
                            uses.clone(),
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
                            if itom == x - 1 {
                                new_vec.push(contents[itom].clone());
                                for item in imp.clone() {
                                    new_vec.push(item);
                                }
                            } else {
                                new_vec.push(contents[itom].clone());
                            }
                        }
                        contents = new_vec;
                    } else if "dec" == contents[x].as_str() {
                        let _memory_names1 = memory_names.clone();
                        let _memory_values1 = memory_values.clone();
                        let _memory_types1 = memory_types.clone();
                        let _func_names1 = func_names.clone();
                        let _func_par1 = func_par.clone();
                        let _func_code1 = func_code.clone();
                        let _memory_names_save = memory_names.clone();
                        let _memory_values_save = memory_values.clone();
                        let mut types = false;
                        let mut position = x + 1;
                        let _square_brackets = 0;
                        let mut exists = memory_names.len();

                        if memory_names.contains(&contents[position + 1]) {
                            for item in 0..memory_names.len() {
                                if memory_names[item] == contents[position + 1] {
                                    exists = item;
                                }
                            }
                        }

                        // get type
                        if contents[position] == "int"
                            || contents[position] == "str"
                            || contents[position] == "arr"
                            || contents[position] == "grp"
                            || contents[position] == "inf"
                        {
                            if exists != memory_names.len() {
                                memory_types[exists] = contents[position].to_string();
                                memory_names[exists] = contents[position + 1].clone();
                            } else {
                                memory_types.push(contents[position].to_string());
                                memory_names.push(contents[position + 1].clone());
                            }
                            position += 1;
                        } else if contents[position] == "anon" {
                            types = true;
                            if exists != memory_names.len() {
                                memory_types[exists] = "anon".to_string();
                            } else {
                                memory_types.push(String::from("anon"));
                            }
                        }

                        //more vars
                        let clone_class = String::from("");
                        let _value = String::new();
                        let value_array: Vec<String> = Vec::new();
                        let mut value_group = Vec::new();

                        //more vars
                        let _n = 0;
                        let _quote = 0;
                        let _squig = 0; // brace/squiggly bracket checker
                        let _brackets = 0;

                        //more vars
                        position += 2;
                        let _group = false;

                        let mut pass_vec: Vec<String> = Vec::new();
                        pass_vec.push("a".to_string());
                        pass_vec.push("(".to_string());
                        if contents[x + 1] == "int" {
                            pass_vec.push("math".to_string());
                            pass_vec.push("(".to_string());
                        }
                        loop {
                            if contents[position] == "\n" || contents[position] == ";" {
                                break;
                            }
                            pass_vec.push(contents[position].clone().to_string());
                            position += 1;
                        }
                        pass_vec.push(")".to_string());
                        if contents[x + 1] == "int" {
                            pass_vec.push(")".to_string());
                        }
                        let value = functions::getstring(
                            0,
                            pass_vec.clone(),
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
                        .join("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                        .to_string();
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
                                        && group_memory[items + 1].parse::<f64>().is_ok()
                                        && group_memory[items] == clone_class.clone()
                                    {
                                        location = items + (d * 2) + 3;
                                    }
                                }
                                name.push_str(&*group_memory[location]);
                                if exists != memory_names.len() {
                                    memory_names[exists] = name.clone();
                                    memory_values[exists] = value_group[d].clone();
                                    memory_types[exists] = "str".parse().unwrap();
                                } else {
                                    memory_names.push(name.clone());
                                    memory_values.push(value_group[d].clone());
                                    memory_types.push("str".parse().unwrap());
                                }
                            }
                        } else if memory_types[memory_types.len() - 1] == "int" {
                            let number = meval::eval_str(value.clone().as_str());
                            if number.is_ok() {
                                if exists != memory_values.len() {
                                    memory_values[exists] = number.unwrap().to_string();
                                } else {
                                    memory_values.push(number.unwrap().to_string());
                                }
                            } else if exists != memory_values.len() {
                                memory_values[exists] = value.clone();
                            } else {
                                memory_values.push(value.clone());
                            }
                        } else if exists != memory_values.len() {
                            memory_values[exists] = value.clone();
                        } else {
                            memory_values.push(value.clone());
                        }

                        if types {
                            if exists != memory_names.len() {
                                memory_names[exists] = value.clone();
                            } else {
                                memory_names.push(value.clone());
                            }
                        }
                        if dev {
                            println!("memory_names: {:?}", memory_names);
                            println!("memory_types: {:?}", memory_types);
                            println!("memory_values: {:?}", memory_values);
                        }
                    } else if "group" == contents[x].as_str() {
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
                    } else if "append" == contents[x].as_str() {
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
                    } else if "cut" == contents[x].as_str() {
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
                    } else if "func" == contents[x].as_str() {
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
                        let par = functions::getstring(
                            x + 2,
                            contents.clone(),
                            memory_names.clone(),
                            memory_values.clone(),
                            memory_types.clone(),
                            func_names.clone(),
                            func_par.clone(),
                            func_code.clone(),
                            dev,
                            uses.clone(),
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
                    } else if "if" == contents[x].as_str() {
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
                                    vec.push(contents[y].parse().unwrap());
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
                                        vec.push(contents[z].parse().unwrap());
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
                            uses.clone(),
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
                                result.push(string[last..index].parse().unwrap());
                            }
                            result.push(matched.parse().unwrap());
                            last = index + matched.len();
                        }
                        if last < string.len() {
                            result.push(string[last..].parse().unwrap());
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
                                    && output[item - 1].parse::<f64>().unwrap()
                                        >= output[item + 1].parse::<f64>().unwrap())
                                || (output[item] == "<="
                                    && output[item - 1].parse::<f64>().unwrap()
                                        <= output[item + 1].parse::<f64>().unwrap())
                                || (output[item] == "<"
                                    && output[item - 1].parse::<f64>().unwrap()
                                        < output[item + 1].parse::<f64>().unwrap())
                                || (output[item] == ">"
                                    && output[item - 1].parse::<f64>().unwrap()
                                        > output[item + 1].parse::<f64>().unwrap())
                            {
                                output[item] = "true".to_string();
                                output[item - 1] = "".to_string();
                                output[item + 1] = "".to_string();
                            } else if (output[item] == "==" && output[item - 1] != output[item + 1])
                                || (output[item] == "!=" && output[item - 1] == output[item + 1])
                                || (output[item] == ">="
                                    && !(output[item - 1].parse::<f64>().unwrap()
                                        >= output[item + 1].parse::<f64>().unwrap()))
                                || (output[item] == "<="
                                    && !(output[item - 1].parse::<f64>().unwrap()
                                        <= output[item + 1].parse::<f64>().unwrap()))
                                || (output[item] == "<"
                                    && !(output[item - 1].parse::<f64>().unwrap()
                                        < output[item + 1].parse::<f64>().unwrap()))
                                || (output[item] == ">"
                                    && !(output[item - 1].parse::<f64>().unwrap()
                                        > output[item + 1].parse::<f64>().unwrap()))
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
                    } else {
                        //function names
                        if x > 2 && contents[x - 2] != "func" {
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
                                for t in
                                    func_code[postion].split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                                {
                                    contetntstr.push(t.to_string());
                                }
                                let mut contetntstr1: Vec<String> = Vec::new();
                                let mut contetntstr3: Vec<String> = Vec::new();
                                let contetntstr2: Vec<String> = functions::getstring(
                                    x,
                                    contents.clone(),
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
                                for t in
                                    func_par[postion].split("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                                {
                                    contetntstr1.push(t.to_string());
                                    contetntstr3.push("str".to_string());
                                }
                                if dev {
                                    println!("contetntstr1: {:?}", contetntstr1);
                                    println!("contetntstr2: {:?}", contetntstr2);
                                    println!("contetntstr3: {:?}", contetntstr3);
                                }
                                let _output = run(
                                    contetntstr,
                                    dev,
                                    uses.clone(),
                                    contetntstr1.clone(),
                                    contetntstr2.clone(),
                                    contetntstr3.clone(),
                                    func_names.clone(),
                                    func_par.clone(),
                                    func_code.clone(),
                                )
                                .as_str();
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
                                    && (contents[x + 1].trim() == ":"
                                        || contents[x + 1].trim() == "=")
                                    && contents[x - 2].trim() != "dec"
                                {
                                    let mut position = x + 2;
                                    let _value = String::new();
                                    let _n = 0;
                                    let _quote = 0;
                                    let _memory_names_save = memory_names.clone();
                                    let _memory_values_save = memory_values.clone();
                                    let _memmory_types_save = memory_types.clone();
                                    let _func_names_save = func_names.clone();
                                    let _func_code_save = func_code.clone();
                                    let _func_par_save = func_par.clone();
                                    let mut pass_vec: Vec<String> =
                                        vec!["a".to_string(), "(".to_string()];
                                    loop {
                                        if contents[position] == "\n" || contents[position] == ";" {
                                            break;
                                        }
                                        pass_vec.push(contents[position].clone().to_string());
                                        position += 1;
                                    }
                                    pass_vec.push(")".to_string());
                                    let value = functions::getstring(
                                        0,
                                        pass_vec.clone(),
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
                                    .join("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                                    .to_string();
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
                                        uses.clone(),
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
                                    let _value = String::new();
                                    let _n = 0;
                                    let _quote = 0;
                                    let _memory_names_save = memory_names.clone();
                                    let _memory_values_save = memory_values.clone();
                                    let _memmory_types_save = memory_types.clone();
                                    let _func_names_save = func_names.clone();
                                    let _func_code_save = func_code.clone();
                                    let _func_par_save = func_par.clone();
                                    let mut pass_vec: Vec<String> =
                                        vec!["a".to_string(), "(".to_string()];
                                    loop {
                                        if contents[position] == "\n" || contents[position] == ";" {
                                            break;
                                        }
                                        pass_vec.push(contents[position].clone().to_string());
                                        position += 1;
                                    }
                                    pass_vec.push(")".to_string());
                                    let value = functions::getstring(
                                        0,
                                        pass_vec.clone(),
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
                                    .join("zzGVgfHaNtPMe7H9RRyx3rWC9JyyZdMkc2v")
                                    .to_string();
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
    }
    format!("{}}}", returns)
}

fn code_to_add() -> String {
    //Put code here to add it everywhere
    "

    "
    .to_string()
}

pub fn error(error: String) {
    println!("{}", error);
    std::process::exit(1);
}

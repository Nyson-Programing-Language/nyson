#![allow(warnings, unused)]
use std::ops::{Add, Sub, Mul, Div};
use rand::Rng;
use crate::lexer;
use std::fs;
use std::thread;
use std::fs::File;
use std::io::Write;
use std::str::SplitWhitespace;
use std::process::Command;

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
    while read {
        read = false;
        skiperwiper = false;
        for mut x in readfrom..contents.len() {
            if skiperwiper == false {
                if contents[x] == "\"" || contents[x] == "\'" || contents[x] == r"\`" {
                    quotes = quotes + 1;
                }
                if contents[x] == "{" || contents[x] == "}" {
                    squigle = squigle + 1;
                }
                if quotes%2 == 0 && squigle%2 == 0 {
                    if contents[x] == "log" {
                        log(x, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev);
                    }
                    else if contents[x] == "loop" {
                        _loop(x, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev, func_names.clone(), func_par.clone(), func_code.clone());
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
                        let move_up = 1;
                        if contents[x+move_up] == "int" {
                            memory_types.push(String::from("int"));
                        } else if contents[x+move_up] == "str"  {
                            memory_types.push(String::from("str"));
                        }
                        memory_names.push(String::from(contents[x+move_up+move_up].clone()));
                        let move_up_up = 1;
                        if contents[x+move_up+move_up+1] == ":" {
                            let move_up_up = 1;
                        }
                        if contents[x+move_up+move_up+1] == " " {
                            let move_up_up = 2;
                        }
                        if contents[x+move_up+move_up+move_up_up] == " " {
                            let mut value = String::new();
                            let mut move_final = 2;
                            if contents[x+move_up+move_up+move_up_up+2] == " " {
                                let move_final = 2;
                            } else {
                                let move_final = 1;
                            }
                            let mut n = 0;
                            let mut quote = 0;
                            loop {
                                if contents[x+move_up+move_up+move_up_up+move_final] == ";" {
                                    if dev {
                                        println!("contents[x+move_up+move_up+move_up_up+move_final]: {:?}", contents[x+move_up+move_up+move_up_up+move_final]);
                                    }
                                    break;
                                }
                                else {
                                    if contents[x+move_up+move_up+move_up_up+move_final] == "\"" || contents[x+move_up+move_up+move_up_up+move_final] == "\'" || contents[x+move_up+move_up+move_up_up+move_final] == r"\`" {
                                        quote = quote + 1;
                                    }
                                    else {
                                        if contents[x+move_up+move_up+move_up_up+move_final] == "math" {
                                            value.push_str(math(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else if contents[x+move_up+move_up+move_up_up+move_final] == "round" {
                                            value.push_str(round(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else if contents[x+move_up+move_up+move_up_up+move_final] == "replace" {
                                            value.push_str(replace(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else if contents[x+move_up+move_up+move_up_up+move_final] == "input" {
                                            value.push_str(input(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else if contents[x+move_up+move_up+move_up_up+move_final] == "exec" {
                                            value.push_str(exec(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else if contents[x+move_up+move_up+move_up_up+move_final] == "trim" {
                                            value.push_str(trim(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else if contents[x+move_up+move_up+move_up_up+move_final] == "getcont" {
                                            value.push_str(get_contents(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else {
                                            if n == 0 {
                                                if quote%2 == 1 {
                                                    value.push_str(contents[x+move_up+move_up+move_up_up+move_final].as_str());
                                                }
                                                else {
                                                    let mut position = memory_names_save.len();
                                                    let mut skip = false;
                                                    for pos in 0..memory_names_save.len() {
                                                        if skip == false {
                                                            if memory_names_save[pos].to_string() == contents[x+move_up+move_up+move_up_up+move_final].to_string() {
                                                                position = pos;
                                                                skip = true;
                                                            }
                                                        }
                                                    }
                                                    if position != memory_names_save.len() {
                                                        value.push_str(memory_values_save[position].to_string().as_str());
                                                    }
                                                    else {
                                                        value.push_str(contents[x+move_up+move_up+move_up_up+move_final].as_str());
                                                    }
                                                }
                                            }
                                        }
                                        if n >= 1 && contents[x+move_up+move_up+move_up_up+move_final] == "(" {
                                            n = n + 1
                                        }
                                        else if n >= 1 && contents[x+move_up+move_up+move_up_up+move_final] == ")" {
                                            n = n - 1;
                                            if n == 1 {
                                                n = 0;
                                            }
                                        }
                                    }
                                }
                                move_final = move_final+1;
                            }
                            memory_values.push(value);
                        } else {
                            let mut value = String::new();
                            let mut move_final = 2;
                            if contents[x+move_up+move_up+move_up_up+2] == " " {
                                let move_final = 2;
                            } else {
                                let move_final = 1;
                            }
                            let mut n = 0;
                            let mut quote = 0;
                            loop {
                                if contents[x+move_up+move_up+move_up_up+move_final] == ";" {
                                    if dev {
                                        println!("contents[x+move_up+move_up+move_up_up+move_final]: {:?}", contents[x+move_up+move_up+move_up_up+move_final]);
                                    }
                                    break;
                                }
                                else {
                                    if contents[x+move_up+move_up+move_up_up+move_final] == "\"" || contents[x+move_up+move_up+move_up_up+move_final] == "\'" || contents[x+move_up+move_up+move_up_up+move_final] == r"\`" {
                                        quote = quote + 1;
                                    }
                                    else {
                                        if contents[x+move_up+move_up+move_up_up+move_final] == "math" {
                                            value.push_str(math(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else if contents[x+move_up+move_up+move_up_up+move_final] == "round" {
                                            value.push_str(round(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else if contents[x+move_up+move_up+move_up_up+move_final] == "replace" {
                                            value.push_str(replace(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else if contents[x+move_up+move_up+move_up_up+move_final] == "input" {
                                            value.push_str(input(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else if contents[x+move_up+move_up+move_up_up+move_final] == "exec" {
                                            value.push_str(exec(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else if contents[x+move_up+move_up+move_up_up+move_final] == "trim" {
                                            value.push_str(trim(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else if contents[x+move_up+move_up+move_up_up+move_final] == "getcont" {
                                            value.push_str(get_contents(x+move_up+move_up+move_up_up+move_final, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(),  dev).to_string().as_str());
                                            n = 1;
                                        }
                                        else {
                                            if n == 0 {
                                                if quote%2 == 1 {
                                                    value.push_str(contents[x+move_up+move_up+move_up_up+move_final].as_str());
                                                }
                                                else {
                                                    let mut position = memory_names_save.len();
                                                    let mut skip = false;
                                                    for pos in 0..memory_names_save.len() {
                                                        if skip == false {
                                                            if memory_names_save[pos].to_string() == contents[x+move_up+move_up+move_up_up+move_final].to_string() {
                                                                position = pos;
                                                                skip = true;
                                                            }
                                                        }
                                                    }
                                                    if position != memory_names_save.len() {
                                                        value.push_str(memory_values_save[position].to_string().as_str());
                                                    }
                                                    else {
                                                        value.push_str(contents[x+move_up+move_up+move_up_up+move_final].as_str());
                                                    }
                                                }
                                            }
                                        }
                                        if n >= 1 && contents[x+move_up+move_up+move_up_up+move_final] == "(" {
                                            n = n + 1
                                        }
                                        else if n >= 1 && contents[x+move_up+move_up+move_up_up+move_final] == ")" {
                                            n = n - 1;
                                            if n == 1 {
                                                n = 0;
                                            }
                                        }
                                    }
                                }
                                move_final = move_final+1;
                                if dev {
                                    println!("move_final: {:?}", move_final);
                                    println!("contents[x+move_up+move_up+move_up_up+move_final]: {:?}", contents[x+move_up+move_up+move_up_up+move_final]);
                                }
                            }
                            memory_values.push(value);
                        }
                        if dev {
                            println!("memory_names: {:?}", memory_names);
                            println!("memory_types: {:?}", memory_types);
                            println!("memory_values: {:?}", memory_values);
                        }
                    }
                    else if contents[x] == "if" {
                        let mut condition = String::new();
                        let move_up = 0;
                        let mut cond_vec: Vec<usize> = Vec::new();
                        let mut final_check = 0;
                        loop {
                            if contents[x] == "{" {
                                final_check = x;
                                break;

                            } else {
                                x=x+1;
                                condition.push_str(&contents[x]);
                                cond_vec.push(x);
                            }
                        }
                        condition = condition.replace("{", "");
                        let mut cond_var: String = String::new();
                        for letter in condition.chars() {
                            if letter.to_string() == "=" {
                                break;
                            }
                            else {
                                cond_var.push(letter);
                            }
                        }
                        cond_var = cond_var.replace(" ", "");
                        let mut cond_equal = String::new();
                        let mut h = 0;
                        for letter in condition.chars() {
                            if letter.to_string() == ":" {
                                h = h + 1;
                                let mut j = 0;
                                for letter in condition.chars() {
                                    j = j + 1;
                                    if h < j {
                                        cond_equal.push(letter)
                                    }
                                }
                            } else {
                                h = h + 1;
                            }
                        }
                        let mut quote_count = 0;
                        let mut cond_eq = String::new();
                        for letter in cond_equal.chars() {
                            if quote_count == 1 {
                                cond_eq.push(letter)
                            }
                            if letter.to_string() == "\"" {
                                quote_count = quote_count+1
                            } else if quote_count == 0 {
                                continue;
                            } else if quote_count == 2 {
                                break;
                            } else {
                                continue;
                            }
                        }
                        cond_eq.pop();
                        let mut index = 0;
                        for j  in 0..memory_names.len() {
                            if memory_names[j].to_string() == cond_var {
                                index = j;
                            }
                        }
                        let mut value = String::new();
                        for char in memory_values[index].chars() {
                            value.push(char);
                        }
                        if value.to_string() == cond_eq.to_string() {
                            if dev {
                                println!("equal");
                            }
                            let mut vecvec:Vec<String> = Vec::new();
                            let mut skiper = false;
                            let mut n = 0;
                            let mut reacheded = false;
                            for yy in x..contents.len() {
                                if skiper == false {
                                    if contents[yy] == "{" {
                                        n = n +1;
                                        reacheded = true;
                                    }
                                    else if contents[yy] == "}" {
                                        n = n-1;
                                    }
                                    if n > 0 {
                                        vecvec.push((&contents[yy]).parse().unwrap());
                                    }
                                    else if reacheded == true {
                                        skiper = true;
                                    }
                                }
                            }
                            vecvec.remove(0);
                            run(vecvec.clone(), dev, memory_names.clone(), memory_values.clone(), memory_types.clone(), func_names.clone(), func_par.clone(), func_code.clone());
                        } else {
                            if dev {
                                println!("not equal");
                                println!("value: {:?}, cond_eq: {:?}", value, cond_eq);
                            }
                            let find_brack = 0;
                            let position_rem = 0;
                            for ele in x..contents.len() {
                                if contents[ele] == "}" {
                                    break;
                                } else {
                                    contents[ele] = String::from("-");
                                }
                            }
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
                                    let to_parse = lexer::lexer(func_code[postion].to_string(), dev);
                                    run(to_parse, dev, memory_names.clone(), memory_values.clone(), memory_types.clone(), func_names.clone(), func_par.clone(), func_code.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
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
                if vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`" {
                    n = n + 1;
                }else if vec[y] == "(" {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" {
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
                        string.push_str(&*memory_values[postion].to_string());
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
                if vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`" {
                    n = n + 1;
                }else if vec[y] == "(" {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" {
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
                        string.push_str(&*memory_values[postion].to_string());
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
    if vec[0] == "\"" || vec[0] == "\'" || vec[0] == r"\`" {
        vec.remove(0);
        vec.remove(vec.len()-1);
    }
    for y in 0..vec.len() {
        if vec[y] == "(" {
            n = n +1;
        }
        else if vec[y] == ")" {
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
                if vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`" {
                    n = n + 1;
                }else if vec[y] == "(" {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" {
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
                if text[y] == "\"" || text[y] == "\'" || text[y] == r"\`" {
                    n = n + 1;
                }else if text[y] == "(" {
                    n1 = n1 + 1;
                }
                else if text[y] == ")" {
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
                } else if vec[y] == "round" {
                    text_s.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
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
    return  line;
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
                if vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`" {
                    n = n + 1;
                }else if vec[y] == "(" {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" {
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
                        string.push_str(&*memory_values[postion].to_string());
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
                }else if vec[y] == "(" {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" {
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
                }else if find[y] == "(" {
                    n1 = n1 + 1;
                }
                else if find[y] == ")" {
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
                }else if replacer[y] == "(" {
                    n1 = n1 + 1;
                }
                else if replacer[y] == ")" {
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
                } else if vec[y] == "round" {
                    replacer_s.push_str(round(y, vec.to_vec(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev).to_string().as_str());
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
    let maybe_contents = fs::read_to_string(string);
    let mut contents = if maybe_contents.is_ok() {
        maybe_contents.unwrap()
    } else {
        panic!("Could not open file for reading.");
    };
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

pub fn _loop(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool, func_names: Vec<String>, func_par: Vec<String>, func_code: Vec<String>) {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let number_of_times = math(x, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev);
    let mut n = 0;
    let mut reached = false;
    for y in x+1..contents.len() {
        if skip == false {
            if contents[y] == "{" {
                n = n +1;
                reached = true;
            }
            else if contents[y] == "}" {
                n = n-1;
            }
            if n > 0 {
                vec.push((&contents[y]).parse().unwrap());
            }
            else if reached == true {
                skip = true;
            }
        }
    }
    vec.remove(0);
    for q in 0..number_of_times.round() as i32 {
        run(vec.clone(), dev, memory_names.clone(), memory_values.clone(), memory_types.clone(), func_names.clone(), func_par.clone(), func_code.clone());
    }
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
                    if vec[y].to_lowercase() == "random" {
                        vec[y] = rng.gen::<f32>().to_string();
                        skip = true;
                    }
                    else if vec[y] == "+" {
                        if vec[y + 1].to_lowercase() == "random" {
                            vec[y+1] = rng.gen::<f32>().to_string();
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
                        vec[y - 1] = vec[y - 1].parse::<f32>().unwrap().sub(vec[y + 1].parse::<f32>().unwrap()).to_string();
                        vec.remove(y);
                        vec.remove(y);
                        skip = true;
                    }
                    else if vec[y] == "*" {
                        if vec[y + 1].to_lowercase() == "random" {
                            vec[y+1] = rng.gen::<f32>().to_string();
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
                        vec[y - 1] = vec[y - 1].parse::<f32>().unwrap().div(vec[y + 1].parse::<f32>().unwrap()).to_string();
                        vec.remove(y);
                        vec.remove(y);
                        skip = true;
                    }
                    else if vec[y] == "^" {
                        if vec[y + 1].to_lowercase() == "random" {
                            vec[y+1] = rng.gen::<f32>().to_string();
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
                if vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`" {
                    n = n + 1;
                }else if vec[y] == "(" {
                    n1 = n1 + 1;
                }
                else if vec[y] == ")" {
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
    let mut start_with_space = true;
    while start_with_space {
        start_with_space = false;
        if imput_s.chars().nth(0).unwrap().to_string() == " " || imput_s.chars().nth(0).unwrap().to_string() == r"\t" || imput_s.chars().nth(0).unwrap().to_string() == r"\n" {
            imput_s.remove(0);
            start_with_space = true;
        }
        if imput_s.chars().nth(imput_s.len()-1).unwrap().to_string() == " " || imput_s.chars().nth(imput_s.len()-1).unwrap().to_string() == r"\t" || imput_s.chars().nth(imput_s.len()-1).unwrap().to_string() == r"\n" {
            imput_s.remove(imput_s.len()-1);
            start_with_space = true;
        }
    }
    return imput_s;
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

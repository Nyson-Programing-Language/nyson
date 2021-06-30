use std::ops::{Add, Sub, Mul, Div};
use rand::Rng;
use crate::lexer;

pub fn run(mut contents: Vec<String>, dev: bool, mut memory_names: Vec<String>, mut memory_values: Vec<String>, mut memory_types: Vec<String>, mut func_names: Vec<String>, mut func_par: Vec<String>, mut func_code: Vec<String>) {
    if dev {
        println!("{:?}", contents);
    }
    let mut quotes = 0;
    let mut squigle = 0;
    for mut x in 0..contents.len() {
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
            else if contents[x] == "func" {
                let mut vec:Vec<String> = Vec::new();
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
                    println!("{}", par);
                    println!("{}", code);
                    println!("{}", name);
                }
                func_par.push(par);
                func_code.push(code);
                func_names.push(name);
                if dev {
                    println!("{:?}", func_par);
                    println!("{:?}", func_code);
                    println!("{:?}", func_names);
                }
            }
            else if contents[x] == "dec" {
                let memory_names_save = memory_names.clone();
                let memory_types_save = memory_types.clone();
                let memory_values_save = memory_values.clone();
                let move_up = 2;
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
                        let mut move_final = 2;
                    } else {
                        let mut move_final = 1;
                    }
                    let mut n = 0;
                    let mut quote = 0;
                    loop {
                        if contents[x+move_up+move_up+move_up_up+move_final] == ";" {
                            if dev {
                                println!("{:?}", contents[x+move_up+move_up+move_up_up+move_final]);
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
                        let mut move_final = 2;
                    } else {
                        let mut move_final = 1;
                    }
                    let mut n = 0;
                    let mut quote = 0;
                    loop {
                        if contents[x+move_up+move_up+move_up_up+move_final] == ";" {
                            if dev {
                                println!("{:?}", contents[x+move_up+move_up+move_up_up+move_final]);
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
                            println!("{:?}", move_final);
                            println!("{:?}", contents[x+move_up+move_up+move_up_up+move_final]);
                        }
                    }
                    memory_values.push(value);
                }
                if dev {
                    println!("{:?}", memory_names);
                    println!("{:?}", memory_types);
                    println!("{:?}", memory_values);
                }
            }
            else if contents[x] == "if" {
                let mut condition = String::new();
                let mut move_up = 0;
                let mut cond_vec: Vec<usize> = Vec::new();
                let mut final_check = 0;
                loop {
                    if contents[x] == "<" {
                        final_check = x;
                        break;
                        
                    } else {
                        x=x+1;
                        condition.push_str(&contents[x]);
                        cond_vec.push(x);
                    }
                }
                condition = condition.replace("<", "");
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
                    if letter.to_string() == "=" {
                        h = h + 2;
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
                let mut yh = 0;
                for char in memory_values[index].chars() {
                    if yh > 0 {
                        value.push(char)
                    }
                    yh = yh + 1
                }
                if value.to_string() == cond_eq.to_string() {
                    println!("equal");
                } else {
                    println!("not equal");
                    let mut find_brack = 0;
                    let mut position_rem = 0;
                    for ele in x..contents.len() {
                        if contents[ele] == ">" {
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
        println!("{:?}",  vec);
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
                    let mut skip1 = false;
                    for y in x+1..vec.len() {
                        if skip1 == false {
                            if contents[y] == "(" {
                                n = n +1;
                            }
                            else if contents[y] == ")" {
                                n = n-1;
                            }
                            if n%2 == 0 {
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

pub fn _loop(x:usize, contents: Vec<String>, memory_names: Vec<String>, memory_values: Vec<String>, memory_types: Vec<String>, dev: bool, mut func_names: Vec<String>, mut func_par: Vec<String>, mut func_code: Vec<String>) {
    let mut vec:Vec<String> = Vec::new();
    let mut skip = false;
    let mut number_of_times = math(x, contents.clone(), memory_names.clone(), memory_values.clone(), memory_types.clone(), dev);
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

pub fn get_line(x:usize, contents: Vec<String>) -> i32 {
    let mut line = 1;
    for n in 0..x {
        if contents[n] == "\n" {
            line = line + 1;
        }
    }
    return line;
}

use std::ops::{Add, Sub, Mul, Div};

pub fn run(contents: Vec<String>) {
    println!("{:?}", contents);
    let mut memory_names: Vec<String> = Vec::new();
    let mut memory_values: Vec<String> = Vec::new();
    let mut memory_types: Vec<String> = Vec::new();
    for x in 0..contents.len() {
        if contents[x] == "log" {
            let mut vec = Vec::new();
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
                    if n%2 == 0 {
                        skip = true;
                        for z in x+1..y+1 {
                            vec.push(&contents[z]);
                        }
                    }
                }
            }
            let mut z = 0;
            for y in vec.to_vec() {
                if y == "(" || y == ")" {
                    z = z + 1;
                }
            }
            let mut string: String = "".to_string();
            if z > 2 {
                // if you have more than 2 parentheses
            }
            else {
                let mut n = 0;
                for y in 0..vec.len() {
                    if vec[y] == "\"" || vec[y] == "\'" || vec[y] == r"\`" {
                        n = n + 1;
                    }
                    else if n%2 == 1 {
                        string.push_str(vec[y])
                    }
                }
            }
            println!("{}", string);
        }
        else if contents[x] == "math" {
            let mut vec:Vec<String> = Vec::new();
            let mut skip = false;
            let mut n = 0;
            for y in x+1..contents.len() {
                if skip == false {
                    if contents[x+1] != "(" {
                        println!("You have to put a parentheses after a math");
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
                            if vec[y] == "+" {
                                vec[y - 1] = vec[y - 1].parse::<i32>().unwrap().add(vec[y + 1].parse::<i32>().unwrap()).to_string();
                                vec.remove(y);
                                vec.remove(y);
                                skip = true;
                            }
                            else if vec[y] == "-" {
                                vec[y - 1] = vec[y - 1].parse::<i32>().unwrap().sub(vec[y + 1].parse::<i32>().unwrap()).to_string();
                                vec.remove(y);
                                vec.remove(y);
                                skip = true;
                            }
                            else if vec[y] == "*" {
                                vec[y - 1] = vec[y - 1].parse::<i32>().unwrap().mul(vec[y + 1].parse::<i32>().unwrap()).to_string();
                                vec.remove(y);
                                vec.remove(y);
                                skip = true;
                            }
                            else if vec[y] == "/" {
                                vec[y - 1] = vec[y - 1].parse::<i32>().unwrap().div(vec[y + 1].parse::<i32>().unwrap()).to_string();
                                vec.remove(y);
                                vec.remove(y);
                                skip = true;
                            }
                            else if vec[y] == "^" {
                                vec[y - 1] = vec[y - 1].parse::<i32>().unwrap().pow(vec[y + 1].parse::<i32>().unwrap() as u32).to_string();
                                vec.remove(y);
                                vec.remove(y);
                                skip = true;
                            }
                        }
                    }
                    if vec.len() == 1 {
                        keep_going = false;
                    }
                }
            }
            println!("{:?}", vec);
        }
        else if contents[x] == "dec" {
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
                loop {
                    if contents[x+move_up+move_up+move_up_up+move_final] == ";" {
                        break;
                    }
                    else {
                        value.push_str(contents[x+move_up+move_up+move_up_up+move_final].as_str());
                        move_final = move_final+1;
                    }
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
                loop {
                    if contents[x+move_up+move_up+move_up_up+move_final] == ";" {
                        println!("{:?}", contents[x+move_up+move_up+move_up_up+move_final]);
                        break;
                    }
                    else {
                        value.push_str(contents[x+move_up+move_up+move_up_up+move_final].as_str());
                        move_final = move_final+1;
                        println!("{:?}", move_final);
                        println!("{:?}", contents[x+move_up+move_up+move_up_up+move_final]);
                    }
                }
                memory_values.push(value);
            }
            println!("{:?}", memory_names);
            println!("{:?}", memory_types);
            println!("{:?}", memory_values);
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

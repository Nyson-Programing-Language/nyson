#![allow(unused)]

pub enum Tokens {
    Id{value: String},
    Op{value: String},
    Num{value: i32},
    Newline{value: String},
    Quote{value: String}
}

pub fn lexer(contents: String){
    let contents_literal = remove_whitespace(&contents);
    println!("{:?}", contents_literal);
    for character in contents_literal.chars() {
        let lex_vec: Vec<String> = Vec::new();
        let mut lex_str = String::new();
        lex_str.push(character);
        check_enum(String::from(character));
        println!("{:?}", character)
    }
}

fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

fn check_enum(input_string: String) -> String{
    let check = input_string.as_str();
    match check{
        "log" => println!("id"),
        "(" | ")" | "+" | "-" | "/" | "*" => println!("op"),
        r##"""## => println!("quote"),
        "1" | "2" | "3" | "4" | "5" | "6" | "7"| "8"| "9" => print!("num"),
        _ => println!("num"),

    }
}

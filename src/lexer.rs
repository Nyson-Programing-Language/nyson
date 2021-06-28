#![allow(unused)]
pub enum Tokens {
    Id{value: String},
    Op{value: String},
    Num{value: i32},
    Newline{value: String},
    Quote{value: String}
}

pub fn lexer(contents: String) -> Vec<String>{
    let contents_literal = split(contents);
    /*
    for n in 0..contents_literal.len() {
        let lex_vec: Vec<String> = Vec::new();
        let mut lex_str = String::new();
        lex_str.push(contents_literal.chars().nth(n).unwrap());
        check_enum(String::from(contents_literal.chars().nth(n).unwrap()));
        println!("{:?}", contents_literal.chars().nth(n).unwrap())
    }
    */
    let mut outputs:Vec<String> = Vec::new();
    for x in contents_literal {
        outputs.push(String::from(x));
    }
    
    return outputs;
}

fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

fn split(text: String) -> Vec<String> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(|c: char| !(c.is_alphanumeric() || c == '\'' || c == ' ')) {
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    
    let mut need_split = Vec::new();
    
    for n in 0..result.len() {
        if (result[n].contains(' ')) {
            let mut number_of_string_selectors = 0;
            for x in 0..n {
                if (result[x].contains('\"') || result[x].contains('\'') || result[x].contains(r"\`")) {
                    number_of_string_selectors = number_of_string_selectors + 1;
                }
            }
            if number_of_string_selectors % 2 == 0 {
                need_split.push(n);
            }
        }
    }

    let mut output:Vec<&str> = Vec::new();
    let mut inc = 0;
    
    for n in need_split {
        for x in output.len()..n-inc {
            if output.len() < n {
                output.push(result[x]);
            }
        }
        let text = result[n + inc];
        let mut vec = Vec::new();
        let mut last = 0;
        for (index, matched) in text.match_indices(|c: char| !(c.is_alphanumeric() || c == '\'')) {
            if last != index {
                vec.push(&text[last..index]);
            }
            vec.push(matched);
            last = index + matched.len();
        }
        if last < text.len() {
            vec.push(&text[last..]);
        }
        inc = inc + vec.len() - 1;
        
        for x in vec {
            output.push(x);
        }
        
    }
    
    let mut outputs:Vec<String> = Vec::new();
    
    for x in output {
        outputs.push(String::from(x));
    }
    
    return outputs;
}

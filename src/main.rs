use std::fs;
#[allow(unused)]

fn main() {
    // read file
    println!("In file {}", r"C:\Users\nyels\nyelx\examples\test.nys");
    let contents = fs::read_to_string(r"C:\Users\nyels\nyelx\examples\test.nys")
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
    let clear_contents = contents.replace(" ", "");
    println!("{:?}", clear_contents);
    let mut token_list: Vec<String> = Vec::new();
    let mut mutable_char = String::from("");
    for character in clear_contents.chars() { 
        mutable_char.push(character);
        match mutable_char.as_str() {
            // ids
            "log" => {
                token_list.push(mutable_char.to_string());
                mutable_char = String::new();
            }
            // ops
            "+" | "-" | "/" | "*" | "%" | "(" | ")" => {
                token_list.push(mutable_char.to_string());
                mutable_char = String::new();
            }
            _ => print!("")
        }

    }
    println!("{:?}", token_list)
}

pub fn match_string(character: String, vector: &mut Vec<String>){
    match character.as_str() {
        "log" => vector.push(character),
        _ => print!("")
    }
}

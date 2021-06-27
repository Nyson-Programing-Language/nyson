pub enum Tokens {
    Id,
    Op,
    Num,
    Newline,
    Char,
}

pub fn lexer(contents: String) {
    println!("{:?}", contents)
}

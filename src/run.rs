pub fn run(contents: Vec<String>) {
    println!("{:?}", contents);
    let memory_names: Vec<String> = Vec::new();
    let memory_values: Vec<String> = Vec::new();
    let memory_types: Vec<String> = Vec::new();
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
            println!("{:?}", vec);
        }
        else if contents[x] == "math" {
            let mut vec = Vec::new();
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
                            vec.push(&contents[z]);
                        }
                    }
                }
            }
            println!("{:?}", vec);
        }
        else if contents[x] == "dec" {
            let mut contents_clear = contents.clone();
            for y in 0..contents_clear.len() {
                if contents_clear[y] == "" {
                    contents_clear.remove(y);
                } else if contents_clear[y] == r"\n" {
                    contents_clear.remove(y);
                } else if contents_clear[y] == r"\r" {
                    contents_clear.remove(y);
                }
            }
            println!("{:?}", contents_clear)
        }
    }
}

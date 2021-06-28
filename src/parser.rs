pub fn create_tree(lexed: Vec<String>){
    perform_match(lexed)
}

pub fn perform_match(items: Vec<String>) {
    for (count, item) in items.iter().enumerate() {
        let check: String = String::from(item);
        match &check[..] {
            "log" => {
                println!("{:?}",items[count+3])
            }
            _ => continue
        }
    }
}

fn main() {
    let name = "Shohel".to_string();
    let mut v: Vec<String> = Vec::new();
    v.push(name);
    println!("{:?}", v);
    let v: Vec<String> = std::vec::Vec::new(); // This works because Rust inserts use std::prelude::v1*; at the beginning of every module by default
    println!("{:?}", v); //  so it exports many symbols like Vec, String, option , Result.
}

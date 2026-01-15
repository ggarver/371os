use std::string::String;
use std::env;
use std::fs;

fn main() {
    // get file path from cl arg
    let path: String = env::args().nth(1).unwrap();
    
    // Read into string, unwrap panics if empty
    let string: String = fs::read_to_string(path).unwrap();

    // Split into vector of string slices
    let words: Vec<&str> = string.split_whitespace().collect();
    let count = words.len();

    println!("{:}", count);
}

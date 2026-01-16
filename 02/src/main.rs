use std::string::String;
use std::env;
use std::fs;

fn main() {
    // -----------------WORDS-------------------
    // get file path from cl arg
    let path: String = env::args().nth(1).unwrap();
    
    // Read into string, unwrap panics if empty
    let string: String = fs::read_to_string(&path).unwrap();

    // Split into vector of string slices
    let words: Vec<&str> = string.split_whitespace().collect();
    let w_count = words.len();

    // count newlines
    let mut l_count = 0;
    for _line in string.lines() {
        l_count += 1;
    }

    // count bytes
    let mut b_count = 0;
    for _byte in string.bytes() {
        b_count += 1;
    }

    println!("{} {} {} {}", l_count, w_count, b_count, path);
}

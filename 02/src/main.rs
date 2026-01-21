use std::string::String;
use std::env;
use std::fs;

fn main() {
    // get file path from cl arg
    let arg_1: String = env::args().nth(1).unwrap();

    // check if character flags - otherwise try to read file
    // try to iter through args
    for arg in env::args() {
        if arg.starts_with('-') {
            if arg.contains("c"){ 
                let bytes = true;
            } else if arg.contains("m"){
                let chars = true;
            } else if arg.contains("l"){
                let lines = true;
            } else if arg.contains("L"){
                let max_lines = true;
            } else if arg.contains("w"){
                let words = true;
            }
            // string flags
            if arg.starts_with("--") {
                if arg == "--bytes"{
                    let bytes = true;
                } else if arg == "--chars"{
                    let chars = true;
                } else if arg == "--lines"{
                    let lines = true;
                    // This one needs to be fixed
                } else if arg == "--files0-from=F"{
                    let file_f = true;
                } else if arg == "--max-line-length"{
                    let max_l = true;
                } else if arg == "--words"{
                    let words = true;
                }
            }

        } else {
            // no char flags
            // Read into string, unwrap panics if empty
            let string: String = fs::read_to_string(&arg_1).unwrap();

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
            println!(" {} {} {} {}", l_count, w_count, b_count, arg_1);
        }
    }
}


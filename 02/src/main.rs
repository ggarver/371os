use std::env;
use std::fs;

fn main() {
    // parse
    let mut bytes = false;
    let mut chars = false;
    let mut lines = false;
    let mut max_l = false;
    let mut words = false;
    let mut file_f = false;
    let mut files = Vec::new();

    // check if character flags - otherwise try to read file
    // try to iter through args
    for arg in env::args().skip(1) {
        if arg.starts_with('-') {
            if arg.contains("c"){ 
                bytes = true;
            } else if arg.contains("m"){
                chars = true;
            } else if arg.contains("l"){
                lines = true;
            } else if arg.contains("L"){
                max_l = true;
            } else if arg.contains("w"){
                words = true;
            }
            // string flags
            if arg.starts_with("--") {
                if arg == "--bytes"{
                    bytes = true;
                } else if arg == "--chars"{
                    chars = true;
                } else if arg == "--lines"{
                    lines = true;
                    // This one needs to be fixed
                } else if arg == "--files0-from=F"{
                    file_f = true;
                } else if arg == "--max-line-length"{
                    max_l = true;
                } else if arg == "--words"{
                    words = true;
                }
            }
            // if not flag, should be file
        } else {
            files.push(arg);
        }
    }

    for filepath in &files {
        let cont = fs::read_to_string(&filepath).unwrap();

        // calculate counts
        let b_count = cont.bytes().count();
        let l_count = cont.lines().count();
        let c_count = cont.chars().count();
        let w_count = cont.split_whitespace().count();

        // let ml_length = cont...


        // print for char flags
        if bytes || chars || lines || max_l || words {
            if lines {
                println!("{}", l_count);
            }
            if words {
                println!("{}", w_count);
            }
            if chars {
                println!("{}", c_count);
            }
            if bytes {
                println!("{}", b_count);
            }
            if max_l {
                println!("NA");
            }
            println!("{}", filepath);
        } else {
            println!(" {} {} {} {}", l_count, w_count, b_count, filepath);

        }
    }
}


use std::string::String;
use std::env;
use std::fs;

fn main() {
    // parse
    let mut bytes = false;
    let mut chars = false;
    let mut lines = false;
    let mut max_lines = false;
    let mut words = false;


    // get file path from cl arg
    let mut files = Vec::new();
    let path: String = env::args().nth(1).unwrap();

    // check if character flags - otherwise try to read file
    // try to iter through args
    for arg in env::args().skip(1) {
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
            // if not flag, should be file
        } else {
            files.push(arg);
        }

        for filepath in files {
            let cont = fs::read_to_string(&filepath).unwrap();

            // calculate counts
            let b_counts = cont.bytes().count();
            let l_count = cont.lines().count();
            let w_count = cont.words().count();
            let c_count = cont.chars().count();
            //let ml_length = cont...
        }        

        //base case no flags
        if !bytes && !chars && !lines && !max_lines && !words {
            println!(" {} {} {} {}", l_count, w_count, b_count, path);
        }
    }
}


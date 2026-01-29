fn main() {
    unsafe {
        // transmute changes type without altering underlying memory
        
        // Hello World! in i32s, i32=4bytes
        let address: [i32; 3] = [1819043144, 1867980911, 560229490];
        // each byte = char, "Hello World!" = 12
        let chars: &[u8; 12] = std::mem::transmute(&address);
        println!("{:?}", std::str::from_utf8_unchecked(chars));
    }
}



// Testing the VGA buffer 
#[test_case]
fn test_println(){
    println!("test println output");
}

#[test_case]
fn linewrap(){
    for _ in 0..100{
        println!("linewrap");
    }
}

#[test_case]
fn fill_vga(){
    for _ in 0..400{
        println!("beginning");
    }
    for _ in 400..600{
        println!("end");
    }
}

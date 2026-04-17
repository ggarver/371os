#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]

use osirs::println;
use osirs::_clear_vga;

#[test_case]
fn simple_assert(){
    _clear_vga();
    // test simple assertion 
    println!("test_1");
    osirs::print!("assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}


// Testing the VGA buffer 
#[test_case]
fn test_println(){
    _clear_vga();
    println!("A");
    // peek at VGA buff to make - FIRST slot 
    let val: u8 = unsafe { *(0xb8000 as *const u8) };
    assert_eq!(val, b'A');
}

#[test_case]
fn linewrap(){
    _clear_vga();
    println!("{:081x}", 1);
    println!("{:x}", 2);
    // 80 columns per row 
    // check if wrap line0 to line1
    let l1_val = unsafe {*((0xb8000 + 160) as *const u8) };
    // use byte literal
    assert_eq!(l1_val, b'1');
}

#[test_case]
fn test_scroll(){
    _clear_vga();
    for i in 0..26 {
        println!("{}", i);
    }
    // check that row 0 contains what was on row 1
    let val = unsafe { *(0xb8000 as *const u8) };
    assert_eq!(val, b'1');
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    osirs::test_panic_handler(info)
}

#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(_test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga;
pub mod serial;


#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}


// make exit codes for Qemu 
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// exit code for qemu 
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe{
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}




// clear to make tests easier 
fn _clear_vga() {
    for i in 0..2000 {
        unsafe {
            *((0xb8000 + i * 2) as *mut u8) = b' ';
        }
    }
    vga::_reset();
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

#[test_case]
fn simple_assert(){
    _clear_vga();
    // test simple assertion 
    println!("test_1");
    print!("assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}


// Panic when test 
pub fn test_panic_handler(_info: &core::panic::PanicInfo) -> ! {
    serial_println!( "[failed]");
    serial_print!("\n{}\n", _info);
    exit_qemu(QemuExitCode::Failed);
    serial_print!("\n");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic_handler(info)
}


pub fn _test_runner(tests: &[&dyn Fn()]) {
    for (i, test) in tests.iter().enumerate() {
        serial_print!("Running test {:0x}...", i);
        test();
        serial_println!(" [pass]");
    }
    exit_qemu(QemuExitCode::Success);
}




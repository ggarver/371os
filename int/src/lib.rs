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


// Testing the VGA buffer 
#[test_case]
fn test_println(){
    println!("A");
    // peek at VGA buff to make - FIRST slot 
    let val: u8 = unsafe { *(0xb8000 as *const u8) };
    assert_eq!(val, b'A');
}

#[test_case]
fn linewrap(){
    println!("{:081x}", 1);
    println!("{:x}", 2);
    // 80 columns per row, always *2 because char byte
    // check if wrap line0 to line1
    let l1_val = unsafe {*((0xb8000 + 160 * 2) as *const u8) };
    // use byte literal
    assert_eq!(l1_val, b'2');
}

//#[test_case]
//fn fill_vga(){
//    for _ in 0..400{
//        println!("beginning");
//    }
//    for _ in 400..600{
//        println!("end");
//    }
// 

#[test_case]
fn simple_assert(){
    // test simple assertion 
    println!("test_1");
    print!("assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}

// should fail 
#[test_case]
fn should_fail(){
    assert_eq!(1,2);
}


// Panic when not test 
#[cfg(not(test))]
#[panic_handler]
#[allow(unconditional_recursion)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("panic! at the panic handler");
    loop {}
    // panic(info)
}

// Panic when test 
#[cfg(test)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial_println!( "[failed]");
    serial_print!("\n{}\n", _info);
    exit_qemu(QemuExitCode::Failed);
    serial_print!("\n");
    loop {}
}


pub fn _test_runner(tests: &[&dyn Fn()]) {
    for (i, test) in tests.iter().enumerate() {
        serial_print!("Running test {:0x}...", i);
        test();
        serial_println!(" [pass]");
    }
    exit_qemu(QemuExitCode::Success);
}




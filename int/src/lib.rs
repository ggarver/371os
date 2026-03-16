#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(_test_runner)]

pub mod vga;
pub mod serial;


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


fn _t0(){
    // test simple assertion 
    println!("test_1");
    print!("assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}

// should fail 
fn _t1(){
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


#[cfg(test)]
fn _test_runner(_tests: &[&dyn Fn()]) {
    let fs = [_t0, _t1, test_println]; 
    for i in 0..fs.len() {
        serial_print!("Running test case {:0x}...", i);
        fs[i]();
        serial_println!(" [pass]");
    }
    unsafe { x86_64::instructions::port::Port::new(0xf4).write(0xAu32) }
}




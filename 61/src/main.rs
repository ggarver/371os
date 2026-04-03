#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga;
mod serial;

// make exit codes for Qemu 
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe{
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_runner(&[]);

    loop {}
}

// Panic when not test 
#[cfg(not(test))]
#[panic_handler]
#[allow(unconditional_recursion)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("panic! at the panic handler");
    loop {}
    // panic(info)
}

// Panic when test 
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!( "[failed]");
    serial_print!("\n{}\n", info);
    exit_qemu(QemuExitCode::Failed);
    serial_print!("\n");
    loop {}
}


fn t0(){
    // test simple assertion 
    println!("test_1");
    print!("assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}

// should fail 
fn t1(){
    assert_eq!(1,2);
}

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    let fs = [t0, t1]; 
    for i in 0..fs.len() {
        serial_print!("Running test case {:0x}...", i);
        fs[i]();
        serial_println!(" [pass]");
    }
    unsafe { x86_64::instructions::port::Port::new(0xf4).write(0xAu32) }
}



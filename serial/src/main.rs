#![no_std]
#![no_main]

mod vga;
mod serial;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    use core::fmt::Write;
    //let mut d = serial::Dummy { };
    //let _ = write!(d, "Hello {}!", "world");
    // use core::fmt::Write;
    // let mut d = vga::Dummy { };
    // let _ = write!(d, "Hello {}!", "world");
    // vga::str_to_vga("Hello, world!");
    loop {}
}

#[panic_handler]
#[allow(unconditional_recursion)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("panic! at the panic handler");
    loop {}
    // panic(info)
}

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    let fs = [_ex, _ex, _ex]; // _ex are supposed to be written tests 
    for i in 0..fs.len() {
        serial_print!("Running test case {:0x}...", i);
        fs[i]();
        serial_println!("Success!");
    }
    unsafe { x86_64::instructions::port::Port::new(0xf4).write(0xAu32) }
}

fn test_serial_print(){
    serial_print("print");
}









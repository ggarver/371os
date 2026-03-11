#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga;
mod serial;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_runner(&[]);

    loop {}
}

#[panic_handler]
#[allow(unconditional_recursion)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("panic! at the panic handler");
    loop {}
    // panic(info)
}

fn t0(){
    // test simple assertion 
    println!("test_1");
    print!("assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}

fn t1(){
    print!("test");
}

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    let fs = [t0, t1]; // _ex are supposed to be written tests 
    for i in 0..fs.len() {
        serial_print!("Running test case {:0x}...", i);
        fs[i]();
        serial_println!(" [pass]");
    }
    unsafe { x86_64::instructions::port::Port::new(0xf4).write(0xAu32) }
}



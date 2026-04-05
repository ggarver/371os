#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]

use osirs::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();
    println!("Hello World{}", "!");
    // println needs to be called before this line
    unsafe { *(0xdeadbeef as *mut u8) = 42; };
    // need to add handler for int3

    #[cfg(test)]
    osirs::qemu_quit(osirs::QemuExitCode::Success);
    
    loop {
    }
}



// call test panic handler 
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    osirs::test_panic_handler(info)
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


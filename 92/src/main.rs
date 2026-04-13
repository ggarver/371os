#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]

use osirs::println;
use osirs::clock;


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();

    // println!("Enter the current time to start the clock:");
    x86_64::instructions::interrupts::enable();

    // Ask user for starting time
    println!("Enter start time as HH MM SS (e.g. 13 45 00):");
    // let (h, m, s) = read_time_from_keyboard(); // your existing keyboard input

    
    loop {
        x86_64::instructions::hlt();
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


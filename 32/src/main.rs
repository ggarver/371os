#![no_main]
#![no_std]

use core::panic::PanicInfo;
mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}","!");
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#![no_std]
#![no_main]

mod colors;
mod vga;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    colors::colors();
    // println!("Hello World{}", "!");
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

#![no_std]
#![no_main]
#[unsafe(no_mangle)]

#[allow(unconditional_recursion)]

pub extern "C" fn _start() -> ! { _start() }

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic(info)
}

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();
    x86_64::instructions::interrupts::int3();
    // breakpoint handler returns, execution resumes here
    // osirs::serial_println!("[ok] breakpoint exception returned");
    osirs::qemu_quit(osirs::QEMU_PASS);  // ← actually exit
    loop {}
}

#[panic_handler]
fn test_panic(info: &core::panic::PanicInfo) -> ! {
    osirs::serial_println!("[failed]");
    osirs::qemu_quit(osirs::QEMU_FAIL);  // panic = failure
    loop {}
}

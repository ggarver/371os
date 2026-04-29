#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();

    #[cfg(test)]
    test_main();

    osirs::qemu_quit(osirs::QemuExitCode::Failed);
    loop {}
}

#[test_case]
fn except_test(){
    x86_64::instructions::interrupts::int3();
}

#[panic_handler]
fn test_panic(info: &core::panic::PanicInfo) -> ! {
    osirs::test_panic_handler(info)
}

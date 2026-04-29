#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use osirs::{serial_print, serial_println, qemu_quit, QemuExitCode};

#[panic_handler]
fn test_panic(_info: &core::panic::PanicInfo) -> ! {
    serial_println!("[pass]");
    qemu_quit(QemuExitCode::Success);
    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for (i, test) in tests.iter().enumerate() {
        serial_print!("Running test {:0x}...", i);
        test();
        serial_println!("[fail] expected panic but none occurred");
        qemu_quit(QemuExitCode::Failed);
    }
    qemu_quit(QemuExitCode::Failed);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();

    #[cfg(test)]
    test_main();

    qemu_quit(QemuExitCode::Failed);
    loop {}
}

fn bad() {
    unsafe { *(0xdeadbeef as *mut u8) = 42; }
}

#[test_case]
fn test_bad() {
    bad();
}

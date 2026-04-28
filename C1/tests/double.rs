#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]
#![feature(abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]

use x86_64::structures::idt::InterruptStackFrame;

#[panic_handler]
fn test_panic(info: &core::panic::PanicInfo) -> ! {
    // osirs::serial_println!("{}", info);
    osirs::serial_println!("[Pass]");
    osirs::qemu_quit(osirs::QemuExitCode::Success);
    loop {}
}

fn bad() {
    unsafe { *(0xdeadbeef as *mut u8) = 42; };
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();
    osirs::_test_runner(&[&bad]);
    osirs::qemu_quit(osirs::QemuExitCode::Failed);
    loop {}
}

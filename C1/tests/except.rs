#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::_test_runner(&[&except_test]);
    osirs::qemu_quit(osirs::QemuExitCode::Failed);
    loop {}
}

fn except_test(){
    x86_64::instructions::interrupts::int3();
    // breakpoint handler returns, execution resumes here
    osirs::serial_println!("[ok] breakpoint exception returned");
}

#[panic_handler]
fn test_panic(info: &core::panic::PanicInfo) -> ! {
    osirs::serial_println!("[failed]");
    osirs::qemu_quit(osirs::QemuExitCode::Failed);  // panic = failure
    loop {}
}

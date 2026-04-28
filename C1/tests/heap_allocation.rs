#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use osirs::println;

#[test_case]
fn simple_allocation() {
    let heap_value_1 = alloc::boxed::Box::new(41);
    let heap_value_2 = alloc::boxed::Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    osirs::init();
    // test_main();
    osirs::qemu_quit(osirs::QemuExitCode::Success);
    osirs::halt();
    
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    osirs::test_panic_handler(info)
}

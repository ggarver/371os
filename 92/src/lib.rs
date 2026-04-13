#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(_test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod vga;
pub mod serial;
pub mod interrupts;
pub mod gdt;
pub mod clock;

pub fn init(){
    gdt::init_gdt();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}


#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}


// make exit codes for Qemu 
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// exit code for qemu 
pub fn qemu_quit(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe{
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}



// clear to make tests easier 
pub fn _clear_vga() {
    for i in 0..2000 {
        unsafe {
            *((0xb8000 + i * 2) as *mut u8) = b' ';
        }
    }
    vga::_reset();
}


// Panic when test 
pub fn test_panic_handler(_info: &core::panic::PanicInfo) -> ! {
    serial_println!( "[failed]");
    serial_print!("\n{}\n", _info);
    qemu_quit(QemuExitCode::Failed);
    serial_print!("\n");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic_handler(info)
}


pub fn _test_runner(tests: &[&dyn Fn()]) {
    init();
    for (i, test) in tests.iter().enumerate() {
        serial_print!("Running test {:0x}...", i);
        test();
        serial_println!(" [pass]");
    }
    qemu_quit(QemuExitCode::Success);
}

pub fn halt() -> ! {
    loop { x86_64::instructions::hlt(); }
}


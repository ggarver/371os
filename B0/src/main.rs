#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]

use osirs::println;
use osirs::clock;
use osirs::memory;

#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    osirs::init();
    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { osirs::memory::init(offset) };
    let addresses = [0xb8000u64, 0x201008, boot_info.physical_memory_offset];
    for &address in &addresses {
        let virt = x86_64::VirtAddr::new(address);
        use x86_64::structures::paging::Translate;
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }
    println!("FIN");
    x86_64::instructions::interrupts::enable();
    println!("Enter start time as HH MM SS (e.g. 13 45 00):");
    println!("Level 4 page table at: {:?}", x86_64::registers::control::Cr3::read().0.start_address());
    loop {
        x86_64::instructions::hlt();
    }
}


// call test panic handler 
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    osirs::test_panic_handler(info)
}


// Panic when not test 
#[cfg(not(test))]
#[panic_handler]
#[allow(unconditional_recursion)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("panic! at the panic handler");
    loop {}
    // panic(info)
}


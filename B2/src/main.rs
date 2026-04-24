#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]

use osirs::println;
use osirs::clock;
use osirs::memory;
use x86_64::structures::paging::Page;
use x86_64::VirtAddr;

#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    osirs::init();

    x86_64::instructions::interrupts::enable();

    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { osirs::memory::init(offset) };
    let mut frame_allocator = osirs::memory::EmptyFrameAllocator;
    // let page: x86_64::structures::paging::Page<S> 
    //    = Page::containing_address(VirtAddr::new(0xdeadbeaf000));

    // map to unused page
    let page = x86_64::structures::paging::Page::containing_address(x86_64::VirtAddr::new(0));
    osirs::memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { ptr.write_volatile(0x_f021_f077_f065_f04e) };


    // println!("FIN");
    // println!("Enter start time as HH MM SS (e.g. 13 45 00):");
    // println!("Level 4 page table at: {:?}", x86_64::registers::control::Cr3::read().0.start_address());
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


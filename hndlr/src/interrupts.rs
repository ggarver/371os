use x86_64::structures::idt::InterruptDescriptorTable;
use core::ptr::addr_of_mut;
use core::mem::MaybeUninit;

static mut IDT: MaybeUninit<InterruptDescriptorTable> = MaybeUninit::uninit();

#[allow(static_mut_refs)]
pub fn init_idt() {
    unsafe {
        let idt = addr_of_mut!(IDT) as *mut InterruptDescriptorTable;
        idt.write(InterruptDescriptorTable::new());
        (*idt).breakpoint.set_handler_fn(breakpoint_handler);
        (*idt).load();
    }
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame)
{
    crate::println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

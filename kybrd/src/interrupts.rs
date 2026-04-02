use pic8259::ChainedPics;
use spin::{Mutex, Once};
use x86_64::structures::idt::InterruptDescriptorTable;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

static IDT: Once<InterruptDescriptorTable> = Once::new();

#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

pub fn init_idt() {
    IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt[InterruptIndex::Timer as usize].set_handler_fn(timer_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX as u16);
        }
        idt
    });
    IDT.get().unwrap().load();
    unsafe {
        PICS.lock().initialize();
        x86_64::instructions::interrupts::enable();
    }
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame)
{
    crate::println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
    error_code: u64,
) -> ! {
    assert!(error_code == 0);
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_handler(
    _stack_frame: x86_64::structures::idt::InterruptStackFrame)
{
    crate::println!("INTERRUPT: TIMER");
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8) };
}

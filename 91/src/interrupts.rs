use pic8259::ChainedPics;
use spin::{Mutex, Once};
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use crate::print;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

static IDT: Once<InterruptDescriptorTable> = Once::new();

// ---------------- interrupts ----------------------------
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub fn init_idt() {
    IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX as u16);
        }
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]  // <-- added
            .set_handler_fn(keyboard_interrupt_handler);
        idt
    });
    IDT.get().unwrap().load();
    unsafe {
        PICS.lock().initialize();
        x86_64::instructions::interrupts::enable();
    }
}
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    crate::println!("EXCEPTION: BREAKPOINT");
}
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    assert!(error_code == 0);
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    print!(".");
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8) };
}
extern "x86-interrupt" fn keyboard_interrupt_handler(  // <-- added
    _stack_frame: InterruptStackFrame)
{
    use x86_64::instructions::port::Port;
    
    crate::println!("KEY HIT");

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    let key = match scancode {
        0x02 => Some('1'), 0x03 => Some('2'), 0x04 => Some('3'),
        0x05 => Some('4'), 0x06 => Some('5'), 0x07 => Some('6'),
        0x08 => Some('7'), 0x09 => Some('8'), 0x0A => Some('9'),
        0x0B => Some('0'), 0x1E => Some('a'), 0x30 => Some('b'),
        0x2E => Some('c'), 0x20 => Some('d'), 0x12 => Some('e'),
        0x21 => Some('f'), 0x22 => Some('g'), 0x23 => Some('h'),
        0x17 => Some('i'), 0x24 => Some('j'), 0x25 => Some('k'),
        0x26 => Some('l'), 0x32 => Some('m'), 0x31 => Some('n'),
        0x18 => Some('o'), 0x19 => Some('p'), 0x10 => Some('q'),
        0x13 => Some('r'), 0x1F => Some('s'), 0x14 => Some('t'),
        0x16 => Some('u'), 0x2F => Some('v'), 0x11 => Some('w'),
        0x2D => Some('x'), 0x15 => Some('y'), 0x2C => Some('z'),
        0x39 => Some(' '), 0x1C => Some('\n'),
        _ => None,
    };

    if let Some(c) = key {
        print!("{}", c);
    }

    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8) };
}


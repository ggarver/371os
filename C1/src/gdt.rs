#![allow(static_mut_refs)]

use x86_64::structures::gdt::Descriptor;
use x86_64::registers::segmentation::CS;
use x86_64::registers::segmentation::SS;
use x86_64::instructions::tables::load_tss;
use core::sync::atomic::{AtomicBool, Ordering};

// GDT
static mut GDT: x86_64::structures::gdt::GlobalDescriptorTable =
    x86_64::structures::gdt::GlobalDescriptorTable::new();

// TSS
const STACK_SIZE: usize = 4096 * 5;
static mut TSS: x86_64::structures::tss::TaskStateSegment =
    x86_64::structures::tss::TaskStateSegment::new();
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;


static GDT_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn init_gdt() {
    if GDT_INITIALIZED.swap(true, Ordering::SeqCst) {
        return;
    }
    unsafe {
        TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] =
            x86_64::VirtAddr::from_ptr(&raw const STACK) + STACK_SIZE as u64;
        let kcs = GDT.add_entry(Descriptor::kernel_code_segment());
        let kds = GDT.add_entry(Descriptor::kernel_data_segment());
        let tss = GDT.add_entry(Descriptor::tss_segment(&TSS));
        GDT.load();
        use x86_64::instructions::segmentation::Segment;
        CS::set_reg(kcs);
        SS::set_reg(kds);
        load_tss(tss);
    }
}

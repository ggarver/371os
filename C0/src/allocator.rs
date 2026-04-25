pub struct Dummy;

unsafe impl alloc::alloc::GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: alloc::alloc::Layout) -> *mut u8 {
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: alloc::alloc::Layout) {
        panic!("dealloc should be never called")
    }
}

#[global_allocator]
static ALLOCATOR: Dummy = Dummy;

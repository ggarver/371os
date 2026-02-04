// Return an index in BUS of s reserved bytes

// Zero the array except the mask.
// reserve the beginning of the array for bitmask
fn init() {
    unsafe {
        // Initialize mask
        // The following explodes if SIZE isn't a power of 2
        assert!(SIZE & (SIZE - 1) == 0);
        // First SIZE >> 3 bits are reserved as a validty byte/bit mask
        < snip >
        // Which has to reserve enough bytes for itself.
        < snip >
        // Set to 1
        < snip >

        // Initialize memory
        // Set to zero.
        < snip >
    }
    return;
}




pub fn malloc(s: usize) -> Option<usize> {
    unsafe {
        // Ensure BUS is initialized.
        // SIZE is 1024 bits
        pub const SIZE:usize = 0x80;
        static mut BUS: [u8; SIZE] = [0u8; SIZE]; 

        // Reserve a block of s bytes
        < 4 lines snipped > 

        // Scan for a contigious region of size s
        // In s > 8, word level allocation
        // "Could be more efficient" it's an exercise!
        < ~20 lines snipped> 
    }
    return None;
}

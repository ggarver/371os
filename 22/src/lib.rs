// Return an index in BUS of s reserved bytes

// size is amnt of bytes 
pub const SIZE:usize = 0x80;
static mut BUS: [u8; SIZE] = [0u8;SIZE];
const res: usize = SIZE >> 3 >> 3;


// reserve the beginning of the array for bitmask
fn init() {
    unsafe {
        // Initialize mask - zero array
        // The following explodes if SIZE isn't a power of 2
        assert!(SIZE & (SIZE - 1) == 0);
        // First SIZE >> 3 bits are reserved as a validty byte/bit mask

        // if a bit is set to 1, corresponding byte is in use 
        // Which has to reserve enough bytes for itself.
        // Going from 0 to max reserved bytes
        for i in 0..res {
            BUS[i] = 0b11111111;
        }         
    }
    return;
}

fn search_space(s: usize, bindex: usize, zcount: usize) -> Option<usize> {
    let bydex = bindex / 8; // byte index 
    let local_bindex = bindex % 8;
    if bydex >= SIZE { // not enough space 
        return None;
    } 
    if (unsafe {BUS[bydex]} >> local_bindex) & 1 == 0 {
        if zcount + 1 == s { // if enough space
            return Some(bindex);
        } else {
            return search_space(s, bindex+1, zcount+1);
        } 
    } else { // found a 1 
        return search_space(s, bindex+1, 0);
    }


}

fn update_bmask(start: usize, end:usize) {
    if start == end {
        return;
    } else {
        let bydex = start / 8;
        let local_bindex = start % 8;
        unsafe {BUS[bydex] |= (1 << local_bindex);}
        update_bmask(start+1, end);
    }
}

// usize = # of bytes to allocate 
pub fn malloc(s: usize) -> Option<usize> {
    unsafe {
        // Ensure BUS is initialized.
        if BUS[0] != 0{
            init();
        }

        // find block, update bitmask
        let address = search_space(s, res, 0);
        match address {
            Some(bindex) => { 
                update_bmask(bindex, bindex+s);
                Some(bindex)
            }
            None => None
        }
    }
}



// Place val at loc
// No safety checks so good luck out there.
pub fn setter<T>(val: T, loc: usize) {
    unsafe {
        (&raw mut BUS).cast::<u8>().add(loc).cast::<T>().write(val);
    }
    return;
}

// Should check the validity bitmask search_spaceere...
pub fn getter<T>(loc: usize) -> T {
    unsafe {
        (&raw mut BUS).cast::<u8>().add(loc).cast::<T>().read()
    }
}








// snake implimentation lives here

pub fn snake() {
// let color_codes: (*u8)= [];
    let vga_buff = 0xb8000 as *mut u8;

    // 80 horizontal by 25 vert, 2 dimensions
    for row in 0..25_usize {
        for col in 0..80_usize {

            // 16 colors, this cycles through, int div
            let color: u8 = 0x9;
            // slide over, * 2 to byte address
            let offset = (row * 80 + col) * 2;

            unsafe {
                // 0xDB is solid block char 
                vga_buff.add(offset).write_volatile(0xDB);
                // color << 4 shift to background, 
                vga_buff.add(offset + 1).write_volatile((color << 4) | color);
            }
        }
    }
}

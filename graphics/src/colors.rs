pub fn colors() {
    // let colors: [u8; 16] = [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf];
    // 720 x 400 pix
    // display background colors to screendump.ppm 
    // let color_codes: (*u8)= [];
    let vga_buff = 0xb8000 as *mut u8;

    // 80 horizontal by 25 vert, 2 dimensions
    for row in 0..25_usize {
        for col in 0..80_usize {

            // 16 colors, this cycles through, int div
            let color: u8 = (col / 5) as u8;
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

pub fn image() {
    // let colors: [u8; 16] = [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf];
    // 720 x 400 pix
    // display background colors to screendump.ppm 
    // let color_codes: (*u8)= [];
    let vga_buff = 0xb8000 as *mut u8;

    // 80 horizontal by 25 vert, 2 dimensions
    for row in 0..25_usize {
        for col in 0..80_usize {

            // 16 colors, this cycles through, int div
            let color = crate::image::IMG[row * 80 + col] as u8;
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

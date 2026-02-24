pub fn colors() {
    // 720 x 400 pix
    // display background colors to screendump.ppm 
    // let color_codes: (*u8)= [];
    let vga_buff = 0xb8000 as *mut u8;

    // 80 horizontal by 25 vert 
    for i in 0..(80 * 25 * 2) {
        unsafe {
                // Write color byte
                vga_buff.offset(i).write_volatile((i as u8) | 5);
            }
        }
    }

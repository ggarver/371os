// snake implimentation lives here
use osirs::println;

const SNAKE: u8 = 0xDB;
struct Snake { length: usize }

impl Snake {
    pub fn init_snake(&self){
        println!("{}", SNAKE);
        // snake starts small
        self.length == 1;

        let color = 0x0f;
        // let offset = (row * 80 + col) * 2;

        let mid_offset = (12 * 80 + 37) * 2; 
        let buff_ptr = 0xb8000 as *mut u8;

        let mid_y = 12;
        let mid_x = 35;

        for row in 0..12_usize {
            for col in 0..35_usize {

                // 16 colors, this cycles through, int div
                let color: u8 = 0x9;
                // slide over, * 2 to byte address
                let offset = (row * 80 + col) * 2;

                unsafe {
                    // 0xDB is solid block char 
                    buff_ptr.add(offset).write_volatile(0xDB);
                    // color << 4 shift to background, 
                    buff_ptr.add(offset + 1).write_volatile((color << 4) | color);
                }
            }
        }
        buff_ptr.add( (12 * 80 + 37) * 2; 
).write_volatile(0xDB);
        // color << 4 shift to background, 
        buff_ptr.add( (12 * 80 + 37) * 2; 
 + 2).write_volatile((color << 4) | color);

    }
}



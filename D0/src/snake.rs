// snake implimentation lives here
use osirs::println;

const SNAKE: u8 = 0xDB;

pub struct Snake {
    pub length: usize,
}

impl Snake {
    pub fn init_snake(&mut self) {
        self.length = 1;
        let color: u8 = 0x09;
        let buff_ptr = 0xb8000 as *mut u8;

        // draw initial snake head at center (row 12, col 37)
        let offset = (12 * 80 + 37) * 2;
        unsafe {
            buff_ptr.add(offset).write_volatile(SNAKE);
            buff_ptr.add(offset + 1).write_volatile((color << 4) | color);
        }
    }
}

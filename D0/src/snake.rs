// snake implimentation lives here


const BODY: u8 = 0xDB;
const BUFF_PTR: *mut u8 = 0xb8000 as *mut u8;
const COL: u8 = 0x09;

static mut _SNAKE: Snake = Snake { length: 0, pos: (12 * 80 + 37) * 2 };


pub fn get_snake() -> &'static mut Snake {
    unsafe {
        (&raw mut _SNAKE as *mut Snake)
            .as_mut()
            .unwrap()
    }
}

#[derive(Default)]
pub struct Snake {
    pub length: usize,
    pub pos: usize
}

impl Snake {
    pub fn init_snake(&mut self) {
        self.length = 0;

        // draw initial snake head at center (row 12, col 37)
        let pos = (12 * 80 + 37) * 2;
        unsafe {
            BUFF_PTR.add(self.pos).write_volatile(BODY);
            BUFF_PTR.add(self.pos + 1).write_volatile((COL << 4) | COL);
        }

        *get_snake() = Snake::new(self.length, self.pos);
    }

    pub fn new(length: usize, pos:usize) -> Snake {
        Snake { length, pos }
    }

    pub fn right(&mut self){
        self.pos = self.pos + 2;

        unsafe {
            BUFF_PTR.add(self.pos).write_volatile(BODY);
            BUFF_PTR.add(self.pos + 1).write_volatile((COL << 4) | COL);
        }



    }
}

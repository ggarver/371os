// snake implimentation lives here

use crate::clock::get_timer;
use crate::clock::{INDEX, CHARS, Timer, TIMER_ACTIVE};
const BODY: u8 = 0xDB;
const BUFF_PTR: *mut u8 = 0xb8000 as *mut u8;
const COL: u8 = 0x09;

static mut _SNAKE: Snake = Snake { length: 0, pos: (12 * 80 + 37) * 2 };
use crate::println;

pub fn border(){
    // draw first and last horizontal lines, 
    for col in 1..79_usize {
        let top_line = (0 * 80 + col) * 2;
        let bot_line = (24 * 80 + col) * 2;

        unsafe {
            // perhaps I should make this its own function...
            BUFF_PTR.add(top_line).write_volatile(0xC4);
            BUFF_PTR.add(top_line + 1).write_volatile(0x0f);


            BUFF_PTR.add(bot_line).write_volatile(0xC4);
            BUFF_PTR.add(bot_line + 1).write_volatile(0x0f);
        }
    }

    // R and L lines 
    for row in 1..24_usize {
        let r_line = (row * 80 + 79) * 2;
        let l_line = (row * 80 + 0) * 2;

        unsafe {
            BUFF_PTR.add(r_line).write_volatile(0xB3);
            BUFF_PTR.add(r_line + 1).write_volatile(0x0f);

            BUFF_PTR.add(l_line).write_volatile(0xB3);
            BUFF_PTR.add(l_line + 1).write_volatile(0x0f);
        }

    }
    // corners 
    unsafe {
        let tl_c = (0 * 80 + 0) * 2;
        BUFF_PTR.add(tl_c ).write_volatile(0xDA);
        BUFF_PTR.add(tl_c + 1).write_volatile(0x0f);
        let bl_c = (24 * 80 + 0) * 2;
        BUFF_PTR.add(bl_c ).write_volatile(0xC0);
        BUFF_PTR.add(bl_c + 1).write_volatile(0x0f);
        let tr_c =(0 * 80 + 79) * 2;
        BUFF_PTR.add(tr_c ).write_volatile(0xBF);
        BUFF_PTR.add(tr_c + 1).write_volatile(0x0f);
        let br_c =(24 * 80 + 79) * 2;
        BUFF_PTR.add(br_c ).write_volatile(0xD9);
        BUFF_PTR.add(br_c + 1).write_volatile(0x0f);

    }
}

pub fn food(){
        // I am so confused why nothing is showing up 
        let foodspot = (10 * 80 + 10) * 2;

        unsafe {
            BUFF_PTR.add(foodspot).write_volatile(0x03);
            BUFF_PTR.add(foodspot + 1).write_volatile(0x04);

        }

    }

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
        border();
        food();
        self.length = 1;

        // draw initial snake head at center (row 12, col 37)
        let pos = (12 * 80 + 37) * 2;
        unsafe {
            BUFF_PTR.add(pos).write_volatile(BODY);
            BUFF_PTR.add(pos + 1).write_volatile(COL);
        }


        *get_snake() = Snake::new(self.length, self.pos);
    }

    pub fn new(length: usize, pos:usize) -> Snake {
        Snake { length, pos }
    }


    pub fn right(&mut self){
        unsafe {
            if BUFF_PTR.add(self.pos + 2).read_volatile() == 0xB3 {
                panic!();
            }
        }

        self.pos = self.pos + 2;
        unsafe {
            BUFF_PTR.add(self.pos).write_volatile(BODY);
            BUFF_PTR.add(self.pos + self.length).write_volatile(COL);

            // erase to left
            BUFF_PTR.add(self.pos - self.length).write_volatile(0x0);
        }
    }

    pub fn left(&mut self){
        unsafe {
            if BUFF_PTR.add(self.pos - 2).read_volatile() == 0xB3 {
                panic!();
            }
        }

        self.pos = self.pos - 2;
        unsafe {
            BUFF_PTR.add(self.pos).write_volatile(BODY);
            BUFF_PTR.add(self.pos + self.length).write_volatile(COL);

            //erase right
            BUFF_PTR.add((self.pos + 2) + self.length).write_volatile(0x0);
        }
    }

    pub fn up(&mut self){
        unsafe {
            if BUFF_PTR.add(self.pos - 160).read_volatile() == 0xC4 {
                panic!();
            }
        }
        self.pos = self.pos - 160;
        unsafe {
            BUFF_PTR.add(self.pos).write_volatile(BODY);
            BUFF_PTR.add(self.pos + 1).write_volatile(COL);

            //erase 
            BUFF_PTR.add(self.pos + 160 + self.length).write_volatile(0x0);
        }
    }

    pub fn down(&mut self){
        unsafe {
            if BUFF_PTR.add(self.pos + 160).read_volatile() == 0xC4 {
                panic!();
            }
        }

        self.pos = self.pos + 160;
        unsafe {
            BUFF_PTR.add(self.pos).write_volatile(BODY);
            BUFF_PTR.add(self.pos + 1).write_volatile(COL);

            //erase
            BUFF_PTR.add(self.pos + self.length - 160).write_volatile(0x0);
        }

    }


}



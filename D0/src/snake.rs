// snake implimentation lives here

use crate::clock::get_timer;
use crate::clock::{INDEX, CHARS, Timer, TIMER_ACTIVE};
const BODY: u8 = 0x0A;
const FOOD: u8 = 0x03;
const BUFF_PTR: *mut u8 = 0xb8000 as *mut u8;
const COL: u8 = 0x09;
const MAX_LEN: usize = 2000;

static mut PSEUDO_RAND: usize = 1;

pub fn scramble(input: u8) {
    unsafe {
        PSEUDO_RAND = PSEUDO_RAND
            .wrapping_mul(31)
            .wrapping_add(input as usize)
            .wrapping_mul(1000003);
        }
}


pub const START_POS: usize = (12 * 80 + 37) * 2;

static mut _SNAKE: Snake = Snake { 
    length: 0, 
    pos: START_POS,
    body: [START_POS; MAX_LEN],
    head: 0,
};

use crate::println;


pub fn write_vga(coord: usize, character: u8, color: u8) {
    unsafe {
        BUFF_PTR.add(coord).write_volatile(character);
        BUFF_PTR.add(coord + 1).write_volatile(color);
    }
}

pub fn border(){
    // Top and bottom lines
    for col in 1..79_usize {
        write_vga((0 * 80 + col) * 2, 0xC4, 0x0f);
        write_vga((24 * 80 + col) * 2, 0xC4, 0x0f);
    }
    // R and L lines 
    for row in 1..24_usize {
        write_vga((row * 80 + 79) * 2, 0xB3, 0x0f);
        write_vga((row * 80 + 0) * 2, 0xB3, 0x0f);
    }
    // corners 
    write_vga((0 * 80 + 0) * 2, 0xDA, 0x0f);
    write_vga((24 * 80 + 0) * 2, 0xC0, 0x0f);
    write_vga((0 * 80 + 79) * 2, 0xBF, 0x0f);
    write_vga((24 * 80 + 79) * 2, 0xD9, 0x0f);
}


pub fn get_snake() -> &'static mut Snake {
    unsafe {
        (&raw mut _SNAKE as *mut Snake)
            .as_mut()
            .unwrap()
    }
}

pub struct Snake {
    pub length: usize,
    pub pos: usize,
    pub body: [usize; MAX_LEN],
    pub head: usize,
}

impl Snake {
    pub fn init_snake(&mut self) {
        border();
        self.place_food();
        self.length = 1;

        // draw initial snake head at center (row 12, col 37)
        let pos = (12 * 80 + 37) * 2;
        self.body = [pos; MAX_LEN];
        self.head = 0;
        write_vga(pos, BODY, COL);
        *get_snake() = Snake::new(self.length, self.pos );
    }

    pub fn new(length: usize, pos:usize ) -> Snake {
        Snake { length, pos,
        body: [pos; MAX_LEN],
        head: 0,
        }
    }

    fn move_to(&mut self, next_pos: usize) {
        // erase actual tail, add max to prevent negative
        let tail_idx = (self.head + MAX_LEN + 1 - self.length) % MAX_LEN;
        write_vga(self.body[tail_idx], b' ', 0x00);

        // store new head
        self.head = (self.head + 1) % MAX_LEN;
        self.body[self.head] = next_pos;
        self.pos = next_pos;

        write_vga(self.pos, BODY, COL);
    }

    pub fn right(&mut self){
        let next_pos = self.pos + 2;
        unsafe { 
            if BUFF_PTR.add(next_pos).read_volatile() == FOOD {
                self.length += 1;
                self.place_food();
            }

            if BUFF_PTR.add(next_pos).read_volatile() == 0xB3 {
                panic!();
            }
        }

    self.move_to(next_pos);
    }


    pub fn left(&mut self){
        let next_pos = self.pos - 2;
        unsafe {
            if BUFF_PTR.add(next_pos).read_volatile() == FOOD {
                self.length += 1;
                self.place_food();
            }

            if BUFF_PTR.add(next_pos).read_volatile() == 0xB3 {
                panic!();
            }
        }
        self.move_to(next_pos);
    }

    pub fn up(&mut self){
        let next_pos = self.pos - 160;
        unsafe {
            if BUFF_PTR.add(next_pos).read_volatile() == FOOD {
                self.length += 1;
                self.place_food();
            }

            if BUFF_PTR.add(next_pos).read_volatile() == 0xC4 {
                panic!();
            }
        }
        self.move_to(next_pos);

    }

    pub fn down(&mut self) {
        let next_pos = self.pos + 160;

        unsafe {
            if BUFF_PTR.add(next_pos).read_volatile() == FOOD {
                self.length += 1;
                self.place_food();
            }
            if BUFF_PTR.add(next_pos).read_volatile() == 0xC4 {
                panic!();
            }
            self.move_to(next_pos);
        }
    }

    pub fn place_food(&mut self){
        unsafe {
            PSEUDO_RAND = PSEUDO_RAND.wrapping_mul(6700417).wrapping_add(179426549);
            let row = (PSEUDO_RAND % 23) + 1;
            let col = (PSEUDO_RAND.wrapping_mul(1299709) % 78) + 1;
            let pos = (row * 80 + col) * 2;
            write_vga(pos, FOOD, 0x04);
        }
    }
}


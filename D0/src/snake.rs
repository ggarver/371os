// snake implimentation lives here

use crate::clock::get_timer;
use crate::clock::{INDEX, CHARS, Timer, TIMER_ACTIVE};
const BODY: u8 = 0xDB;
const FOOD: u8 = 0x03;
const BUFF_PTR: *mut u8 = 0xb8000 as *mut u8;
const COL: u8 = 0x09;
const MAX_LEN: usize = 100;


static mut _SNAKE: Snake = Snake { length: 0, pos: (12 * 80 + 37) * 2 };
use crate::println;


pub fn write_vga(coord: usize, character: u8, color: u8) {
    unsafe {
        BUFF_PTR.add(coord).write_volatile(character);
        BUFF_PTR.add(coord + 1).write_volatile(color);
    }}

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
}

impl Snake {
    pub fn init_snake(&mut self) {
        border();
        self.food();
        self.length = 1;

        // draw initial snake head at center (row 12, col 37)
        let pos = (12 * 80 + 37) * 2;
        write_vga(pos, BODY, COL);
        *get_snake() = Snake::new(self.length, self.pos);
    }

    pub fn new(length: usize, pos:usize) -> Snake {
        Snake { length, pos }
    }

    pub fn right(&mut self){
        unsafe { 
            if BUFF_PTR.add(self.pos + 2).read_volatile() == FOOD {
                self.length += 1
            }

            if BUFF_PTR.add(self.pos + 2).read_volatile() == 0xB3 {
                panic!();
            }
        }

        self.pos = self.pos + 2;
        write_vga(self.pos, BODY, COL);
        unsafe {
            // this only erases left 
            BUFF_PTR.add((self.pos - 1) - self.length).write_volatile(0x0);
        }
    }

    pub fn left(&mut self){
        unsafe {
            if BUFF_PTR.add(self.pos - 2).read_volatile() == FOOD {
                self.length += 1;
            }

            if BUFF_PTR.add(self.pos - 2).read_volatile() == 0xB3 {
                panic!();
            }
        }

        self.pos = self.pos - 2;
        write_vga(self.pos, BODY, COL);
        unsafe {
            //erase right
            BUFF_PTR.add((self.pos + 2) + self.length).write_volatile(0x0);
        }
    }

    pub fn up(&mut self){
        unsafe {
            if BUFF_PTR.add(self.pos - 160).read_volatile() == FOOD {
                self.length += 1;
            }

            if BUFF_PTR.add(self.pos - 160).read_volatile() == 0xC4 {
                panic!();
            }
        }

        self.pos = self.pos - 160;
        write_vga(self.pos, BODY, COL);
        unsafe {
            BUFF_PTR.add(self.pos + (160 * self.length)).write_volatile(0x0);
        }
    }

    pub fn down(&mut self) {
        let next_pos = self.pos + 160;

        unsafe {
            if BUFF_PTR.add(next_pos).read_volatile() == FOOD {
                self.length += 1;
            }
            if BUFF_PTR.add(next_pos).read_volatile() == 0xC4 {
                panic!();
            }
        }

        self.pos = next_pos;
        write_vga(self.pos, BODY, COL);
        unsafe {
            BUFF_PTR.add(self.pos - (self.length * 160)).write_volatile(0x0);
        }
    }

    pub fn food(&mut self){
        let foodspot = (10 * 80 + 30) * 2;
        write_vga(foodspot, FOOD, 0x04);

    }
}



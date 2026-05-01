use core::fmt::{self, Display, Formatter};


pub static mut TIMER_ACTIVE: bool = false;
static mut _TIMER: Timer = Timer { hrs: 0, min: 0, sec: 0 };

pub fn get_timer() -> &'static mut Timer {
    unsafe {
        (&raw mut _TIMER as *mut Timer)
            .as_mut()
            .unwrap()
    }
}


pub struct Timer {
    hrs: u8,
    min: u8,
    sec: u8
}

pub static mut CHARS: [u8; 6] = [0; 6];
pub static mut INDEX: usize = 0;

impl Timer {
    pub fn init_timer() {
        unsafe {
            let hrs: u8 = 0;
            let min: u8 = 0;
            let sec: u8 = 0;

            *get_timer() = Timer::new(hrs, min, sec);
            TIMER_ACTIVE = true;
        }
        
    }
    pub fn new(hrs: u8, min:u8, sec:u8) -> Timer {
        Timer { hrs, min, sec }
    }
    pub fn tick(&mut self) {
        let Timer { hrs, min, sec } = *self;

        let inc_sec = sec + 1;
        let new_sec = (inc_sec) % 60;

        let inc_min = min +  (inc_sec/ 60);
        let new_min = inc_min % 60;

        let new_hrs = (hrs + (inc_min / 60)) % 24;

        *self = Timer { hrs: new_hrs, min: new_min, sec: new_sec };

    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Timer { hrs, min, sec } = *self;
        write!(f, "{hrs}:{min}:{sec}")
    }
}

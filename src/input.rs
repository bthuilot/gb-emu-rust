pub(crate) type Button = u8;
use crate::bit_functions::{test};

const A: button = 0;
const B: button = 1;
const SELECT: button = 2;
const START: button = 3;
const RIGHT: button = 4;
const LEFT: button = 5;
const UP: button = 6;
const DOWN: button = 7;

pub struct Input {
    pub mask: u8,
}

impl Input {
    pub fn joypad_value(&mut self, current: u8) -> u8 {
        let mut i: u8 = 0xF;
        if test(current, 4) {
            i = self.mask & 0xF;
        }else {
            i = self.mask.wrapping_shr(4) & 0xF
        }
        return current | 0xC0 | i
    }
}





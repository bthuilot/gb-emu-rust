pub(crate) type Button = u8;
use crate::bit_functions::{test};

const A: Button = 0;
const B: Button = 1;
const SELECT: Button = 2;
const START: Button = 3;
const RIGHT: Button = 4;
const LEFT: Button = 5;
const UP: Button = 6;
const DOWN: Button = 7;

pub struct Input {
    pub mask: u8,
}

impl Input {
    pub fn joypad_value(&self, current: u8) -> u8 {
        let mut i: u8 = 0xF;
        if test(current, 4) {
            i = self.mask & 0xF;
        }else {
            i = self.mask.wrapping_shr(4) & 0xF
        }
        return current | 0xC0 | i
    }
}





pub(crate) type Button = u8;
use crate::bit_functions::{test};

pub const A: Button = 0;
pub const B: Button = 1;
pub const SELECT: Button = 2;
pub const START: Button = 3;
pub const RIGHT: Button = 4;
pub const LEFT: Button = 5;
pub const UP: Button = 6;
pub const DOWN: Button = 7;

pub struct Input {
    pub mask: u8,
}

impl Input {
    pub fn joypad_value(&self, current: u8) -> u8 {
        let mut i: u8 = 0xF;
        if test(current, 4) {
            i = self.mask & 0xF;
        }else if test(current, 5){
            i = (self.mask >> 4) & 0xF
        }
        return current | 0xC0 | i
    }
}





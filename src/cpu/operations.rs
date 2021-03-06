use crate::bit_functions::{b, half_carry_add, reset, set};
use crate::cpu::Z80;

impl Z80 {

    pub fn init(&mut self, cgb: bool) {
        self.af.set_full(if cgb {0x1180} else {0x1B0});
        self.af.mask = 0xFFF0;
        self.bc.set_full(0x0000);
        self.de.set_full(0xFF56);
        self.hl.set_full(0x000D);

        self.pc = 0x100;
        self.sp.set_full(0xFFFE);
    }

    pub fn set_flag(&mut self, index: u8, on: bool) {
        if on {
            let val = set(self.af.lo(), index);
            self.af.set_lo(val)
        } else {
            let val = reset(self.af.lo(), index);
            self.af.set_lo(val);
        }
    }
    // SetZ sets the value of the Z flag.
    pub fn set_z(&mut self, on: bool) {
        self.set_flag(7, on)
    }

    // SetN sets the value of the N flag.
    pub fn set_n(&mut self, on: bool) {
        self.set_flag(6, on)
    }

    // SetH sets the value of the H flag.
    pub fn set_h(&mut self, on: bool) {
        self.set_flag(5, on)
    }

    // SetC sets the value of the C flag.
    pub fn set_c(&mut self, on: bool) {
        self.set_flag(4, on)
    }

    pub fn set_flags(&mut self, c: bool, h: bool, n: bool, z: bool) {
        self.set_z(z);
        self.set_n(n);
        self.set_h(h);
        self.set_c(c);
    }

    // Z gets the value of the Z flag.
    pub fn z(&mut self) -> bool {
        return (self.af.full() >> 7) & 1 == 1;
    }

    // N gets the value of the N flag.
    pub fn n(&mut self) -> bool {
        return (self.af.full() >> 6) & 1 == 1;
    }

    // H gets the value of the H flag.
    pub fn h(&mut self) -> bool {
        return (self.af.full() >> 5) & 1 == 1;
    }

    // C gets the value of the C flag.
    pub fn c(&mut self) -> bool {
        return (self.af.full() >> 4) & 1 == 1;
    }

    pub fn add(&mut self, reg: &str, high: bool, val1: u8, val2: u8, carry: bool) {
        let carry_bit = b(self.c() && carry) as u16;
        let result = (val1 as u16)
            .wrapping_add(val2 as u16)
            .wrapping_add(carry_bit);
        let result_u8 = result as u8;
        if high {
            self.set_hi(reg, result_u8);
        } else {
            self.set_lo(reg, result_u8);
        }
        self.set_z(result_u8 == 0);
        self.set_n(false);
        self.set_h(
            (val2 & 0xF)
                .wrapping_add(val1 & 0xF)
                .wrapping_add(carry_bit as u8)
                > 0xF,
        );
        self.set_c(result > 0xFF);
    }

    pub fn sub(&mut self, reg: &str, high: bool, val1: u8, val2: u8, carry: bool) {
        let carry_bit = b(self.c() && carry) as i16;
        let result = (val1 as i16)
            .wrapping_sub(val2 as i16)
            .wrapping_sub(carry_bit);
        let result_u8 = result as u8;
        if high {
            self.set_hi(reg, result_u8);
        } else {
            self.set_lo(reg, result_u8);
        }
        self.set_z(result_u8 == 0);
        self.set_n(true);
        self.set_h(
            ((val1 & 0x0F) as i16)
                .wrapping_sub((val2 & 0xF) as i16)
                .wrapping_sub(carry_bit)
                < 0,
        );
        self.set_c(result < 0);
    }

    pub fn and(&mut self, reg: &str, high: bool, val1: u8, val2: u8) {
        let result = val1 & val2;
        if high {
            self.set_hi(reg, result);
        } else {
            self.set_lo(reg, result);
        }
        self.set_z(result == 0);
        self.set_n(false);
        self.set_h(true);
        self.set_c(false);
    }

    pub fn or(&mut self, reg: &str, high: bool, val1: u8, val2: u8) {
        let result = val1 | val2;
        if high {
            self.set_hi(reg, result);
        } else {
            self.set_lo(reg, result);
        }
        self.set_z(result == 0);
        self.set_n(false);
        self.set_h(false);
        self.set_c(false);
    }

    pub fn set_hi(&mut self, reg: &str, val: u8) {
        match reg {
            "af" => {
                self.af.set_hi(val);
            }
            "bc" => {
                self.bc.set_hi(val);
            }
            "de" => {
                self.de.set_hi(val);
            }
            "hl" => {
                self.hl.set_hi(val);
            }
            _ => {}
        }
    }

    pub fn set_lo(&mut self, reg: &str, val: u8) {
        match reg {
            "af" => {
                self.af.set_lo(val);
            }
            "bc" => {
                self.bc.set_lo(val);
            }
            "de" => {
                self.de.set_lo(val);
            }
            "hl" => {
                self.hl.set_lo(val);
            }
            _ => {}
        }
    }

    pub fn set(&mut self, reg: &str, val: u16) {
        match reg {
            "af" => {
                self.af.set_full(val);
            }
            "bc" => {
                self.bc.set_full(val);
            }
            "de" => {
                self.de.set_full(val);
            }
            "hl" => {
                self.hl.set_full(val);
            }
            _ => {}
        }
    }

    pub fn xor(&mut self, reg: &str, set_hi: bool, val1: u8, val2: u8) {
        let result = val1 ^ val2;
        if set_hi {
            self.set_hi(reg, result);
        } else {
            self.set_lo(reg, result);
        }
        self.set_z(result == 0);
        self.set_n(false);
        self.set_h(false);
        self.set_c(false);
    }

    pub fn cp(&mut self, val1: u8, val2: u8) {
        let result = val2.wrapping_sub(val1);
        self.set_z(result == 0);
        self.set_n(true);
        self.set_h((val1 & 0x0F) > (val2 & 0x0F));
        self.set_c(val1 > val2);
    }

    pub fn inc(&mut self, reg: &str, set_hi: bool, byte: u8) {
        let result = byte.wrapping_add(1);
        if set_hi {
            self.set_hi(reg, result);
        } else {
            self.set_lo(reg, result);
        }
        self.set_z(result == 0);
        self.set_n(false);
        self.set_h(half_carry_add(byte, 1));
    }

    pub fn dec(&mut self, reg: &str, set_hi: bool, byte: u8) {
        let result = byte.wrapping_sub(1);
        if set_hi {
            self.set_hi(reg, result);
        } else {
            self.set_lo(reg, result);
        }
        self.set_z(result == 0);
        self.set_n(true);
        self.set_h(byte & 0x0f == 0);
    }

    pub fn add_16(&mut self, reg: &str, val1: u16, val2: u16) {
        let result = (val1 as i32).wrapping_add(val2 as i32);
        self.set(reg, result as u16);
        self.set_n(false);
        self.set_h((val1 as i32) & 0xFFF > (result & 0xFFF));
        self.set_c(result > 0xFFFF);
    }

    pub fn add_16_signed(&mut self, reg: &str, val1: u16, val2: i8) {
        let result = (val1 as i32).wrapping_add(val2 as i32) as u16;
        self.set(reg, result);
        let tmp = val1 ^ (val2 as u16) ^ result;
        self.set_z(false);
        self.set_n(false);
        self.set_h((tmp & 0x10) == 0x10);
        self.set_c((tmp & 0x100) == 0x100);
    }

    pub fn inc_16(&mut self, byte: u16) -> u16 {
        return byte.wrapping_add(1);
    }

    pub fn dec_16(&mut self, byte: u16) -> u16 {
        return byte.wrapping_sub(1);
    }

    pub fn jump(&mut self, next: u16) {
        self.pc = next;
    }
}

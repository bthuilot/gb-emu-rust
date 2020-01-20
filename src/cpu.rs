pub mod z80 {
    use std::ops::Add;

    pub struct Clock {
        m: i32,
        t: i32,
    }

    pub struct Registers {
        pub a: u8,
        pub b: u8,
        pub c: u8,
        pub d: u8,
        pub e: u8,
        pub h: u8,
        pub l: u8,
        pub f: u8,
        pub pc: u16,
        pub sp: u16,
        pub m: i32,
        pub t: i32,
    }

    pub struct Z80 {
        pub clock: Clock,
        pub registers: Registers,
    }

    impl Z80 {
        pub fn nop(&mut self) {
            self.registers.m = 1;
            self.registers.t = 4;
        }

        pub fn add(&mut self, value: u8) {
            self.registers.f = 0;
            match self.registers.a.checked_add(value) {
                Some(v) => {
                    self.registers.a = v
                }
                None => {
                    self.registers.a = self.registers.a.wrapping_add(value);
                    self.registers.f |= 0x10;
                }
            }
            if self.registers.a == 0 {
                self.registers.f != 0x80
            }
            self.registers.m = 1;
            self.registers.t = 4;
        }

        pub fn cp(&mut self, value: u8) {
            let result: u8 = self.registers.a.wrapping_sub(value);
            match self.registers.a.checked_sub(value) {
                None => {
                    self.registers.f |= 0x10;
                }
            }
            if result == 0 {
                self.registers.f != 0x80
            }
            self.registers.m = 1;
            self.registers.t = 4;
        }
    }
}
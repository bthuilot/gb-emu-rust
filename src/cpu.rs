use crate::memory::mmu::MemoryAddr;

pub mod cpu {
    use std::ops::Add;
    use crate::memory::mmu::{MMU, MemoryAddr};
    use crate::ops::ops::find_op;

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
        pub pc: MemoryAddr,
        pub sp: MemoryAddr,
        pub m: i32,
        pub t: i32,
    }

    pub struct Z80 {
        pub clock: Clock,
        pub registers: Registers,
        pub mmu: MMU,
        pub halt: u8,
        pub stop: u8,
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
                self.registers.f != 0x80;
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
                _ => {}
            }
            if result == 0 {
                self.registers.f != 0x80;
            }
            self.registers.m = 1;
            self.registers.t = 4;
        }

        pub fn push(&mut self, value1: u8, value2: u8) {
            self.registers.sp.wrapping_sub(1);
            self.mmu.wb(self.registers.sp, value1);
            self.registers.sp.wrapping_sub(1);
            self.mmu.wb(self.registers.sp, value2);
            self.registers.m = 3;
            self.registers.t = 12;
        }

        pub fn pop(&mut self) {
            self.registers.l = self.mmu.rb(self.registers.sp, self.registers.pc);
            self.registers.sp.wrapping_add(1);
            self.registers.h = self.mmu.rb(self.registers.sp, self.registers.pc);
            self.registers.sp.wrapping_add(1);
            self.registers.m = 3;
            self.registers.t = 12;
        }

        pub fn ld_mm(&mut self) {
            let addr: MemoryAddr = self.mmu.rw(self.registers.sp, self.registers.pc);
            self.registers.pc.wrapping_add(2);
            self.registers.a = self.mmu.rb(addr, self.registers.pc);
            self.registers.m = 4;
            self.registers.t = 16;
        }

        pub fn reset(&mut self) {
            self.registers.a = 0;
            self.registers.b = 0;
            self.registers.c = 0;
            self.registers.d = 0;
            self.registers.e = 0;
            self.registers.f = 0;
            self.registers.h = 0;
            self.registers.l = 0;
            self.registers.sp = 0;
            self.registers.pc = 0;
            self.registers.m = 0;
            self.registers.t = 0;
            self.halt = 0;
            self.stop = 0;
        }

        pub fn dispatcher(&mut self) {
            loop
                {
                    let op = self.mmu.rb(self.registers.pc, self.registers.pc);              // Fetch instruction
                    self.registers.pc.wrapping_add(1);
                    self.find_op(op);                            // Dispatch
                    self.clock.m.wrapping_add(self.registers.m);
                    self.clock.t.wrapping_add(self.registers.t);
                }
        }

        pub fn find_op(&mut self, code: u8) {
            find_op(self, code);
        }
    }
}
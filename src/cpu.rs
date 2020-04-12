use std::ops::Add;
use crate::memory::{MMU, MemoryAddr};
use crate::ops::{find_op, OPCODE_CYCLES};
use crate::bit_functions::{set,reset};

pub struct Clock {
    m: usize,
    t: usize,
}

pub struct Register  {
    // The value of the register.
    pub value: u16,

    // A mask over the possible values in the register.
    // Only used for the AF register where lower bits of
    // F cannot be set.
    pub mask: u16,
}

impl Register {
    pub fn hi(&self) -> u8 {
        return self.value.wrapping_shr(8) as u8;
    }
    pub fn lo(&self) -> u8 {
        return self.value as u8;
    }
    pub fn full(&self) -> u16 {
        return self.value;
    }

    pub fn set_lo(&mut self, byte: u8) {
        self.value = (byte as u16) | (self.value as u16) & 0xFF00;
        self.updateMask()
    }
    pub fn set_hi(&mut self, byte: u8){
        self.value = (byte as u16).wrapping_shl(8) | (self.value as u16) & 0xFF;
        self.updateMask()
    }
    pub fn set_full(&mut self, word: u16) {
        self.value = word;
        self.updateMask()
    }
    pub fn update_mask(&mut self) {
        if self.mask != 0 {
            self.value &= self.mask
        }
    }

    pub fn new() -> Register {
        return Register {
            value: 0,
            mask: 0
        }
    }
}

pub struct Z80 {

    pub af: Register,
    pub bc: Register,
    pub de: Register,
    pub hl: Register,
    pub pc: u16,
    pub sp: Register,
    pub divider: usize,

    pub clock: Clock,
    pub mmu: &'static MMU,
    pub halt: u8,
    pub stop: u8,
}

impl Z80 {
    pub fn nop(&mut self) {
        self.registers.m = 1;
        self.registers.t = 4;
    }

    pub fn init(&mut self, cgb: bool) {
        self.pc = 0x100;
        if cgb {
            self.af.set_full(0x1180);
        } else {
            self.af.set_full(0x01B0);
        }
        self.bc.set_full(0x0000);
        self.de.set_full(0xFF56);
        self.hl.set_full(0x000D);
        self.sp.set_full(0xFFFE);
        self.af.mask = 0xFFF0;
    }

    pub fn set_flag(&mut self, index: u8, on: bool) {
        if on {
            self.af.set_lo(set(self.af.lo(), index))
        } else {
            self.af.set_lo(reset(self.af.lo(), index))
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

    // Z gets the value of the Z flag.
    pub fn z(&mut self) -> bool {
        return self.af.full().wrapping_shr(7) & 1 == 1;
    }

    // N gets the value of the N flag.
    pub fn n(&mut self) -> bool {
        return self.af.full().wrapping_shr(6) & 1 == 1;
    }

    // H gets the value of the H flag.
    pub fn h(&mut self) -> bool {
        return self.af.full().wrapping_shr(5) & 1 == 1;
    }

    // C gets the value of the C flag.
    pub fn c(&mut self) -> bool {
        return self.af.full().wrapping_shr(4) & 1 == 1;
    }

    pub fn pop_pc(&mut self) -> u8{
        let opcode = self.mmu.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        return opcode
    }

    pub fn pop_pc16(&mut self) -> u16 {
        byte_1 = self.pop_pc() as u16;
        byte_2 = self.pop_pc() as u16;
        return byte_2.wrapping_shl(8) | byte_1;
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

    pub fn execute_next_opcode(&mut self) -> usize {
        let opcode = self.pop_pc();
        self.clock.t = OPCODE_CYCLES[opcode] * 4;
        find_op(self, opcode);
        return self.clock.t;
    }

    pub fn new(mmu: &MMU) -> Z80 {
        return Z80 {
            af: Register::new(),
            bc: Register::new(),
            de: Register::new(),
            hl: Register::new(),
            pc: 0,
            sp: Register::new(),
            divider: 0,
            clock: Clock {
                m: 0,
                t: 0
            },
            mmu,
            halt: 0,
            stop: 0
        }
    }

}

use crate::gameboy::Gameboy;
use crate::memory::MemoryAddr;

mod cb_instructions;
mod instructions;
mod operations;

pub struct Clock {
    pub m: usize,
    pub t: usize,
}

pub struct Register {
    // The value of the register.
    pub value: u16,

    // A mask over the possible values in the register.
    // Only used for the AF register where lower bits of
    // F cannot be set.
    pub mask: u16,
}

impl Register {
    pub fn hi(&self) -> u8 {
        return (self.value >> 8) as u8;
    }
    pub fn lo(&self) -> u8 {
        return self.value as u8;
    }
    pub fn full(&self) -> u16 {
        return self.value;
    }

    pub fn set_lo(&mut self, byte: u8) {
        self.value = (byte as u16) | (self.value as u16) & 0xFF00;
        self.update_mask()
    }
    pub fn set_hi(&mut self, byte: u8) {
        self.value = (byte as u16) << 8 | (self.value as u16) & 0xFF;
        self.update_mask();
    }
    pub fn set_full(&mut self, word: u16) {
        self.value = word;
        self.update_mask()
    }
    pub fn update_mask(&mut self) {
        if self.mask != 0 {
            self.value &= self.mask
        }
    }

    pub fn new() -> Register {
        return Register { value: 0, mask: 0 };
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
    pub halt: u8,
    pub stop: u8,
}

impl Z80 {
    pub fn new() -> Z80 {
        return Z80 {
            af: Register::new(),
            bc: Register::new(),
            de: Register::new(),
            hl: Register::new(),
            pc: 0,
            sp: Register::new(),
            divider: 0,
            clock: Clock { m: 0, t: 0 },
            halt: 0,
            stop: 0,
        };
    }
}

impl Gameboy {
    pub fn pop_pc(&mut self) -> u8 {
        let opcode = self.read(self.cpu.pc);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        return opcode;
    }

    pub fn pop_pc16(&mut self) -> u16 {
        let byte_1 = self.pop_pc() as u16;
        let byte_2 = self.pop_pc() as u16;
        return (byte_2 << 8) | byte_1;
    }

    pub fn pop_stack(&mut self) -> u16 {
        let sp = self.cpu.sp.full();
        let lo = self.read(sp) as u16;
        let hi = (self.read(sp.wrapping_add(1)) as u16) << 8;
        self.cpu.sp.set_full(sp.wrapping_add(2));
        return lo | hi;
    }

    pub fn push_stack(&mut self, addr: MemoryAddr) {
        let sp = self.cpu.sp.full();
        self.write(sp.wrapping_sub(1), ((addr & 0xFF00) >> 8) as u8);
        self.write(sp.wrapping_sub(2), (addr & 0xFF) as u8);
        self.cpu.sp.set_full(sp.wrapping_sub(2));
    }

    pub fn call(&mut self, next: u16) {
        self.push_stack(self.cpu.pc);
        self.cpu.pc = next;
    }

    pub fn ret(&mut self) {
        self.cpu.pc = self.pop_stack();
    }
}

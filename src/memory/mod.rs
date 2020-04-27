mod read;
mod write;
use crate::bit_functions::{test, val};
use crate::cart::controller::{Cart, CBG_MODE};
use crate::gameboy::Gameboy;
use crate::input::Input;
use mockall::automock;

pub const CLOCK_SPEED: usize = 4194304;
pub const FRAMES_PER_SECOND: usize = 60;
pub const DIV: u16 = 0xFF04;
pub const TIMA: u16 = 0xFF05;
pub const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

pub type MemoryAddr = u16;

pub struct Timer {
    pub value: usize,
}

pub struct Speed {
    pub current: u8,
    pub prepare: bool,
    pub cycle_frames: usize,
}

impl Timer {
    pub fn reset_timer(&mut self) {
        self.value = 0;
    }
}

#[automock]
pub struct MMU {
    pub cart: Cart,
    pub timer: Timer,
    pub input: Input,
    pub speed: Speed,
    pub ram: [u8; 0x100],
    pub vram: [u8; 0x4000],
    vram_bank: u8,

    wram: [u8; 0x9000],
    wram_bank: u8,

    oam: [u8; 0x100],

    hdma_len: u8,
    hdma_active: bool,
}

impl MMU {
    pub fn init(&mut self) {
        self.ram[(0x04) as usize] = 0x1E;
        self.ram[(0x05) as usize] = 0x00;
        self.ram[(0x06) as usize] = 0x00;
        self.ram[(0x07) as usize] = 0xF8;
        self.ram[(0x0F) as usize] = 0xE1;
        self.ram[(0x10) as usize] = 0x80;
        self.ram[(0x11) as usize] = 0xBF;
        self.ram[(0x12) as usize] = 0xF3;
        self.ram[(0x14) as usize] = 0xBF;
        self.ram[(0x16) as usize] = 0x3F;
        self.ram[(0x17) as usize] = 0x00;
        self.ram[(0x19) as usize] = 0xBF;
        self.ram[(0x1A) as usize] = 0x7F;
        self.ram[(0x1B) as usize] = 0xFF;
        self.ram[(0x1C) as usize] = 0x9F;
        self.ram[(0x1E) as usize] = 0xBF;
        self.ram[(0x20) as usize] = 0xFF;
        self.ram[(0x21) as usize] = 0x00;
        self.ram[(0x22) as usize] = 0x00;
        self.ram[(0x23) as usize] = 0xBF;
        self.ram[(0x24) as usize] = 0x77;
        self.ram[(0x25) as usize] = 0xF3;
        self.ram[(0x26) as usize] = 0xF1;
        self.ram[(0x40) as usize] = 0x91;
        self.ram[(0x41) as usize] = 0x85;
        self.ram[(0x42) as usize] = 0x00;
        self.ram[(0x43) as usize] = 0x00;
        self.ram[(0x45) as usize] = 0x00;
        self.ram[(0x47) as usize] = 0xFC;
        self.ram[(0x48) as usize] = 0xFF;
        self.ram[(0x49) as usize] = 0xFF;
        self.ram[(0x4A) as usize] = 0x00;
        self.ram[(0x4B) as usize] = 0x00;
        self.ram[(0xFF) as usize] = 0x00;

        self.wram_bank = 1;
    }

    pub fn new(filename: &str) -> MMU {
        return MMU {
            cart: Cart::new(filename),
            timer: Timer { value: 0 },
            input: Input { mask: 0xFF },
            speed: Speed {
                current: 0,
                prepare: false,
                cycle_frames: CLOCK_SPEED / FRAMES_PER_SECOND,
            },
            ram: [0; 0x100],
            vram: [0; 0x4000],
            vram_bank: 0,
            wram: [0; 0x9000],
            wram_bank: 0,
            oam: [0; 0x100],
            hdma_len: 0,
            hdma_active: false,
        };
    }

    pub fn has_cgb_mode(&self) -> bool {
        return self.cart.mode & CBG_MODE != 0;
    }
}

impl Gameboy {

    pub fn check_speed(&mut self) {
        if self.memory.speed.prepare {
            self.memory.speed.prepare = false;
            self.memory.speed.current = if self.memory.speed.current == 0 { 1 } else { 0 };
            self.halted = false;
        }
    }
    pub fn is_clock_enabled(&self) -> bool {
        return test(self.read(TAC), 2);
    }

    pub fn get_clock_freq(&self) -> u8 {
        return self.read(TAC) & 0x3;
    }

    fn transfer(&mut self, len: u16) {
        let mut source = ((self.memory.ram[0x51_usize] as u16) << 8)
            | (((self.memory.ram[0x52_usize]) as u16) & 0xFFF0_u16);
        let mut destination = ((self.memory.ram[0x53_usize] as u16) << 8)
            | ((self.memory.ram[0x54_usize] as u16) & 0x1FF0);
        destination = destination.wrapping_add(0x8000);

        let mut i = 0_u16;
        while i < len {
            let val = self.read(source);
            self.write(destination, val);
            destination = destination.wrapping_add(1);
            source = source.wrapping_add(1);
            i = i.wrapping_add(1);
        }

        self.memory.ram[0x51] = (source >> 8) as u8;
        self.memory.ram[0x52] = (source & 0xFF) as u8;
        self.memory.ram[0x53] = (destination >> 8) as u8;
        self.memory.ram[0x54] = (destination & 0xF0) as u8;
    }

    pub fn cgb_dma_transfer(&mut self, value: u8) {
        if self.memory.hdma_active && val(value, 7) == 0 {
            self.memory.hdma_active = false;
            self.memory.ram[0x55_usize] |= 0x80; // Set bit 7
            return;
        }

        let len = ((value as u16) & 0x7F).wrapping_add(1).wrapping_mul(0x10);
        if value >> 7 == 0 {
            self.transfer(len);
            self.memory.ram[0x55] = 0xFF;
        } else {
            self.memory.hdma_len = value as u8;
            self.memory.hdma_active = true;
        }
    }

    pub fn dma_transfer(&mut self, val: u8) {
        let address = (val as u16) << 8; // (data * 100)

        for i in  0_u16..0xA0 {
            let val = self.read(address.wrapping_add(i));
            self.write(0xFE00_u16.wrapping_add(i), val);
        }
    }

    pub fn hdma_transfer(&mut self) {
        if self.memory.hdma_active {
            self.transfer(0x10);
            if self.memory.hdma_len > 0 {
                self.memory.hdma_len = self.memory.hdma_len.wrapping_sub(1);
                self.memory.ram[0x55_usize] = self.memory.hdma_len;
            } else {
                self.memory.ram[0x55_usize] = 0xFF;
                self.memory.hdma_active = false;
            }
        }
    }
}

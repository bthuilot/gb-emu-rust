use crate::gameboy::Gameboy;
use crate::memory::MemoryAddr;
use crate::bit_functions::test;
use crate::memory::{DIV, TIMA, TMA, TAC};

impl Gameboy {
    pub fn write_upper_ram(&mut self, addr: MemoryAddr, value: u8) {
        match addr {
            0xFEA0..=0xFEFF => {}
            0xFF10..=0xFF26 => {
                // TODO: Write Sound
            }
            0xFF30..=0xFF3F => {
                // TODO: Write Wave Form
            }
            0xFF02 => {
                // TODO: Serial transfer control
            }
            DIV => {
                self.memory.timer.reset_timer();
                self.cpu.divider = 0;
                self.memory.ram[(DIV - 0xFF00) as usize] = 0
            }
            TIMA => {
                self.memory.ram[(TIMA - 0xFF00) as usize] = value;
            }
            TMA => {
                self.memory.ram[(TMA - 0xFF00) as usize] = value;
            }
            TAC => {
                let current_freq = self.get_clock_freq();
                self.memory.ram[(TAC - 0xFF00) as usize] = value | 0xF8;
                let new_freq = self.get_clock_freq();
                if current_freq != new_freq {
                    self.memory.timer.reset_timer();
                }
            }
            0xFF41 => {
                self.memory.ram[(0x41) as usize] = value | 0x80;
            }
            0xFF44 => self.memory.ram[(0x44) as usize] = 0,
            0xFF46 => {
                self.dma_transfer(value);
            }
            0xFF4D => {
                if self.cgb_mode {
                    self.memory.speed.prepare = test(value, 0)
                }
            }
            0xFF4F => {
                if self.cgb_mode && !self.memory.hdma_active {
                    self.memory.vram_bank = value & 0x1;
                }
            }
            0xFF55 => {
                if self.cgb_mode {
                    self.cgb_dma_transfer(value);
                }
            }
            0xFF68 => {
                if self.cgb_mode {
                    self.bg_palette.update_index(value);
                }
            }
            0xFF69 => {
                if self.cgb_mode {
                    self.bg_palette.write(value);
                }
            }
            0xFF6A => {
                if self.cgb_mode {
                    self.sprite_palette.update_index(value);
                }
            }
            0xFF6B => {
                if self.cgb_mode {
                    self.sprite_palette.write(value);
                }
            }
            0xFF70 => {
                if self.cgb_mode {
                    self.memory.wram_bank = value & 0x7;
                    if self.memory.wram_bank == 0 {
                        self.memory.wram_bank = 1
                    }
                }
            }
            0xFF72..=0xFF77 => {
                //TODO: Need to figure out what to do here
            }
            _ => {
                self.memory.ram[(addr.wrapping_sub(0xFF00)) as usize] = value;
            }
        }
    }

    pub fn write(&mut self, addr: MemoryAddr, value: u8) {
        match addr {
            0..=0x7FFF => {
                self.memory.cart.write_rom(addr, value);
            }
            0x8000..=0x9FFF => {
                let offset = (self.memory.vram_bank as u16).wrapping_mul(0x2000);
                self.memory.vram[(addr.wrapping_sub(0x8000).wrapping_add(offset)) as usize] = value
            }
            0xA000..=0xBFFF => {
                self.memory.cart.write_ram(addr, value);
            }
            0xC000..=0xCFFF => {
                self.memory.wram[(addr.wrapping_sub(0xC000)) as usize] = value;
            }
            0xD000..=0xDFFF => {
                self.memory.wram[addr
                    .wrapping_sub(0xC000)
                    .wrapping_add((self.memory.wram_bank as u16).wrapping_mul(0x1000))
                    as usize] = value
            }
            0xE000..=0xFDFF => {
                // TODO: echo RAM
                //mem.Write(address-0x2000, value)
            }
            0xFE00..=0xFE9F => self.memory.oam[addr.wrapping_sub(0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => {
                // Not usable
            }
            _ => {
                self.write_upper_ram(addr, value);
            }
        }
    }
}

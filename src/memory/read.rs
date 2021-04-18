use crate::gameboy::Gameboy;
use crate::memory::MemoryAddr;
use crate::bit_functions::b;

impl Gameboy {
    pub fn read(&self, addr: MemoryAddr) -> u8 {
        return match addr {
            // BIOS (256b)/ROM0
            0x0000..=0x7FFF => {
                self.memory.cart.read(addr)
            }
            // ROM0
            0x8000..=0x9FFF => {
                let offset = (self.memory.vram_bank as u16).wrapping_mul(0x2000);
                self.memory.vram[addr.wrapping_sub(0x8000).wrapping_add(offset) as usize]
            }

            // External RAM (8k)
            0xA000..=0xBFFF => {
                self.memory.cart.read(addr)
            }

            // Working RAM (8k)
            0xC000..=0xCFFF => {
                self.memory.wram[(addr.wrapping_sub(0xC000)) as usize]
            }
            // Working RAM shadow
            0xD000..=0xDFFF => {
                self.memory.wram[((addr - 0xC000)
                    + (self.memory.wram_bank as u16).wrapping_mul(0x1000))
                    as usize]
            }
            // Mirror of C000~~DDFF (Echo RAM)
            // Nintendo prohibits use of this area
            0xE000..=0xFDFF => {
                0xFF
            }
            // Sprite attribute table (OAM)
            0xFE00..=0xFE9F => {
                self.memory.oam[(addr - 0xFE00) as usize]
            }
            0xFEA0..=0xFEFF => {
                0xFF
            }
            _ => self.read_upper_ram(addr),
        }
    }

    pub fn read_upper_ram(&self, addr: MemoryAddr) -> u8 {
        match addr {
            0xFF00 => {
                return self
                    .memory
                    .input
                    .joypad_value(self.memory.ram[(0x00) as usize]);
            }
            0xFF10..=0xFF26 => {
                // TODO Read Sound
            }
            0xFF30..=0xFF3F => {} // TODO read wave form
            0xFF0F => return self.memory.ram[(0x0F) as usize] | 0xE0,
            0xFF72..=0xFF77 => return 0,
            0xFF68 => {
                if self.cgb_mode {
                    return self.bg_palette.index;
                }
                return 0;
            }
            0xFF69 => {
                if self.cgb_mode {
                    return self.bg_palette.read();
                }
                return 0;
            }
            0xFF6A => {
                if self.cgb_mode {
                    self.sprite_palette.index;
                }
                return 0;
            }
            0xFF6B => {
                if self.cgb_mode {
                    return self.sprite_palette.read();
                }
                return 0;
            }
            0xFF4D => return self.memory.speed.current << 7 | b(self.memory.speed.prepare),
            0xFF4F => return self.memory.vram_bank,
            0xFF70 => return self.memory.wram_bank,
            _ => return self.memory.ram[(addr - 0xFF00) as usize],
        }
        return 0;
    }
}

use crate::cpu::Z80;
use crate::input::Input;
use crate::speed::{Speed};
use crate::cart::controller::Cart;
use crate::bit_functions::{val, test, b};
use crate::gameboy::Gameboy;

// DIV is the divider register which is incremented periodically by
// the Gameboy.
pub const DIV: u16 = 0xFF04;
// TIMA is the timer counter register which is incremented by a clock
// frequency specified in the TAC register.
pub const TIMA: u16 = 0xFF05;
// TMA is the timer modulo register. When the TIMA overflows, this data
// will be loaded into the TIMA register.
pub const TMA: u16 = 0xFF06;
// TAC is the timer control register. Writing to this register will
// start and stop the timer, and select the clock speed for the timer.
pub const TAC: u16 = 0xFF07;

pub type MemoryAddr = u16;


pub struct Timer {
    pub value: usize,
}

impl Timer {
    pub fn reset_timer(&mut self) {
        self.value = 0;
    }
}


pub struct MMU {
    pub cart: Cart,
    pub timer: Timer,
    pub input: Input,
    pub speed: Speed,
    cgb: bool,
    pub ram: [u8; 0x100],
    pub vram: [u8; 0x4000],
    // Index of the current VRAM bank
    vram_bank: u8,

    wram: [u8; 0x9000],
    wram_bank: u8,

    oam: [u8; 0x100],

    // CGB HDMA transfer variables
    hdma_len: u8,
    hdma_active: bool
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
        let cgb = true;
         return MMU {
             cart: Cart::new(filename),
             timer: Timer {
                 value: 0
             },
             input: Input {
                 mask: 0
             },
             speed : Speed {
                 current: 0,
                 prepare: false
             },
             cgb,
             ram: [0; 0x100],
             vram: [0; 0x4000],
             vram_bank: 0,
             wram: [0; 0x9000],
             wram_bank: 0,
             oam: [0; 0x100],
             hdma_len: 0,
             hdma_active: false
         }
    }


}

impl Gameboy {



    pub fn is_clock_enabled(&self) -> bool {
        return test(self.read(TAC), 2)
    }

    pub fn get_clock_freq(&self) -> u8 {
        return self.read(TAC) & 0x3
    }


    fn transfer(&mut self, len: u16) {
        let mut source = ((self.memory.ram[0x51_usize] as u16)<<8 | (self.memory.ram[0x52_usize]) as u16) & 0xFFF0_u16;
        let mut destination = ((self.memory.ram[0x53_usize] as u16)<<8 | (self.memory.ram[0x54_usize] as u16)) & 0x1FF0;
        destination = destination.wrapping_add(0x8000);

        // Transfer the data from the source to the destination
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
            return
        }

        let mut len = (((value as u16) & 0x7F) + 1) * 0x10;
        if value >> 7 == 0 {
            self.memory.hdma_len = self.memory.hdma_len.wrapping_sub(1);
            self.transfer(len);
        } else {
            self.memory.ram[0x55 as usize] = 0xFF_u8;
            self.memory.hdma_active = false;
        }
    }

    pub fn dma_transfer(&mut self, val: u8) {
        // TODO: This may need to be done instead of CPU ticks
        let address = (val as u16) << 8;// (data * 100)

        let mut i = 0_16;
        while i < 0xA0 {
            // TODO: Check this doesn't prevent
            let val = self.read(address.wrapping_add(i));
            self.write(0xFE00_u16.wrapping_add(i), val);
            i = i.wrapping_add(1);
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

    pub fn write_upper_ram(&mut self, addr: MemoryAddr, value: u8) {
        match addr {
            0xFEA0..=0xFEFE => {
            },
            0xFF10..=0xFF26 => {} // Sound,
            0xFF30..=0xFF3F => {} // WaveForm
            0xFF02 => {
                println!("Amybe?")
                // Serial transfer control
            }
            DIV => {
                self.memory.timer.reset_timer();
                self.cpu.divider = 0;
                self.memory.ram[(DIV-0xFF00) as usize] = 0
            },
            TIMA => {
                self.memory.ram[(TIMA-0xFF00) as usize] = value;
            },
            TMA => {
                self.memory.ram[(TMA-0xFF00) as usize] = value;
            },
            TAC => {
                let current_freq = self.get_clock_freq();
                self.memory.ram[(TAC-0xFF00) as usize] = value | 0xF8;
                let new_freq = self.get_clock_freq();
                if current_freq != new_freq {
                    self.memory.timer.reset_timer();
                }
            },
            0xFF41 => {
                self.memory.ram[(0x41) as usize] = value | 0x80;
            },
            0xFF44 => {
                self.memory.ram[(0x44) as usize] = 0
            },
            0xFF46 => {
                self.dma_transfer(value);
            },
            0xFF4D => {
                if self.cgb_mode {
                    self.memory.speed.prepare = test(value, 0)
                }
            },
            0xFF4F => {
                if self.cgb_mode && !self.memory.hdma_active {
                    self.memory.vram_bank = value & 0x1;
                }
            },
            0xFF55 => {
                if self.cgb_mode{
                    self.cgb_dma_transfer(value);
                }
            },
            0xFF68 => {
                if self.cgb_mode{
                    self.bg_palette.update_index(value);
                }
            },
            0xFF69 => {
                if self.cgb_mode{
                    self.bg_palette.write(value);
                }
            },
            0xFF6A => {
                if self.cgb_mode{
                    self.sprite_palette.update_index(value);
                }
            },
            0xFF6B => {
                if self.cgb_mode{
                    self.sprite_palette.write(value);
                }
            },
            0xFF70 => {
                if self.cgb_mode{
                    self.memory.wram_bank = value & 0x7;
                    if self.memory.wram_bank == 0 {
                        self.memory.wram_bank = 1
                    }
                }
            },
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
                self.memory.wram[addr.wrapping_sub(0xC000).wrapping_add((self.memory.wram_bank as u16).wrapping_mul(0x1000)) as usize] = value
            }
            0xE000..=0xFDFF => {
                // TODO: echo RAM
                //mem.Write(address-0x2000, value)
            }
            0xFE00..=0xFE9F => {
                self.memory.oam[addr.wrapping_sub(0xFE00) as usize] = value
            }
            0xFEA0..=0xFEFF => {
                // Not usable
            }
            _ => {
                self.write_upper_ram(addr, value);
            }
        }
    }

    pub fn read(&self, addr: MemoryAddr) -> u8 {
        match addr  {
            // BIOS (256b)/ROM0
            0x0000..=0x7FFF=> {
                return self.memory.cart.read(addr);
            }
            // ROM0
            0x8000..=0x9FFF => {
                let offset = (self.memory.vram_bank as u16).wrapping_mul(0x2000);
                return self.memory.vram[addr.wrapping_sub(0x8000).wrapping_add(offset) as usize];
            }

            // External RAM (8k)
            0xA000..=0xBFFF => {
                return self.memory.cart.read(addr);
            }

            // Working RAM (8k)
            0xC000..=0xCFFF => {
                return self.memory.wram[(addr.wrapping_sub(0xC000)) as usize];
            }
            // Working RAM shadow
            0xD000..=0xDFFF => {
                return self.memory.wram[((addr - 0xC000) + (self.memory.wram_bank as u16 * 0x1000)) as usize];
            }
            // Working RAM shadow, I/O, Zero-page RAM
            0xE000..=0xFDFF => {
                // TODO: re-enable echo RAM?
                return 0xFF;
            }
            0xFE00..=0xFE9F => {
                return self.memory.oam[(addr - 0xFE00) as usize];
            }
            0xFEA0..=0xFEFF => {
                return 0xFF;
            }
            _ => {
                return self.read_upper_ram(addr)
            }
        }
    }

    pub fn read_upper_ram(&self, addr: MemoryAddr) -> u8 {
        match addr {
            0xFF00 => {
                return self.memory.input.joypad_value(self.memory.ram[(0x00) as usize]);
            },
            0xFF10..=0xFF26 => {
                // TODO Read Sound
            },
            0xFF30..=0xFF3F => {}, // TODO read wave form
            0xFF0F => return self.memory.ram[(0x0F) as usize] | 0xE0,
            0xFF72..=0xFF77 => return 0,
            0xFF68 => {
                if self.cgb_mode{
                    return self.bg_palette.index;
                }
                return 0;
            }
            0xFF69 => {
                if self.cgb_mode{
                    return self.bg_palette.read();
                }
                return 0;
            }
            0xFF6A => {
                if self.cgb_mode{
                    self.sprite_palette.index;
                }
                return 0
            }
            0xFF6B => {
                if self.cgb_mode{
                    return self.sprite_palette.read();
                }
                return 0
            }
            0xFF4D =>{
                return self.memory.speed.current<<7 | b(self.memory.speed.prepare)
            },
            0xFF4F => return self.memory.vram_bank,
            0xFF70 => return self.memory.wram_bank,
            _=> return self.memory.ram[(addr-0xFF00) as usize]
        }
        return 0
    }
}

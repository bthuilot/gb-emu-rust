use crate::cpu::Z80;
use crate::gameboy::Timer;
use crate::input::Input;
use crate::speed::{Speed};
use crate::cart::controller::Cart;
use crate::bit_functions::{val, test};

// DIV is the divider register which is incremented periodically by
// the Gameboy.
const DIV: u16 = 0xFF04;
// TIMA is the timer counter register which is incremented by a clock
// frequency specified in the TAC register.
const TIMA: u16 = 0xFF05;
// TMA is the timer modulo register. When the TIMA overflows, this data
// will be loaded into the TIMA register.
const TMA: u16 = 0xFF06;
// TAC is the timer control register. Writing to this register will
// start and stop the timer, and select the clock speed for the timer.
const TAC: u16 = 0xFF07;

pub type MemoryAddr = u16;

pub struct MMU {
    pub cart: Cart,
    timer: &'static Timer,
    input: &'static mut Input,
    speed: &'static mut Speed,
    cgb: bool,
    ram: [u8; 0x100],
    vram: [u8; 0x4000],
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
        self.ram[0x04] = 0x1E;
        self.ram[0x05] = 0x00;
        self.ram[0x06] = 0x00;
        self.ram[0x07] = 0xF8;
        self.ram[0x0F] = 0xE1;
        self.ram[0x10] = 0x80;
        self.ram[0x11] = 0xBF;
        self.ram[0x12] = 0xF3;
        self.ram[0x14] = 0xBF;
        self.ram[0x16] = 0x3F;
        self.ram[0x17] = 0x00;
        self.ram[0x19] = 0xBF;
        self.ram[0x1A] = 0x7F;
        self.ram[0x1B] = 0xFF;
        self.ram[0x1C] = 0x9F;
        self.ram[0x1E] = 0xBF;
        self.ram[0x20] = 0xFF;
        self.ram[0x21] = 0x00;
        self.ram[0x22] = 0x00;
        self.ram[0x23] = 0xBF;
        self.ram[0x24] = 0x77;
        self.ram[0x25] = 0xF3;
        self.ram[0x26] = 0xF1;
        self.ram[0x40] = 0x91;
        self.ram[0x41] = 0x85;
        self.ram[0x42] = 0x00;
        self.ram[0x43] = 0x00;
        self.ram[0x45] = 0x00;
        self.ram[0x47] = 0xFC;
        self.ram[0x48] = 0xFF;
        self.ram[0x49] = 0xFF;
        self.ram[0x4A] = 0x00;
        self.ram[0x4B] = 0x00;
        self.ram[0xFF] = 0x00;

        self.WRAMBank = 1;
    }

    pub fn new(filename: String, timer: &Timer, input: &mut Input, speed: &mut Speed) -> MMU {
        let cart = Cart::new(filename);
        cgb = true;
         return MMU {
             cart,
             timer,
             input,
             speed,
             cgb,
             ram: [u8; 0x100],
             vram: [u8; 0x4000],
             vram_bank: 0,
             wram: [u8; 0x9000],
             wram_bank: 0,
             oam: [u8; 0x100],
             hdma_len: 0,
             hdma_active: false
         }
    }

    pub fn write_upper_ram(&mut self, addr: MemoryAddr, value: u8) {
        match addr {
            0xFEA0..0xFEFF => {
                if value == 0x81 {
                    f(mem.ReadHighRam(0xFF01))
                }
            },
            0xFF10..=0xFF26 => {} // Sound,
            0xFF30..=0xFF3F => {} // WaveForm
            0xFF02 => {
                // Serial transfer control
            }
            DIV => {
                self.timer.reset_timer();
                // TODO Need to rest divider
                self.ram[DIV-0xFF00] = 0
            },
            TIMA => {
                self.ram[TIMA-0xFF00] = value;
            },
            TMA => {
                self.ram[TMA-0xFF00] = value;
            },
            TAC => {
                let current_freq = self.timer.get_clock_freq();
                self.ram[TAC-0xFF00] = value | 0xF8;
                let new_freq = self.timer.get_clock_freq();
                if current_freq != new_freq {
                    self.timer.reset_timer();
                }
            },
            0xFF41 => {
                self.ram[0x41] = value | 0x80;
            },
            0xFF44 => {
                self.ram[0x44] = 0
            },
            0xFF46 => {
                self.dma_transfer(value);
            },
            0xFF4D => {
                if self.cgb {
                    self.speed.prepare = bits.Test(value, 0)
                }
            },
            0xFF4F => {
                if self.cgb && !self.hdma_active {
                    self.vram_bank = value & 0x1;
                }
            },
            0xFF55 => {
                if self.cgb {
                    self.dma_transfer(value);
                }
            },
            0xFF68 => {
                if self.cgb {
                    // BG palette index
                }
            },
            0xFF69 => {
                if self.cgb {
                    // BG Palette data
                }
            },
            0xFF6A => {
                if self.cgb {
                    // Sprite Palette index
                }
            },
            0xFF6B => {
                if self.cgb {
                    // Sprite Palette data
                }
            },
            0xFF70 => {
                if self.cgb {
                    self.wram_bank = value & 0x7;
                    if self.wram_bank == 0 {
                        self.wram_bank = 1
                    }
                }            },
            _ => {
                self.ram[addr - 0xFF00] = value;
            }
        }
    }

    pub fn write(&mut self, addr: MemoryAddr, value: u8) {
        match addr {
            0..0x8000 => {
                self.cart.write_rom(addr, value);
            }
            0x8000..0xA000 => {
                let offset = (self.vram_bank as u16) * 0x2000;
                self.vram[addr-0x8000+offset] = value
            }
            0xA000..0xC000 => {
                self.cart.write_ram(addr, value);
            }
            0xC000..0xD000 => {
                self.wram[addr-0xC000] = value;
            }
            0xD000..0xE000 => {
                self.wram[(addr-0xC000)+((self.wram_bankas as u16)*0x1000)] = value
            }
            0xE000..0xFE00 => {
                // TODO: echo RAM
                //mem.Write(address-0x2000, value)
            }
            0xFE00..0xFEA0 => {
                self.oam[addr - 0xFE00] = value
            }
            0xFEA0..0xFF00 => {
                // Not usable
            }
            _ => {
                self.write_upper_ram(addr, value);
            }
        }
    }

    pub fn read(&mut self, addr: MemoryAddr) -> u8 {
        match addr  {
            // BIOS (256b)/ROM0
            0x0000..0x8000=> {
                return self.cart.read(addr);
            }
            // ROM0
            0x8000..0xA000 => {
                let offset = (mem.vram_bank as u16) * 0x2000;
                return self.vram[addr-0x8000 + offset];
            }

            // External RAM (8k)
            0xA000..0xC000 => {
                return self.cart.read(addr);
            }

            // Working RAM (8k)
            0xC000..0xD000 => {
                return self.wram[(addr - 0xC000)];
            }
            // Working RAM shadow
            0xD000..0xE000 => {
                return self.wram[(addr - 0xC000) + (self.wram_bank as u16 * 0x1000)];
            }
            // Working RAM shadow, I/O, Zero-page RAM
            0xE000..0xFE00 => {
                // TODO: re-enable echo RAM?
                return 0xFF;
            }
            0xFE00..0xFEA0 => {
                return self.oam[addr - 0xFE00];
            }
            0xFEA0..0xFF00 => {
                return 0xFF;
            }
            _ => {
                return self.read_upper_ram(addr)
            }
        }
    }

    pub fn read_upper_ram(&mut self, addr: MemoryAddr) -> u8 {
        match addr {
            0xFF00 => {
                self.input.joypad_value(self.ram[0x00]);
            },
            0xFF10..=0xFF26 => {
                // TODO Read Sound
            },
            0xFF30..=0xFF3F => {}, // TODO read wave form
            0xFF0F => return self.ram[0x0F] | 0xE0,
            0xFF72..=0xFF77 => return 0,
            0xFF68 => {
                if self.cgb {
                    // Read BG Palette index
                }
                return 0;
            }
            0xFF69 => {
                if self.cgb {
                    // Read BG Palette data
                }
                return 0;
            }
            0xFF6A => {
                if self.cgb {
                    // read Sprite index
                }
                return 0
            }
            0xFF6B => {
                if self.cgb {
                    // read Sprite
                }
                return 0
            }
            0xFF4D =>{
                self.speed.prepare = test(value, 0)
            },
            0xFF4F => return self.vram_bank,
            0xFF70 => return self.wram_bank,
            _=> return self.ram[addr-0xFF00]
        }
        return 0
    }
}

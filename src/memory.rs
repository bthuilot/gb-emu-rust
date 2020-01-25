pub mod mmu {
    use crate::cpu::cpu::Z80;
    use std::fs;
    use std::env;

    const BIOS: [u8; 256] = [
        0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
        0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
        0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
        0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
        0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
        0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
        0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
        0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
        0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xF2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
        0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
        0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
        0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
        0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
        0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3c, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x4C,
        0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
        0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50
    ];

    pub type MemoryAddr = u16;
    pub struct MMU {
        inbios: bool,
        bios: [u8; 0],
        rom: Vec<u8>,
        wram: [u8; 8192],
        eram: [u8; 8192],
        zram: [u8; 127],

    }

    impl MMU {
        pub fn rb(&mut self, addr: MemoryAddr, pc: u16) -> u8 {
            match addr & 0xF000 {
                // BIOS (256b)/ROM0
                0x0000 => {
                    if self.inbios {
                        if addr < 0x0100 {
                            return self.bios[addr as usize];
                        } else if pc == 0x0100 {
                            self.inbios = false;
                        }
                    }
                    return self.rom[addr as usize];
                }

                // ROM0
                0x1000 | 0x2000 | 0x3000 => {
                    return self.rom[addr as usize];
                }

                // ROM1 (unbanked) (16k)
                0x4000 | 0x5000 | 0x6000 | 0x7000 => {
                    return self.rom[addr as usize];
                }

                // Graphics: VRAM (8k)
                0x8000 | 0x9000 => {
                    // return GPU.vram
                    return self.wram[(addr & 0x1FFF) as usize];
                }

                // External RAM (8k)
                0xA000 | 0xB000 => {
                    return self.eram[(addr & 0x1FFF) as usize];
                }

                // Working RAM (8k)
                0xC000 | 0xD000 => {
                    return self.wram[(addr & 0x1FFF) as usize];
                }

                // Working RAM shadow
                0xE000 => {
                    return self.wram[(addr & 0x1FFF) as usize];
                }
                // Working RAM shadow, I/O, Zero-page RAM
                0xF000 => {
                    match addr & 0x0F00 {
                        // Working RAM shadow
                        0x000 | 0x100 | 0x200 | 0x300 | 0x400 | 0x500 | 0x600 | 0x700 |
                        0x800 | 0x900 | 0xA00 | 0xB00 | 0xC00 | 0xD00 => {
                            return self.wram[(addr & 0x1FFF) as usize];
                        }

                        // Graphics: object attribute memory
                        // OAM is 160 bytes, remaining bytes read as 0
                        0xE00 => {
                            if addr < 0xFEA0 {
                                // GPU.ram
                                return self.rom[(addr & 0xFF) as usize];
                            } else {
                                return 0;
                            }
                        }

                        0xF00 => {
                            if addr >= 0xFF80 {
                                return self.zram[(addr & 0x7F) as usize];
                            } else {
                                // I/O control handling
                                // Currently unhandled
                                return 0;
                            }
                        }
                        _ => {
                            return 0
                        }
                    }
                }
                _ => {
                    return 0
                }
            }
        }
        pub fn wb(&mut self, addr: MemoryAddr, value: u8) {
            match addr&0xF000 {
                // ROM bank 0
                0x0000 => {
                    if self.inbios && addr < 0x0100 {
                        return;
                    }
                }
                // fall through
                0x1000 | 0x2000 | 0x3000 => {
                    return;
                }

                // ROM bank 1
                 0x4000 | 0x5000 |0x6000 | 0x7000 => {
                     return;
                 }

                // VRAM
                0x8000 | 0x9000 => {
                    GPU._vram[addr & 0x1FFF] = val;
                    GPU.updatetile(addr & 0x1FFF, val);
                    return;
                }

                // External RAM
                0xA000 |  0xB000 => {
                    self.eram[addr & 0x1FFF] = val;
                }

                // Work RAM and echo
                0xC000 | 0xD000 | 0xE000 => {
                    self.wram[addr & 0x1FFF] = val;
                }

                // Everything else
                 0xF000 => {
                     match addr & 0x0F00 {
                         // Echo RAM
                         0x000| 0x100| 0x200| 0x300 |
                         0x400| 0x500| 0x600| 0x700 |
                         0x800| 0x900| 0xA00| 0xB00 |
                         0xC00| 0xD00 => {
                             self.wram[addr & 0x1FFF] = val;
                             return;
                         }

                         // OAM
                          0xE00 => {
                              if ((addr & 0xFF) < 0xA0) {
                                  GPU._oam[addr & 0xFF] = val;
                              }
                              GPU.updateoam(addr, val);
                              return;
                          }

                         // Zeropage RAM, I/O
                         0xF00 => {
                             if addr > 0xFF7F {
                                 self.zram[addr & 0x7F] = val;
                             } else {
//                                 switch(addr & 0xF0)
                                 return;
                             }
                         }
                         _ => {
                             return;
                         }
                     }
                     return;
                 }
                _ => {
                    return;
                }
            }
        }
        pub fn rw(&mut self, addr: MemoryAddr, pc: u16) -> u16{
            return (self.rb(addr,pc) as u16) + (self.rb(addr+1, pc) as u16) << 8;
        }
        pub fn ww(&mut self, addr: MemoryAddr, value: u16) {
            self.wb(addr, (value&255) as u8);
            self.wb(addr+1, (value >> 8) as u8);
        }

        pub fn load_file(&mut self, filename: &str) {
            self.rom = fs::read(filename).expect("Unable to read file");
        }
    }
}
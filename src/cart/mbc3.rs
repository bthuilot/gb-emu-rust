use crate::cart::BankingController;

pub struct MBC3 {
    rom: Vec<u8>,
    rom_bank: u32,

    ram: Vec<u8>,
    ram_bank: u32,
    ram_enabled: bool,

    clock: [u8; 0x10],
    latched_clock: [u8; 0x10],
    latched: bool,
}

impl MBC3 {
    fn update_rom_bank(&mut self) {
        if self.rom_bank == 0x00 {
            self.rom_bank += 1
        }
    }

    pub(crate) fn new(data: Vec<u8>) -> MBC3 {
        return MBC3 {
            rom: data,
            rom_bank: 1,
            ram: vec![0; 0x8000],
            ram_bank: 0,
            ram_enabled: false,
            clock: [0; 0x10],
            latched_clock: [0; 0x10],
            latched: false,
        };
    }

    pub fn new_as_bc(data: Vec<u8>) -> impl BankingController {
        return MBC3::new(data);
    }
}

impl BankingController for MBC3 {
    fn read(&self, address: u16) -> u8 {
        return match address {
            0..=0x3FFF => self.rom[address as usize],
            0x4000..=0x7FFF => {
                self.rom[((address - 0x4000) as u32 + (self.rom_bank * 0x4000)) as usize]
            }
            _ => {
                if self.ram_bank >= 0x4 {
                    if self.latched {
                        return self.latched_clock[self.ram_bank as usize];
                    }
                    return self.clock[self.ram_bank as usize];
                }
                return self.ram[((0x2000 * self.ram_bank) + (address - 0xA000) as u32) as usize];
            }
        };
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0..=0x1FFF => {
                self.ram_enabled = (value & 0xA) != 0;
            }
            0x2000..=0x3FFF => {
                self.rom_bank = (value & 0x7f) as u32;
                self.update_rom_bank()
            }
            0x4000..=0x5FFF => {
                self.ram_bank = (value) as u32;
            }
            0x6000..=0x7FFF => {
                if value == 0x1 {
                    self.latched = false
                } else if value == 0x0 {
                    self.latched = true;
                    self.clock.clone_from_slice(&self.latched_clock)
                }
            }
            _ => {}
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.ram_enabled {
            if self.ram_bank >= 0x4 {
                self.clock[self.ram_bank as usize] = value
            } else {
                let index =
                    0x2000_u32.wrapping_mul(self.ram_bank) + (address.wrapping_sub(0xA000) as u32);
                self.ram[index as usize] = value
            }
        }
    }

    fn get_save_data(&self) -> Vec<u8> {
        return self.ram.to_vec();
    }

    fn load_save_data(&mut self, data: Vec<u8>) {
        self.ram = data;
        self.ram.resize(0x8000, 0);
    }
}

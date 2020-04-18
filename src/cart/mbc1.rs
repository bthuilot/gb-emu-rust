use crate::cart::BankingController;

pub struct MBC1 {
    rom: Vec<u8>,
    rom_bank: u32,
    ram: Vec<u8>,
    ram_bank: u32,

    ram_enabled: bool,
    rom_banking: bool,
}

impl MBC1 {
    fn update_rom_bank(&mut self) {
        if self.rom_bank == 0x00
            || self.rom_bank == 0x20
            || self.rom_bank == 0x40
            || self.rom_bank == 0x60
        {
            self.rom_bank = self.rom_bank.wrapping_add(1)
        }
    }

    pub(crate) fn new(data: Vec<u8>) -> MBC1 {
        return MBC1 {
            rom: data,
            rom_bank: 1,
            ram: vec![0; 0x8000],
            ram_bank: 0,
            ram_enabled: false,
            rom_banking: false,
        };
    }

    pub fn new_as_bc(data: Vec<u8>) -> impl BankingController {
        return MBC1::new(data);
    }
}

impl BankingController for MBC1 {
    fn read(&self, address: u16) -> u8 {
        return match address {
            0..=0x3FFF => self.rom[address as usize],
            0x4000..=0x7FFF => {
                self.rom[(address.wrapping_sub(0x4000) as u32)
                    .wrapping_add(self.rom_bank.wrapping_mul(0x4000))
                    as usize]
            }
            _ => {
                self.ram[0x2000_u32
                    .wrapping_mul(self.ram_bank)
                    .wrapping_add(address.wrapping_sub(0xA000) as u32)
                    as usize]
            }
        };
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0..=0x1FFF => {
                if value & 0xF == 0xA {
                    self.ram_enabled = true;
                } else if value & 0xF == 0x0 {
                    self.ram_enabled = false;
                }
            }
            0x2000..=0x3FFF => {
                self.rom_bank = (self.rom_bank & 0xe0) | (value & 0x1f) as u32;
                self.update_rom_bank();
            }
            0x4000..=0x5FFF => {
                // ROM/RAM banking
                if self.rom_banking {
                    self.rom_bank = (self.rom_bank & 0x1F) | (value & 0xe0) as u32;
                    self.update_rom_bank();
                } else {
                    self.ram_bank = (value & 0x3) as u32;
                }
            }
            0x6000..=0x7FFF => {
                // ROM/RAM select mode
                self.rom_banking = value & 0x1 == 0x00;
                if self.rom_banking {
                    self.ram_bank = 0
                } else {
                    self.rom_bank = self.rom_bank & 0x1F
                }
            }
            _ => {}
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.ram_enabled {
            let index = 0x2000_u32
                .wrapping_mul(self.ram_bank)
                .wrapping_add(address.wrapping_sub(0xA000) as u32);
            self.ram[index as usize] = value;
        }
    }

    fn get_save_date(&self) -> Vec<u8> {
        return self.ram.to_vec();
    }

    fn load_save_data(&mut self, data: Vec<u8>) {
        self.ram = data
    }
}

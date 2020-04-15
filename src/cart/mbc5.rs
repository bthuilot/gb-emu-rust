use crate::cart::BankingController;

pub struct MBC5 {
    rom: Vec<u8>,
    rom_bank: u32,
    ram: Vec<u8>,
    ram_bank: u32,

    ram_enabled: bool,
}

impl MBC5 {
    fn update_rom_bank(&mut self) {
        if self.rom_bank == 0x00 || self.rom_bank == 0x20 || self.rom_bank == 0x40 || self.rom_bank == 0x60 {
            self.rom_bank += 1
        }
    }

    pub(crate) fn new(data: Vec<u8>) -> MBC5 {
        return MBC5 {
            rom: data,
            rom_bank: 1,
            ram: Vec::new(),
            ram_bank: 0,
            ram_enabled: false,
        }
    }

    pub fn new_as_bc(data: Vec<u8>) -> impl BankingController {
        return MBC5::new(data);
    }
}

impl BankingController for MBC5 {
    fn read(&self, address: u16) -> u8 {
        return match address {
            0..=0x3FFF => self.rom[address as usize],
            0x4000..=0x7FFF => self.rom[((address-0x4000) as u32 + (self.rom_bank*0x4000)) as usize],
            _ => self.ram[((0x2000*self.ram_bank)+ (address-0xA000) as u32) as usize],
        }

    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0..=0x1FFF => {
                if value&0xF == 0xA {
                    self.ram_enabled = true;
                } else if value&0xF == 0x0 {
                    self.ram_enabled = false;
                }
            },
            0x2000..=0x3FFF => {
                self.rom_bank = (self.rom_bank & 0x100) | (value as u32);
            },
            0x4000..=0x5FFF => {
                    self.rom_bank = (self.rom_bank & 0xFF) | ((value&0x01) as u32).wrapping_shl(8);
            },
            0x6000..=0x7FFF => {
                    self.ram_bank = (value & 0xF) as u32;
            },
            _ => {}
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.ram_enabled {
            self.ram[((0x2000*self.ram_bank)+(address-0xA000) as u32) as usize] = value
        }
    }

    fn get_save_date(&self) -> Vec<u8> {
        return self.ram.to_vec();
    }

    fn load_save_data(&mut self, data: Vec<u8>) {
        self.ram = data
    }

}
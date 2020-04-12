use crate::cart::BankingController;

pub struct ROM {
    rom: Vec<u8>,
}

impl ROM {
    fn update_rom_bank(&mut self) {
        if self.rom_bank == 0x00 || self.rom_bank == 0x20 || self.rom_bank == 0x40 || self.rom_bank == 0x60 {
            self.rom_bank += 1
        }
    }

    pub(crate) fn new(data: Vec<u8>) -> ROM {
        return ROM {
            rom: data,
        }
    }
}

impl BankingController for ROM {
    fn read(&self, address: u16) -> u8 {
        return self.rom[address];
    }

    fn write_rom(&mut self, address: u16, value: u8) {
    }

    fn write_ram(&mut self, address: u16, value: u8) {
    }

    fn get_save_date(&self) -> Vec<u8> {
        return Vec::new()
    }

    fn load_save_data(&mut self, data: Vec<u8>) {
    }
}
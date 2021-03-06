use crate::cart::BankingController;

pub struct ROM {
    rom: Vec<u8>,
}

impl ROM {
    pub(crate) fn new(data: Vec<u8>) -> ROM {
        return ROM { rom: data };
    }

    pub fn new_as_bc(data: Vec<u8>) -> impl BankingController {
        return ROM::new(data);
    }
}

impl BankingController for ROM {
    fn read(&self, address: u16) -> u8 {
        return self.rom[address as usize];
    }

    fn write_rom(&mut self, _address: u16, _value: u8) {}

    fn write_ram(&mut self, _address: u16, _value: u8) {}

    fn get_save_data(&self) -> Vec<u8> {
        return Vec::new();
    }

    fn load_save_data(&mut self, _data: Vec<u8>) {}
}

use std::io::prelude::*;
use std::fs::File;
use crate::cart::BankingController;
use crate::cart::rom::ROM;
use crate::cart::mbc1::MBC1;
use crate::cart::mbc2::MBC2;
use crate::cart::mbc3::MBC3;
use crate::cart::mbc5::MBC5;

const GB_MODE: u8 = 1;
const CBG_MODE: u8 = 2;
const BOTH_MODE: u8 = 3;

pub(crate) struct Cart {
    banking_controller: &'static Box<dyn BankingController>,
    pub(crate) title: String,
    filename: String,
    mode: u8,
}


impl Cart {

    pub fn read(&self, address: u16) -> u8 {
        return self.banking_controller.read(address);
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        self.banking_controller.write_rom(address, value);
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        self.banking_controller.write_ram(address, value);
    }

    pub fn get_save_date(&self) -> Vec<u8> {
        return self.banking_controller.get_save_date();
    }

    pub fn load_save_data(&mut self) {
        let mut save_data: Vec<u8> = Vec::new();
        let mut file = File::open(self.filename.as_str() + ".sav")?;
        file.read_to_end(&mut save_data)?;
        self.banking_controller.load_save_data(save_data);
    }

    pub fn save(&self) {
        let data = self.banking_controller.get_save_date();
        if data.len() > 0 {
            let mut file = File::open(self.filename.as_str() + ".sav")?;
            let result = file.write(data.as_slice());
            if result.is_err() {
                // TODO Handle this
            }
        }
    }

    pub fn new(&mut filename: String) -> Cart{
        let rom = Cart::read_rom_data(filename);
        let mut mode: u8;
        match rom[0x0143] {
            0x80 => mode = BOTH_MODE,
            0xC0 => mode = CBG_MODE,
            _ => mode = GB_MODE,
        }

        let banking_controller: dyn BankingController;

        let flag = rom[0x147];
        match flag {
            0x00 | 0x08 | 0x09 | 0x0B | 0x0C | 0x0D => banking_controller = ROM::new(rom) ,
            0..=0x03 => banking_controller = MBC1::new(rom) ,
            0x03..=0x06 => banking_controller = MBC2::new(rom),
            0x06..=0x13 => banking_controller = MBC3::new(rom),
            0x13..=0x17 => banking_controller = MBC1::new(rom),
            0x17..=0x1F => banking_controller = MBC5::new(rom),
            _ => banking_controller = MBC1::new(rom),
        }



        let mut title = String::new();
        let mut i: u16 = 0x134;
        while i < 0x142 {
            let chr = &rom[i] as char;
            if chr != (0x00 as char) {
                title.push(chr);
            }
            i+=1;
        }
        let cart = Cart {
            banking_controller,
            title: String::from(title.trim()),
            filename,
            mode,
        };
        match flag {
            0x3| 0x6| 0x9| 0xD| 0xF| 0x10| 0x13| 0x17| 0x1B| 0x1E| 0xFF => {
                cart.load_save_data();
            },
            _ => {}
        }

        return cart;
    }

    pub fn read_rom_data(filename: String) -> Vec<u8> {
        let file = File::open(filename);
        if file.is_err() {
            // TODO
            return Vec::new();
        }
        let mut contents: Vec<u8> = Vec::new();
        file.unwrap().read_to_end(&mut contents);
        return contents;
    }
}

pub fn new_cart(filename: String) -> Cart{

}




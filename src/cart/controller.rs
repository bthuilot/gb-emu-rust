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

pub struct Cart {
    banking_controller: Box<dyn BankingController>,
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
        let mut file = self.filename.clone();
        file.push_str(".sav");
        let mut file_result = File::open(file.as_str());
        if !file_result.is_err() {
            let mut file = file_result.unwrap();
            file.read_to_end(&mut save_data).expect("");
        }
        self.banking_controller.load_save_data(save_data);
    }

    pub fn save(&self) {
        let data = self.banking_controller.get_save_date();
        if data.len() > 0 {
            let mut file = self.filename.clone();
            file.push_str(".sav");
            let mut file = File::open(file.as_str()).expect("Unable to open file");
            let result = file.write(data.as_slice());
            if result.is_err() {
                // TODO Handle this
            }
        }
    }

    fn get_banking_controller(flag: u8, rom: Vec<u8>) -> Box<dyn  BankingController> {
        match flag {
            0x00 | 0x08 | 0x09 | 0x0B | 0x0C | 0x0D => return Box::new(ROM::new_as_bc(rom)),
            0..=0x03 => return Box::new(MBC1::new_as_bc(rom)),
            0x03..=0x06 => return Box::new(MBC2::new_as_bc(rom)),
            0x06..=0x13 => return Box::new(MBC3::new_as_bc(rom)),
            0x13..=0x17 => return Box::new(MBC1::new_as_bc(rom)),
            0x17..=0x1F => return Box::new(MBC5::new_as_bc(rom)),
            _ => return Box::new(MBC1::new_as_bc(rom)),
        }
    }

    pub fn new(filename: &str) -> Cart{
        let rom = Cart::read_rom_data(String::from(filename));
        let mode: u8;
        match rom[0x0143] {
            0x80 => mode = BOTH_MODE,
            0xC0 => mode = CBG_MODE,
            _ => mode = GB_MODE,
        }

        let flag = rom[0x147];
        let banking_controller = Cart::get_banking_controller(flag, rom.clone());

        let mut title = String::new();
        let mut i: usize = 0x134;
        while i < 0x142 {
            let chr = &rom[i as usize];
            if *chr != (0x00) {
                title.push((*chr) as char);
            }
            i+=1;
        }

        let mut cart = Cart {
            banking_controller,
            title: String::from(title.trim()),
            filename: String::from(filename),
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

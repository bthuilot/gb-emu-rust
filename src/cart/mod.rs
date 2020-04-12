pub(crate) mod controller;
mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;
mod rom;

// BankingController provides methods for accessing and writing data to a
// cartridge, which provides different banking functionality depending on
// the implementation.
pub trait BankingController {
    // Read returns the value from the cartridges ROM or RAM depending on
    // the banking.
    fn read(&self, address: u16) -> u8;

    // WriteROM attempts to write a value to an address in ROM. This is
    // generally used for switching memory banks depending on the implementation.
    fn write_rom(&mut self, address: u16, value: u8);

    // WriteRAM sets a value on an address in the internal cartridge RAM.
    // Like the ROM, this can be banked depending on the implementation
    // of the memory controller. Furthermore, if the cartridge supports
    // RAM+BATTERY, then this data can be saved between sessions.
    fn write_ram(&mut self, address: u16, value: u8);

    // GetSaveData returns the save data for this banking controller. In
    // general this will the contents of the RAM, however controllers may
    // choose to store this data in their own format.
    fn get_save_date(&self) -> Vec<u8>;

    // LoadSaveData loads some save data into the cartridge. The banking
    // controller implementation can decide how this data should be loaded.
    fn load_save_data(&mut self, data: Vec<u8>);
}

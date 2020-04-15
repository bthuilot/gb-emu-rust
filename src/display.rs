use crate::gameboy::Gameboy;
use crate::bit_functions::{reset, set, test};

pub const SCREEN_WIDTH: u8 = 160;
pub const SCREEN_HEIGHT: u8 = 144;
pub const LCDC: u16 = 0xFF40;
const LCD_MODE_2_BOUNDS: isize = 456- 80;
const LCD_MODE_3_BOUNDS: isize = LCD_MODE_2_BOUNDS - 172;

impl Gameboy {
    pub fn update_graphics(&mut self, cycles: isize) {
        self.set_lcd_status();

        if !self.is_lcd_enabled() {
            return
        }

        self.scanline_counter -= cycles;
        if self.scanline_counter <=0 {
            self.memory.ram[0x44] += 1;
            if self.memory.ram[0x44] > 153 {
                self.prepared_screen.clone_from_slice(&self.screen_data);
                self.screen_data = [[[0; 3]; SCREEN_HEIGHT]; SCREEN_WIDTH];
                self.bg_priority = [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH];
                self.memory.ram[0x44] = 0;
            }

            let current = self.memory.read_upper_ram(0xFF44);
            self.scanline_counter += 465 * (self.memory.speed.current as isize);

            if current == SCREEN_HEIGHT {
                self.request_interrupt(0_u8);
            }
        }
    }

    pub fn set_lcd_status(&mut self) {
        let mut status = self.memory.read_upper_ram(0xFF41);

        if !self.is_lcd_enabled() {
            self.clear_screen();

            self.scanline_counter = 456;
            self.memory.ram[0x44] = 0;
            status &= 252;
            status = reset(status, 0);
            status = reset(status, 1);
            self.memory.write(0xFF41, status);
            return
        }

        self.screen_cleared = false;
        let current_line = self.memory.read_upper_ram(0xFF44);
        let current_mode = status & 0x3;

        let mode: u8;
        let request_interrupt: bool;
        if current_line >= 144 {
            mode = 1;
            status = set(status, 0);
            status = reset(status, 1);
            request_interrupt = test(status, 4);
        } else if self.scanline_counter >= LCD_MODE_2_BOUNDS {
            mode = 2;
            status = reset(status, 0);
            status = set(status, 1);
            request_interrupt = test(status, 5);
        } else if self.scanline_counter >= LCD_MODE_3_BOUNDS {
            mode = 2;
            status = set(status, 0);
            status = set(status, 1);
            request_interrupt = false;
            if mode != current_mode {
                self.draw_scanline(current_line);
            }
        } else {
            mode = 0;
            status = reset(status, 0);
            status = reset(status, 1);
            request_interrupt = test(status, 3);
            if mode != current_mode {
                self.memory.hdma_transfer();
            }
        }

        if request_interrupt && mode != current_mode {
            self.request_interrupt(1);
        }

        if current_line == self.memory.read_upper_ram(0xFF45) {
            status = set(status, 2);
            if test(status,  6) {
                self.request_interrupt(1);
            }
        } else {
            status = reset(status, 2);
        }

        self.memory.write(0xFF41, status);
    }

    fn is_lcd_enabled(&self) -> bool {
        return test(self.memory.read_upper_ram(LCDC), 7);
    }

    fn draw_scanline(&self, scanline: u8) {
        control = self.memory.read_upper_ram(LCDC);

        if self.cgb_mode || test(control, 0) {
            self.render_tiles(control, scanline);
        }

        if test(control, 1) {
            self.render_sprites(control, scanline as i32);
        }
    }
}
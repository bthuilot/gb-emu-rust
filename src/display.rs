use crate::gameboy::Gameboy;
use crate::bit_functions::{reset, set, test, val};

pub struct ColorPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Clone for ColorPixel {
    fn clone(&self) -> Self {
        return ColorPixel {
            r: self.r.clone(),
            b: self.b.clone(),
            g: self.g.clone()
        }
    }
}

impl Copy for ColorPixel {

}

pub const SCREEN_WIDTH: u8 = 160;
pub const SCREEN_HEIGHT: u8 = 144;

const LCDC: u16 = 0xFF40;
const LCD_MODE_2_BOUNDS: isize = 456- 80;
const LCD_MODE_3_BOUNDS: isize = LCD_MODE_2_BOUNDS - 172;
const SPRITE_PRIORITY_OFFSET: isize = 100;

pub struct TileSettings {
    using_window: bool,
    unsigned: bool,
    tile_data: u16,
    background_memory: u16,
}


impl Gameboy {
    pub fn update_graphics(&mut self, cycles: isize) {
        self.set_lcd_status();

        if !self.is_lcd_enabled() {
            return
        }


        self.scanline_counter -= cycles;
        if self.scanline_counter <=0 {
            self.memory.ram[0x44] = self.memory.ram[0x44].wrapping_add(1);
            if self.memory.ram[0x44] > 153 {
                self.prepared_screen.clone_from_slice(&self.screen_data);
                self.screen_data = [[ColorPixel { r: 0, g: 0, b:255 }; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize];
                self.bg_priority = [[false; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize];
                self.memory.ram[0x44] = 0;
            }

            let current = self.read_upper_ram(0xFF44);
            self.scanline_counter += 465 * (self.memory.speed.current as isize);

            if current == SCREEN_HEIGHT {
                self.request_interrupt(0_u8);
            }
        }
    }

    pub fn set_lcd_status(&mut self) {
        let mut status = self.read_upper_ram(0xFF41);

        if !self.is_lcd_enabled() {
            self.clear_screen();
            // println!("Here");
            self.scanline_counter = 456;
            self.memory.ram[0x44] = 0;
            status &= 252;
            status = reset(status, 0);
            status = reset(status, 1);
            self.write(0xFF41, status);
            return
        }

        self.screen_cleared = false;
        let current_line = self.read_upper_ram(0xFF44);
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
            mode = 3;
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
                self.hdma_transfer();
            }
        }

        if request_interrupt && mode != current_mode {
            self.request_interrupt(1);
        }

        if current_line == self.read_upper_ram(0xFF45) {
            status = set(status, 2);
            if test(status,  6) {
                self.request_interrupt(1);
            }
        } else {
            status = reset(status, 2);
        }

        self.write(0xFF41, status);
    }

    fn is_lcd_enabled(&self) -> bool {
        return test(self.read_upper_ram(LCDC), 7);
    }

    fn draw_scanline(&mut self, scanline: u8) {
        let control = self.read_upper_ram(LCDC);

        if self.cgb_mode || test(control, 0) {
            self.render_tiles(control, scanline);
        }

        if test(control, 1) {
            self.render_sprite(control, scanline as i32);
        }
    }

    pub fn get_tile_settings(&self, lcd_control: u8, window_y: u8) -> TileSettings {
        let mut tile_data = 0x8800_u16;
        let mut unsigned = false;
        let mut using_window = false;
        if test(lcd_control, 5) {
            if window_y < self.read(0xFF44) {
                using_window = true;
            }
        }

        if test(lcd_control, 4) {
            tile_data = 0x8000;
            unsigned = true;
        }

        let mut background_memory = 0x9800_u16;
        if test(lcd_control, if using_window {6} else {3}) {
            background_memory = 0x9C00_u16;
        }

        return TileSettings {
            using_window,
            unsigned,
            tile_data,
            background_memory
        }
    }

    pub fn render_tiles(&mut self, lcd_control: u8, scanline: u8) {
        let scroll_y = self.read_upper_ram(0xFF42);
        let scroll_x = self.read_upper_ram(0xFF43);
        let window_y = self.read_upper_ram(0xFF4A);
        let window_x: u8 = self.read_upper_ram(0xFF4B).wrapping_sub(7);

        let settings = self.get_tile_settings(lcd_control, window_y);
        let y_pos = if !settings.using_window {scroll_y.wrapping_add(scanline)} else {scanline.wrapping_sub(window_y)};

        let tile_row = (y_pos/8) as u16 * 32_u16;

        let palette = self.read_upper_ram(0xFF47);

        self.tile_scanline = [0; 160];
        for pixel in 0_u8..160_u8 {
            let mut x_pos: u8 = if !settings.using_window && (pixel) >= (window_x) {
                (pixel).wrapping_add(scroll_x)
            } else {
                (pixel).wrapping_sub(window_x )
            };

            let tile_col = (x_pos/8) as u16;

            let tile_address = settings.background_memory + tile_row + tile_col;

            let mut tile_location = settings.tile_data;
            let tile_num: i16;
            if settings.unsigned {
                tile_num = self.memory.vram[tile_address.wrapping_sub(0x8000) as usize] as i16;
                tile_location = tile_location.wrapping_add(tile_num.wrapping_mul(16) as u16);
            } else {
                tile_num = (self.memory.vram[tile_address.wrapping_sub(0x8000) as usize] as i8)as i16;
                tile_location = (tile_location as i32).wrapping_add((tile_num.wrapping_add(128)).wrapping_mul(16) as i32) as u16;
            }

            let mut bank_offset = 0x8000_u16;
            // Attributes used in CGB mode TODO: check in CGB mode
            //
            //    Bit 0-2  Background Palette number  (BGP0-7)
            //    Bit 3    Tile VRAM Bank number      (0=Bank 0, 1=Bank 1)
            //    Bit 5    Horizontal Flip            (0=Normal, 1=Mirror horizontally)
            //    Bit 6    Vertical Flip              (0=Normal, 1=Mirror vertically)
            //    Bit 7    BG-to-OAM Priority         (0=Use OAM priority bit, 1=BG Priority)
            let tile_attr = self.memory.vram[tile_address.wrapping_sub(0x6000) as usize];
            if self.cgb_mode && test(tile_attr, 3) {
                bank_offset = 0x6000_u16;
            }
            let priority = test(tile_attr, 7);
            let line: u8;
            if self.cgb_mode && test(tile_attr, 6) {
                line = ((7_u8.wrapping_sub(y_pos)) % 8).wrapping_mul(2);
            } else {
                line = (y_pos % 8).wrapping_mul(2);
            }

            let data_1 = self.memory.vram[tile_location.wrapping_add(line as u16).wrapping_sub(bank_offset) as usize];
            let data_2 = self.memory.vram[tile_location.wrapping_add(line as u16).wrapping_add(1).wrapping_sub(bank_offset) as usize];

            if self.cgb_mode && test(tile_attr, 5) {
                x_pos = x_pos.wrapping_sub(7);
            }
            let color_bit = ((x_pos%8) as i8).wrapping_sub(7).wrapping_mul(-1) as u8;
            let color_num = (val(data_2, color_bit) << 1) | val(data_1, color_bit);
            self.set_tile_pixel(pixel, scanline, tile_attr, color_num, palette, priority);
        }
    }

    fn set_tile_pixel(&mut self, x: u8, y: u8, tile_attr: u8, color_num: u8, palette: u8, priority: bool) {
        if self.cgb_mode {
            let cgb_palette = tile_attr & 0x7;
            let color = self.bg_palette.get(cgb_palette, color_num);
            self.set_pixel(x,y, color, true);
        } else {
            let color = self.get_color(color_num, palette);
            self.set_pixel(x,y, color, true);
        }

        self.tile_scanline[x as usize] = color_num;
    }

    fn get_color(&self, color_num: u8, palette: u8) -> ColorPixel {
        let hi = color_num << 1 | 1;
        let lo = color_num << 1;
        let col = val(palette, hi) << 1 | val(palette, lo);
        return self.get_palette_color(col as usize);
    }

    fn render_sprite(&mut self, lcd_control: u8, scanline: i32) {
        let y_size = if test(lcd_control, 2) {16_i32} else {8_i32};

        let palette_1 = self.read_upper_ram(0xFF48);
        let palette_2 = self.read_upper_ram(0xFF49);

        let mut minx: [i32; SCREEN_WIDTH as usize ] = [0_i32; SCREEN_WIDTH as usize];
        let mut line_sprites = 0;
        for sprite in 0_u16..40_u16 {
            let index = sprite.wrapping_mul(4);
            let y_pos = (self.read(0xFE00_u16.wrapping_add(index)) as i32).wrapping_mul(16);
            if scanline < y_pos || scanline >= y_pos.wrapping_add(y_size) {
                continue
            }

            if line_sprites >= 10 {
                break;
            }

            line_sprites += 1;
            let x_pos = (self.read(0xFE00_u16.wrapping_add(index).wrapping_add(1)) as i32).wrapping_sub(8);
            let tile_location = self.read(0xFE00_u16.wrapping_add(index).wrapping_add(2)) as u16;
            let attributes = self.read(0xFE00_u16.wrapping_add(index).wrapping_add(3));

            let y_filp = test(attributes, 6);
            let x_flip = test(attributes, 5);
            let priority = test(attributes, 7);


            let bank = if self.cgb_mode && test(attributes, 3) {1_u16} else {0_u16};

            let mut line = scanline - y_pos;
            if y_filp {
                line = y_size.wrapping_sub(line).wrapping_sub(1);
            }

            let addr = (tile_location.wrapping_mul(16).wrapping_add(line.wrapping_mul(2) as u16)).wrapping_add(bank.wrapping_mul(0x2000));
            let data_1 = self.memory.vram[addr as usize];
            let data_2 = self.memory.vram[(addr.wrapping_add(1)) as usize];

            for tile_pixel in 0_u8..8_u8{
                let pixel = x_pos as i16 + (7_i16 - tile_pixel as i16);
                if pixel < 0 || pixel >= SCREEN_WIDTH as i16{
                    continue
                }

                if minx[pixel as usize] != 0 && (self.cgb_mode || minx[pixel as usize] <= x_pos+SPRITE_PRIORITY_OFFSET as i32) {
                    continue
                }

                let mut color_bit = if x_flip { ((tile_pixel - 7) as i8 * -1) as u8 } else {tile_pixel};

                let color_num = (val(data_2, color_bit) << 1) | val(data_1, color_bit);

                // Colour 0 is transparent for sprites
                if color_num == 0 {
                    continue
                }

                if self.cgb_mode {
                    let cgb_palette = attributes & 0x7;
                    let color = self.sprite_palette.get(cgb_palette, color_num);
                    self.set_pixel(pixel as u8, scanline as u8, color, priority);
                } else {
                    let palette = if test(attributes, 4) {palette_2} else {palette_1};
                    let color = self.get_color(color_num, palette);
                    self.set_pixel((pixel as u8), (scanline as u8), color, priority)
                }

                minx[pixel as usize] = x_pos + SPRITE_PRIORITY_OFFSET as i32;
            }
        }

    }

    fn set_pixel(&mut self, x: u8, y: u8, color: ColorPixel, priority: bool) {
        // If priority is false then sprite pixel is only set if tile colour is 0
        if (priority && !self.bg_priority[x as usize][y as usize]) || self.tile_scanline[x as usize] == 0 {
            self.screen_data[x as usize][y as usize] = color;
        }
    }

    pub fn clear_screen(&mut self) {
        if self.screen_cleared {
            return;
        }

        for x in 0..(self.screen_data.len()) {
            for y in 0..(self.screen_data[x].len()) {
                self.screen_data[x][y] = ColorPixel {
                    r:255,
                    g:255,
                    b:255
                }
            }
        }

        self.prepared_screen.clone_from_slice(&self.screen_data);
        self.screen_cleared = true;

    }
}
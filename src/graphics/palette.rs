use crate::gameboy::Gameboy;
use crate::bit_functions::test;
use crate::graphics::{ColorPixel, CGBPalette};

const COLOR_ARRAY: [u8; 0x20] = [
    0x0, 0x8, 0x10, 0x18, 0x20, 0x29, 0x31, 0x39,
    0x41, 0x4a, 0x52, 0x5a, 0x62, 0x6a, 0x73, 0x7b,
    0x83, 0x8b, 0x94, 0x9c, 0xa4, 0xac, 0xb4, 0xbd,
    0xc5, 0xcd, 0xd5, 0xde, 0xe6, 0xee, 0xf6, 0xff];

// Palettes is an mapping from colour palettes to their colour values
// to be used by the emulator.
const PALETTE: [[ColorPixel; 4]; 3] = [
    [ColorPixel {r: 255, g: 255, b: 255},
    ColorPixel {r: 204, g: 204, b: 204},
    ColorPixel {r:119, g: 119, b: 119},
    ColorPixel {r:0 , g:0, b:0}],
    [ColorPixel {r: 0x9B, g: 0xBC, b: 0x0F},
        ColorPixel {r: 0x8b, g: 0xAC, b: 0x0F},
        ColorPixel {r: 0x30, g: 0x62, b: 0x30},
        ColorPixel {r: 0x0F, g:0x38, b: 0x0F}],
    [ColorPixel {r: 0xE0, g: 0xF8, b: 0xD0},
        ColorPixel {r: 0x88, g: 0xC0, b: 0x70},
        ColorPixel {r: 0x34, g: 0x68, b: 0x56},
        ColorPixel {r: 0x08, g:0x18, b: 0x20}],
];


impl CGBPalette {

    pub fn new() -> CGBPalette {
        CGBPalette {
            palette: [0xFF; 0x40],
            index: 0,
            inc: false
        }
    }

    pub fn update_index(&mut self, value: u8) {
        self.index = value & 0x3f;
        self.inc = test(value, 7);
    }

    pub fn read(&self) -> u8 {
       return self.palette[self.index as usize];
    }

    pub fn write(&mut self, value: u8) {
        self.palette[self.index as usize] = value;
        if self.inc {
            self.index = self.index.wrapping_add(1) & 0x3F;
        }
    }

    pub fn get(&mut self, palette: u8, num: u8) -> ColorPixel {
        let index = (palette * 8) + (num * 2);
        // println!("index: {}", index);
        let color = self.palette[index as usize] as u16 | ((self.palette[(index+1) as usize] as u16) << 8);
        let red = (color & 0x1F) as u8;
        let green = ((color >> 5) & 0x1F) as u8;
        let blue = ((color >> 10) & 0x1F) as u8;
        let color = ColorPixel {
            r: COLOR_ARRAY[red as usize],
            g: COLOR_ARRAY[green as usize],
            b: COLOR_ARRAY[blue as usize],
        };
        return color;
    }
}


impl Gameboy {
    pub fn get_palette_color(&self, index: usize) -> ColorPixel {
        return PALETTE[self.current_palette][index];
    }
}



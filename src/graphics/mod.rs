mod palette;
mod rendering;


pub const PALETTE_GRAYSCALE: u8 = 0;
pub const PALETTE_ORIGINAL: u8 = 1;
pub const PALETTE_BGB: u8 = 2;

pub const SCREEN_WIDTH: u8 = 160;
pub const SCREEN_HEIGHT: u8 = 144;

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

impl Copy for ColorPixel {}

pub struct CGBPalette {
    palette: [u8; 0x40],
    pub index: u8,
    pub inc: bool,
}

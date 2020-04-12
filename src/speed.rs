
pub const CLOCK_SPEED: usize = 4194304;
pub const FRAMES_PER_SECOND: usize = 60;
pub const CYCLES_FRAME: usize = CLOCK_SPEED / FRAMES_PER_SECOND;

pub struct Speed {
    pub current: u8,
    pub prepare: bool,
}

impl Speed {

}
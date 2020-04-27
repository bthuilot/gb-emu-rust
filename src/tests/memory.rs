#[cfg(test)]
use mockall::{automock, mock, predicate::*};

cfg_if! {
    if #[cfg(test)] {
        use crate::memory::MockMMU as MMU;
    } else {
        use crate::memory::MMU;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    pub fn setup() -> MMU {
        let mut mock = ;
        return MMU {
            cart: (),
            timer: Timer {},
            input: Input {},
            speed: Speed {},
            ram: [],
            vram: [],
            vram_bank: 0,
            wram: [],
            wram_bank: 0,
            oam: [],
            hdma_len: 0,
            hdma_active: false
        }
    }

}
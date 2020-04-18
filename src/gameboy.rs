use crate::bit_functions::{reset, set, test};
use crate::cpu::{Z80};
use crate::graphics::{ColorPixel, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::memory::{DIV, MMU, TIMA, TMA, CYCLES_FRAME};
use crate::graphics::{CGBPalette, PALETTE_BGB};

pub struct Gameboy {
    pub memory: MMU,
    pub cpu: Z80,
    // TODO: Sounds
    paused: bool,

    pub working_screen: [[ColorPixel; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],
    pub bg_priority: [[bool; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],

    pub tile_scanline: [u8; SCREEN_WIDTH as usize],
    pub scanline_counter: isize,
    pub cleared: bool,

    pub rendered_screen: [[ColorPixel; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],

    pub interrupts_enabling: bool,
    pub interrupts_on: bool,
    pub halted: bool,

    pub cgb_mode: bool,
    pub current_palette: usize,
    pub bg_palette: CGBPalette,
    pub sprite_palette: CGBPalette,
}

impl Gameboy {
    pub fn update(&mut self) -> usize {
        if self.paused {
            return 0;
        }

        let mut cycles = 0;
        while cycles < CYCLES_FRAME * (self.memory.speed.current as usize + 1) {
            let mut cycles_op = 4;
            if !self.halted {
                cycles_op = self.execute_next_opcode();
            } else {
                // TODO: Something here
            }
            cycles += cycles_op;
            self.update_graphics(cycles_op as isize);
            self.update_timers(cycles_op);
            cycles += self.do_interrupts();
        }
        return cycles;
    }

    pub fn update_timers(&mut self, cycles: usize) {
        self.divider_register(cycles);
        if self.is_clock_enabled() {
            self.memory.timer.value += cycles;

            let freq = self.get_clock_freq_count();
            while self.memory.timer.value >= freq {
                self.memory.timer.value = self.memory.timer.value.wrapping_sub(freq);
                let tima = self.read(TIMA);
                if tima == 0xFF {
                    self.memory.ram[TIMA.wrapping_sub(0xFF00) as usize] = self.read(TMA);
                    self.request_interrupt(2);
                } else {
                    self.memory.ram[TIMA.wrapping_sub(0xFF00) as usize] = tima.wrapping_add(1);
                }
            }
        }
    }

    pub fn get_clock_freq_count(&self) -> usize {
        return match self.get_clock_freq() {
            0 => 1024,
            1 => 16,
            2 => 64,
            _ => 256,
        };
    }

    pub fn divider_register(&mut self, cycles: usize) {
        self.cpu.divider += cycles;
        if self.cpu.divider >= 255 {
            self.cpu.divider -= 255;
            self.memory.ram[(DIV - 0xFF00) as usize] =
                self.memory.ram[(DIV - 0xFF00) as usize].wrapping_add(1);
        }
    }

    pub fn request_interrupt(&mut self, interrupt: u8) {
        let mut req = self.read_upper_ram(0xFF0F);
        req = set(req, interrupt);
        self.write(0xFF0F, req);
    }

    pub fn do_interrupts(&mut self) -> usize {
        if self.interrupts_enabling {
            self.interrupts_on = true;
            self.interrupts_enabling = false;
            return 0;
        }
        if !self.interrupts_on && !self.halted {
            return 0;
        }
        let req = self.read_upper_ram(0xFF0F);
        let enabled = self.read_upper_ram(0xFFFF);
        if req > 0 {
            for i in 0_u8..5 {
                if test(req, i) && test(enabled, i) {
                    self.service_interrupt(i);
                    return 20;
                }
            }
        }
        return 0;
    }

    pub fn service_interrupt(&mut self, interrupt: u8) {
        if !self.interrupts_on && self.halted {
            self.halted = false;
            return;
        }
        self.interrupts_on = false;
        self.halted = false;

        let mut req = self.read_upper_ram(0xFF0F);
        req = reset(req, interrupt);
        self.write(0xFF0F, req);

        self.push_stack(self.cpu.pc);
        self.cpu.pc = match interrupt {
            0 => 0x40,
            1 => 0x48,
            2 => 0x50,
            3 => 0x58,
            4 => 0x60,
            _ => 0x00, // Shouldnt happen
        }
    }

    pub fn new(rom: &str) -> Gameboy {
        let mut cpu = Z80::new();
        let mut memory = MMU::new(rom);
        memory.init();
        let cgb_mode = memory.has_cgb_mode();
        cpu.init(cgb_mode);
        return Gameboy {
            memory,
            cpu,
            paused: false,
            working_screen: [[ColorPixel {
                r: 255,
                b: 255,
                g: 255,
            }; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],
            bg_priority: [[false; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],
            tile_scanline: [0; SCREEN_WIDTH as usize],
            scanline_counter: 456,
            cleared: false,
            rendered_screen: [[ColorPixel {
                r: 255,
                b: 255,
                g: 255,
            }; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],
            interrupts_enabling: false,
            interrupts_on: false,
            halted: false,
            cgb_mode,
            current_palette: PALETTE_BGB as usize,
            bg_palette: CGBPalette::new(),
            sprite_palette: CGBPalette::new(),
        };
    }
}

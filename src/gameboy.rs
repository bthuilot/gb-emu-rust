use crate::cpu::Z80;
use crate::memory::{MMU, DIV, TIMA, TMA, TAC};
use crate::input::{Input, Button};
use crate::speed::{CYCLES_FRAME, Speed};
use crate::bit_functions::{test, set, reset};




pub struct Options {
    pub sound: bool,
    pub cgb: bool,
}


pub struct Timer {
    pub value: usize,
}

impl Timer {


    // TODO Fix Clock
    pub fn is_clock_enabled(&self) -> bool {
        return test(self.mmu.rb(TAC), 2)
    }

    pub fn get_clock_freq(&self) -> u8 {
        return self.mmu.read(TAC) & 0x3
    }

    pub fn reset_timer(&mut self) {
        self.value = 0;
    }
}



pub struct Gameboy {
    memory: &'static mut MMU,
    pub cpu: &'static mut Z80,
//    Sound  *apu.APU

    paused: bool,
    timer: &'static mut Timer,

    // Matrix of pixel data which is used while the screen is rendering. When a
    // frame has been completed, this data is copied into the PreparedData matrix.
//    screenData [ScreenWidth][ScreenHeight][3]uint8
//    bgPriority [ScreenWidth][ScreenHeight]bool

    // Track colour of tiles in scanline for priority management.
//    tileScanline    [ScreenWidth]uint8
//    scanlineCounter int
//    screenCleared   bool

    // PreparedData is a matrix of screen pixel data for a single frame which has
    // been fully rendered.
//    PreparedData [ScreenWidth][ScreenHeight][3]uint8

    pub interrupts_enabling: bool,
    pub interrupts_on:       bool,
    pub halted:             bool,

    // Mask of the currenly pressed buttons.
    input: &'static mut Input,

    // Flag if the game is running in cgb mode. For this to be true the game
    // rom must support cgb mode and the option be true.
    pub cgb_mode:       bool,
//    BGPalette     *cgbPalette
//    SpritePalette *cgbPalette

    pub speed: &'static mut Speed,
    current_speed: u8,
    prepare_speed: bool,
}


impl Gameboy {
    pub fn update(&mut self) -> usize {
        if self.paused {
            return 0
        }

        let mut cycles = 0;
        while cycles < CYCLES_FRAME * (self.speed.current + 1) as usize {
            let mut cycles_op = 4;
            if !self.halted {
//                if gb.Debug.OutputOpcodes {
//                    LogOpcode(gb, false)
//                }
                cycles_op = self.execute_next_opcode();
            } else {
                // TODO: This is incorrect
            }
            cycles += cycles_op;
            // TODO update graphics
//            gb.updateGraphics(cycles_op);
            self.update_timers(cycles_op);
            cycles += self.do_interrupts();
        }
        return cycles
    }

    // BGMapString returns a string of the values in the background map.
    pub fn bg_map_string(&mut self) -> String {
        let mut out = String::new();
        let mut y: u16 = 0;
        while y < 0x20 {
            out.push_str(format!("{:2x}", y).as_str());
            let mut x: u16 = 0;
            while x < 0x20 {
                out.push_str(format!("{:2x}", self.memory.read(0x9800_u16.wrapping_add(y*0x20).wrapping_add(x))).as_str());
                x += 1;
            }
            out.push_str("\n");
            y += 1;
        }
        return out
    }

    pub fn check_speed(&mut self) {
        if self.speed.prepare {
            self.speed.prepare = false;
            self.speed.current = if self.speed.current == 0 {1} else {0};
            self.halted = false;
        }
    }

    pub fn update_timers(&mut self, cycles: usize) {
        self.divider_register(cycles);
        if self.timer.is_clock_enabled() {
            self.timer.value += cycles;

            let freq = self.get_clock_freq_count();
            while self.timer.value >= freq {
                self.timer.value -= freq;
                let tima = self.memory.read(TIMA);
                if tima == 0xFF {
                    self.memory.ram[TIMA - 0xFF00] = self.memory.read(TMA);
                    self.request_interrupt(2);
                } else {
                    self.memory.ram[TIMA-0xFF00] = tima + 1;
                }
            }
        }
    }


    pub fn press_button(&mut self, button: Button) {
        self.input.mask = reset(self.input.mask, button);
        self.request_interrupt(4);
    }

    pub fn release_button(&mut self, button: Button) {
        self.input.mask = set(self.input.mask, button)
    }

    pub fn get_clock_freq_count(&self) -> usize {
        return match self.get_clock_freq() {
            0 => 1024,
            1 => 16,
            2 => 64,
            _ => 256,
        }
    }

    pub fn divider_register(&mut self, cycles: usize) {
        self.cpu.divider += cycles;
        if self.cpu.divider >= 255 {
            self.cpu.divider -= 255;
            self.memory.ram[DIV-0xFF00] += 1
        }
    }

    pub fn request_interrupt(&mut self, interrupt: u8) {
        let mut req = self.memory.read_upper_ram(0xFF0F);
        req = set(req, interrupt);
        self.memory.wb(0xFF0F, req)
    }

    pub fn do_interrupts(&mut self) -> usize {
        if self.interrupts_enabling {
            self.interrupts_on = true;
            self.interrupts_enabling = false;
            return 0;
        }
        if !self.interrupts_on && self.halted {
            return 0;
        }
        let req = self.memory.read_upper_ram(0xFF0F);
        let enabled = self.memory.read_upper_ram(0xFFFF);
        if req > 0 {
            let mut i: u8 = 0;
            while i < 5 {
                if test(req, i) && test(enabled, i) {
                    self.service_interrupt(i);
                    return 20;
                }
                i += 1;
            }
        }
        return 0;
    }

    pub fn service_interrupt(&mut self, interrupt: u8) {
        if !self.interrupts_on && self.halted {
            self.halted = false;
            return
        }
        self.interrupts_on = false;
        self.halted = false;

        let mut req = self.memory.read_upper_ram(0xFF0F);
        req = reset(req, interrupt);
        self.memory.write(0xFF0F, req);

        self.cpu.push_stack(self.cpu.pc);
        self.cpu.pc = match interrupt {
            0 => 0x40,
            1 => 0x48,
            2 => 0x50,
            3 => 0x58,
            4 => 0x60,
            _ => 0x00, // Shouldnt happen
        }

    }

    pub fn is_game_loaded(&self) -> bool {
        return self.memory.cart.title.len() != 0 // TODO
    }

    pub fn new(rom: String, options: Options) -> Gameboy {
        let mut timer = Timer { value: 0 };
        let mut speed = Speed { current: 0, prepare: false};
        let mut input = Input{mask: 0};
        let mut memory = MMU::new(rom, &timer, &mut input, &mut speed);
        let mut cpu = Z80::new(&memory);
        cpu.init(options.cgb);
        return Gameboy {
            memory: &mut memory,
            cpu: &mut cpu,
            paused: false,
            timer: &mut timer,
            interrupts_enabling: false,
            interrupts_on: false,
            halted: false,
            input: &mut input,
            cgb_mode: true,
            speed: &mut speed,
            current_speed: 0,
            prepare_speed: false
        };
    }
}


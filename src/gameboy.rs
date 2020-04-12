use crate::cpu::Z80;
use crate::memory::MMU;
use crate::input::{Input, Button};
use crate::speed::{CYCLES_FRAME, Speed};
use crate::bit_functions::{test, set, reset};
use std::ptr::null;
use crate::ops::ops::find_op;




pub struct Options {
    pub sound: bool,
    pub cgb: bool,
}


pub struct Timer {
    pub value: usize,
}

impl Timer {

    pub fn is_clock_enabled(&self) -> bool {
        return test(self.mmu.rb(TAC), 2)
    }

    pub fn get_clock_freq(&self) -> u8 {
        return self.mmu.rb(TAC) & 0x3
    }

    pub fn reset_timer(&mut self) {
        self.timer.value = 0;
    }
}



pub struct Gameboy {
    memory: &'static mut MMU,
    cpu: &'static mut Z80,
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

    interrupts_enabling: bool,
    interrupts_on:       bool,
    halted:             bool,

    // Mask of the currenly pressed buttons.
    input: &'static mut Input,

    // Flag if the game is running in cgb mode. For this to be true the game
    // rom must support cgb mode and the option be true.
    cgb_mode:       bool,
//    BGPalette     *cgbPalette
//    SpritePalette *cgbPalette

    speed: &'static mut Speed,
    current_speed: u8,
    prepare_speed: bool,
}


impl Gameboy {
    pub fn update(&mut self) -> usize {
        if self.paused {
            return 0
        }

        let cycle = 0;
        while cycles < CYCLES_FRAME * (self.speed.current + 1) as usize {
            let mut cycles_op = 4;
            if !self.halted {
//                if gb.Debug.OutputOpcodes {
//                    LogOpcode(gb, false)
//                }
                cycles_op = self.cpu.execute_next_opcode();
            } else {
                // TODO: This is incorrect
            }
            cycles += cycles_op;
            // TODO update
//            gb.updateGraphics(cycles_op);
            self.update_timers(cycles_op);
            cycles += self.do_interrupts();
        }
        return cycles
    }

    // BGMapString returns a string of the values in the background map.
    pub fn bg_map_string(&mut self) -> String {
        out = String::new();
        y: u16 = 0;
        while y < 0x20 {
            out.push_str(format!("{:2x}", y));
            x: u16 = 0;
            while x < 0x20 {
                out.push_str(fomrat!("{:2x}", self.memory.rb(0x9800.wrapping_add(y*0x20).wrapping_add(x), self.cpu.pc)));
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
                let tima = self.memory.read(TIMA, self.cpu.pc);
                if tima == 0xFF {
                    gb.Memory.HighRAM[TIMA - 0xFF00] = self.memory.read(TMA, self.cpu.pc);
                    self.request_interrupt(2);
                } else {
                    gb.Memory.HighRAM[TIMA - 0xFF00] = tima + 1
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
            gb.Memory.HighRAM[DIV-0xFF00]+=1
        }
    }

    pub fn request_interrupt(&mut self, interrupt: u8) {
        let mut req = self.memory.ReadHighRam(0xFF0F);
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
        let req = self.memory.ReadHighRam(0xFF0F);
        let enabled = self.memory.ReadHighRam(0xFFFF);
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

        let mut req = self.memory.ReadHighRam(0xFF0F);
        req = reset(req, interrupt);
        self.memory.wb(0xFF0F, req);

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

    // Push a 16 bit value onto the stack and decrement SP.
    pub fn push_stack(&mut self, address: u16) {
        let sp = self.cpu.sp.full();
        self.memory.wb(sp.wrapping_sub(1), (address&0xFF00).wrapping_shr(8) as u8);
        self.memory.wb(sp.wrapping_sub(2), (address&0xFF) as u8);
        self.cpu.sp.set_full(sp.wrapping_sub(2));
    }

    // Pop the next 16 bit value off the stack and increment SP.
    pub fn pop_stack(&mut self) -> u16 {
        let sp = self.cpu.sp.full();
        let low= self.memory.read(sp, self.cpu.pc) as u16;
        let hi = (self.memory.read(sp + 1, self.cpu.pc) as u16).wrapping_shl(8);
        self.cpu.sp.set_full(sp.wrapping_add(2));
        return low | hi;
    }

    pub fn is_game_loaded(&mut self) -> bool {
        return self.memory.cart.title.len() != 0 // TODO
    }

    pub fn new(rom: string, options: Options) -> Gameboy {
        let mut timer = Timer { value: 0 };
        let mut speed = Speed { current: 0, prepare: false};
        let mut input = Input{mask: 0};
        let mut memory = MMU::new(rom, &timer, &mut input);
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
            cgb_mode: has_cgb,
            speed: &mut speed,
            current_speed: 0,
            prepare_speed: false
        };
    }
}


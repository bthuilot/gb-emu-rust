use crate::cpu::Z80;
use crate::memory::{MMU, DIV, TIMA, TMA, TAC, MemoryAddr};
use crate::input::{Input, Button};
use crate::speed::{CYCLES_FRAME, Speed};
use crate::bit_functions::{test, set, reset};
use crate::display::{SCREEN_HEIGHT, SCREEN_WIDTH}

pub struct Options {
    pub sound: bool,
    pub cgb: bool,
}

pub struct Gameboy {
    pub memory: MMU,
    pub cpu: Z80,
//    Sound  *apu.APU

    paused: bool,

    pub screen_data: [[[u8; 3]; SCREEN_HEIGHT]; SCREEN_WIDTH],
    pub bg_priority: [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH],

    // Track colour of tiles in scanline for priority management.
    pub tile_scanline: [u8; SCREEN_WIDTH],
    pub scanline_counter: isize,
    pub screen_cleared: bool,

    pub prepared_screen: [[[u8; 3]; SCREEN_HEIGHT]; SCREEN_WIDTH],

    pub interrupts_enabling: bool,
    pub interrupts_on:       bool,
    pub halted:             bool,

    // Flag if the game is running in cgb mode. For this to be true the game
    // rom must support cgb mode and the option be true.
    pub cgb_mode:       bool,
//    BGPalette     *cgbPalette
//    SpritePalette *cgbPalette
}


impl Gameboy {
    pub fn update(&mut self) -> usize {
        if self.paused {
            return 0
        }

        let mut cycles = 0;
        while cycles < CYCLES_FRAME * (self.memory.speed.current + 1) as usize {
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
            self.update_graphics(cycles_op as isize);
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
        if self.memory.speed.prepare {
            self.memory.speed.prepare = false;
            self.memory.speed.current = if self.memory.speed.current == 0 {1} else {0};
            self.halted = false;
        }
    }

    pub fn update_timers(&mut self, cycles: usize) {
        self.divider_register(cycles);
        if self.memory.is_clock_enabled() {
            self.memory.timer.value += cycles;

            let freq = self.get_clock_freq_count();
            while self.memory.timer.value >= freq {
                self.memory.timer.value -= freq;
                let tima = self.memory.read(TIMA);
                if tima == 0xFF {
                    self.memory.ram[(TIMA - 0xFF00) as usize] = self.memory.read(TMA);
                    self.request_interrupt(2);
                } else {
                    self.memory.ram[(TIMA-0xFF00) as usize] = tima + 1;
                }
            }
        }
    }

    pub fn press_button(&mut self, button: Button) {
        self.memory.input.mask = reset(self.memory.input.mask, button);
        self.request_interrupt(4);
    }

    pub fn release_button(&mut self, button: Button) {
        self.memory.input.mask = set(self.memory.input.mask, button)
    }

    pub fn get_clock_freq_count(&self) -> usize {
        return match self.memory.get_clock_freq() {
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
            self.memory.ram[(DIV-0xFF00) as usize] += 1
        }
    }

    pub fn request_interrupt(&mut self, interrupt: u8) {
        let mut req = self.memory.read_upper_ram(0xFF0F);
        req = set(req, interrupt);
        self.memory.write(0xFF0F, req);
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

    pub fn pop_pc(&mut self) -> u8{
        let opcode = self.memory.read(self.cpu.pc);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        return opcode
    }

    pub fn pop_pc16(&mut self) -> u16 {
        let byte_1 = self.pop_pc() as u16;
        let byte_2 = self.pop_pc() as u16;
        return byte_2.wrapping_shl(8) | byte_1;
    }

    pub fn pop_stack(&mut self) -> u16{
        let sp = self.cpu.sp.full();
        let lo = self.memory.read(sp) as u16;
        let hi = (self.memory.read(sp+1) as u16).wrapping_shl(8);
        self.cpu.sp.set_full(sp + 2);
        return lo | hi;
    }

    pub fn push_stack(&mut self, addr: MemoryAddr) {
        let sp = self.cpu.sp.full();
        self.memory.write(sp-1, (addr & 0xFF00).wrapping_shr(8) as u8);
        self.memory.write(sp-2, (addr & 0xFF) as u8);
        self.cpu.sp.set_full(sp-2)
    }


    pub fn call(&mut self, next: u16) {
        self.push_stack(self.cpu.pc);
        self.cpu.pc = next;
    }

    pub fn ret(&mut self) {
        self.cpu.pc = self.pop_stack();
    }


    pub fn new(rom: &str, options: Options) -> Gameboy {
        let mut cpu = Z80::new();
        cpu.init(options.cgb);
        return Gameboy {
            memory: MMU::new(rom),
            cpu,
            paused: false,
            interrupts_enabling: false,
            interrupts_on: false,
            halted: false,
            cgb_mode: true,
        };
    }
}


use crate::cpu::Z80;
use crate::gameboy::Gameboy;
use crate::cb_ops;

// OPCODE_CYCLES is the number of self.cpu cycles for each normal opcode.
pub(crate) const OPCODE_CYCLES: [u8; 256] = [
    1, 3, 2, 2, 1, 1, 2, 1, 5, 2, 2, 2, 1, 1, 2, 1, // 0;
    0, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1, 2, 1, // 1;
    2, 3, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 2;
    2, 3, 2, 2, 3, 3, 3, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 3;
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 4;
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 5;
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 6;
    2, 2, 2, 2, 2, 2, 0, 2, 1, 1, 1, 1, 1, 1, 2, 1, // 7;
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 8;
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 9;
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // a
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // b
    2, 3, 3, 4, 3, 4, 2, 4, 2, 4, 3, 0, 3, 6, 2, 4, // c
    2, 3, 3, 0, 3, 4, 2, 4, 2, 4, 3, 0, 3, 0, 2, 4, // d
    3, 3, 2, 0, 0, 4, 2, 4, 4, 1, 4, 0, 0, 0, 2, 4, // e
    3, 3, 2, 1, 0, 4, 2, 4, 3, 2, 4, 1, 0, 0, 2, 4, // f
]; //0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f

// CBOPCODE_CYCLES is the number of self.cpu cycles for each CB opcode.
const CBOPCODE_CYCLES: [u8; 256] = [
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 0;
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 1;
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 2;
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 3;
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 4;
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 5;
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 6;
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 7;
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 8;
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 9;
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // A
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // B
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // C
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // D
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // E
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // F
]; //0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f


impl Gameboy {
    pub fn execute_next_opcode(&mut self) -> usize {
        let opcode = self.pop_pc();
        println!("{}", opcode);
        self.cpu.clock.t = (OPCODE_CYCLES[opcode as usize] * 4) as usize;
        self.find_op(opcode);
        return self.cpu.clock.t;
    }

    pub fn find_op(&mut self, code: u8) {
        match code {
            0x06 => {
                // LD B, n
                let stack_val = self.pop_pc();
                self.cpu.bc.set_hi(stack_val);
            }
            0x0E => {
                // LD C, n
                let val = self.pop_pc();
self.cpu.bc.set_lo(val);
            }
            0x16 => {
                // LD D, n
                let val = self.pop_pc();
self.cpu.de.set_hi(val);
            }
            0x1E => {
                // LD E, n
                let val = self.pop_pc();
self.cpu.de.set_lo(val);
            }
            0x26 => {
                // LD H, n
                let val = self.pop_pc();
self.cpu.hl.set_hi(val);
            }
            0x2E => {
                // LD L, n
                let val = self.pop_pc();
self.cpu.hl.set_lo(val);
            }
            0x7F => {
                // LD A,A
                self.cpu.af.set_hi(self.cpu.af.hi());
            }
            0x78 => {
                // LD A,B
                self.cpu.af.set_hi(self.cpu.bc.hi());
            }
            0x79 => {
                // LD A,C
                self.cpu.af.set_hi(self.cpu.bc.lo());
            }
            0x7A => {
                // LD A,D
                self.cpu.af.set_hi(self.cpu.de.hi());
            }
            0x7B => {
                // LD A,E
                self.cpu.af.set_hi(self.cpu.de.lo());
            }
            0x7C => {
                // LD A,H
                self.cpu.af.set_hi(self.cpu.hl.hi());
            }
            0x7D => {
                // LD A,L
                self.cpu.af.set_hi(self.cpu.hl.lo());
            }
            0x0A => {
                // LD A,(bc);
                let val = self.read(self.cpu.bc.full());
                self.cpu.af.set_hi(val);
            }
            0x1A => {
                // LD A,(DE);
                let val = self.read(self.cpu.de.full());
                self.cpu.af.set_hi(val);
            }
            0x7E => {
                // LD A,(HL);
                let val = self.read(self.cpu.hl.full());
                self.cpu.af.set_hi(val);
            }
            0xFA => {
                // LD A,(nn);
                let addr = self.pop_pc16();
                let val = self.read(addr);
                self.cpu.af.set_hi(val);
            }
            0x3E => {
                // LD A,(nn);
                let val = self.pop_pc();
                self.cpu.af.set_hi(val);
            }
            0x47 => {
                // LD B,A
                self.cpu.bc.set_hi(self.cpu.af.hi());
            }
            0x40 => {
                // LD B,B
                self.cpu.bc.set_hi(self.cpu.bc.hi());
            }
            0x41 => {
                // LD B,C
                self.cpu.bc.set_hi(self.cpu.bc.lo());
            }
            0x42 => {
                // LD B,D
                self.cpu.bc.set_hi(self.cpu.de.hi());
            }
            0x43 => {
                // LD B,E
                self.cpu.bc.set_hi(self.cpu.de.lo());
            }
            0x44 => {
                // LD B,H
                self.cpu.bc.set_hi(self.cpu.hl.hi());
            }
            0x45 => {
                // LD B,L
                self.cpu.bc.set_hi(self.cpu.hl.lo());
            }
            0x46 => {
                // LD B,(HL);
                let val = self.read(self.cpu.hl.full());
                self.cpu.bc.set_hi(val);
            }
            0x4F => {
                // LD C,A
                self.cpu.bc.set_lo(self.cpu.af.hi());
            }
            0x48 => {
                // LD C,B
                self.cpu.bc.set_lo(self.cpu.bc.hi());
            }
            0x49 => {
                // LD C,C
                self.cpu.bc.set_lo(self.cpu.bc.lo());
            }
            0x4A => {
                // LD C,D
                self.cpu.bc.set_lo(self.cpu.de.hi());
            }
            0x4B => {
                // LD C,E
                self.cpu.bc.set_lo(self.cpu.de.lo());
            }
            0x4C => {
                // LD C,H
                self.cpu.bc.set_lo(self.cpu.hl.hi());
            }
            0x4D => {
                // LD C,L
                self.cpu.bc.set_lo(self.cpu.hl.lo());
            }
            0x4E => {
                // LD C,(HL);
                let val = self.read(self.cpu.hl.full());
                self.cpu.bc.set_lo(val);
            }
            0x57 => {
                // LD D,A
                self.cpu.de.set_hi(self.cpu.af.hi());
            }
            0x50 => {
                // LD D,B
                self.cpu.de.set_hi(self.cpu.bc.hi());
            }
            0x51 => {
                // LD D,C
                self.cpu.de.set_hi(self.cpu.bc.lo());
            }
            0x52 => {
                // LD D,D
                self.cpu.de.set_hi(self.cpu.de.hi());
            }
            0x53 => {
                // LD D,E
                self.cpu.de.set_hi(self.cpu.de.lo());
            }
            0x54 => {
                // LD D,H
                self.cpu.de.set_hi(self.cpu.hl.hi());
            }
            0x55 => {
                // LD D,L
                self.cpu.de.set_hi(self.cpu.hl.lo());
            }
            0x56 => {
                // LD D,(HL);
                let val = self.read(self.cpu.hl.full());
                self.cpu.de.set_hi(val);
            }
            0x5F => {
                // LD E,A
                self.cpu.de.set_lo(self.cpu.af.hi());
            }
            0x58 => {
                // LD E,B
                self.cpu.de.set_lo(self.cpu.bc.hi());
            }
            0x59 => {
                // LD E,C
                self.cpu.de.set_lo(self.cpu.bc.lo());
            }
            0x5A => {
                // LD E,D
                self.cpu.de.set_lo(self.cpu.de.hi());
            }
            0x5B => {
                // LD E,E
                self.cpu.de.set_lo(self.cpu.de.lo());
            }
            0x5C => {
                // LD E,H
                self.cpu.de.set_lo(self.cpu.hl.hi());
            }
            0x5D => {
                // LD E,L
                self.cpu.de.set_lo(self.cpu.hl.lo());
            }
            0x5E => {
                // LD E,(HL);
                let val = self.read(self.cpu.hl.full());
                self.cpu.de.set_lo(val);
            }
            0x67 => {
                // LD H,A
                self.cpu.hl.set_hi(self.cpu.af.hi());
            }
            0x60 => {
                // LD H,B
                self.cpu.hl.set_hi(self.cpu.bc.hi());
            }
            0x61 => {
                // LD H,C
                self.cpu.hl.set_hi(self.cpu.bc.lo());
            }
            0x62 => {
                // LD H,D
                self.cpu.hl.set_hi(self.cpu.de.hi());
            }
            0x63 => {
                // LD H,E
                self.cpu.hl.set_hi(self.cpu.de.lo());
            }
            0x64 => {
                // LD H,H
                self.cpu.hl.set_hi(self.cpu.hl.hi());
            }
            0x65 => {
                // LD H,L
                self.cpu.hl.set_hi(self.cpu.hl.lo());
            }
            0x66 => {
                // LD H,(HL);
                let val = self.read(self.cpu.hl.full());
                self.cpu.hl.set_hi(val);
            }
            0x6F => {
                // LD L,A
                self.cpu.hl.set_lo(self.cpu.af.hi());
            }
            0x68 => {
                // LD L,B
                self.cpu.hl.set_lo(self.cpu.bc.hi());
            }
            0x69 => {
                // LD L,C
                self.cpu.hl.set_lo(self.cpu.bc.lo());
            }
            0x6A => {
                // LD L,D
                self.cpu.hl.set_lo(self.cpu.de.hi());
            }
            0x6B => {
                // LD L,E
                self.cpu.hl.set_lo(self.cpu.de.lo());
            }
            0x6C => {
                // LD L,H
                self.cpu.hl.set_lo(self.cpu.hl.hi());
            }
            0x6D => {
                // LD L,L
                self.cpu.hl.set_lo(self.cpu.hl.lo());
            }
            0x6E => {
                // LD L,(HL);
                let val = self.read(self.cpu.hl.full());
                self.cpu.hl.set_lo(val);
            }
            0x77 => {
                // LD (HL),A
                let val = self.cpu.af.hi();
                self.write(self.cpu.hl.full(), val);
            }
            0x70 => {
                // LD (HL),B
                let val = self.cpu.bc.hi();
                self.write(self.cpu.hl.full(), val);
            }
            0x71 => {
                // LD (HL),C
                let val = self.cpu.bc.lo();
                self.write(self.cpu.hl.full(), val);
            }
            0x72 => {
                // LD (HL),D
                let val = self.cpu.de.hi();
                self.write(self.cpu.hl.full(), val);
            }
            0x73 => {
                // LD (HL),E
                let val = self.cpu.de.lo();
                self.write(self.cpu.hl.full(), val);
            }
            0x74 => {
                // LD (HL),H
                let val = self.cpu.hl.hi();
                self.write(self.cpu.hl.full(), val);
            }
            0x75 => {
                // LD (HL),L
                let val = self.cpu.hl.lo();
                self.write(self.cpu.hl.full(), val);
            }
            0x36 => {
                // LD (HL),n 36;
                let val = self.pop_pc();
                self.write(self.cpu.hl.full(), val);
            }
            0x02 => {
                // LD (bc),A
                let val = self.cpu.af.hi();
                self.write(self.cpu.bc.full(), val);
            }
            0x12 => {
                // LD (DE),A
                let val = self.cpu.af.hi();
                self.write(self.cpu.de.full(), val);
            }
            0xEA => {
                // LD (nn),A
                let val = self.cpu.af.hi();
                let addr = self.pop_pc16();
                self.write(addr, val);
            }
            0xF2 => {
                // LD A,(C);
                let val = 0xFF00_u16.wrapping_add(self.cpu.bc.lo() as u16);
                self.cpu.af.set_hi(self.read(val));
            }
            0xE2 => {
                // LD (C),A
                let val = self.cpu.af.hi();
                let mem = 0xFF00_u16.wrapping_add(self.cpu.bc.lo() as u16);
                self.write(mem, val);
            }
            0x3A => {
                // LDD A,(HL);
                let val = self.read(self.cpu.hl.full());
                self.cpu.af.set_hi(val);
                self.cpu.hl.set_full(self.cpu.hl.full().wrapping_sub(1));
            }
            0x32 => {
                // LDD (HL),A
                let val = self.cpu.hl.full();
                self.write(val, self.cpu.af.hi());
                self.cpu.hl.set_full(self.cpu.hl.full().wrapping_sub(1));
            }
            0x2A => {
                // LDI A,(HL);
                let val = self.read(self.cpu.hl.full());
                self.cpu.af.set_hi(val);
                self.cpu.hl.set_full(self.cpu.hl.full().wrapping_add(1));
            }
            0x22 => {
                // LDI (HL),A
                let val = self.cpu.hl.full();
                self.write(val, self.cpu.af.hi());
                self.cpu.hl.set_full(self.cpu.hl.full().wrapping_add(1));
            }
            0xE0 => {
                // LD (0xFF00+n),A
                let val = 0xFF00_u16.wrapping_add(self.pop_pc() as u16);
                self.write(val, self.cpu.af.hi());
            }
            0xF0 => {
                // LD A,(0xFF00+n);
                let addr =self.pop_pc() as u16;
                let val = self.read_upper_ram(0xFF00_u16.wrapping_add(addr));
                self.cpu.af.set_hi(val);
            }
            // ========== 16-Bit loads ===========
            0x01 => {
                // LD bc,nn
                let val = self.pop_pc16();
                self.cpu.bc.set_full(val);
            }
            0x11 => {
                // LD DE,nn
                let val = self.pop_pc16();
                self.cpu.de.set_full(val);
            }
            0x21 => {
                // LD HL,nn
                let val = self.pop_pc16();
                self.cpu.hl.set_full(val);
            }
            0x31 => {
                // LD SP,nn
                let val = self.pop_pc16();
                self.cpu.sp.set_full(val);
            }
            0xF9 => {
                // LD SP,HL
                let val = self.cpu.hl.value;
                let mask = self.cpu.hl.mask;
                self.cpu.sp.value = val;
                self.cpu.sp.mask = mask;
            }
            0xF8 => {
                // LD HL,SP+n
                let val2 = (self.pop_pc()) as i8;
                let val = self.cpu.add_16_signed(self.cpu.sp.full(), val2);
                self.cpu.hl.set_full(val)
            }
            0x08 => {
                // LD (nn),SP
                let address = self.pop_pc16();
                self.write(address, self.cpu.sp.lo());
                self.write(address.wrapping_add(1), self.cpu.sp.hi());
            }
            0xF5 => {
                // PUSH AF
                self.push_stack(self.cpu.af.full());
            }
            0xC5 => {
                // PUSH bc
                self.push_stack(self.cpu.bc.full());
            }
            0xD5 => {
                // PUSH DE
                self.push_stack(self.cpu.de.full());
            }
            0xE5 => {
                // PUSH HL
                self.push_stack(self.cpu.hl.full());
            }
            0xF1 => {
                // POP AF
                let val = self.pop_stack();
                self.cpu.af.set_full(val);
            }
            0xC1 => {
                // POP bc
                let val = self.pop_stack();
                self.cpu.bc.set_full(val);
            }
            0xD1 => {
                // POP DE
                let val = self.pop_stack();
                self.cpu.de.set_full(val);
            }
            0xE1 => {
                // POP HL
                let val = self.pop_stack();
                self.cpu.hl.set_full(val);
            }
            // ========== 8-Bit ALU ===========
            0x87 => {
                // ADD A,A
                let val = self.cpu.add(self.cpu.af.hi(), self.cpu.af.hi(), false);
                self.cpu.af.set_hi(val);
            }
            0x80 => {
                // ADD A,B
                let val = self.cpu.add(self.cpu.bc.hi(), self.cpu.af.hi(), false);
                self.cpu.af.set_hi(val);
            }
            0x81 => {
                // ADD A,C
                let val = self.cpu.add(self.cpu.bc.lo(), self.cpu.af.hi(), false);
                self.cpu.af.set_hi(val);
            }
            0x82 => {
                // ADD A,D
                let val = self.cpu.add(self.cpu.de.hi(), self.cpu.af.hi(), false);
                self.cpu.af.set_hi(val);
            }
            0x83 => {
                // ADD A,E
                let val = self.cpu.add(self.cpu.de.lo(), self.cpu.af.hi(), false);
                self.cpu.af.set_hi(val);
            }
            0x84 => {
                // ADD A,H
                let val = self.cpu.add(self.cpu.hl.hi(), self.cpu.af.hi(), false);
                self.cpu.af.set_hi(val);
            }
            0x85 => {
                // ADD A,L
                let val = self.cpu.add(self.cpu.hl.lo(), self.cpu.af.hi(), false);
                self.cpu.af.set_hi(val);
            }
            0x86 => {
                // ADD A,(HL);
                let val = self.cpu.add(self.read(self.cpu.hl.full()), self.cpu.af.hi(), false);
                self.cpu.af.set_hi(val);
            }
            0xC6 => {
                // ADD A,#
                let val1 = self.pop_pc();
                let val = self.cpu.add(val1, self.cpu.af.hi(), false);
                self.cpu.af.set_hi(val);
            }
            0x8F => {
                // ADC A,A
                let val = self.cpu.add(self.cpu.af.hi(), self.cpu.af.hi(), true);
                self.cpu.af.set_hi(val);
            }
            0x88 => {
                // ADC A,B
                let val = self.cpu.add(self.cpu.bc.hi(), self.cpu.af.hi(), true);
                self.cpu.af.set_hi(val);
            }
            0x89 => {
                // ADC A,C
                let val = self.cpu.add(self.cpu.bc.lo(), self.cpu.af.hi(), true);
                self.cpu.af.set_hi(val);
            }
            0x8A => {
                // ADC A,D
                let val = self.cpu.add(self.cpu.de.hi(), self.cpu.af.hi(), true);
                self.cpu.af.set_hi(val);
            }
            0x8B => {
                // ADC A,E
                let val = self.cpu.add(self.cpu.de.lo(), self.cpu.af.hi(), true);
                self.cpu.af.set_hi(val);
            }
            0x8C => {
                // ADC A,H
                let val = self.cpu.add(self.cpu.hl.hi(), self.cpu.af.hi(), true);
                self.cpu.af.set_hi(val);
            }
            0x8D => {
                // ADC A,L
                let val = self.cpu.add(self.cpu.hl.lo(), self.cpu.af.hi(), true);
                self.cpu.af.set_hi(val);
            }
            0x8E => {
                // ADC A,(HL);
                let val = self.cpu.add(self.read(self.cpu.hl.full()), self.cpu.af.hi(), true);
                self.cpu.af.set_hi(val);
            }
            0xCE => {
                // ADC A,#
                let val1 = self.pop_pc();
                let val = self.cpu.add(val1, self.cpu.af.hi(), true);
                self.cpu.af.set_hi(val);
            }
            0x97 => {
                // SUB A,A
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.af.hi(), false);
                self.cpu.af.set_hi(val);
            }
            0x90 => {
                // SUB A,B
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.bc.hi(), false);
                self.cpu.af.set_hi(val);
            }
            0x91 => {
                // SUB A,C
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.bc.lo(), false);
                self.cpu.af.set_hi(val);
            }
            0x92 => {
                // SUB A,D
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.de.hi(), false);
                self.cpu.af.set_hi(val);
            }
            0x93 => {
                // SUB A,E
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.de.lo(), false);
                self.cpu.af.set_hi(val);
            }
            0x94 => {
                // SUB A,H
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.hl.hi(), false);
                self.cpu.af.set_hi(val);
            }
            0x95 => {
                // SUB A,L
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.hl.lo(), false);
                self.cpu.af.set_hi(val);
            }
            0x96 => {
                // SUB A,(HL);
                let val = self.cpu.sub(self.cpu.af.hi(), self.read(self.cpu.hl.full()), false);
                self.cpu.af.set_hi(val);
            }
            0xD6 => {
                // SUB A,#
                let val2 = self.pop_pc();
                let val = self.cpu.sub(self.cpu.af.hi(), val2, false);
                self.cpu.af.set_hi(val);
            }
            0x9F => {
                // Sbc A,A
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.af.hi(), true);
                self.cpu.af.set_hi(val);
            }
            0x98 => {
                // Sbc A,B
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.bc.hi(), true);
                self.cpu.af.set_hi(val);
            }
            0x99 => {
                // Sbc A,C
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.bc.lo(), true);
                self.cpu.af.set_hi(val);
            }
            0x9A => {
                // Sbc A,D
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.de.hi(), true);
                self.cpu.af.set_hi(val);
            }
            0x9B => {
                // Sbc A,E
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.de.lo(), true);
                self.cpu.af.set_hi(val);
            }
            0x9C => {
                // Sbc A,H
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.hl.hi(), true);
                self.cpu.af.set_hi(val);
            }
            0x9D => {
                // Sbc A,L
                let val = self.cpu.sub(self.cpu.af.hi(), self.cpu.hl.lo(), true);
                self.cpu.af.set_hi(val);
            }
            0x9E => {
                // Sbc A,(HL);
                let val = self.cpu.sub(self.cpu.af.hi(), self.read(self.cpu.hl.full()), true);
                self.cpu.af.set_hi(val);
            }
            0xDE => {
                // Sbc A,#
                let val2 = self.pop_pc();
                let val = self.cpu.sub(self.cpu.af.hi(), val2, true);
                self.cpu.af.set_hi(val);
            }
            0xA7 => {
                // AND A,A
                let val = self.cpu.and(self.cpu.af.hi(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xA0 => {
                // AND A,B
                let val = self.cpu.and(self.cpu.bc.hi(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xA1 => {
                // AND A,C
                let val = self.cpu.and(self.cpu.bc.lo(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xA2 => {
                // AND A,D
                let val = self.cpu.and(self.cpu.de.hi(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xA3 => {
                // AND A,E
                let val = self.cpu.and(self.cpu.de.lo(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xA4 => {
                // AND A,H
                let val = self.cpu.and(self.cpu.hl.hi(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xA5 => {
                // AND A,L
                let val = self.cpu.and(self.cpu.hl.lo(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xA6 => {
                // AND A,(HL);
                let val = self.cpu.and(self.read(self.cpu.hl.full()), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xE6 => {
                // AND A,#
                let val1 = self.pop_pc();
                let val = self.cpu.and(val1, self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xB7 => {
                // OR A,A
                let val = self.cpu.or(self.cpu.af.hi(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xB0 => {
                // OR A,B
                let val = self.cpu.or(self.cpu.bc.hi(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xB1 => {
                // OR A,C
                let val = self.cpu.or(self.cpu.bc.lo(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xB2 => {
                // OR A,D
                let val = self.cpu.or(self.cpu.de.hi(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xB3 => {
                // OR A,E
                let val = self.cpu.or(self.cpu.de.lo(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xB4 => {
                // OR A,H
                let val = self.cpu.or(self.cpu.hl.hi(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xB5 => {
                // OR A,L
                let val = self.cpu.or(self.cpu.hl.lo(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xB6 => {
                // OR A,(HL);
                let val = self.cpu.or(self.read(self.cpu.hl.full()), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xF6 => {
                // OR A,#
                let val1 = self.pop_pc();
                let val = self.cpu.or(val1, self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xAF => {
                // XOR A,A
                let val = self.cpu.xor(self.cpu.af.hi(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xA8 => {
                // XOR A,B
                let val = self.cpu.xor(self.cpu.bc.hi(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xA9 => {
                // XOR A,C
                let val = self.cpu.xor(self.cpu.bc.lo(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xAA => {
                // XOR A,D
                let val = self.cpu.xor(self.cpu.de.hi(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xAB => {
                // XOR A,E
                let val = self.cpu.xor(self.cpu.de.lo(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xAC => {
                // XOR A,H
                let val = self.cpu.xor(self.cpu.hl.hi(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xAD => {
                // XOR A,L
                let val = self.cpu.xor(self.cpu.hl.lo(), self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xAE => {
                // XOR A,(HL);
                let a = self.read(self.cpu.hl.full());
                let val = self.cpu.xor(a, self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xEE => {
                // XOR A,#
                let val1 = self.pop_pc();
                let val = self.cpu.xor(val1, self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0xBF => {
                // CP A,A
                self.cpu.cp(self.cpu.af.hi(), self.cpu.af.hi());
            }
            0xB8 => {
                // CP A,B
                self.cpu.cp(self.cpu.bc.hi(), self.cpu.af.hi());
            }
            0xB9 => {
                // CP A,C
                self.cpu.cp(self.cpu.bc.lo(), self.cpu.af.hi());
            }
            0xBA => {
                // CP A,D
                self.cpu.cp(self.cpu.de.hi(), self.cpu.af.hi());
            }
            0xBB => {
                // CP A,E
                self.cpu.cp(self.cpu.de.lo(), self.cpu.af.hi());
            }
            0xbc => {
                // CP A,H
                self.cpu.cp(self.cpu.hl.hi(), self.cpu.af.hi());
            }
            0xBD => {
                // CP A,L
                self.cpu.cp(self.cpu.hl.lo(), self.cpu.af.hi());
            }
            0xBE => {
                // CP A,(HL);
                self.cpu.cp(self.read(self.cpu.hl.full()), self.cpu.af.hi());
            }
            0xFE => {
                // CP A,#
                let val1 = self.pop_pc();
                self.cpu.cp(val1, self.cpu.af.hi());
            }
            0x3C => {
                // INC A
                let val = self.cpu.inc(self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0x04 => {
                // INC B
                let val = self.cpu.inc(self.cpu.bc.hi());
                self.cpu.bc.set_hi(val);
            }
            0x0C => {
                // INC C
                let val = self.cpu.inc(self.cpu.bc.lo());
                self.cpu.bc.set_lo(val);
            }
            0x14 => {
                // INC D
                let val = self.cpu.inc(self.cpu.de.hi());
                self.cpu.de.set_hi(val);
            }
            0x1C => {
                // INC E
                let val = self.cpu.inc(self.cpu.de.lo());
                self.cpu.de.set_lo(val);
            }
            0x24 => {
                // INC H
                let val = self.cpu.inc(self.cpu.hl.hi());
                self.cpu.hl.set_hi(val);
            }
            0x2C => {
                // INC L
                let val = self.cpu.inc(self.cpu.hl.lo());
                self.cpu.hl.set_lo(val);
            }
            0x34 => {
                // INC (HL);
                let addr = self.cpu.hl.full();
                let val = self.cpu.inc(self.read(addr));
                self.write(addr, val)
            }
            0x3D => {
                // DEC A
                let val = self.cpu.dec(self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0x05 => {
                // DEC B
                let val = self.cpu.dec(self.cpu.bc.hi());
                self.cpu.bc.set_hi(val);
            }
            0x0D => {
                // DEC C
                let val = self.cpu.dec(self.cpu.bc.lo());
                self.cpu.bc.set_lo(val);
            }
            0x15 => {
                // DEC D
                let val = self.cpu.dec(self.cpu.de.hi());
                self.cpu.de.set_hi(val);
            }
            0x1D => {
                // DEC E
                let val = self.cpu.dec(self.cpu.de.lo());
                self.cpu.de.set_lo(val);
            }
            0x25 => {
                // DEC H
                let val = self.cpu.dec(self.cpu.hl.hi());
                self.cpu.hl.set_hi(val);
            }
            0x2D => {
                // DEC L
                let val = self.cpu.dec(self.cpu.hl.lo());
                self.cpu.hl.set_lo(val);
            }
            0x35 => {
                // DEC (HL);
                let addr = self.cpu.hl.full();
                let val = self.cpu.dec(self.read(addr));
                self.write(addr, val);
            }
            // ========== 16-Bit ALU ===========
            0x09 => {
                // ADD HL,bc
                let val = self.cpu.add_16(self.cpu.hl.full(), self.cpu.bc.full());
                self.cpu.hl.set_full(val)
            }
            0x19 => {
                // ADD HL,DE
                let val = self.cpu.add_16(self.cpu.hl.full(), self.cpu.de.full());
                self.cpu.hl.set_full(val);
            }
            0x29 => {
                // ADD HL,HL
                let val = self.cpu.add_16(self.cpu.hl.full(), self.cpu.hl.full());
                self.cpu.hl.set_full(val);
            }
            0x39 => {
                // ADD HL,SP
                let val = self.cpu.add_16(self.cpu.hl.full(), self.cpu.sp.full());
                self.cpu.hl.set_full(val);
            }
            0xE8 => {
                // ADD SP,n
                let val2 =(self.pop_pc()) as i8;
                let val = self.cpu.add_16_signed(self.cpu.sp.full(), val2);
                self.cpu.sp.set_full(val);
                self.cpu.set_z(false);
            }
            0x03 => {
                // INC bc
                let val = self.cpu.inc_16(self.cpu.bc.full());
                self.cpu.bc.set_full(val);
            }
            0x13 => {
                // INC DE
                let val = self.cpu.inc_16(self.cpu.de.full());
                self.cpu.de.set_full(val);
            }
            0x23 => {
                // INC HL
                let val = self.cpu.inc_16(self.cpu.hl.full());
                self.cpu.hl.set_full(val);
            }
            0x33 => {
                // INC SP
                let val = self.cpu.inc_16(self.cpu.sp.full());
                self.cpu.sp.set_full(val);
            }
            0x0B => {
                // DEC bc
                let val = self.cpu.dec_16(self.cpu.bc.full());
                self.cpu.bc.set_full(val);
            }
            0x1B => {
                // DEC DE
                let val = self.cpu.dec_16(self.cpu.de.full());
                self.cpu.de.set_full(val);
            }
            0x2B => {
                // DEC HL
                let val = self.cpu.dec_16(self.cpu.hl.full());
                self.cpu.hl.set_full(val);
            }
            0x3B => {
                // DEC SP
                let val = self.cpu.dec_16(self.cpu.sp.full());
                self.cpu.sp.set_full(val);
            }
            0x27 => {
                // DAA

                // When this instruction is executed, the A register is bcD
                // corrected using the contents of the flags. The exact process
                // is the following: if the least significant four bits of A
                // contain a non-bcD digit (i. e. it is greater than 9) or the
                // H flag is set, then 0x60 is added to the register. Then the
                // four most significant bits are checked. If this more significant
                // digit also happens to be greater than 9 or the C flag is set,
                // then 0x60 is added.
                if !self.cpu.n() {
                    if self.cpu.c() || self.cpu.af.hi() > 0x99 {
                        self.cpu.af.set_hi(self.cpu.af.hi().wrapping_add(0x60));
                        self.cpu.set_c(true);
                    }
                    if self.cpu.h() || self.cpu.af.hi() & 0xF > 0x9 {
                        self.cpu.af.set_hi(self.cpu.af.hi().wrapping_add(0x06));
                        self.cpu.set_h(false);
                    }
                } else if self.cpu.c() && self.cpu.h() {
                    self.cpu.af.set_hi(self.cpu.af.hi().wrapping_add(0x9A));
                    self.cpu.set_h(false);
                } else if self.cpu.c() {
                    self.cpu.af.set_hi(self.cpu.af.hi().wrapping_add(0xA0));
                } else if self.cpu.h() {
                    self.cpu.af.set_hi(self.cpu.af.hi().wrapping_add(0xFA));
                    self.cpu.set_h(false);
                }
                self.cpu.set_z(self.cpu.af.hi() == 0);
            }
            0x2F => {
                // CPL
                self.cpu.af.set_hi(0xFF ^ self.cpu.af.hi());
                self.cpu.set_n(true);
                self.cpu.set_h(true);
            }
            0x3F => {
                // CCF
                self.cpu.set_n(false);
                self.cpu.set_h(false);
                let c = self.cpu.c();
                self.cpu.set_c(!c);
            }
            0x37 => {
                // SCF
                self.cpu.set_n(false);
                self.cpu.set_h(false);
                self.cpu.set_c(true);
            }
            0x00 => {
                // NOP
            }
            0x76 => {
                // HALT
                self.halted = true;
            }
            0x10 => {
                // STOP
                self.halted = true;
                if self.cgb_mode {
                    // Handle switching to double speed mode
                    self.check_speed();
                }

                // Pop the next value as the STOP instruction is 2 bytes long. The second value
                // can be ignored, although generally it is expected to be 0x00 and any other
                // value is counted as a corrupted STOP instruction.
                self.pop_pc();
            }
            0xF3 => {
                // DI
                self.interrupts_on = false;
            }
            0xFB => {
                // EI
                self.interrupts_enabling = true
            }
            0x07 => {
                // RLCA
                let value = self.cpu.af.hi();
                let result = (value << 1) | (value >> 7);
                self.cpu.af.set_hi(result);
                self.cpu.set_z(false);
                self.cpu.set_n(false);
                self.cpu.set_h(false);
                self.cpu.set_c(value > 0x7F);
            }
            0x17 => {
                // RLA
                let value = self.cpu.af.hi();
                let carry: u8 = if self.cpu.c() { 1 } else { 0 };
                let result = (value.wrapping_shl(1).wrapping_add(carry));
                self.cpu.af.set_hi(result);
                self.cpu.set_z(false);
                self.cpu.set_n(false);
                self.cpu.set_h(false);
                self.cpu.set_c(value > 0x7F);
            }
            0x0F => {
                // RRCA
                let value = self.cpu.af.hi();
                let result = value.wrapping_shr(1) | (value & 1).wrapping_shl(7);
                self.cpu.af.set_hi(result);
                self.cpu.set_z(false);
                self.cpu.set_n(false);
                self.cpu.set_h(false);
                self.cpu.set_c(result > 0x7F);
            }
            0x1F => {
                // RRA
                let value = self.cpu.af.hi();
                let mut carry: u8 = 0;
                if self.cpu.c() {
                    carry = 0x80;
                }
                let result = value.wrapping_shr(1) | carry;
                self.cpu.af.set_hi(result);
                self.cpu.set_z(false);
                self.cpu.set_n(false);
                self.cpu.set_h(false);
                self.cpu.set_c((1 & value) == 1);
            }
            0xC3 => {
                // JP nn
                let next = self.pop_pc16();
                self.cpu.jump(next);
            }
            0xC2 => {
                // JP NZ,nn
                let next = self.pop_pc16();
                if !self.cpu.z() {
                    self.cpu.jump(next);
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(4);
                }
            }
            0xCA => {
                // JP Z,nn
                let next = self.pop_pc16();
                if self.cpu.z() {
                    self.cpu.jump(next);
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(4);
                }
            }
            0xD2 => {
                // JP NC,nn
                let next = self.pop_pc16();
                if !self.cpu.c() {
                    self.cpu.jump(next);
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(4);
                }
            }
            0xDA => {
                // JP C,nn
                let next = self.pop_pc16();
                if self.cpu.c() {
                    self.cpu.jump(next);
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(4);
                }
            }
            0xE9 => {
                // JP HL
                self.cpu.jump(self.cpu.hl.full());
            }
            0x18 => {
                // JR n
                let addr = (self.cpu.pc as i32).wrapping_add((self.pop_pc() as u8) as i32);
                self.cpu.jump((addr) as u16);
            }
            0x20 => {
                // JR NZ,n
                let next = (self.pop_pc() as u8);
                if !self.cpu.z() {
                    let addr = (self.cpu.pc as i32).wrapping_add(next as i32);
                    self.cpu.jump((addr) as u16);
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(4);
                }
            }
            0x28 => {
                // JR Z,n
                let next = (self.pop_pc() as u8);
                if self.cpu.z() {
                    let addr = (self.cpu.pc as i32).wrapping_add(next as i32);
                    self.cpu.jump((addr) as u16);
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(4);
                }
            }
            0x30 => {
                // JR NC,n
                let next = (self.pop_pc() as u8);
                if !self.cpu.c() {
                    let addr = (self.cpu.pc as i32).wrapping_add(next as i32);
                    self.cpu.jump((addr) as u16);
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(4);
                }
            }
            0x38 => {
                // JR C,n
                let next = (self.pop_pc() as u8);
                if self.cpu.c() {
                    let addr = (self.cpu.pc as i32).wrapping_add(next as i32);
                    self.cpu.jump((addr) as u16);
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(4);
                }
            }
            0xCD => {
                // CALL nn
                let next = self.pop_pc16();
                self.call(next);
            }
            0xC4 => {
                // CALL NZ,nn
                let next = self.pop_pc16();
                if !self.cpu.z() {
                    self.call(next);
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(12);
                }
            }
            0xCC => {
                // CALL Z,nn
                let next = self.pop_pc16();
                if self.cpu.z() {
                    self.call(next);
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(12);
                }
            }
            0xD4 => {
                // CALL NC,nn
                let next = self.pop_pc16();
                if !self.cpu.c() {
                    self.call(next);
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(12);
                }
            }
            0xDC => {
                // CALL C,nn
                let next = self.pop_pc16();
                if self.cpu.c() {
                    self.call(next);
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(12);
                }
            }
            0xC7 => {
                // RST 0x00;
                self.call(0x0000);
            }
            0xCF => {
                // RST 0x08;
                self.call(0x0008);
            }
            0xD7 => {
                // RST 0x10;
                self.call(0x0010);
            }
            0xDF => {
                // RST 0x18;
                self.call(0x0018);
            }
            0xE7 => {
                // RST 0x20;
                self.call(0x0020);
            }
            0xEF => {
                // RST 0x28;
                self.call(0x0028);
            }
            0xF7 => {
                // RST 0x30;
                self.call(0x0030);
            }
            0xFF => {
                // RST 0x38;
                self.call(0x0038);
            }
            0xC9 => {
                // RET
                self.ret();
            }
            0xC0 => {
                // RET NZ
                if !self.cpu.z() {
                    self.ret();
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(12);
                }
            }
            0xC8 => {
                // RET Z
                if self.cpu.z() {
                    self.ret();
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(12);
                }
            }
            0xD0 => {
                // RET NC
                if !self.cpu.c() {
                    self.ret();
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(12);
                }
            }
            0xD8 => {
                // RET C
                if self.cpu.c() {
                    self.ret();
                    self.cpu.clock.t = self.cpu.clock.t.wrapping_add(12);
                }
            }
            0xD9 => {
                // RETI
                self.ret();
                self.interrupts_enabling = true
            }
            0xCB => {
                // CB
                let next_inst = self.pop_pc();
                self.cpu.clock.t = self.cpu.clock.t.wrapping_add((CBOPCODE_CYCLES[next_inst as usize] * 4) as usize);
                self.find_cb_op(next_inst);
            }
            _ => {
                // TODO Error
                println!("Unkown opcode")
            }
        }
    }
}

use crate::gameboy::Gameboy;
use crate::bit_functions::{set, reset};

impl Gameboy {
    pub fn find_cb_op(&mut self, code: u8) {
        match code {
            0x0 => {
                let val = self.rlc(self.cpu.bc.hi());
                self.cpu.bc.set_hi(val);
            }
            0x1 => {
                let val = self.rlc(self.cpu.bc.lo());
                self.cpu.bc.set_lo(val);
            }
            0x2 => {
                let val = self.rlc(self.cpu.de.hi());
                self.cpu.de.set_hi(val);
            }
            0x3 => {
                let val = self.rlc(self.cpu.de.lo());
                self.cpu.de.set_lo(val);
            }
            0x4 => {
                let val = self.rlc(self.cpu.hl.hi());
                self.cpu.hl.set_hi(val);
            }
            0x5 => {
                let val = self.rlc(self.cpu.hl.lo());
                self.cpu.hl.set_lo(val);
            }
            0x6 => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = self.rlc(addr);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0x7 => {
                let val = self.rlc(self.cpuaf.hi());
                self.cpu.af.set_hi(val);
            }
            0x8 => {
                let val = self.rrc(self.cpu.bc.hi());
                self.cpu.bc.set_hi(val);
            }
            0x9 => {
                let val = self.rrc(self.cpu.bc.lo());
                self.cpu.bc.set_lo(val);
            }
            0xa => {
                let val = self.rrc(self.cpu.de.hi());
                self.cpu.de.set_hi(val);
            }
            0xb => {
                let val = self.rrc(self.cpu.de.lo());
                self.cpu.de.set_lo(val);
            }
            0xc => {
                let val = self.rrc(self.cpu.hl.hi());
                self.cpu.hl.set_hi(val);
            }
            0xd => {
                let val = self.rrc(self.cpu.hl.lo());
                self.cpu.hl.set_lo(val);
            }
            0xe => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = self.rrc(addr);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0xf => {
                let val = self.rrc(self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0x10 => {
                let val = self.rl(self.cpu.bc.hi());
                self.cpu.bc.set_hi(val);
            }
            0x11 => {
                let val = self.rl(self.cpu.bc.lo());
                self.cpu.bc.set_lo(val);
            }
            0x12 => {
                let val = self.rl(self.cpu.de.hi());
                self.cpu.de.set_hi(val);
            }
            0x13 => {
                let val = self.rl(self.cpu.de.lo());
                self.cpu.de.set_lo(val);
            }
            0x14 => {
                let val = self.rl(self.cpu.hl.hi());
                self.cpu.hl.set_hi(val);
            }
            0x15 => {
                let val = self.rl(self.cpu.hl.lo());
                self.cpu.hl.set_lo(val);
            }
            0x16 => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = self.rl(addr);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0x17 => {
                let val = self.rl(self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0x18 => {
                let val = self.rr(self.cpu.bc.hi());
                self.cpu.bc.set_hi(val);
            }
            0x19 => {
                let val = self.rr(self.cpu.bc.lo());
                self.cpu.bc.set_lo(val);
            }
            0x1a => {
                let val = self.rr(self.cpu.de.hi());
                self.cpu.de.set_hi(val);
            }
            0x1b => {
                let val = self.rr(self.cpu.de.lo());
                self.cpu.de.set_lo(val);
            }
            0x1c => {
                let val = self.rr(self.cpu.hl.hi());
                self.cpu.hl.set_hi(val);
            }
            0x1d => {
                let val = self.rr(self.cpu.hl.lo());
                self.cpu.hl.set_lo(val);
            }
            0x1e => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = self.rr(addr);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0x1f => {
                let val = self.rr(self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0x20 => {
                let val = self.sla(self.cpu.bc.hi());
                self.cpu.bc.set_hi(val);
            }
            0x21 => {
                let val = self.sla(self.cpu.bc.lo());
                self.cpu.bc.set_lo(val);
            }
            0x22 => {
                let val = self.sla(self.cpu.de.hi());
                self.cpu.de.set_hi(val);
            }
            0x23 => {
                let val = self.sla(self.cpu.de.lo());
                self.cpu.de.set_lo(val);
            }
            0x24 => {
                let val = self.sla(self.cpu.hl.hi());
                self.cpu.hl.set_hi(val);
            }
            0x25 => {
                let val = self.sla(self.cpu.hl.lo());
                self.cpu.hl.set_lo(val);
            }
            0x26 => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = self.sla(addr);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0x27 => {
                let val = self.sla(self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0x28 => {
                let val = self.sra(self.cpu.bc.hi());
                self.cpu.bc.set_hi(val);
            }
            0x29 => {
                let val = self.sra(self.cpu.bc.lo());
                self.cpu.bc.set_lo(val);
            }
            0x2a => {
                let val = self.sra(self.cpu.de.hi());
                self.cpu.de.set_hi(val);
            }
            0x2b => {
                let val = self.sra(self.cpu.de.lo());
                self.cpu.de.set_lo(val);
            }
            0x2c => {
                let val = self.sra(self.cpu.hl.hi());
                self.cpu.hl.set_hi(val);
            }
            0x2d => {
                let val = self.sra(self.cpu.hl.lo());
                self.cpu.hl.set_lo(val);
            }
            0x2e => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = self.sra(addr);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0x2f => {
                let val = self.sra(self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0x30 => {
                let val = self.swap(self.cpu.bc.hi());
                self.cpu.bc.set_hi(val);
            }
            0x31 => {
                let val = self.swap(self.cpu.bc.lo());
                self.cpu.bc.set_lo(val);
            }
            0x32 => {
                let val = self.swap(self.cpu.de.hi());
                self.cpu.de.set_hi(val);
            }
            0x33 => {
                let val = self.swap(self.cpu.de.lo());
                self.cpu.de.set_lo(val);
            }
            0x34 => {
                let val = self.swap(self.cpu.hl.hi());
                self.cpu.hl.set_hi(val);
            }
            0x35 => {
                let val = self.swap(self.cpu.hl.lo());
                self.cpu.hl.set_lo(val);
            }
            0x36 => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = self.swap(addr);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0x37 => {
                let val = self.swap(self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0x38 => {
                let val = self.srl(self.cpu.bc.hi());
                self.cpu.bc.set_hi(val);
            }
            0x39 => {
                let val = self.srl(self.cpu.bc.lo());
                self.cpu.bc.set_lo(val);
            }
            0x3a => {
                let val = self.srl(self.cpu.de.hi());
                self.cpu.de.set_hi(val);
            }
            0x3b => {
                let val = self.srl(self.cpu.de.lo());
                self.cpu.de.set_lo(val);
            }
            0x3c => {
                let val = self.srl(self.cpu.hl.hi());
                self.cpu.hl.set_hi(val);
            }
            0x3d => {
                let val = self.srl(self.cpu.hl.lo());
                self.cpu.hl.set_lo(val);
            }
            0x3e => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = self.srl(addr);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0x3f => {
                let val = self.srl(self.cpu.af.hi());
                self.cpu.af.set_hi(val);
            }
            0x40 => {
                bit(0, self.cpu.bc.hi());
            }
            0x41 => {
                bit(0, self.cpu.bc.lo());
            }
            0x42 => {
                bit(0, self.cpu.de.hi());
            }
            0x43 => {
                bit(0, self.cpu.de.lo());
            }
            0x44 => {
                bit(0, self.cpu.hl.hi());
            }
            0x45 => {
                bit(0, self.cpu.hl.lo());
            }
            0x46 => {
                let addr = self.memory.read(self.cpu.hl.full());
                bit(0, addr);
            }
            0x47 => {
                bit(0, self.cpu.af.hi());
            }
            0x48 => {
                bit(1, self.cpu.bc.hi());
            }
            0x49 => {
                bit(1, self.cpu.bc.lo());
            }
            0x4a => {
                bit(1, self.cpu.de.hi());
            }
            0x4b => {
                bit(1, self.cpu.de.lo());
            }
            0x4c => {
                bit(1, self.cpu.hl.hi());
            }
            0x4d => {
                bit(1, self.cpu.hl.lo());
            }
            0x4e => {
                let addr = self.memory.read(self.cpu.hl.full());
                bit(1, addr);
            }
            0x4f => {
                bit(1, self.cpu.af.hi());
            }
            0x50 => {
                bit(2, self.cpu.bc.hi());
            }
            0x51 => {
                bit(2, self.cpu.bc.lo());
            }
            0x52 => {
                bit(2, self.cpu.de.hi());
            }
            0x53 => {
                bit(2, self.cpu.de.lo());
            }
            0x54 => {
                bit(2, self.cpu.hl.hi());
            }
            0x55 => {
                bit(2, self.cpu.hl.lo());
            }
            0x56 => {
                let addr = self.memory.read(self.cpu.hl.full());
                bit(2, addr);
            }
            0x57 => {
                bit(2, self.cpu.af.hi());
            }
            0x58 => {
                bit(3, self.cpu.bc.hi());
            }
            0x59 => {
                bit(3, self.cpu.bc.lo());
            }
            0x5a => {
                bit(3, self.cpu.de.hi());
            }
            0x5b => {
                bit(3, self.cpu.de.lo());
            }
            0x5c => {
                bit(3, self.cpu.hl.hi());
            }
            0x5d => {
                bit(3, self.cpu.hl.lo());
            }
            0x5e => {
                let addr = self.memory.read(self.cpu.hl.full());
                bit(3, addr);
            }
            0x5f => {
                bit(3, self.cpu.af.hi());
            }
            0x60 => {
                bit(4, self.cpu.bc.hi());
            }
            0x61 => {
                bit(4, self.cpu.bc.lo());
            }
            0x62 => {
                bit(4, self.cpu.de.hi());
            }
            0x63 => {
                bit(4, self.cpu.de.lo());
            }
            0x64 => {
                bit(4, self.cpu.hl.hi());
            }
            0x65 => {
                bit(4, self.cpu.hl.lo());
            }
            0x66 => {
                let addr = self.memory.read(self.cpu.hl.full());
                bit(4, addr);
            }
            0x67 => {
                bit(4, self.cpu.af.hi());
            }
            0x68 => {
                bit(5, self.cpu.bc.hi());
            }
            0x69 => {
                bit(5, self.cpu.bc.lo());
            }
            0x6a => {
                bit(5, self.cpu.de.hi());
            }
            0x6b => {
                bit(5, self.cpu.de.lo());
            }
            0x6c => {
                bit(5, self.cpu.hl.hi());
            }
            0x6d => {
                bit(5, self.cpu.hl.lo());
            }
            0x6e => {
                let addr = self.memory.read(self.cpu.hl.full());
                bit(5, addr);
            }
            0x6f => {
                bit(5, self.cpu.af.hi());
            }
            0x70 => {
                bit(6, self.cpu.bc.hi());
            }
            0x71 => {
                bit(6, self.cpu.bc.lo());
            }
            0x72 => {
                bit(6, self.cpu.de.hi());
            }
            0x73 => {
                bit(6, self.cpu.de.lo());
            }
            0x74 => {
                bit(6, self.cpu.hl.hi());
            }
            0x75 => {
                bit(6, self.cpu.hl.lo());
            }
            0x76 => {
                let addr = self.memory.read(self.cpu.hl.full());
                bit(6, addr);
            }
            0x77 => {
                bit(6, self.cpu.af.hi());
            }
            0x78 => {
                bit(7, self.cpu.bc.hi());
            }
            0x79 => {
                bit(7, self.cpu.bc.lo());
            }
            0x7a => {
                bit(7, self.cpu.de.hi());
            }
            0x7b => {
                bit(7, self.cpu.de.lo());
            }
            0x7c => {
                bit(7, self.cpu.hl.hi());
            }
            0x7d => {
                bit(7, self.cpu.hl.lo());
            }
            0x7e => {
                let addr = self.memory.read(self.cpu.hl.full());
                bit(7, addr);
            }
            0x7f => {
                bit(7, self.cpu.af.hi());
            }
            0x80 => {
                let val = reset(self.cpu.bc.hi(), 0);
                self.cpu.bc.set_hi(val);
            }
            0x81 => {
                let val = reset(self.cpu.bc.lo(), 0);
                self.cpu.bc.set_lo(val);
            }
            0x82 => {
                let val = reset(self.cpu.de.hi(), 0);
                self.cpu.de.set_hi(val);
            }
            0x83 => {
                let val = reset(self.cpu.de.lo(), 0);
                self.cpu.de.set_lo(val);
            }
            0x84 => {
                let val = reset(self.cpu.hl.hi(), 0);
                self.cpu.hl.set_hi(val);
            }
            0x85 => {
                let val = reset(self.cpu.hl.lo(), 0);
                self.cpu.hl.set_lo(val);
            }
            0x86 => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = reset(addr, 0);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0x87 => {
                let val = reset(self.cpu.af.hi(), 0);
                self.cpu.af.set_hi(val);
            }
            0x88 => {
                let val = reset(self.cpu.bc.hi(), 1);
                self.cpu.bc.set_hi(val);
            }
            0x89 => {
                let val = reset(self.cpu.bc.lo(), 1);
                self.cpu.bc.set_lo(val);
            }
            0x8a => {
                let val = reset(self.cpu.de.hi(), 1);
                self.cpu.de.set_hi(val);
            }
            0x8b => {
                let val = reset(self.cpu.de.lo(), 1);
                self.cpu.de.set_lo(val);
            }
            0x8c => {
                let val = reset(self.cpu.hl.hi(), 1);
                self.cpu.hl.set_hi(val);
            }
            0x8d => {
                let val = reset(self.cpu.hl.lo(), 1);
                self.cpu.hl.set_lo(val);
            }
            0x8e => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = reset(addr, 1);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0x8f => {
                let val = reset(self.cpu.af.hi(), 1);
                self.cpu.af.set_hi(val);
            }
            0x90 => {
                let val = reset(self.cpu.bc.hi(), 2);
                self.cpu.bc.set_hi(val);
            }
            0x91 => {
                let val = reset(self.cpu.bc.lo(), 2);
                self.cpu.bc.set_lo(val);
            }
            0x92 => {
                let val = reset(self.cpu.de.hi(), 2);
                self.cpu.de.set_hi(val);
            }
            0x93 => {
                let val = reset(self.cpu.de.lo(), 2);
                self.cpu.de.set_lo(val);
            }
            0x94 => {
                let val = reset(self.cpu.hl.hi(), 2);
                self.cpu.hl.set_hi(val);
            }
            0x95 => {
                let val = reset(self.cpu.hl.lo(), 2);
                self.cpu.hl.set_lo(val);
            }
            0x96 => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = reset(addr, 2);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0x97 => {
                let val = reset(self.cpu.af.hi(), 2);
                self.cpu.af.set_hi(val);
            }
            0x98 => {
                let val = reset(self.cpu.bc.hi(), 3);
                self.cpu.bc.set_hi(val);
            }
            0x99 => {
                let val = reset(self.cpu.bc.lo(), 3);
                self.cpu.bc.set_lo(val);
            }
            0x9a => {
                let val = reset(self.cpu.de.hi(), 3);
                self.cpu.de.set_hi(val);
            }
            0x9b => {
                let val = reset(self.cpu.de.lo(), 3);
                self.cpu.de.set_lo(val);
            }
            0x9c => {
                let val = reset(self.cpu.hl.hi(), 3);
                self.cpu.hl.set_hi(val);
            }
            0x9d => {
                let val = reset(self.cpu.hl.lo(), 3);
                self.cpu.hl.set_lo(val);
            }
            0x9e => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = reset(addr, 3);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0x9f => {
                let val = reset(self.cpu.af.hi(), 3);
                self.cpu.af.set_hi(val);
            }
            0xa0 => {
                let val = reset(self.cpu.bc.hi(), 4);
                self.cpu.bc.set_hi(val);
            }
            0xa1 => {
                let val = reset(self.cpu.bc.lo(), 4);
                self.cpu.bc.set_lo(val);
            }
            0xa2 => {
                let val = reset(self.cpu.de.hi(), 4);
                self.cpu.de.set_hi(val);
            }
            0xa3 => {
                let val = reset(self.cpu.de.lo(), 4);
                self.cpu.de.set_lo(val);
            }
            0xa4 => {
                let val = reset(self.cpu.hl.hi(), 4);
                self.cpu.hl.set_hi(val);
            }
            0xa5 => {
                let val = reset(self.cpu.hl.lo(), 4);
                self.cpu.hl.set_lo(val);
            }
            0xa6 => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = reset(addr, 4);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0xa7 => {
                let val = reset(self.cpu.af.hi(), 4);
                self.cpu.af.set_hi(val);
            }
            0xa8 => {
                let val = reset(self.cpu.bc.hi(), 5);
                self.cpu.bc.set_hi(val);
            }
            0xa9 => {
                let val = reset(self.cpu.bc.lo(), 5);
                self.cpu.bc.set_lo(val);
            }
            0xaa => {
                let val = reset(self.cpu.de.hi(), 5);
                self.cpu.de.set_hi(val);
            }
            0xab => {
                let val = reset(self.cpu.de.lo(), 5);
                self.cpu.de.set_lo(val);
            }
            0xac => {
                let val = reset(self.cpu.hl.hi(), 5);
                self.cpu.hl.set_hi(val);
            }
            0xad => {
                let val = reset(self.cpu.hl.lo(), 5);
                self.cpu.hl.set_lo(val);
            }
            0xae => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = reset(addr, 5);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0xaf => {
                let val = reset(self.cpu.af.hi(), 5);
                self.cpu.af.set_hi(val);
            }
            0xb0 => {
                let val = reset(self.cpu.bc.hi(), 6);
                self.cpu.bc.set_hi(val);
            }
            0xb1 => {
                let val = reset(self.cpu.bc.lo(), 6);
                self.cpu.bc.set_lo(val);
            }
            0xb2 => {
                let val = reset(self.cpu.de.hi(), 6);
                self.cpu.de.set_hi(val);
            }
            0xb3 => {
                let val = reset(self.cpu.de.lo(), 6);
                self.cpu.de.set_lo(val);
            }
            0xb4 => {
                let val = reset(self.cpu.hl.hi(), 6);
                self.cpu.hl.set_hi(val);
            }
            0xb5 => {
                let val = reset(self.cpu.hl.lo(), 6);
                self.cpu.hl.set_lo(val);
            }
            0xb6 => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = reset(addr, 6);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0xb7 => {
                let val = reset(self.cpu.af.hi(), 6);
                self.cpu.af.set_hi(val);
            }
            0xb8 => {
                let val = reset(self.cpu.bc.hi(), 7);
                self.cpu.bc.set_hi(val);
            }
            0xb9 => {
                let val = reset(self.cpu.bc.lo(), 7);
                self.cpu.bc.set_lo(val);
            }
            0xba => {
                let val = reset(self.cpu.de.hi(), 7);
                self.cpu.de.set_hi(val);
            }
            0xbb => {
                let val = reset(self.cpu.de.lo(), 7);
                self.cpu.de.set_lo(val);
            }
            0xbc => {
                let val = reset(self.cpu.hl.hi(), 7);
                self.cpu.hl.set_hi(val);
            }
            0xbd => {
                let val = reset(self.cpu.hl.lo(), 7);
                self.cpu.hl.set_lo(val);
            }
            0xbe => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = reset(addr, 7);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0xbf => {
                let val = reset(self.cpu.af.hi(), 7);
                self.cpu.af.set_hi(val);
            }
            0xc0 => {
                let val = set(self.cpu.bc.hi(), 0);
                self.cpu.bc.set_hi(val);
            }
            0xc1 => {
                let val = set(self.cpu.bc.lo(), 0);
                self.cpu.bc.set_lo(val);
            }
            0xc2 => {
                let val = set(self.cpu.de.hi(), 0);
                self.cpu.de.set_hi(val);
            }
            0xc3 => {
                let val = set(self.cpu.de.lo(), 0);
                self.cpu.de.set_lo(val);
            }
            0xc4 => {
                let val = set(self.cpu.hl.hi(), 0);
                self.cpu.hl.set_hi(val);
            }
            0xc5 => {
                let val = set(self.cpu.hl.lo(), 0);
                self.cpu.hl.set_lo(val);
            }
            0xc6 => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = set(addr, 0);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0xc7 => {
                let val = set(self.cpu.af.hi(), 0);
                self.cpu.af.set_hi(val);
            }
            0xc8 => {
                let val = set(self.cpu.bc.hi(), 1);
                self.cpu.bc.set_hi(val);
            }
            0xc9 => {
                let val = set(self.cpu.bc.lo(), 1);
                self.cpu.bc.set_lo(val);
            }
            0xca => {
                let val = set(self.cpu.de.hi(), 1);
                self.cpu.de.set_hi(val);
            }
            0xcb => {
                let val = set(self.cpu.de.lo(), 1);
                self.cpu.de.set_lo(val);
            }
            0xcc => {
                let val = set(self.cpu.hl.hi(), 1);
                self.cpu.hl.set_hi(val);
            }
            0xcd => {
                let val = set(self.cpu.hl.lo(), 1);
                self.cpu.hl.set_lo(val);
            }
            0xce => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = set(addr, 1);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0xcf => {
                let val = set(self.cpu.af.hi(), 1);
                self.cpu.af.set_hi(val);
            }
            0xd0 => {
                let val = set(self.cpu.bc.hi(), 2);
                self.cpu.bc.set_hi(val);
            }
            0xd1 => {
                let val = set(self.cpu.bc.lo(), 2);
                self.cpu.bc.set_lo(val);
            }
            0xd2 => {
                let val = set(self.cpu.de.hi(), 2);
                self.cpu.de.set_hi(val);
            }
            0xd3 => {
                let val = set(self.cpu.de.lo(), 2);
                self.cpu.de.set_lo(val);
            }
            0xd4 => {
                let val = set(self.cpu.hl.hi(), 2);
                self.cpu.hl.set_hi(val);
            }
            0xd5 => {
                let val = set(self.cpu.hl.lo(), 2);
                self.cpu.hl.set_lo(val);
            }
            0xd6 => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = set(addr, 2);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0xd7 => {
                let val = set(self.cpu.af.hi(), 2);
                self.cpu.af.set_hi(val);
            }
            0xd8 => {
                let val = set(self.cpu.bc.hi(), 3);
                self.cpu.bc.set_hi(val);
            }
            0xd9 => {
                let val = set(self.cpu.bc.lo(), 3);
                self.cpu.bc.set_lo(val);
            }
            0xda => {
                let val = set(self.cpu.de.hi(), 3);
                self.cpu.de.set_hi(val);
            }
            0xdb => {
                let val = set(self.cpu.de.lo(), 3);
                self.cpu.de.set_lo(val);
            }
            0xdc => {
                let val = set(self.cpu.hl.hi(), 3);
                self.cpu.hl.set_hi(val);
            }
            0xdd => {
                let val = set(self.cpu.hl.lo(), 3);
                self.cpu.hl.set_lo(val);
            }
            0xde => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = set(addr, 3);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0xdf => {
                let val = set(self.cpu.af.hi(), 3);
                self.cpu.af.set_hi(val);
            }
            0xe0 => {
                let val = set(self.cpu.bc.hi(), 4);
                self.cpu.bc.set_hi(val);
            }
            0xe1 => {
                let val = set(self.cpu.bc.lo(), 4);
                self.cpu.bc.set_lo(val);
            }
            0xe2 => {
                let val = set(self.cpu.de.hi(), 4);
                self.cpu.de.set_hi(val);
            }
            0xe3 => {
                let val = set(self.cpu.de.lo(), 4);
                self.cpu.de.set_lo(val);
            }
            0xe4 => {
                let val = set(self.cpu.hl.hi(), 4);
                self.cpu.hl.set_hi(val);
            }
            0xe5 => {
                let val = set(self.cpu.hl.lo(), 4);
                self.cpu.hl.set_lo(val);
            }
            0xe6 => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = set(addr, 4);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0xe7 => {
                let val = set(self.cpu.af.hi(), 4);
                self.cpu.af.set_hi(val);
            }
            0xe8 => {
                let val = set(self.cpu.bc.hi(), 5);
                self.cpu.bc.set_hi(val);
            }
            0xe9 => {
                let val = set(self.cpu.bc.lo(), 5);
                self.cpu.bc.set_lo(val);
            }
            0xea => {
                let val = set(self.cpu.de.hi(), 5);
                self.cpu.de.set_hi(val);
            }
            0xeb => {
                let val = set(self.cpu.de.lo(), 5);
                self.cpu.de.set_lo(val);
            }
            0xec => {
                let val = set(self.cpu.hl.hi(), 5);
                self.cpu.hl.set_hi(val);
            }
            0xed => {
                let val = set(self.cpu.hl.lo(), 5);
                self.cpu.hl.set_lo(val);
            }
            0xee => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = set(addr, 5);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0xef => {
                let val = set(self.cpu.af.hi(), 5);
                self.cpu.af.set_hi(val);
            }
            0xf0 => {
                let val = set(self.cpu.bc.hi(), 6);
                self.cpu.bc.set_hi(val);
            }
            0xf1 => {
                let val = set(self.cpu.bc.lo(), 6);
                self.cpu.bc.set_lo(val);
            }
            0xf2 => {
                let val = set(self.cpu.de.hi(), 6);
                self.cpu.de.set_hi(val);
            }
            0xf3 => {
                let val = set(self.cpu.de.lo(), 6);
                self.cpu.de.set_lo(val);
            }
            0xf4 => {
                let val = set(self.cpu.hl.hi(), 6);
                self.cpu.hl.set_hi(val);
            }
            0xf5 => {
                let val = set(self.cpu.hl.lo(), 6);
                self.cpu.hl.set_lo(val);
            }
            0xf6 => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = set(addr, 6);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0xf7 => {
                let val = set(self.cpu.af.hi(), 6);
                self.cpu.af.set_hi(val);
            }
            0xf8 => {
                let val = set(self.cpu.bc.hi(), 7);
                self.cpu.bc.set_hi(val);
            }
            0xf9 => {
                let val = set(self.cpu.bc.lo(), 7);
                self.cpu.bc.set_lo(val);
            }
            0xfa => {
                let val = set(self.cpu.de.hi(), 7);
                self.cpu.de.set_hi(val);
            }
            0xfb => {
                let val = set(self.cpu.de.lo(), 7);
                self.cpu.de.set_lo(val);
            }
            0xfc => {
                let val = set(self.cpu.hl.hi(), 7);
                self.cpu.hl.set_hi(val);
            }
            0xfd => {
                let val = set(self.cpu.hl.lo(), 7);
                self.cpu.hl.set_lo(val);
            }
            0xfe => {
                let addr = self.memory.read(self.cpu.hl.full());
                let val = set(addr, 7);
                self.memory.write(self.cpu.hl.full(), val);
            }
            0xff => {
                let val = set(self.cpu.af.hi(), 7);
                self.cpu.af.set_hi(val);
            }
            _ => {}
        }
    }
}
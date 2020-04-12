use crate::cpu::Z80;

// OPCODE_CYCLES is the number of cpu cycles for each normal opcode.
pub(crate) const OPCODE_CYCLES: [u8; 256] = [
    1, 3, 2, 2, 1, 1, 2, 1, 5, 2, 2, 2, 1, 1, 2, 1, // 0
    0, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1, 2, 1, // 1
    2, 3, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 2
    2, 3, 2, 2, 3, 3, 3, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 3
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 4
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 5
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 6
    2, 2, 2, 2, 2, 2, 0, 2, 1, 1, 1, 1, 1, 1, 2, 1, // 7
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 8
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 9
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // a
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // b
    2, 3, 3, 4, 3, 4, 2, 4, 2, 4, 3, 0, 3, 6, 2, 4, // c
    2, 3, 3, 0, 3, 4, 2, 4, 2, 4, 3, 0, 3, 0, 2, 4, // d
    3, 3, 2, 0, 0, 4, 2, 4, 4, 1, 4, 0, 0, 0, 2, 4, // e
    3, 3, 2, 1, 0, 4, 2, 4, 3, 2, 4, 1, 0, 0, 2, 4, // f
]; //0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f

// CBOPCODE_CYCLES is the number of cpu cycles for each CB opcode.
const CBOPCODE_CYCLES: [u8; 256] = [
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 0
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 1
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 2
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 3
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 4
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 5
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 6
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 7
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 8
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 9
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // A
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // B
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // C
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // D
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // E
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // F
]; //0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f




pub fn find_op(cpu: &mut Z80, code: u8) {
    match code {
        0x06 => {
            // LD B, n
            cpu.bc.set_hi(cpu.pop_pc());
        },
        0x0E=> {
            // LD C, n
            cpu.bc.set_lo(cpu.pop_pc());
        },
        0x16=> {
            // LD D, n
            cpu.de.set_hi(cpu.pop_pc());
        },
        0x1E=> {
            // LD E, n
            cpu.de.set_lo(cpu.pop_pc());
        },
        0x26=> {
            // LD H, n
            cpu.hl.set_hi(cpu.pop_pc());
        },
        0x2E=> {
            // LD L, n
            cpu.hl.set_lo(cpu.pop_pc());
        },
        0x7F=> {
            // LD A,A
            cpu.af.set_hi(cpu.af.hi());
        },
        0x78=> {
            // LD A,B
            cpu.af.set_hi(cpu.bc.hi());
        },
        0x79=> {
            // LD A,C
            cpu.af.set_hi(cpu.bc.lo());
        },
        0x7A=> {
            // LD A,D
            cpu.af.set_hi(cpu.de.hi());
        },
        0x7B=> {
            // LD A,E
            cpu.af.set_hi(cpu.de.lo());
        },
        0x7C=> {
            // LD A,H
            cpu.af.set_hi(cpu.hl.hi());
        },
        0x7D=> {
            // LD A,L
            cpu.af.set_hi(cpu.hl.lo());
        },
        0x0A=> {
            // LD A,(bc);
            let val = cpu.mmu.read(cpu.bc.full());
            cpu.af.set_hi(val);
        },
        0x1A=> {
            // LD A,(DE);
            let val = cpu.mmu.read(cpu.de.full());
            cpu.af.set_hi(val);
        },
        0x7E=> {
            // LD A,(HL);
            let val = cpu.mmu.read(cpu.hl.full());
            cpu.af.set_hi(val);
        },
        0xFA=> {
            // LD A,(nn);
            let val = cpu.mmu.read(cpu.pop_pc16());
            cpu.af.set_hi(val);
        },
        0x3E=> {
            // LD A,(nn);
            let val = cpu.pop_pc();
            cpu.af.set_hi(val);
        },
        0x47=> {
            // LD B,A
            cpu.bc.set_hi(cpu.af.hi());
        },
        0x40=> {
            // LD B,B
            cpu.bc.set_hi(cpu.bc.hi());
        },
        0x41=> {
            // LD B,C
            cpu.bc.set_hi(cpu.bc.lo());
        },
        0x42=> {
            // LD B,D
            cpu.bc.set_hi(cpu.de.hi());
        },
        0x43=> {
            // LD B,E
            cpu.bc.set_hi(cpu.de.lo());
        },
        0x44=> {
            // LD B,H
            cpu.bc.set_hi(cpu.hl.hi());
        },
        0x45=> {
            // LD B,L
            cpu.bc.set_hi(cpu.hl.lo());
        },
        0x46=> {
            // LD B,(HL);
            let val = cpu.mmu.read(cpu.hl.full());
            cpu.bc.set_hi(val);
        },
        0x4F=> {
            // LD C,A
            cpu.bc.set_lo(cpu.af.hi());
        },
        0x48=> {
            // LD C,B
            cpu.bc.set_lo(cpu.bc.hi());
        },
        0x49=> {
            // LD C,C
            cpu.bc.set_lo(cpu.bc.lo());
        },
        0x4A=> {
            // LD C,D
            cpu.bc.set_lo(cpu.de.hi());
        },
        0x4B=> {
            // LD C,E
            cpu.bc.set_lo(cpu.de.lo());
        },
        0x4C=> {
            // LD C,H
            cpu.bc.set_lo(cpu.hl.hi());
        },
        0x4D=> {
            // LD C,L
            cpu.bc.set_lo(cpu.hl.lo());
        },
        0x4E=> {
            // LD C,(HL);
            let val = cpu.mmu.read(cpu.hl.full());
            cpu.bc.set_lo(val);
        },
        0x57=> {
            // LD D,A
            cpu.de.set_hi(cpu.af.hi());
        },
        0x50=> {
            // LD D,B
            cpu.de.set_hi(cpu.bc.hi());
        },
        0x51=> {
            // LD D,C
            cpu.de.set_hi(cpu.bc.lo());
        },
        0x52=> {
            // LD D,D
            cpu.de.set_hi(cpu.de.hi());
        },
        0x53=> {
            // LD D,E
            cpu.de.set_hi(cpu.de.lo());
        },
        0x54=> {
            // LD D,H
            cpu.de.set_hi(cpu.hl.hi());
        },
        0x55=> {
            // LD D,L
            cpu.de.set_hi(cpu.hl.lo());
        },
        0x56=> {
            // LD D,(HL);
            let val = cpu.mmu.read(cpu.hl.full());
            cpu.de.set_hi(val);
        },
        0x5F=> {
            // LD E,A
            cpu.de.set_lo(cpu.af.hi());
        },
        0x58=> {
            // LD E,B
            cpu.de.set_lo(cpu.bc.hi());
        },
        0x59=> {
            // LD E,C
            cpu.de.set_lo(cpu.bc.lo());
        },
        0x5A=> {
            // LD E,D
            cpu.de.set_lo(cpu.de.hi());
        },
        0x5B=> {
            // LD E,E
            cpu.de.set_lo(cpu.de.lo());
        },
        0x5C=> {
            // LD E,H
            cpu.de.set_lo(cpu.hl.hi());
        },
        0x5D=> {
            // LD E,L
            cpu.de.set_lo(cpu.hl.lo());
        },
        0x5E=> {
            // LD E,(HL);
            let val = cpu.mmu.read(cpu.hl.full());
            cpu.de.set_lo(val);
        },
        0x67=> {
            // LD H,A
            cpu.hl.set_hi(cpu.af.hi());
        },
        0x60=> {
            // LD H,B
            cpu.hl.set_hi(cpu.bc.hi());
        },
        0x61=> {
            // LD H,C
            cpu.hl.set_hi(cpu.bc.lo());
        },
        0x62=> {
            // LD H,D
            cpu.hl.set_hi(cpu.de.hi());
        },
        0x63=> {
            // LD H,E
            cpu.hl.set_hi(cpu.de.lo());
        },
        0x64=> {
            // LD H,H
            cpu.hl.set_hi(cpu.hl.hi());
        },
        0x65=> {
            // LD H,L
            cpu.hl.set_hi(cpu.hl.lo());
        },
        0x66=> {
            // LD H,(HL);
            let val = cpu.mmu.read(cpu.hl.full());
            cpu.hl.set_hi(val);
        },
        0x6F=> {
            // LD L,A
            cpu.hl.set_lo(cpu.af.hi());
        },
        0x68=> {
            // LD L,B
            cpu.hl.set_lo(cpu.bc.hi());
        },
        0x69=> {
            // LD L,C
            cpu.hl.set_lo(cpu.bc.lo());
        },
        0x6A=> {
            // LD L,D
            cpu.hl.set_lo(cpu.de.hi());
        },
        0x6B=> {
            // LD L,E
            cpu.hl.set_lo(cpu.de.lo());
        },
        0x6C=> {
            // LD L,H
            cpu.hl.set_lo(cpu.hl.hi());
        },
        0x6D=> {
            // LD L,L
            cpu.hl.set_lo(cpu.hl.lo());
        },
        0x6E=> {
            // LD L,(HL);
            let val = cpu.mmu.read(cpu.hl.full());
            cpu.hl.set_lo(val);
        },
        0x77=> {
            // LD (HL),A
            let val = cpu.af.hi();
            cpu.mmu.write(cpu.hl.full(), val);
        },
        0x70=> {
            // LD (HL),B
            let val = cpu.bc.hi();
            cpu.mmu.write(cpu.hl.full(), val);
        },
        0x71=> {
            // LD (HL),C
            let val = cpu.bc.lo();
            cpu.mmu.write(cpu.hl.full(), val);
        },
        0x72=> {
            // LD (HL),D
            let val = cpu.de.hi();
            cpu.mmu.write(cpu.hl.full(), val);
        },
        0x73=> {
            // LD (HL),E
            let val = cpu.de.lo();
            cpu.mmu.write(cpu.hl.full(), val);
        },
        0x74=> {
            // LD (HL),H
            let val = cpu.hl.hi();
            cpu.mmu.write(cpu.hl.full(), val);
        },
        0x75=> {
            // LD (HL),L
            let val = cpu.hl.lo();
            cpu.mmu.write(cpu.hl.full(), val);
        },
        0x36=> {
            // LD (HL),n 36
            let val = cpu.pop_pc();
            cpu.mmu.write(cpu.hl.full(), val);
        },
        0x02=> {
            // LD (bc),A
            let val = cpu.af.hi();
            cpu.mmu.write(cpu.bc.full(), val);
        },
        0x12=> {
            // LD (DE),A
            let val = cpu.af.hi();
            cpu.mmu.write(cpu.de.full(), val);
        },
        0xEA=> {
            // LD (nn),A
            let val = cpu.af.hi();
            cpu.mmu.write(cpu.pop_pc16(), val);
        },
        0xF2=> {
            // LD A,(C);
            let val = 0xFF00 + (cpu.bc.lo() as u16);
            cpu.af.set_hi(cpu.mmu.read(val));
        },
        0xE2=> {
            // LD (C),A
            let val = cpu.af.hi();
            let mem = 0xFF00 + (cpu.bc.lo() as u16);
            cpu.mmu.write(mem, val);
        },
        0x3A=> {
            // LDD A,(HL);
            let val = cpu.mmu.read(cpu.hl.full());
            cpu.af.set_hi(val);
            cpu.hl.set_full(cpu.hl.full() - 1);
        },
        0x32=> {
            // LDD (HL),A
            let val = cpu.hl.full();
            cpu.mmu.write(val, cpu.af.hi());
            cpu.hl.set_full(cpu.hl.full() - 1);
        },
        0x2A=> {
            // LDI A,(HL);
            let val = cpu.mmu.read(cpu.hl.full());
            cpu.af.set_hi(val);
            cpu.hl.set_full(cpu.hl.full() + 1);
        },
        0x22=> {
            // LDI (HL),A
            let val = cpu.hl.full();
            cpu.mmu.write(val, cpu.af.hi());
            cpu.hl.set_full(cpu.hl.full() + 1);
        },
        0xE0=> {
            // LD (0xFF00+n),A
            let val = 0xFF00 + (cpu.pop_pc() as u16);
            cpu.mmu.write(val, cpu.af.hi());
        },
        0xF0=> {
            // LD A,(0xFF00+n);
            let val = cpu.mmu.readhighRam(0xFF00 + (cpu.pop_pc()) as u16);
            cpu.af.set_hi(val);
        },
        // ========== 16-Bit loads ===========
        0x01=> {
            // LD bc,nn
            let val = cpu.pop_pc16();
            cpu.bc.set_full(val);
        },
        0x11=> {
            // LD DE,nn
            let val = cpu.pop_pc16();
            cpu.de.set_full(val);
        },
        0x21=> {
            // LD HL,nn
            let val = cpu.pop_pc16();
            cpu.hl.set_full(val);
        },
        0x31=> {
            // LD SP,nn
            let val = cpu.pop_pc16();
            cpu.sp.set_full(val);
        },
        0xF9=> {
            // LD SP,HL
            let val = cpu.hl.value;
            let mask = cpu.hl.mask;
            cpu.sp.value = val;
            cpu.sp.mask = mask;
        },
        0xF8=> {
            // LD HL,SP+n
            gb.instAdd16Signed(cpu.hl.Set, cpu.sp.full(), int8(cpu.pop_pc()));
        },
        0x08=> {
            // LD (nn),SP
            let address = cpu.pop_pc16();
            cpu.mmu.write(address, cpu.sp.lo());
            cpu.mmu.write(address+1, cpu.sp.hi());
        },
        0xF5=> {
            // PUSH AF
            gb.pushStack(cpu.af.full());
        },
        0xC5=> {
            // PUSH bc
            gb.pushStack(cpu.bc.full());
        },
        0xD5=> {
            // PUSH DE
            gb.pushStack(cpu.de.full());
        },
        0xE5=> {
            // PUSH HL
            gb.pushStack(cpu.hl.full());
        },
        0xF1=> {
            // POP AF
            cpu.af.set_full(gb.popStack());
        },
        0xC1=> {
            // POP bc
            cpu.bc.set_full(gb.popStack());
        },
        0xD1=> {
            // POP DE
            cpu.de.set_full(gb.popStack());
        },
        0xE1=> {
            // POP HL
            cpu.hl.set_full(gb.popStack());
        },
        // ========== 8-Bit ALU ===========
        0x87=> {
            // ADD A,A
            gb.instAdd(cpu.af.set_hi, cpu.af.hi(), cpu.af.hi(), false);
        },
        0x80=> {
            // ADD A,B
            gb.instAdd(cpu.af.set_hi, cpu.bc.hi(), cpu.af.hi(), false);
        },
        0x81=> {
            // ADD A,C
            gb.instAdd(cpu.af.set_hi, cpu.bc.lo(), cpu.af.hi(), false);
        },
        0x82=> {
            // ADD A,D
            gb.instAdd(cpu.af.set_hi, cpu.de.hi(), cpu.af.hi(), false);
        },
        0x83=> {
            // ADD A,E
            gb.instAdd(cpu.af.set_hi, cpu.de.lo(), cpu.af.hi(), false);
        },
        0x84=> {
            // ADD A,H
            gb.instAdd(cpu.af.set_hi, cpu.hl.hi(), cpu.af.hi(), false);
        },
        0x85=> {
            // ADD A,L
            gb.instAdd(cpu.af.set_hi, cpu.hl.lo(), cpu.af.hi(), false);
        },
        0x86=> {
            // ADD A,(HL);
            gb.instAdd(cpu.af.set_hi, cpu.mmu.read(cpu.hl.full()), cpu.af.hi(), false);
        },
        0xC6=> {
            // ADD A,#
            gb.instAdd(cpu.af.set_hi, cpu.pop_pc(), cpu.af.hi(), false);
        },
        0x8F=> {
            // ADC A,A
            gb.instAdd(cpu.af.set_hi, cpu.af.hi(), cpu.af.hi(), true);
        },
        0x88=> {
            // ADC A,B
            gb.instAdd(cpu.af.set_hi, cpu.bc.hi(), cpu.af.hi(), true);
        },
        0x89=> {
            // ADC A,C
            gb.instAdd(cpu.af.set_hi, cpu.bc.lo(), cpu.af.hi(), true);
        },
        0x8A=> {
            // ADC A,D
            gb.instAdd(cpu.af.set_hi, cpu.de.hi(), cpu.af.hi(), true);
        },
        0x8B=> {
            // ADC A,E
            gb.instAdd(cpu.af.set_hi, cpu.de.lo(), cpu.af.hi(), true);
        },
        0x8C=> {
            // ADC A,H
            gb.instAdd(cpu.af.set_hi, cpu.hl.hi(), cpu.af.hi(), true);
        },
        0x8D=> {
            // ADC A,L
            gb.instAdd(cpu.af.set_hi, cpu.hl.lo(), cpu.af.hi(), true);
        },
        0x8E=> {
            // ADC A,(HL);
            gb.instAdd(cpu.af.set_hi, cpu.mmu.read(cpu.hl.full()), cpu.af.hi(), true);
        },
        0xCE=> {
            // ADC A,#
            gb.instAdd(cpu.af.set_hi, cpu.pop_pc(), cpu.af.hi(), true);
        },
        0x97=> {
            // SUB A,A
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.af.hi(), false);
        },
        0x90=> {
            // SUB A,B
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.bc.hi(), false);
        },
        0x91=> {
            // SUB A,C
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.bc.lo(), false);
        },
        0x92=> {
            // SUB A,D
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.de.hi(), false);
        },
        0x93=> {
            // SUB A,E
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.de.lo(), false);
        },
        0x94=> {
            // SUB A,H
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.hl.hi(), false);
        },
        0x95=> {
            // SUB A,L
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.hl.lo(), false);
        },
        0x96=> {
            // SUB A,(HL);
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.mmu.read(cpu.hl.full()), false);
        },
        0xD6=> {
            // SUB A,#
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.pop_pc(), false);
        },
        0x9F=> {
            // Sbc A,A
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.af.hi(), true);
        },
        0x98=> {
            // Sbc A,B
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.bc.hi(), true);
        },
        0x99=> {
            // Sbc A,C
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.bc.lo(), true);
        },
        0x9A=> {
            // Sbc A,D
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.de.hi(), true);
        },
        0x9B=> {
            // Sbc A,E
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.de.lo(), true);
        },
        0x9C=> {
            // Sbc A,H
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.hl.hi(), true);
        },
        0x9D=> {
            // Sbc A,L
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.hl.lo(), true);
        },
        0x9E=> {
            // Sbc A,(HL);
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.mmu.read(cpu.hl.full()), true);
        },
        0xDE=> {
            // Sbc A,#
            gb.instSub(cpu.af.set_hi, cpu.af.hi(), cpu.pop_pc(), true);
        },
        0xA7=> {
            // AND A,A
            gb.instAnd(cpu.af.set_hi, cpu.af.hi(), cpu.af.hi());
        },
        0xA0=> {
            // AND A,B
            gb.instAnd(cpu.af.set_hi, cpu.bc.hi(), cpu.af.hi());
        },
        0xA1=> {
            // AND A,C
            gb.instAnd(cpu.af.set_hi, cpu.bc.lo(), cpu.af.hi());
        },
        0xA2=> {
            // AND A,D
            gb.instAnd(cpu.af.set_hi, cpu.de.hi(), cpu.af.hi());
        },
        0xA3=> {
            // AND A,E
            gb.instAnd(cpu.af.set_hi, cpu.de.lo(), cpu.af.hi());
        },
        0xA4=> {
            // AND A,H
            gb.instAnd(cpu.af.set_hi, cpu.hl.hi(), cpu.af.hi());
        },
        0xA5=> {
            // AND A,L
            gb.instAnd(cpu.af.set_hi, cpu.hl.lo(), cpu.af.hi());
        },
        0xA6=> {
            // AND A,(HL);
            gb.instAnd(cpu.af.set_hi, cpu.mmu.read(cpu.hl.full()), cpu.af.hi());
        },
        0xE6=> {
            // AND A,#
            gb.instAnd(cpu.af.set_hi, cpu.pop_pc(), cpu.af.hi());
        },
        0xB7=> {
            // OR A,A
            gb.instOr(cpu.af.set_hi, cpu.af.hi(), cpu.af.hi());
        },
        0xB0=> {
            // OR A,B
            gb.instOr(cpu.af.set_hi, cpu.bc.hi(), cpu.af.hi());
        },
        0xB1=> {
            // OR A,C
            gb.instOr(cpu.af.set_hi, cpu.bc.lo(), cpu.af.hi());
        },
        0xB2=> {
            // OR A,D
            gb.instOr(cpu.af.set_hi, cpu.de.hi(), cpu.af.hi());
        },
        0xB3=> {
            // OR A,E
            gb.instOr(cpu.af.set_hi, cpu.de.lo(), cpu.af.hi());
        },
        0xB4=> {
            // OR A,H
            gb.instOr(cpu.af.set_hi, cpu.hl.hi(), cpu.af.hi());
        },
        0xB5=> {
            // OR A,L
            gb.instOr(cpu.af.set_hi, cpu.hl.lo(), cpu.af.hi());
        },
        0xB6=> {
            // OR A,(HL);
            gb.instOr(cpu.af.set_hi, cpu.mmu.read(cpu.hl.full()), cpu.af.hi());
        },
        0xF6=> {
            // OR A,#
            gb.instOr(cpu.af.set_hi, cpu.pop_pc(), cpu.af.hi());
        },
        0xAF=> {
            // XOR A,A
            gb.instXor(cpu.af.set_hi, cpu.af.hi(), cpu.af.hi());
        },
        0xA8=> {
            // XOR A,B
            gb.instXor(cpu.af.set_hi, cpu.bc.hi(), cpu.af.hi());
        },
        0xA9=> {
            // XOR A,C
            gb.instXor(cpu.af.set_hi, cpu.bc.lo(), cpu.af.hi());
        },
        0xAA=> {
            // XOR A,D
            gb.instXor(cpu.af.set_hi, cpu.de.hi(), cpu.af.hi());
        },
        0xAB=> {
            // XOR A,E
            gb.instXor(cpu.af.set_hi, cpu.de.lo(), cpu.af.hi());
        },
        0xAC=> {
            // XOR A,H
            gb.instXor(cpu.af.set_hi, cpu.hl.hi(), cpu.af.hi());
        },
        0xAD=> {
            // XOR A,L
            gb.instXor(cpu.af.set_hi, cpu.hl.lo(), cpu.af.hi());
        },
        0xAE=> {
            // XOR A,(HL);
            gb.instXor(cpu.af.set_hi, cpu.mmu.read(cpu.hl.full()), cpu.af.hi());
        },
        0xEE=> {
            // XOR A,#
            gb.instXor(cpu.af.set_hi, cpu.pop_pc(), cpu.af.hi());
        },
        0xBF=> {
            // CP A,A
            gb.instCp(cpu.af.hi(), cpu.af.hi());
        },
        0xB8=> {
            // CP A,B
            gb.instCp(cpu.bc.hi(), cpu.af.hi());
        },
        0xB9=> {
            // CP A,C
            gb.instCp(cpu.bc.lo(), cpu.af.hi());
        },
        0xBA=> {
            // CP A,D
            gb.instCp(cpu.de.hi(), cpu.af.hi());
        },
        0xBB=> {
            // CP A,E
            gb.instCp(cpu.de.lo(), cpu.af.hi());
        },
        0xbc=> {
            // CP A,H
            gb.instCp(cpu.hl.hi(), cpu.af.hi());
        },
        0xBD=> {
            // CP A,L
            gb.instCp(cpu.hl.lo(), cpu.af.hi());
        },
        0xBE=> {
            // CP A,(HL);
            gb.instCp(cpu.mmu.read(cpu.hl.full()), cpu.af.hi());
        },
        0xFE=> {
            // CP A,#
            gb.instCp(cpu.pop_pc(), cpu.af.hi());
        },
        0x3C=> {
            // INC A
            gb.instInc(cpu.af.set_hi, cpu.af.hi());
        },
        0x04=> {
            // INC B
            gb.instInc(cpu.bc.set_hi, cpu.bc.hi());
        },
        0x0C=> {
            // INC C
            gb.instInc(cpu.bc.set_lo, cpu.bc.lo());
        },
        0x14=> {
            // INC D
            gb.instInc(cpu.de.set_hi, cpu.de.hi());
        },
        0x1C=> {
            // INC E
            gb.instInc(cpu.de.set_lo, cpu.de.lo());
        },
        0x24=> {
            // INC H
            gb.instInc(cpu.hl.set_hi, cpu.hl.hi());
        },
        0x2C=> {
            // INC L
            gb.instInc(cpu.hl.set_lo, cpu.hl.lo());
        },
        0x34=> {
            // INC (HL);
            addr := cpu.hl.full();
            gb.instInc(func(val byte) { cpu.mmu.write(addr, val) }, cpu.mmu.read(addr));
        },
        0x3D=> {
            // DEC A
            gb.instDec(cpu.af.set_hi, cpu.af.hi());
        },
        0x05=> {
            // DEC B
            gb.instDec(cpu.bc.set_hi, cpu.bc.hi());
        },
        0x0D=> {
            // DEC C
            gb.instDec(cpu.bc.set_lo, cpu.bc.lo());
        },
        0x15=> {
            // DEC D
            gb.instDec(cpu.de.set_hi, cpu.de.hi());
        },
        0x1D=> {
            // DEC E
            gb.instDec(cpu.de.set_lo, cpu.de.lo());
        },
        0x25=> {
            // DEC H
            gb.instDec(cpu.hl.set_hi, cpu.hl.hi());
        },
        0x2D=> {
            // DEC L
            gb.instDec(cpu.hl.set_lo, cpu.hl.lo());
        },
        0x35=> {
            // DEC (HL);
            addr := cpu.hl.full();
            gb.instDec(func(val byte) { cpu.mmu.write(addr, val) }, cpu.mmu.read(addr));
        },
        // ========== 16-Bit ALU ===========
        0x09=> {
            // ADD HL,bc
            gb.instAdd16(cpu.hl.Set, cpu.hl.full(), cpu.bc.full());
        },
        0x19=> {
            // ADD HL,DE
            gb.instAdd16(cpu.hl.Set, cpu.hl.full(), cpu.de.full());
        },
        0x29=> {
            // ADD HL,HL
            gb.instAdd16(cpu.hl.Set, cpu.hl.full(), cpu.hl.full());
        },
        0x39=> {
            // ADD HL,SP
            gb.instAdd16(cpu.hl.Set, cpu.hl.full(), cpu.sp.full());
        },
        0xE8=> {
            // ADD SP,n
            gb.instAdd16Signed(cpu.sp.Set, cpu.sp.full(), int8(cpu.pop_pc()));
            cpu.SetZ(false);
        },
        0x03=> {
            // INC bc
            gb.instInc16(cpu.bc.Set, cpu.bc.full());
        },
        0x13=> {
            // INC DE
            gb.instInc16(cpu.de.Set, cpu.de.full());
        },
        0x23=> {
            // INC HL
            gb.instInc16(cpu.hl.Set, cpu.hl.full());
        },
        0x33=> {
            // INC SP
            gb.instInc16(cpu.sp.Set, cpu.sp.full());
        },
        0x0B=> {
            // DEC bc
            gb.instDec16(cpu.bc.Set, cpu.bc.full());
        },
        0x1B=> {
            // DEC DE
            gb.instDec16(cpu.de.Set, cpu.de.full());
        },
        0x2B=> {
            // DEC HL
            gb.instDec16(cpu.hl.Set, cpu.hl.full());
        },
        0x3B=> {
            // DEC SP
            gb.instDec16(cpu.sp.Set, cpu.sp.full());
        },
        0x27=> {
            // DAA

            // When this instruction is executed, the A register is bcD
            // corrected using the contents of the flags. The exact process
            // is the following: if the least significant four bits of A
            // contain a non-bcD digit (i. e. it is greater than 9) or the
            // H flag is set, then 0x60 is added to the register. Then the
            // four most significant bits are checked. If this more significant
            // digit also happens to be greater than 9 or the C flag is set,
            // then 0x60 is added.
            if !cpu.N() {
                if cpu.C() || cpu.af.hi() > 0x99 {
                    cpu.af.set_hi(cpu.af.hi() + 0x60);
                    cpu.SetC(true);
                }
                if cpu.H() || cpu.af.hi()&0xF > 0x9 {
                    cpu.af.set_hi(cpu.af.hi() + 0x06);
                    cpu.SetH(false);
                }
            } else if cpu.C() && cpu.H() {
                cpu.af.set_hi(cpu.af.hi() + 0x9A);
                cpu.SetH(false);
            } else if cpu.C() {
                cpu.af.set_hi(cpu.af.hi() + 0xA0);
            } else if cpu.H() {
                cpu.af.set_hi(cpu.af.hi() + 0xFA);
                cpu.SetH(false);
            }
            cpu.SetZ(cpu.af.hi() == 0);
        },
        0x2F=> {
            // CPL
            cpu.af.set_hi(0xFF ^ cpu.af.hi());
            cpu.SetN(true);
            cpu.SetH(true);
        },
        0x3F=> {
            // CCF
            cpu.SetN(false);
            cpu.SetH(false);
            cpu.SetC(!cpu.C());
        },
        0x37=> {
            // SCF
            cpu.SetN(false);
            cpu.SetH(false);
            cpu.SetC(true);
        },
        0x00=> {
            // NOP
        },
        0x76=> {
            // HALT
            gb.halted = true
        },
        0x10=> {
            // STOP
            gb.halted = true
            if gb.IsCGB() {
                // Handle switching to double speed mode
                gb.checkSpeedSwitch();
            }

            // Pop the next value as the STOP instruction is 2 bytes long. The second value
            // can be ignored, although generally it is expected to be 0x00 and any other
            // value is counted as a corrupted STOP instruction.
            cpu.pop_pc();
        },
        0xF3=> {
            // DI
            gb.interruptsOn = false
        },
        0xFB=> {
            // EI
            gb.interruptsEnabling = true
        },
        0x07=> {
            // RLCA
            value := cpu.af.hi();
            result := byte(value<<1) | (value >> 7);
            cpu.af.set_hi(result);
            cpu.SetZ(false);
            cpu.SetN(false);
            cpu.SetH(false);
            cpu.SetC(value > 0x7F);
        },
        0x17=> {
            // RLA
            value := cpu.af.hi();
            var carry byte
            if cpu.C() {
                carry = 1
            }
            result := byte(value<<1) + carry
            cpu.af.set_hi(result);
            cpu.SetZ(false);
            cpu.SetN(false);
            cpu.SetH(false);
            cpu.SetC(value > 0x7F);
        },
        0x0F=> {
            // RRCA
            value := cpu.af.hi();
            result := byte(value>>1) | byte((value&1)<<7);
            cpu.af.set_hi(result);
            cpu.SetZ(false);
            cpu.SetN(false);
            cpu.SetH(false);
            cpu.SetC(result > 0x7F);
        },
        0x1F=> {
            // RRA
            value := cpu.af.hi();
            var carry byte
            if cpu.C() {
                carry = 0x80
            }
            result := byte(value>>1) | carry
            cpu.af.set_hi(result);
            cpu.SetZ(false);
            cpu.SetN(false);
            cpu.SetH(false);
            cpu.SetC((1 & value) == 1);
        },
        0xC3=> {
            // JP nn
            gb.instJump(cpu.pop_pc16());
        },
        0xC2=> {
            // JP NZ,nn
            next := cpu.pop_pc16();
            if !cpu.Z() {
                gb.instJump(next);
                gb.thisCpuTicks += 4
            }
        },
        0xCA=> {
            // JP Z,nn
            next := cpu.pop_pc16();
            if cpu.Z() {
                gb.instJump(next);
                gb.thisCpuTicks += 4
            }
        },
        0xD2=> {
            // JP NC,nn
            next := cpu.pop_pc16();
            if !cpu.C() {
                gb.instJump(next);
                gb.thisCpuTicks += 4
            }
        },
        0xDA=> {
            // JP C,nn
            next := cpu.pop_pc16();
            if cpu.C() {
                gb.instJump(next);
                gb.thisCpuTicks += 4
            }
        },
        0xE9=> {
            // JP HL
            gb.instJump(cpu.hl.full());
        },
        0x18=> {
            // JR n
            addr := int32(cpu.PC) + int32(int8(cpu.pop_pc()));
            gb.instJump((addr) as u16)
        },
        0x20=> {
            // JR NZ,n
            next := int8(cpu.pop_pc());
            if !cpu.Z() {
                addr := int32(cpu.PC) + int32(next);
                gb.instJump((addr) as u16)
                gb.thisCpuTicks += 4
            }
        },
        0x28=> {
            // JR Z,n
            next := int8(cpu.pop_pc());
            if cpu.Z() {
                addr := int32(cpu.PC) + int32(next);
                gb.instJump((addr) as u16)
                gb.thisCpuTicks += 4
            }
        },
        0x30=> {
            // JR NC,n
            next := int8(cpu.pop_pc());
            if !cpu.C() {
                addr := int32(cpu.PC) + int32(next);
                gb.instJump((addr) as u16)
                gb.thisCpuTicks += 4
            }
        },
        0x38=> {
            // JR C,n
            next := int8(cpu.pop_pc());
            if cpu.C() {
                addr := int32(cpu.PC) + int32(next);
                gb.instJump((addr) as u16)
                gb.thisCpuTicks += 4
            }
        },
        0xCD=> {
            // CALL nn
            gb.instCall(cpu.pop_pc16());
        },
        0xC4=> {
            // CALL NZ,nn
            next := cpu.pop_pc16();
            if !cpu.Z() {
                gb.instCall(next);
                gb.thisCpuTicks += 12
            }
        },
        0xCC=> {
            // CALL Z,nn
            next := cpu.pop_pc16();
            if cpu.Z() {
                gb.instCall(next);
                gb.thisCpuTicks += 12
            }
        },
        0xD4=> {
            // CALL NC,nn
            next := cpu.pop_pc16();
            if !cpu.C() {
                gb.instCall(next);
                gb.thisCpuTicks += 12
            }
        },
        0xDC=> {
            // CALL C,nn
            next := cpu.pop_pc16();
            if cpu.C() {
                gb.instCall(next);
                gb.thisCpuTicks += 12
            }
        },
        0xC7=> {
            // RST 0x00
            gb.instCall(0x0000);
        },
        0xCF=> {
            // RST 0x08
            gb.instCall(0x0008);
        },
        0xD7=> {
            // RST 0x10
            gb.instCall(0x0010);
        },
        0xDF=> {
            // RST 0x18
            gb.instCall(0x0018);
        },
        0xE7=> {
            // RST 0x20
            gb.instCall(0x0020);
        },
        0xEF=> {
            // RST 0x28
            gb.instCall(0x0028);
        },
        0xF7=> {
            // RST 0x30
            gb.instCall(0x0030);
        },
        0xFF=> {
            // RST 0x38
            gb.instCall(0x0038);
        },
        0xC9=> {
            // RET
            gb.instRet();
        },
        0xC0=> {
            // RET NZ
            if !cpu.Z() {
                gb.instRet();
                gb.thisCpuTicks += 12
            }
        },
        0xC8=> {
            // RET Z
            if cpu.Z() {
                gb.instRet();
                gb.thisCpuTicks += 12
            }
        },
        0xD0=> {
            // RET NC
            if !cpu.C() {
                gb.instRet();
                gb.thisCpuTicks += 12
            }
        },
        0xD8=> {
            // RET C
            if cpu.C() {
                gb.instRet();
                gb.thisCpuTicks += 12
            }
        },
        0xD9=> {
            // RETI
            gb.instRet();
            gb.interruptsEnabling = true
        },
        0xCB=> {
            // CB
            nextInst := cpu.pop_pc();
            gb.thisCpuTicks += CBOpcodeCycles[nextInst] * 4
            gb.cbInst[nextInst]();
        },
        _ :=> {}
    }
}

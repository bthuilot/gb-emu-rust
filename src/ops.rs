pub mod ops {
    use crate::cpu::cpu::Z80;

    pub fn find_op(cpu: &mut Z80, code: u8) {
        match code {
            0 => {
                cpu.registers.m = 1;
            }
            1 => {
                cpu.registers.c=cpu.cpu.mmu.rb(cpu.registers.pc, cpu.registers.pc);
                cpu.registers.b=cpu.cpu.mmu.rb(cpu.registers.pc+1, cpu.registers.pc);
                cpu.registers.pc+=2;
                cpu.registers.m=3;
            }
            2 => {
                cpu.cpu.mmu.wb(((cpu.registers.b as u16)<<8) + (cpu.registers.c as u16), cpu.registers.a);
                cpu.registers.m=2;
            }
            3 => {
                cpu.registers.c=(cpu.registers.c+1)&255;
                if cpu.registers.c == 0 {
                    cpu.registers.b=(cpu.registers.b+1)&255;
                }
                cpu.registers.m=1;
            }
            4 => {
                cpu.registers.b += 1;
                cpu.registers.b &= 255;
                cpu.registers.f = if cpu.registers.b != 0 { 0 } else { 0x80 };
                cpu.registers.m=1;
            }
            5 => {
                cpu.registers.b -= 1;
                cpu.registers.b&=255;
                cpu.registers.f = if cpu.registers.b != 0 { 0 } else { 0x80 };
                cpu.registers.m=1;
            }
            6 => {
                cpu.registers.b=cpu.cpu.mmu.rb(cpu.registers.pc, cpu.registers.pc);
                cpu.registers.pc+=1;
cpu.registers.m=2;
            }
            7 => {
                let ci= if cpu.registers.a&0x80 != 0 { 1 } else {0};
                let co= if cpu.registers.a&0x80 != 0 {0x10} else {0};
                cpu.registers.a=(cpu.registers.a<<1)+ci;
                cpu.registers.a&=255;
                cpu.registers.f=(cpu.registers.f&0xEF)+co;
                cpu.registers.m=1;
            }
            8 => {
                // TODO
            }
            9 => {
                let mut hl =((cpu.registers.h as u16 )<<8)+ (cpu.registers.l as u16);
                hl+=((cpu.registers.b as u16) <<8) + (cpu.registers.c as u16);
                if hl>65535 {
                    cpu.registers.f|=0x10;
                } else {
                    cpu.registers.f&=0xEF;
                }
                cpu.registers.h= ((hl>>8) as u8) & 255;
                cpu.registers.l=(hl as u8)&255;
                cpu.registers.m=3;
            }
            10 => {
                cpu.registers.a=cpu.mmu.rb(((cpu.registers.b as u16)<<8)+(cpu.registers.c as u16), cpu.registers.pc);
                cpu.registers.m=2;
            }
            11 => {
                cpu.registers.c=(cpu.registers.c-1)&255;
                if cpu.registers.c == 255 {
                    cpu.registers.b=(cpu.registers.b-1)&255;
                }
                cpu.registers.m=1;
            }
            12 => {
                cpu.registers.c+=1;
                cpu.registers.c&=255;
                cpu.registers.f = if cpu.registers.c != 0 { 0 } else { 0x80};
                cpu.registers.m=1;
            }
            13 => {
                cpu.registers.c-=1;
                cpu.registers.c&=255;
                cpu.registers.f=if cpu.registers.c != 0 { 0 } else {0x80};
                cpu.registers.m=1;
            }
            14 => {
                cpu.registers.c = cpu.mmu.rb(cpu.registers.pc, cpu.registers.pc);
                cpu.registers.pc+=1;
                cpu.registers.m=2;
            }
            15 => {
                let ci= if (cpu.registers.a&1) != 0 {0x80} else {0};
                let co= if (cpu.registers.a&1) != 0 {0x10} else {0};
                cpu.registers.a=(cpu.registers.a>>1)+ci;
                cpu.registers.a&=255;
                cpu.registers.f=(cpu.registers.f&0xEF)+co;
                cpu.registers.m=1;
            }
            16 => {
                let mut i =cpu.mmu.rb(cpu.registers.pc, cpu.registers.pc);
                if i>127 {
                    i = - ((!i + 1) & 255);
                    // TODO clean this up
                }
                cpu.registers.pc+=1;
                cpu.registers.m=2;
                cpu.registers.b-=1;
                if cpu.registers.b != 0 {
                    cpu.registers.pc+= (i as u16);
                    cpu.registers.m+=1;
                }
            }
            17 => {
                cpu.registers. e= cpu.mmu.rb(cpu.registers.pc, cpu.registers.pc);
                cpu.registers.d = cpu.mmu.rb(cpu.registers.pc+1, cpu.registers.pc);
                cpu.registers.pc+=2;
                cpu.registers.m=3;
            }
            18 => {
                cpu.mmu.wb(((cpu.registers.d as u16) <<8) + (cpu.registers.e as u16), cpu.registers.a);
                cpu.registers.m=2;
            }
            19 => {
                cpu.registers.e=(cpu.registers.e+1)&255;
                if cpu.registers.e == 0 {
                    cpu.registers.d = (cpu.registers.d + 1) & 255;
                }
                cpu.registers.m=1;
            }
            20 => {
                cpu.registers.d+=1;
                cpu.registers.d&=255;
                cpu.registers.f= if cpu.registers.d != 0 {0}  else {0x80};
                cpu.registers.m=1;
            }
            21 => {
                cpu.registers.d-=1;
                cpu.registers.d&=255;
                cpu.registers.f = if cpu.registers.d != 0 {0} else {0x80};
                cpu.registers.m=1;
            }
            22 => {
                cpu.registers.d=cpu.mmu.rb(cpu.registers.pc, cpu.registers.pc);
                cpu.registers.pc+=1;
                cpu.registers.m=2;
            }
            23 => {
                let ci=if cpu.registers.f&0x10 != 0 {1} else {0};
                let co=if cpu.registers.a&0x80 != 0 {0x10} else {0};
                cpu.registers.a=(cpu.registers.a<<1)+ci;
                cpu.registers.a&=255;
                cpu.registers.f= (cpu.registers.f&0xEF) + co;
                cpu.registers.m=1;
            }
            24 => {
                let mut i=cpu.mmu.rb(cpu.registers.pc, cpu.registers.pc);
                if i>127 {
                    i=-((!i+1)&255);
                }
                cpu.registers.pc+=1;
                cpu.registers.m=2;
                cpu.registers.pc += (i as u16);
                cpu.registers.m+=1;
            }
            25 => {
                let mut hl= ((cpu.registers.h as u16)<<8) + (cpu.registers.l as u16);
                hl += ((cpu.registers.d as u16) <<8)+ (cpu.registers.e as u16);
                if hl>65535 {
                    cpu.registers.f |= 0x10;
                } else {
                    cpu.registers.f &= 0xEF;
                }
                cpu.registers.h = ((hl>>8) as u8) & 255;
                cpu.registers.l = (hl&255) as u8;
                cpu.registers.m=3;
            }
            26 => {
                cpu.registers.a=cpu.mmu.rb(((cpu.registers.d as u16)<<8)+ (cpu.registers.e as u16), cpu.registers.pc);
                cpu.registers.m=2;
            }
            27 => {
                cpu.registers.e=(cpu.registers.e-1)&255;
                if cpu.registers.e == 255 {
                    cpu.registers.d=(cpu.registers.d-1)&255;
                }
                cpu.registers.m=1;
            }
            28 => {
                cpu.registers.e+=1;
                cpu.registers.e&=255;
                cpu.registers.f = if cpu.registers.e != 0 {0} else {0x80};
                cpu.registers.m=1;
            }
            29 => {
                cpu.registers.e-=1;
                cpu.registers.e&=255;
                cpu.registers.f=if cpu.registers.e != 0 {0} else {0x80};
                cpu.registers.m=1;
            }
            30 => {

                cpu.registers.e=cpu.mmu.rb(cpu.registers.pc); cpu.registers.pc+=1; cpu.registers.m=2;
            }

            31 => {

                let ci=cpu.registers.f&0x10?0x80:0; let co=cpu.registers.a&1?0x10:0; cpu.registers.a=(cpu.registers.a>>1)+ci; cpu.registers.a&=255; cpu.registers.f=(cpu.registers.f&0xEF)+co; cpu.registers.m=1;
            }

            32 => {

                let i=cpu.mmu.rb(cpu.registers.pc); if(i>127) i=-((~i+1)&255); cpu.registers.pc+=1; cpu.registers.m=2; if((cpu.registers.f&0x80)==0x00) { cpu.registers.pc+=i; cpu.registers.m+=1; }
            }

            33 => {

                cpu.registers.l=cpu.mmu.rb(cpu.registers.pc); cpu.registers.h=cpu.mmu.rb(cpu.registers.pc+1); cpu.registers.pc+=2; cpu.registers.m=3;
            }

            34 => {

                cpu.mmu.wb((cpu.registers.h<<8)+cpu.registers.l, cpu.registers.a); cpu.registers.l=(cpu.registers.l+1)&255; if(!cpu.registers.l) cpu.registers.h=(cpu.registers.h+1)&255; cpu.registers.m=2;
            }

            35 => {

                cpu.registers.l=(cpu.registers.l+1)&255; if(!cpu.registers.l) cpu.registers.h=(cpu.registers.h+1)&255; cpu.registers.m=1;
            }

            36 => {

                cpu.registers.h+=1; cpu.registers.h&=255; cpu.registers.f=cpu.registers.h?0:0x80; cpu.registers.m=1;
            }

            37 => {

                cpu.registers.h-=1; cpu.registers.h&=255; cpu.registers.f=cpu.registers.h?0:0x80; cpu.registers.m=1;
            }

            38 => {

                cpu.registers.h=cpu.mmu.rb(cpu.registers.pc); cpu.registers.pc+=1; cpu.registers.m=2;
            }

            39 => {

                let a=cpu.registers.a; if((cpu.registers.f&0x20)||((cpu.registers.a&15)>9)) cpu.registers.a+=6; cpu.registers.f&=0xEF; if((cpu.registers.f&0x20)||(a>0x99)) { cpu.registers.a+=0x60; cpu.registers.f|=0x10; } cpu.registers.m=1;
            }

            40 => {

                let i=cpu.mmu.rb(cpu.registers.pc); if(i>127) i=-((~i+1)&255); cpu.registers.pc+=1; cpu.registers.m=2; if((cpu.registers.f&0x80)==0x80) { cpu.registers.pc+=i; cpu.registers.m+=1; }
            }

            41 => {

                let hl=(cpu.registers.h<<8)+cpu.registers.l; hl+=(cpu.registers.h<<8)+cpu.registers.l; if(hl>65535) cpu.registers.f|=0x10; else cpu.registers.f&=0xEF; cpu.registers.h=(hl>>8)&255; cpu.registers.l=hl&255; cpu.registers.m=3;
            }

            42 => {

                cpu.registers.a=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.l=(cpu.registers.l+1)&255; if(!cpu.registers.l) cpu.registers.h=(cpu.registers.h+1)&255; cpu.registers.m=2;
            }

            43 => {

                cpu.registers.l=(cpu.registers.l-1)&255; if(cpu.registers.l==255) cpu.registers.h=(cpu.registers.h-1)&255; cpu.registers.m=1;
            }

            44 => {

                cpu.registers.l+=1; cpu.registers.l&=255; cpu.registers.f=cpu.registers.l?0:0x80; cpu.registers.m=1;
            }

            45 => {

                cpu.registers.l-=1; cpu.registers.l&=255; cpu.registers.f=cpu.registers.l?0:0x80; cpu.registers.m=1;
            }

            46 => {

                cpu.registers.l=cpu.mmu.rb(cpu.registers.pc); cpu.registers.pc+=1; cpu.registers.m=2;
            }

            47 => {

                cpu.registers.a ^= 255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            48 => {

                let i=cpu.mmu.rb(cpu.registers.pc); if(i>127) i=-((~i+1)&255); cpu.registers.pc+=1; cpu.registers.m=2; if((cpu.registers.f&0x10)==0x00) { cpu.registers.pc+=i; cpu.registers.m+=1; }
            }

            49 => {

                cpu.registers.sp=cpu.mmu.rw(cpu.registers.pc); cpu.registers.pc+=2; cpu.registers.m=3;
            }

            50 => {

                cpu.mmu.wb((cpu.registers.h<<8)+cpu.registers.l, cpu.registers.a); cpu.registers.l=(cpu.registers.l-1)&255; if(cpu.registers.l==255) cpu.registers.h=(cpu.registers.h-1)&255; cpu.registers.m=2;
            }

            51 => {

                cpu.registers.sp=(cpu.registers.sp+1)&65535; cpu.registers.m=1;
            }

            52 => {

                let i=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l)+1; i&=255; cpu.mmu.wb((cpu.registers.h<<8)+cpu.registers.l,i); cpu.registers.f=i?0:0x80; cpu.registers.m=3;
            }

            53 => {

                let i=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l)-1; i&=255; cpu.mmu.wb((cpu.registers.h<<8)+cpu.registers.l,i); cpu.registers.f=i?0:0x80; cpu.registers.m=3;
            }

            54 => {

                cpu.mmu.wb((cpu.registers.h<<8)+cpu.registers.l, cpu.mmu.rb(cpu.registers.pc)); cpu.registers.pc+=1; cpu.registers.m=3;
            }

            55 => {

                cpu.registers.f|=0x10; cpu.registers.m=1;
            }

            56 => {

                let i=cpu.mmu.rb(cpu.registers.pc); if(i>127) i=-((~i+1)&255); cpu.registers.pc+=1; cpu.registers.m=2; if((cpu.registers.f&0x10)==0x10) { cpu.registers.pc+=i; cpu.registers.m+=1; }
            }

            57 => {

                let hl=(cpu.registers.h<<8)+cpu.registers.l; hl+=cpu.registers.sp; if(hl>65535) cpu.registers.f|=0x10; else cpu.registers.f&=0xEF; cpu.registers.h=(hl>>8)&255; cpu.registers.l=hl&255; cpu.registers.m=3;
            }

            58 => {

                cpu.registers.a=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.l=(cpu.registers.l-1)&255; if(cpu.registers.l==255) cpu.registers.h=(cpu.registers.h-1)&255; cpu.registers.m=2;
            }

            59 => {

                cpu.registers.sp=(cpu.registers.sp-1)&65535; cpu.registers.m=1;
            }

            60 => {

                cpu.registers.a+=1; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            61 => {

                cpu.registers.a-=1; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            62 => {

                cpu.registers.a=cpu.mmu.rb(cpu.registers.pc); cpu.registers.pc+=1; cpu.registers.m=2;
            }

            63 => {

                let ci=cpu.registers.f&0x10?0:0x10; cpu.registers.f=(cpu.registers.f&0xEF)+ci; cpu.registers.m=1;
            }

            64 => {

                cpu.registers.b=cpu.registers.b; cpu.registers.m=1;
            }

            65 => {

                cpu.registers.b=cpu.registers.c; cpu.registers.m=1;
            }

            66 => {

                cpu.registers.b=cpu.registers.d; cpu.registers.m=1;
            }

            67 => {

                cpu.registers.b=cpu.registers.e; cpu.registers.m=1;
            }

            68 => {

                cpu.registers.b=cpu.registers.h; cpu.registers.m=1;
            }

            69 => {

                cpu.registers.b=cpu.registers.l; cpu.registers.m=1;
            }

            70 => {

                cpu.registers.b=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.m=2;
            }

            71 => {

                cpu.registers.b=cpu.registers.a; cpu.registers.m=1;
            }

            72 => {

                cpu.registers.c=cpu.registers.b; cpu.registers.m=1;
            }

            73 => {

                cpu.registers.c=cpu.registers.c; cpu.registers.m=1;
            }

            74 => {

                cpu.registers.c=cpu.registers.d; cpu.registers.m=1;
            }

            75 => {

                cpu.registers.c=cpu.registers.e; cpu.registers.m=1;
            }

            76 => {

                cpu.registers.c=cpu.registers.h; cpu.registers.m=1;
            }

            77 => {

                cpu.registers.c=cpu.registers.l; cpu.registers.m=1;
            }

            78 => {

                cpu.registers.c=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.m=2;
            }

            79 => {

                cpu.registers.c=cpu.registers.a; cpu.registers.m=1;
            }

            80 => {

                cpu.registers.d=cpu.registers.b; cpu.registers.m=1;
            }

            81 => {

                cpu.registers.d=cpu.registers.c; cpu.registers.m=1;
            }

            82 => {

                cpu.registers.d=cpu.registers.d; cpu.registers.m=1;
            }

            83 => {

                cpu.registers.d=cpu.registers.e; cpu.registers.m=1;
            }

            84 => {

                cpu.registers.d=cpu.registers.h; cpu.registers.m=1;
            }

            85 => {

                cpu.registers.d=cpu.registers.l; cpu.registers.m=1;
            }

            86 => {

                cpu.registers.d=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.m=2;
            }

            87 => {

                cpu.registers.d=cpu.registers.a; cpu.registers.m=1;
            }

            88 => {

                cpu.registers.e=cpu.registers.b; cpu.registers.m=1;
            }

            89 => {

                cpu.registers.e=cpu.registers.c; cpu.registers.m=1;
            }

            90 => {

                cpu.registers.e=cpu.registers.d; cpu.registers.m=1;
            }

            91 => {

                cpu.registers.e=cpu.registers.e; cpu.registers.m=1;
            }

            92 => {

                cpu.registers.e=cpu.registers.h; cpu.registers.m=1;
            }

            93 => {

                cpu.registers.e=cpu.registers.l; cpu.registers.m=1;
            }

            94 => {

                cpu.registers.e=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.m=2;
            }

            95 => {

                cpu.registers.e=cpu.registers.a; cpu.registers.m=1;
            }

            96 => {

                cpu.registers.h=cpu.registers.b; cpu.registers.m=1;
            }

            97 => {

                cpu.registers.h=cpu.registers.c; cpu.registers.m=1;
            }

            98 => {

                cpu.registers.h=cpu.registers.d; cpu.registers.m=1;
            }

            99 => {

                cpu.registers.h=cpu.registers.e; cpu.registers.m=1;
            }

            100 => {

                cpu.registers.h=cpu.registers.h; cpu.registers.m=1;
            }

            101 => {

                cpu.registers.h=cpu.registers.l; cpu.registers.m=1;
            }

            102 => {

                cpu.registers.h=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.m=2;
            }

            103 => {

                cpu.registers.h=cpu.registers.a; cpu.registers.m=1;
            }

            104 => {

                cpu.registers.l=cpu.registers.b; cpu.registers.m=1;
            }

            105 => {

                cpu.registers.l=cpu.registers.c; cpu.registers.m=1;
            }

            106 => {

                cpu.registers.l=cpu.registers.d; cpu.registers.m=1;
            }

            107 => {

                cpu.registers.l=cpu.registers.e; cpu.registers.m=1;
            }

            108 => {

                cpu.registers.l=cpu.registers.h; cpu.registers.m=1;
            }

            109 => {

                cpu.registers.l=cpu.registers.l; cpu.registers.m=1;
            }

            110 => {

                cpu.registers.l=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.m=2;
            }

            111 => {

                cpu.registers.l=cpu.registers.a; cpu.registers.m=1;
            }

            112 => {

                cpu.mmu.wb((cpu.registers.h<<8)+cpu.registers.l,cpu.registers.b); cpu.registers.m=2;
            }

            113 => {

                cpu.mmu.wb((cpu.registers.h<<8)+cpu.registers.l,cpu.registers.c); cpu.registers.m=2;
            }

            114 => {

                cpu.mmu.wb((cpu.registers.h<<8)+cpu.registers.l,cpu.registers.d); cpu.registers.m=2;
            }

            115 => {

                cpu.mmu.wb((cpu.registers.h<<8)+cpu.registers.l,cpu.registers.e); cpu.registers.m=2;
            }

            116 => {

                cpu.mmu.wb((cpu.registers.h<<8)+cpu.registers.l,cpu.registers.h); cpu.registers.m=2;
            }

            117 => {

                cpu.mmu.wb((cpu.registers.h<<8)+cpu.registers.l,cpu.registers.l); cpu.registers.m=2;
            }

            118 => {

                Z80._halt=1; cpu.registers.m=1;
            }

            119 => {

                cpu.mmu.wb((cpu.registers.h<<8)+cpu.registers.l,cpu.registers.a); cpu.registers.m=2;
            }

            120 => {

                cpu.registers.a=cpu.registers.b; cpu.registers.m=1;
            }

            121 => {

                cpu.registers.a=cpu.registers.c; cpu.registers.m=1;
            }

            122 => {

                cpu.registers.a=cpu.registers.d; cpu.registers.m=1;
            }

            123 => {

                cpu.registers.a=cpu.registers.e; cpu.registers.m=1;
            }

            124 => {

                cpu.registers.a=cpu.registers.h; cpu.registers.m=1;
            }

            125 => {

                cpu.registers.a=cpu.registers.l; cpu.registers.m=1;
            }

            126 => {

                cpu.registers.a=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.m=2;
            }

            127 => {

                cpu.registers.a=cpu.registers.a; cpu.registers.m=1;
            }

            128 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.b; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.b^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            129 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.c; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.c^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            130 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.d; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.d^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            131 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.e; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.e^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            132 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.h; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.h^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            133 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.l; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.l^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            134 => {

                let a=cpu.registers.a; let m=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.a+=m; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^a^m)&0x10) cpu.registers.f|=0x20; cpu.registers.m=2;
            }

            135 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.a; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.a^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            136 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.b; cpu.registers.a+=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.b^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            137 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.c; cpu.registers.a+=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.c^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            138 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.d; cpu.registers.a+=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.d^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            139 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.e; cpu.registers.a+=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.e^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            140 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.h; cpu.registers.a+=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.h^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            141 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.l; cpu.registers.a+=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.l^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            142 => {

                let a=cpu.registers.a; let m=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.a+=m; cpu.registers.a+=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^m^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=2;
            }

            143 => {

                let a=cpu.registers.a; cpu.registers.a+=cpu.registers.a; cpu.registers.a+=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.a^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            144 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.b; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.b^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            145 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.c; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.c^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            146 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.d; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.d^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            147 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.e; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.e^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            148 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.h; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.h^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            149 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.l; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.l^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            150 => {

                let a=cpu.registers.a; let m=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.a-=m; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^m^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=2;
            }

            151 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.a; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.a^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            152 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.b; cpu.registers.a-=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.b^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            153 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.c; cpu.registers.a-=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.c^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            154 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.d; cpu.registers.a-=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.d^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            155 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.e; cpu.registers.a-=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.e^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            156 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.h; cpu.registers.a-=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.h^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            157 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.l; cpu.registers.a-=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.l^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            158 => {

                let a=cpu.registers.a; let m=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.a-=m; cpu.registers.a-=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^m^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=2;
            }

            159 => {

                let a=cpu.registers.a; cpu.registers.a-=cpu.registers.a; cpu.registers.a-=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.a^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            160 => {

                cpu.registers.a&=cpu.registers.b; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            161 => {

                cpu.registers.a&=cpu.registers.c; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            162 => {

                cpu.registers.a&=cpu.registers.d; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            163 => {

                cpu.registers.a&=cpu.registers.e; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            164 => {

                cpu.registers.a&=cpu.registers.h; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            165 => {

                cpu.registers.a&=cpu.registers.l; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            166 => {

                cpu.registers.a&=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=2;
            }

            167 => {

                cpu.registers.a&=cpu.registers.a; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            168 => {

                cpu.registers.a^=cpu.registers.b; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            169 => {

                cpu.registers.a^=cpu.registers.c; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            170 => {

                cpu.registers.a^=cpu.registers.d; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            171 => {

                cpu.registers.a^=cpu.registers.e; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            172 => {

                cpu.registers.a^=cpu.registers.h; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            173 => {

                cpu.registers.a^=cpu.registers.l; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            174 => {

                cpu.registers.a^=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=2;
            }

            175 => {

                cpu.registers.a^=cpu.registers.a; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            176 => {

                cpu.registers.a|=cpu.registers.b; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            177 => {

                cpu.registers.a|=cpu.registers.c; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            178 => {

                cpu.registers.a|=cpu.registers.d; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            179 => {

                cpu.registers.a|=cpu.registers.e; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            180 => {

                cpu.registers.a|=cpu.registers.h; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            181 => {

                cpu.registers.a|=cpu.registers.l; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            182 => {

                cpu.registers.a|=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=2;
            }

            183 => {

                cpu.registers.a|=cpu.registers.a; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=1;
            }

            184 => {

                let i=cpu.registers.a; i-=cpu.registers.b; cpu.registers.f=(i<0)?0x50:0x40; i&=255; if(!i) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.b^i)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            185 => {

                let i=cpu.registers.a; i-=cpu.registers.c; cpu.registers.f=(i<0)?0x50:0x40; i&=255; if(!i) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.c^i)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            186 => {

                let i=cpu.registers.a; i-=cpu.registers.d; cpu.registers.f=(i<0)?0x50:0x40; i&=255; if(!i) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.d^i)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            187 => {

                let i=cpu.registers.a; i-=cpu.registers.e; cpu.registers.f=(i<0)?0x50:0x40; i&=255; if(!i) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.e^i)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            188 => {

                let i=cpu.registers.a; i-=cpu.registers.h; cpu.registers.f=(i<0)?0x50:0x40; i&=255; if(!i) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.h^i)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            189 => {

                let i=cpu.registers.a; i-=cpu.registers.l; cpu.registers.f=(i<0)?0x50:0x40; i&=255; if(!i) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.l^i)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            190 => {

                let i=cpu.registers.a; let m=cpu.mmu.rb((cpu.registers.h<<8)+cpu.registers.l); i-=m; cpu.registers.f=(i<0)?0x50:0x40; i&=255; if(!i) cpu.registers.f|=0x80; if((cpu.registers.a^i^m)&0x10) cpu.registers.f|=0x20; cpu.registers.m=2;
            }

            191 => {

                let i=cpu.registers.a; i-=cpu.registers.a; cpu.registers.f=(i<0)?0x50:0x40; i&=255; if(!i) cpu.registers.f|=0x80; if((cpu.registers.a^cpu.registers.a^i)&0x10) cpu.registers.f|=0x20; cpu.registers.m=1;
            }

            192 => {

                cpu.registers.m=1; if((cpu.registers.f&0x80)==0x00) { cpu.registers.pc=cpu.mmu.rw(cpu.registers.sp); cpu.registers.sp+=2; cpu.registers.m+=2; }
            }

            193 => {

                cpu.registers.c=cpu.mmu.rb(cpu.registers.sp); cpu.registers.sp+=1; cpu.registers.b=cpu.mmu.rb(cpu.registers.sp); cpu.registers.sp+=1; cpu.registers.m=3;
            }

            194 => {

                cpu.registers.m=3; if((cpu.registers.f&0x80)==0x00) { cpu.registers.pc=cpu.mmu.rw(cpu.registers.pc); cpu.registers.m+=1; } else cpu.registers.pc+=2;
            }

            195 => {

                cpu.registers.pc = cpu.mmu.rw(cpu.registers.pc); cpu.registers.m=3;
            }

            196 => {

                cpu.registers.m=3; if((cpu.registers.f&0x80)==0x00) { cpu.registers.sp-=2; cpu.mmu.ww(cpu.registers.sp,cpu.registers.pc+2); cpu.registers.pc=cpu.mmu.rw(cpu.registers.pc); cpu.registers.m+=2; } else cpu.registers.pc+=2;
            }

            197 => {

                cpu.registers.sp-=1; cpu.mmu.wb(cpu.registers.sp,cpu.registers.b); cpu.registers.sp-=1; cpu.mmu.wb(cpu.registers.sp,cpu.registers.c); cpu.registers.m=3;
            }

            198 => {

                let a=cpu.registers.a; let m=cpu.mmu.rb(cpu.registers.pc); cpu.registers.a+=m; cpu.registers.pc+=1; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^a^m)&0x10) cpu.registers.f|=0x20; cpu.registers.m=2;
            }

            199 => {

                Z80._ops.rsv(); cpu.registers.sp-=2; cpu.mmu.ww(cpu.registers.sp,cpu.registers.pc); cpu.registers.pc=0x00; cpu.registers.m=3;
            }

            200 => {

                cpu.registers.m=1; if((cpu.registers.f&0x80)==0x80) { cpu.registers.pc=cpu.mmu.rw(cpu.registers.sp); cpu.registers.sp+=2; cpu.registers.m+=2; }
            }

            201 => {

                cpu.registers.pc=cpu.mmu.rw(cpu.registers.sp); cpu.registers.sp+=2; cpu.registers.m=3;
            }

            202 => {

                cpu.registers.m=3; if((cpu.registers.f&0x80)==0x80) { cpu.registers.pc=cpu.mmu.rw(cpu.registers.pc); cpu.registers.m+=1; } else cpu.registers.pc+=2;
            }

            203 => {


                let i=cpu.mmu.rb(cpu.registers.pc); cpu.registers.pc+=1;
                cpu.registers.pc &= 65535;
                if(Z80._cbmap[i]) Z80._cbmap[i]();
                else console.log(i);

            }

            204 => {

                cpu.registers.m=3; if((cpu.registers.f&0x80)==0x80) { cpu.registers.sp-=2; cpu.mmu.ww(cpu.registers.sp,cpu.registers.pc+2); cpu.registers.pc=cpu.mmu.rw(cpu.registers.pc); cpu.registers.m+=2; } else cpu.registers.pc+=2;
            }

            205 => {

                cpu.registers.sp-=2; cpu.mmu.ww(cpu.registers.sp,cpu.registers.pc+2); cpu.registers.pc=cpu.mmu.rw(cpu.registers.pc); cpu.registers.m=5;
            }

            206 => {

                let a=cpu.registers.a; let m=cpu.mmu.rb(cpu.registers.pc); cpu.registers.a+=m; cpu.registers.pc+=1; cpu.registers.a+=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a>255)?0x10:0; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^m^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=2;
            }

            207 => {

                Z80._ops.rsv(); cpu.registers.sp-=2; cpu.mmu.ww(cpu.registers.sp,cpu.registers.pc); cpu.registers.pc=0x08; cpu.registers.m=3;
            }

            208 => {

                cpu.registers.m=1; if((cpu.registers.f&0x10)==0x00) { cpu.registers.pc=cpu.mmu.rw(cpu.registers.sp); cpu.registers.sp+=2; cpu.registers.m+=2; }
            }

            209 => {

                cpu.registers.e=cpu.mmu.rb(cpu.registers.sp); cpu.registers.sp+=1; cpu.registers.d=cpu.mmu.rb(cpu.registers.sp); cpu.registers.sp+=1; cpu.registers.m=3;
            }

            210 => {

                cpu.registers.m=3; if((cpu.registers.f&0x10)==0x00) { cpu.registers.pc=cpu.mmu.rw(cpu.registers.pc); cpu.registers.m+=1; } else cpu.registers.pc+=2;
            }

            211 => {


                /*Undefined map entry*/
                let opc = cpu.registers.pc-1;
                LOG.out('Z80', 'Unimplemented instruction at $'+opc.toString(16)+', stopping.');
                Z80._stop=1;

            }

            212 => {

                cpu.registers.m=3; if((cpu.registers.f&0x10)==0x00) { cpu.registers.sp-=2; cpu.mmu.ww(cpu.registers.sp,cpu.registers.pc+2); cpu.registers.pc=cpu.mmu.rw(cpu.registers.pc); cpu.registers.m+=2; } else cpu.registers.pc+=2;
            }

            213 => {

                cpu.registers.sp-=1; cpu.mmu.wb(cpu.registers.sp,cpu.registers.d); cpu.registers.sp-=1; cpu.mmu.wb(cpu.registers.sp,cpu.registers.e); cpu.registers.m=3;
            }

            214 => {

                let a=cpu.registers.a; let m=cpu.mmu.rb(cpu.registers.pc); cpu.registers.a-=m; cpu.registers.pc+=1; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^m^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=2;
            }

            215 => {

                Z80._ops.rsv(); cpu.registers.sp-=2; cpu.mmu.ww(cpu.registers.sp,cpu.registers.pc); cpu.registers.pc=0x10; cpu.registers.m=3;
            }

            216 => {

                cpu.registers.m=1; if((cpu.registers.f&0x10)==0x10) { cpu.registers.pc=cpu.mmu.rw(cpu.registers.sp); cpu.registers.sp+=2; cpu.registers.m+=2; }
            }

            217 => {

                cpu.registers.ime=1; Z80._ops.rrs(); cpu.registers.pc=cpu.mmu.rw(cpu.registers.sp); cpu.registers.sp+=2; cpu.registers.m=3;
            }

            218 => {

                cpu.registers.m=3; if((cpu.registers.f&0x10)==0x10) { cpu.registers.pc=cpu.mmu.rw(cpu.registers.pc); cpu.registers.m+=1; } else cpu.registers.pc+=2;
            }

            219 => {


                /*Undefined map entry*/
                let opc = cpu.registers.pc-1;
                LOG.out('Z80', 'Unimplemented instruction at $'+opc.toString(16)+', stopping.');
                Z80._stop=1;

            }

            220 => {

                cpu.registers.m=3; if((cpu.registers.f&0x10)==0x10) { cpu.registers.sp-=2; cpu.mmu.ww(cpu.registers.sp,cpu.registers.pc+2); cpu.registers.pc=cpu.mmu.rw(cpu.registers.pc); cpu.registers.m+=2; } else cpu.registers.pc+=2;
            }

            221 => {


                /*Undefined map entry*/
                let opc = cpu.registers.pc-1;
                LOG.out('Z80', 'Unimplemented instruction at $'+opc.toString(16)+', stopping.');
                Z80._stop=1;

            }

            222 => {

                let a=cpu.registers.a; let m=cpu.mmu.rb(cpu.registers.pc); cpu.registers.a-=m; cpu.registers.pc+=1; cpu.registers.a-=(cpu.registers.f&0x10)?1:0; cpu.registers.f=(cpu.registers.a<0)?0x50:0x40; cpu.registers.a&=255; if(!cpu.registers.a) cpu.registers.f|=0x80; if((cpu.registers.a^m^a)&0x10) cpu.registers.f|=0x20; cpu.registers.m=2;
            }

            223 => {

                Z80._ops.rsv(); cpu.registers.sp-=2; cpu.mmu.ww(cpu.registers.sp,cpu.registers.pc); cpu.registers.pc=0x18; cpu.registers.m=3;
            }

            224 => {

                cpu.mmu.wb(0xFF00+cpu.mmu.rb(cpu.registers.pc),cpu.registers.a); cpu.registers.pc+=1; cpu.registers.m=3;
            }

            225 => {

                cpu.registers.l=cpu.mmu.rb(cpu.registers.sp); cpu.registers.sp+=1; cpu.registers.h=cpu.mmu.rb(cpu.registers.sp); cpu.registers.sp+=1; cpu.registers.m=3;
            }

            226 => {

                cpu.mmu.wb(0xFF00+cpu.registers.c,cpu.registers.a); cpu.registers.m=2;
            }

            227 => {


                /*Undefined map entry*/
                let opc = cpu.registers.pc-1;
                LOG.out('Z80', 'Unimplemented instruction at $'+opc.toString(16)+', stopping.');
                Z80._stop=1;

            }

            228 => {


                /*Undefined map entry*/
                let opc = cpu.registers.pc-1;
                LOG.out('Z80', 'Unimplemented instruction at $'+opc.toString(16)+', stopping.');
                Z80._stop=1;

            }

            229 => {

                cpu.registers.sp-=1; cpu.mmu.wb(cpu.registers.sp,cpu.registers.h); cpu.registers.sp-=1; cpu.mmu.wb(cpu.registers.sp,cpu.registers.l); cpu.registers.m=3;
            }

            230 => {

                cpu.registers.a&=cpu.mmu.rb(cpu.registers.pc); cpu.registers.pc+=1; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=2;
            }

            231 => {

                Z80._ops.rsv(); cpu.registers.sp-=2; cpu.mmu.ww(cpu.registers.sp,cpu.registers.pc); cpu.registers.pc=0x20; cpu.registers.m=3;
            }

            232 => {

                let i=cpu.mmu.rb(cpu.registers.pc); if(i>127) i=-((~i+1)&255); cpu.registers.pc+=1; cpu.registers.sp+=i; cpu.registers.m=4;
            }

            233 => {

                cpu.registers.pc=(cpu.registers.h<<8)+cpu.registers.l; cpu.registers.m=1;
            }

            234 => {

                cpu.mmu.wb(cpu.mmu.rw(cpu.registers.pc), cpu.registers.a); cpu.registers.pc+=2; cpu.registers.m=4;
            }

            235 => {


                /*Undefined map entry*/
                let opc = cpu.registers.pc-1;
                LOG.out('Z80', 'Unimplemented instruction at $'+opc.toString(16)+', stopping.');
                Z80._stop=1;

            }

            236 => {


                /*Undefined map entry*/
                let opc = cpu.registers.pc-1;
                LOG.out('Z80', 'Unimplemented instruction at $'+opc.toString(16)+', stopping.');
                Z80._stop=1;

            }

            237 => {


                /*Undefined map entry*/
                let opc = cpu.registers.pc-1;
                LOG.out('Z80', 'Unimplemented instruction at $'+opc.toString(16)+', stopping.');
                Z80._stop=1;

            }

            238 => {

                cpu.registers.a^=cpu.mmu.rb(cpu.registers.pc); cpu.registers.pc+=1; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=2;
            }

            239 => {

                Z80._ops.rsv(); cpu.registers.sp-=2; cpu.mmu.ww(cpu.registers.sp,cpu.registers.pc); cpu.registers.pc=0x28; cpu.registers.m=3;
            }

            240 => {

                cpu.registers.a=cpu.mmu.rb(0xFF00+cpu.mmu.rb(cpu.registers.pc)); cpu.registers.pc+=1; cpu.registers.m=3;
            }

            241 => {

                cpu.registers.f=cpu.mmu.rb(cpu.registers.sp); cpu.registers.sp+=1; cpu.registers.a=cpu.mmu.rb(cpu.registers.sp); cpu.registers.sp+=1; cpu.registers.m=3;
            }

            242 => {

                cpu.registers.a=cpu.mmu.rb(0xFF00+cpu.registers.c); cpu.registers.m=2;
            }

            243 => {

                cpu.registers.ime=0; cpu.registers.m=1;
            }

            244 => {


                /*Undefined map entry*/
                let opc = cpu.registers.pc-1;
                LOG.out('Z80', 'Unimplemented instruction at $'+opc.toString(16)+', stopping.');
                Z80._stop=1;

            }

            245 => {

                cpu.registers.sp-=1; cpu.mmu.wb(cpu.registers.sp,cpu.registers.a); cpu.registers.sp-=1; cpu.mmu.wb(cpu.registers.sp,cpu.registers.f); cpu.registers.m=3;
            }

            246 => {

                cpu.registers.a|=cpu.mmu.rb(cpu.registers.pc); cpu.registers.pc+=1; cpu.registers.a&=255; cpu.registers.f=cpu.registers.a?0:0x80; cpu.registers.m=2;
            }

            247 => {

                Z80._ops.rsv(); cpu.registers.sp-=2; cpu.mmu.ww(cpu.registers.sp,cpu.registers.pc); cpu.registers.pc=0x30; cpu.registers.m=3;
            }

            248 => {

                let i=cpu.mmu.rb(cpu.registers.pc); if(i>127) i=-((~i+1)&255); cpu.registers.pc+=1; i+=cpu.registers.sp; cpu.registers.h=(i>>8)&255; cpu.registers.l=i&255; cpu.registers.m=3;
            }

            249 => {


                /*Undefined map entry*/
                let opc = cpu.registers.pc-1;
                LOG.out('Z80', 'Unimplemented instruction at $'+opc.toString(16)+', stopping.');
                Z80._stop=1;

            }

            250 => {

                cpu.registers.a=cpu.mmu.rb(cpu.mmu.rw(cpu.registers.pc)); cpu.registers.pc+=2; cpu.registers.m=4;
            }

            251 => {

                cpu.registers.ime=1; cpu.registers.m=1;
            }

            252 => {


                /*Undefined map entry*/
                let opc = cpu.registers.pc-1;
                LOG.out('Z80', 'Unimplemented instruction at $'+opc.toString(16)+', stopping.');
                Z80._stop=1;

            }

            253 => {


                /*Undefined map entry*/
                let opc = cpu.registers.pc-1;
                LOG.out('Z80', 'Unimplemented instruction at $'+opc.toString(16)+', stopping.');
                Z80._stop=1;

            }

            254 => {

                let i=cpu.registers.a; let m=cpu.mmu.rb(cpu.registers.pc); i-=m; cpu.registers.pc+=1; cpu.registers.f=(i<0)?0x50:0x40; i&=255; if(!i) cpu.registers.f|=0x80; if((cpu.registers.a^i^m)&0x10) cpu.registers.f|=0x20; cpu.registers.m=2;
            }

            255 => {

                Z80._ops.rsv(); cpu.registers.sp-=2; cpu.mmu.ww(cpu.registers.sp,cpu.registers.pc); cpu.registers.pc=0x38; cpu.registers.m=3;
            }
            _ => {
                // pass
                cpu.registers.m = 1;
            }
        }
    }
}

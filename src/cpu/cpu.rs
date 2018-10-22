use Rom;

use std::collections::{HashMap};
use cpu::address_mode::*;
use cpu::memory::*;

struct StatusFlags {
    break_flag:         bool,
    carry:              bool,
    decimal_mode:       bool,
    emulation_mode:     bool,
    interrupt_disable:  bool,
    accumulator_width:  bool,
    negative:           bool,
    overflow:           bool,
    index_width:        bool,
    zero:               bool,
}

impl StatusFlags {
    pub fn new() -> StatusFlags {
        StatusFlags {
            break_flag: false,
            carry: false,
            decimal_mode: false,
            emulation_mode: false,
            interrupt_disable: false,
            accumulator_width: false,
            negative: false,
            overflow: false,
            index_width: false,
            zero: false,
        }
    }
}

pub struct CPU {
    a: u16,
    x: u16,
    y: u16,
    sp: u16,  // stack pointer
    dbr: u16, // data bank register    -- memory access
    pbr: u16, // program bank register -- op codes
    d: u16,   // direct register       -- Address offset for all instruction using "direct addressing" mode.
    pc: u32,  // program counter
    p: StatusFlags,

    cy: u16, // cycle_counter

    // emulation: bool,
    // wai: bool
    // trace: bool,
    should_exit: bool,
    mem: Mem,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            a:   0,
            x:   0,
            y:   0,
            sp:  0, // stack pointer
            dbr: 0, // data bank register    -- memory access
            pbr: 0, // program bank register -- op codes
            d:   0, // direct register       -- Address offset for all instruction using "direct addressing" mode.
            pc:  0, // program counter
            p: StatusFlags::new(),

            cy: 10,

            mem: SimpleMemory::new(),
            should_exit: false,
        }
    }

    pub fn a(&self) -> u16 {
        self.a
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn dbr(&self) -> u16 {
        self.dbr
    }

    pub fn pbr(&self) -> u16 {
        self.pbr
    }

    pub fn mem(&self) -> Box<dyn Mem> {
        self.mem
    }

    fn get_cycles(op: u8) -> u8 {
        // todo -> impl;
        7
    }

    fn next_b(&self) -> u8 {
        self.pc += 1;
        self.load_8(self.pbr, self.pc);
    }

    fn load_8(&self, bank: u8, addr_mode: AddressMode) -> u8 {
        let address = match addr_mode {
            AddressMode::Accumulator => {
                self.a 
            },
            AddressMode::Immediate => {
                self.pc + 1 
            },
            AddressMode::Implied => {
                panic!("trying to load with implied addressing mode"); 
            },
            AddressMode::Relative8 => {
                let branch_offset = self.next_b();
                
                if branch_offset <= 0x7F {
                    (self.pbr << 16 | self.pc + 2 + branch_offset)
                } else {
                    (self.pbr << 16 | self.pc - 254 + branch_offset)
                }
            },
            AddressMode::Relative16 => {
                let pc = self.pc();
                let lo = self.next_b();
                let hi = self.next_b();
                let offset = pc + 3 + (hi|lo);
                self.pbr << 16 | offset 
            },
            AddressMode::Absolute => {
                let lo = self.next_b();
                let hi = self.next_b();

                // TODO -> if instr is JMP or JSR, bank is pbr instead of dbr
                
                (self.dbr << 16) | (hi << 8) | lo 
            },
            AddressMode::ZeroPage => {
                self.next_b()
                    // todo -> find which bank  
            },
            AddressMode::Indirect => {
                let lo = self.next_b(); 
               
                let (lo_p, hi_p) = if self.e == 1 && self.dl() == 0 {
                    let lp = (self.dh() << 8) | lo;
                    let hp = (self.dh() << 8) | (lo + 1);
                    (lp, hp)
                } else {
                    let lp = self.d + lo;
                    (lp, lp + 1) 
                };

                let one = self.mem.load(lo_p);
                let two = self.mem.load(hi_p);

                (one << 8) | two
            },
            AddressMode::AbsoluteIndexedX => {
                let lo = self.next_b();
                let hi = self.next_b();
                
                let absolute = (self.dbr << 16) | (hi << 8) | lo; 
                absolute + self.x
            },
            AddressMode::AbsoluteIndexedY => {
                let lo = self.next_b();
                let hi = self.next_b();
                
                let absolute = (self.dbr << 16) | (hi << 8) | lo; 
                absolute + self.y
            },
            AddressMode::ZeroPageIndexedX => {
                self.next_b() + self.x
            },
            AddressMode::ZeroPageIndexedY => {
                self.next_b() + self.x
            },
            AddressMode::IndexedIndirect => {
                let loc = self.next_b(); 
    
                let addr = (loc + self.x) & 0xFF;
                let final_addr = self.mem.load(addr);

                self.mem.load(final_addr)
            },
            AddressMode::IndirectIndexed => {
                let loc = self.next_b(); 
    
                let addr = self.mem.load(loc);
                let final_addr = addr + self.y;
                
                self.mem.load(final_addr)
            }
        };
        
        self.mem.load(self, address) 
    }

    fn load_16(&self, addr_mode: AddressMode) -> u16 {
       let lo = self.load_8(addr_mode) as u16;
       self.pc += 1;
       let hi = self.load_8(addr_mode) as u16;

       ( hi << 8 | lo )
    }

    fn store_8(&self, bank: u8, addr_mode: AddressMode, to_store: u8) {
        self.mem.store(self, bank, addr_mode, to_store);
    }

    fn store_16(&self, bank: u8, addr_mode: AddressMode, to_store: u16) {
        self.mem.store(self, bank, addr_mode, to_store); 
    }

    pub fn run(&mut self, ops: Rom) {
        let interrupt_period = 7; // TODO -> this was arbitrary. find out what number i need

        loop {
            // todo -> let op_code = self.mem.load_op(self.pc);
            let opcode = ops.get(self.pc.clone() as usize).unwrap();
            self.pc += 1;

            self.cy -= Self::get_cycles(*opcode) as u16;

            match opcode {
                0x18 => self.clc(AddressMode::Implied),
                0xD8 => self.cld(AddressMode::Implied),
                0x58 => self.cli(AddressMode::Implied),
                0xB8 => self.clv(AddressMode::Implied),
                0x38 => self.sec(AddressMode::Implied),
                0xF8 => self.sed(AddressMode::Implied),
                0x78 => self.sei(AddressMode::Implied),
                0xA1 => self.lda(AddressMode::DirectIndexedIndirect),
                0xA3 => self.lda(AddressMode::StackRelative),
                0xA5 => self.lda(AddressMode::Direct),
                0xA7 => self.lda(AddressMode::DirectIndirectLong),
                0xA9 => self.lda(AddressMode::Immediate), 
                0xAD => self.lda(AddressMode::Absolute),
                0xAF => self.lda(AddressMode::AbsoluteLong),
                0xB1 => self.lda(AddressMode::DirectIndirectIndexed),
                0xB2 => self.lda(AddressMode::DirectIndirect),
                0xB3 => self.lda(AddressMode::StackRelativeIndirectIndexed),
                0xB5 => self.lda(AddressMode::DirectIndexedX),
                0xB7 => self.lda(AddressMode::DirectIndirectIndexedLong),
                0xB9 => self.lda(AddressMode::AbsoluteIndexedY),
                0xBD => self.lda(AddressMode::AbsoluteIndexedX),
                0xBF => self.lda(AddressMode::AbsoluteLongIndexedX),
                0xA2 => self.ldx(AddressMode::Immediate),
                0xA6 => self.ldx(AddressMode::Direct),
                0xAE => self.ldx(AddressMode::Absolute),
                0xB6 => self.ldx(AddressMode::DirectIndexedY),
                0xBE => self.ldx(AddressMode::AbsoluteIndexedY),
                0xA0 => self.ldy(AddressMode::Immediate),        
                0xA4 => self.ldy(AddressMode::Direct),
                0xAC => self.ldy(AddressMode::Absolute),   
                0xB4 => self.ldy(AddressMode::DirectIndexedX),
                0xBC => self.ldy(AddressMode::AbsoluteIndexedX),
                0x81 => self.sta(AddressMode::DirectIndexedIndirect),
                0x83 => self.sta(AddressMode::StackRelative),
                0x85 => self.sta(AddressMode::Direct),
                0x87 => self.sta(AddressMode::DirectIndirectLong),
                0x8D => self.sta(AddressMode::Absolute),
                0x8F => self.sta(AddressMode::AbsoluteLong),
                0x91 => self.sta(AddressMode::DirectIndirectIndexed),
                0x92 => self.sta(AddressMode::DirectIndirect),
                0x93 => self.sta(AddressMode::StackRelativeIndirectIndexed),
                0x95 => self.sta(AddressMode::DirectIndexedX),
                0x97 => self.sta(AddressMode::DirectIndirectIndexedLong),
                0x99 => self.sta(AddressMode::AbsoluteIndexedY),
                0x9D => self.sta(AddressMode::AbsoluteIndexedX),
                0x9F => self.sta(AddressMode::AbsoluteLongIndexedX),
                0x86 => self.stx(AddressMode::Direct),
                0x8E => self.stx(AddressMode::Absolute),
                0x96 => self.stx(AddressMode::DirectIndexedY),
                0x84 => self.sty(AddressMode::Direct),
                0x8C => self.sty(AddressMode::Absolute),
                0x94 => self.sty(AddressMode::DirectIndexedX),    
                0x64 => self.stz(AddressMode::Direct),
                0x74 => self.stz(AddressMode::DirectIndexedX),
                0x9C => self.stz(AddressMode::Absolute),
                0x9E => self.stz(AddressMode::AbsoluteIndexedX),
                _ => panic!("Opcode {:X} is not implemented", opcode)
            }

            if self.cy <= 0 {
                // Check for interrupts
                // and cyclic tasks here
                
                self.cy += interrupt_period;

                if self.should_exit {
                    return
                }
            }
        }
    }

//    The n flag is 0 when the high bit of the result (bit 15 when the m flag is 0, bit 7 when the m flag is 1) is 0, and the n flag is 1 when the high bit of the result is 1.
//    The v flag is 0 when there is not a signed arithmetic overflow, and the v flag is 1 when there is a signed arithmetic overflow.
//              For 8-bit signed numbers, $00 to $7F represents 0 to 127, and $80 to $FF represents -128 to -1;
//              an 8-bit arithmetic overflow occurs when the result is outside the range -128 to 127.
//              For 16-bit signed numbers, $0000 to $7FFF represents 0 to 32767, and $8000 to $FFFF represents -32768 to -1;
//              a 16-bit arithmetic overflow occurs when the result is outside the range -32768 to 32767.
//    The z flag is 0 when the 16-bit (when m flag is 0) or 8-bit (when the m flag is 1) result is nonzero, and the z flag is 1 when the result is zero.
//    The c flag is 0 when there is not an unsigned carry, and the c flag is 1 when there is an unsigned carry.
//              For 8-bit unsigned numbers, $00 to $FF represents 0 to 255; for addition, an 8-bit carry occurs when the result is greater than 255.
//              For 16-bit unsigned numbers, $0000 to $FFFF represents 0 to 65535;
//              for addition, an 16-bit carry occurs when the result is greater than 65535.
//              For subtraction (8-bit or 16-bit), there is a carry when the accumulator is greater than or equal to the data.

    ////////////////////////////////////
    //
    //              ADC
    //
    ////////////////////////////////////

    // TODO -> refactor this to take an address mode instead of to_add
    fn adc_8(&mut self, to_add: u8) {
        // accumulator += data + carry;

        if self.p.decimal_mode {
            // bcd mode
            let mut a1: u8  = self.a as u8 & 0x0F;
            let mut a2: u16 = self.a & 0xF0;

            let to_add1: u8  = to_add & 0x0F;
            let to_add2: u8  = to_add & 0xF0;

            a1 += to_add1 + self.p.carry as u8;

            if a1 > 0x09 {
                a1 -= 0x0A; // subtract 10
                a1 &= 0x0F; // zero out unnecessary bits
                a2 += 0x10; // add 16 to second digit
            }

            a2 += to_add2 as u16;

            if a2 > 0x90 {
                a2 -= 0x0A; // subtract 10
                a2 &= 0xF0; // zero out unnecessary bits
                self.p.carry = true;
            } else {
                self.p.carry = false;
            }

            let result_8 = a1 | a2 as u8;

            // TODO -> DETERMINE OVERFLOW

            // todo -> only set lower 8 bits
            self.a = self.a | result_8 as u16;

            // TODO -> SET ZN FLAGS

        } else {
            // binary mode
            let result_16: u16 = self.a + to_add as u16 + self.p.carry as u16;

            if result_16 >= 0x100 {
                self.p.carry = true;
            } else {
                self.p.carry = false;
            }

            // TODO -> DETERMINE OVERFLOW

            self.a = result_16 as u16;

            // TODO -> SET ZN FLAGS
        }
    }

    // TODO -> refactor this to take an address mode instead of to_add
    fn adc_16(&mut self, to_add: u16) {
        // accumulator += data + carry;

        if self.p.decimal_mode {
            // bcd mode
            let mut a1: u16 = self.a & 0x000F;
            let mut a2: u16 = self.a & 0x00F0;
            let mut a3: u16 = self.a & 0x0F00;
            let mut a4: u32 = self.a as u32 & 0xF000;

            let to_add1: u16 = to_add & 0x000F;
            let to_add2: u16 = to_add & 0x00F0;
            let to_add3: u16 = to_add & 0x0F00;
            let to_add4: u16 = to_add & 0xF000;

            a1 += to_add1 + self.p.carry as u16;

            if a1 > 0x09 {
                a1 -= 0x000A; // subtract 10
                a1 &= 0x000F; // zero out unnecessary bits
                a2 += 0x0010; // add 16 to second digit
            }

            a2 += to_add2;

            if a2 > 0x90 {
                a2 -= 0x00A0;
                a2 &= 0x00F0; // zero out unnecessary bits
                a3 += 0x0100; // add 16 to third digits
            }

            a3 += to_add3;

            if a3 > 0x0900 {
                a3 -= 0x0A00; // subtract 10
                a3 &= 0x0F00; // zero out uneccessary bits
                a4 += 0x1000; // add 16 to last digit;
            }

            a4 += to_add4 as u32;

            if a4 > 0x9000 {
                a4 -= 0xA000;
                a4 &= 0xF000;
                self.p.carry = true;
            } else {
                self.p.carry = false;
            }

            let result_32 = a1 | a2 | a3 | (a4 as u16);

            // TODO -> DETERMINE OVERFLOW

            self.a = result_32 as u16;

            // TODO -> SET ZN FLAGS

        } else {
            // binary mode

            let result_32: u32 = self.a as u32 + to_add as u32 + self.p.carry as u32;

            if result_32 >= 0x10000 {
                self.p.carry = true;
            } else {
                self.p.carry = false;
            }

            // TODO -> DETERMINE OVERFLOW

            self.a = result_32 as u16;

            // TODO -> SET ZN FLAGS
        }
    }

    ////////////////////////////////////
    //
    //              SBC
    //
    ////////////////////////////////////

    // TODO -> refactor this to take an address mode instead of to_subtract
    fn sbc_8(&self) {
        // accumulator -= data - 1 + carry;

        if self.p.decimal_mode {
            // bcd mode
        } else {
            // binary mode
        }
    }

    // TODO -> refactor this to take an address mode instead of to_subtract
    fn sbc_16(&self) {
        // accumulator -= data - 1 + carry;

        if self.p.decimal_mode {
            // bcd mode
        } else {
            // binary mode
        }
    }

    ////////////////////////////////////
    //
    //              CMP
    //
    ////////////////////////////////////

    fn compare16(&mut self, a: u16, b: u16) {
        self.p.carry = a >= b;
        self.p.negative = a < b;
        self.p.zero = a == b;
    }

    fn compare8(&mut self, a: u8, b: u8) {
        self.p.carry = a >= b;
        self.p.negative = a < b;
        self.p.zero = a == b;
    }

    fn cmp_8(&mut self, am: AddressMode) {
        let to_compare = self.load_8(self, am);
        let a = self.a as u8;
        self.compare8(a, to_compare);
    }

    fn cmp_16(&mut self, am: AddressMode) {
        let to_compare = self.load_16(self, am);
        let a = self.a;
        self.compare16(a, to_compare);
    }

    ////////////////////////////////////
    //
    //              CPX
    //
    ////////////////////////////////////

    fn cpx_8(&mut self, am: AddressMode) {
        let to_compare = self.load_8(self, am);
        let x = self.x as u8;
        self.compare8(x, to_compare);
    }

    fn cpx_16(&mut self, am: AddressMode) {
        let to_compare = self.load_16(self, am);
        let x = self.x;
        self.compare16(x, to_compare);
    }

    ////////////////////////////////////
    //
    //              CPY
    //
    ////////////////////////////////////

    fn cpy_8(&mut self, am: AddressMode) {
        let to_compare = self.load_8(self, am);
        let y = self.y as u8;
        self.compare8(y, to_compare);
    }

    fn cpy_16(&mut self, am: AddressMode) {
        let to_compare = self.load_16(self, am);
        let y = self.y;
        self.compare16(y, to_compare);
    }

    ////////////////////////////////////
    //
    //           CLEAR / SET
    //
    ////////////////////////////////////

    fn clc(&mut self) {
        self.p.carry = false;
    }

    fn cld(&mut self) {
        self.p.decimal_mode = false;
    }

    fn cli(&mut self) {
        self.p.interrupt_disable = false;
    }

    fn clv(&mut self) {
        self.p.overflow = false;
    }

    fn sec(&mut self) {
        self.p.carry = true;
    }

    fn sed(&mut self) {
        self.p.decimal_mode = true;
    }

    fn sei(&mut self) {
        self.p.interrupt_disable = true;
    }

    ////////////////////////////////////
    //
    //       LOADING / STORING 
    //
    ////////////////////////////////////
  
    fn lda_8(&self, am: AddressMode) {
        // TODO -> bank 
        let val = self.load_8(am);
        self.a = val;
    }

    fn lda_16(&self, am: AddressMode) {
        // TODO -> bank 
        let val = self.load_16(am);
        self.a = val;
    }
    
    fn ldx_8(&self, am: AddressMode) {
        // TODO -> bank 
        let val = self.load_8(am);
        self.x = val;
    }

    fn ldx_16(&self, am: AddressMode) {
        // TODO -> bank 
        let val = self.load_16(am);
        self.x = val;
    }

    fn ldy_8(&self, am: AddressMode) {
        // TODO -> bank 
        let val = self.load_8(am);
        self.y = val;
    }

    fn ldy_16(&self, am: AddressMode) {
        // TODO -> bank 
        let val = self.load_16(am);
        self.y = val;
    }

    fn sta_8(&self, am: AddressMode) {
        // TODO -> bank 
        self.store_8(am, self.a as u8);
    }

    fn sta_16(&self, am: AddressMode) {
        // TODO -> bank 
        self.store_16(am, self.a);
    }

    fn stx_8(&self, am: AddressMode) {
        // TODO -> bank 
        self.store_8(am, self.x as u8);
    }

    fn stx_16(&self, am: AddressMode) {
        // TODO -> bank 
        self.store_16(am, self.x);
    }

    fn sty_8(&self, am: AddressMode) {
        // TODO -> bank 
        self.store_8(am, self.y as u8);
    }

    fn sty_16(&self, am: AddressMode) {
        // TODO -> bank 
        self.store_16(am, self.y);
    }

    fn stz_8(&self, am: AddressMode) {
        // TODO -> bank 
        self.store_8(am, 0);
    }

    fn stz_16(&self, am: AddressMode) {
        // TODO -> bank 
        self.store_16(am, 0);
    }
}

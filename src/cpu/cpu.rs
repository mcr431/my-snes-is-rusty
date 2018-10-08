use Rom;

use std::collections::{HashMap};
use cpu::address_mode::{AddressMode, ADDRESSING_MODES};
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
    mem: Mem,
    should_exit: bool,
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

    pub fn mem(&self) -> impl Mem {
        self.mem
    }

    fn get_cycles(op: u8) -> u8 {
        // todo -> impl;
        7
    }

    pub fn run(&mut self, ops: Rom) {
        let interrupt_period = 7; // TODO -> this was arbitrary. find out what number i need

        loop {
            // todo -> let op_code = self.mem.load_op(self.pc);
            let opcode = ops.get(self.pc.clone() as usize).unwrap();
            self.pc += 1;

            self.cy -= Self::get_cycles(*opcode) as u16;

            match opcode {
                0x18 => self.clc(),
                0xD8 => self.cld(),
                0x58 => self.cli(),
                0xB8 => self.clv(),
                0x38 => self.sec(),
                0xF8 => self.sed(),
                0x78 => self.sei(),
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

    // TODO -> refactor this to take an address mode instead of to_compare
    fn cmp_8(&mut self, to_compare: u8) {
        let a = self.a as u8;
        self.compare8(a, to_compare);
    }

    // TODO -> refactor this to take an address mode instead of to_compare
    fn cmp_16(&mut self, to_compare: u16) {
        let a = self.a;
        self.compare16(a, to_compare);
    }

    ////////////////////////////////////
    //
    //              CPX
    //
    ////////////////////////////////////

    // TODO -> refactor this to take an address mode instead of to_compare
    fn cpx_8(&mut self, to_compare: u8) {
        let x = self.x as u8;
        self.compare8(x, to_compare);
    }

    // TODO -> refactor this to take an address mode instead of to_compare
    fn cpx_16(&mut self, to_compare: u16) {
        let x = self.x;
        self.compare16(x, to_compare);
    }

    ////////////////////////////////////
    //
    //              CPY
    //
    ////////////////////////////////////

    // TODO -> refactor this to take an address mode instead of to_compare
    fn cpy_8(&mut self, to_compare: u8) {
        let y = self.y as u8;
        self.compare8(y as u8, to_compare);
    }

    // TODO -> refactor this to take an address mode instead of to_compare
    fn cpy_16(&mut self, to_compare: u16) {
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
}

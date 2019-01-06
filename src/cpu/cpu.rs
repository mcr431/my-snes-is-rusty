use Rom;

use cpu::memory::*;
use cpu::address_mode::AddressMode;
use std::sync::{RwLock, Arc};

type StatusFlags = u8;

impl StatusFlags {
    pub fn new() -> StatusFlags {
        0
    }

    pub fn carry(&self) -> bool {
        self & 0x0000_0001 == 0x0000_0001
    }

    pub fn zero(&self) -> bool {
        self & 0x0000_0010 == 0x0000_0010
    }

    pub fn interrupt_disable(&self) -> bool {
        self & 0x0000_0100 == 0x0000_0100
    }

    pub fn decimal(&self) -> bool {
        self & 0x0000_1000 == 0x0000_1000
    }

    // bit 4 if native
    pub fn index_width(&self) -> bool {
        self & 0x0001_0000 == 0x0001_0000
    }

    // bit 4 if emulation
    pub fn break_flag(&self) -> bool {
        self & 0x0001_0000 == 0x0001_0000
    }

    pub fn accumulator_width(&self) -> bool {
        self & 0x0010_0000 == 0x0010_0000
    }

    pub fn overflow(&self) -> bool {
        self & 0x0100_0000 == 0x0100_0000
    }

    pub fn negative(&self) -> bool {
        self & 0x1000_0000 == 0x1000_0000
    }

    pub fn set_carry(&mut self, enabled: bool) {
        if enabled {
            self = self | 0x0000_0001
        } else {
            self = self & !(0x000_0001)
        }
    }

    pub fn set_zero(&mut self, enabled: bool) {
        if enabled {
            self = self | 0x0000_0010
        } else {
            self = self & !(0x000_0010)
        }
    }

    pub fn set_zero_from_data(&mut self, data: u16) {
        if data == 0 {
            self = self | 0x0000_0010
        } else {
            self = self & !(0x000_0010)
        }
    }

    pub fn set_interrupt_disable(&mut self, enabled: bool) {
        if enabled {
            self = self | 0x0000_0100
        } else {
            self = self & !(0x000_0100)
        }
    }

    pub fn set_decimal(&mut self, enabled: bool) {
        if enabled {
            self = self | 0x0000_1000
        } else {
            self = self & !(0x000_1000)
        }
    }

    // bit 4 if native
    pub fn set_index_width(&mut self, enabled: bool) {
        if enabled {
            self = self | 0x0001_0000
        } else {
            self = self & !(0x0001_0000)
        }
    }

    // bit 4 if emulation
    pub fn set_break_flag(&mut self, enabled: bool) {
        if enabled {
            self = self | 0x0001_0000
        } else {
            self = self & !(0x0001_0000)
        }
    }

    pub fn set_accumulator_width(&mut self, enabled: bool) {
        if enabled {
            self = self | 0x0010_0000
        } else {
            self = self & !(0x0010_0000)
        }
    }

    pub fn set_overflow(&mut self, enabled: bool) {
        if enabled {
            self = self | 0x0100_0000
        } else {
            self = self & !(0x0100_0000)
        }
    }

    pub fn set_negative(&mut self, enabled: bool) {
        if enabled {
            self = self | 0x1000_0000
        } else {
            self = self & !(0x1000_0000)
        }
    }

    pub fn set_negative_from_data_8(&mut self, data: u8) {
        if data >> 7 == 1 {
            self = self | 0x1000_0000
        } else {
            self = self & !(0x1000_0000)
        }
    }

    pub fn set_negative_from_data_16(&mut self, data: u16) {
        if data >> 15 == 1 {
            self = self | 0x1000_0000
        } else {
            self = self & !(0x1000_0000)
        }
    }
}

pub struct CPU {
    a: u16,
    x: u16,
    y: u16,
    sp: u16,  // stack pointer
    dbr: u8, // data bank register    -- memory access
    pbr: u8, // program bank register -- op codes
    d: u16,   // direct register      -- Address offset for all instruction using "direct addressing" mode.
    pc: u16,  // program counter
    p: StatusFlags,

    cy: u16, // cycle_counter

    // emulation: bool,
    // wai: bool
    // trace: bool,
    should_exit: bool,
    mem: Arc<RwLock<Mem>>,
}

impl CPU {
    pub fn new(mem: Arc<RwLock<Mem>>) -> CPU {
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

            mem,
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

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn dbr(&self) -> u8 {
        self.dbr
    }

    pub fn d(&self) -> u16 {
        self.d
    }

    pub fn dl(&self) -> u8 {
        self.d as u8
    }

    pub fn dh(&self) -> u8 {
        (self.d >> 8) as u8
    }

    pub fn pbr(&self) -> u8 {
        self.pbr
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    fn get_cycles(op: u8) -> u8 {
        // todo -> impl;
        7
    }

    pub fn next_b(&mut self) -> u8 {
        self.load_8(&AddressMode::Immediate)
    }

    pub fn load_8(&mut self, addr_mode: &AddressMode) -> u8 {
        let (bank, addr) = addr_mode.get_address_8(self, &self.mem);
        self.mem.load(bank, addr)
    }

    pub fn load_16(&mut self, addr_mode: &AddressMode) -> u16 {
        let ((lo_bank, lo_addr), (hi_bank, hi_addr)) = addr_mode.get_address_16(self, &self.mem);

        let lo = self.mem.load(lo_bank, lo_addr) as u16;
        let hi = self.mem.load(hi_bank, hi_addr) as u16;

        hi << 8 | lo
    }

    pub fn store_8(&mut self, addr_mode: &AddressMode, to_store: u8) {
        let (bank, address) = addr_mode.get_address_8(self, &self.mem);
        self.mem.store(bank, address, to_store);
    }

    pub fn store_16(&mut self, addr_mode: &AddressMode, to_store: u16) {
        let ((lo_bank, lo_addr), (hi_bank, hi_addr)) = addr_mode.get_address_16(self, &self.mem);

        let lo_to_store = (to_store & 0x00FF) as u8;
        let hi_to_store = (to_store >> 8) as u8;

        self.mem.store(lo_bank, lo_address, lo_to_store);
        self.mem.store(hi_bank, hi_address, hi_to_store);
    }

    pub fn run(&mut self, ops: Rom) {
        let interrupt_period = 7; // TODO -> this was arbitrary. find out what number i need

        loop {
            // todo -> let op_code = self.mem.load_op(self.pc);

            // todo -> load the rom into the pbr;

            let opcode = rom.get(self.pc.clone() as usize).unwrap();
            self.pc += 1;

            self.cy -= Self::get_cycles(*opcode) as u16;

            match opcode {
                // add w carry
                0x61 => self.adc(AddressMode::DirectIndexedIndirect),
                0x63 => self.adc(AddressMode::StackRelative),
                0x65 => self.adc(AddressMode::Direct(*opcode)),
                0x67 => self.adc(AddressMode::DirectIndirectLong),
                0x69 => self.adc(AddressMode::Immediate),
                0x6D => self.adc(AddressMode::Absolute(*opcode)),
                0x6F => self.adc(AddressMode::AbsoluteLong),
                0x71 => self.adc(AddressMode::DirectIndirectIndexed),
                0x72 => self.adc(AddressMode::DirectIndirect),
                0x73 => self.adc(AddressMode::StackRelativeIndirectIndexed),
                0x75 => self.adc(AddressMode::DirectIndexedX),
                0x77 => self.adc(AddressMode::DirectIndirectIndexedLong),
                0x79 => self.adc(AddressMode::AbsoluteIndexedY),
                0x7D => self.adc(AddressMode::AbsoluteIndexedX),
                0x7F => self.adc(AddressMode::AbsoluteLongIndexedX),

                // sub w carry
                0xE1 => self.sbc(AddressMode::DirectIndexedIndirect),
                0xE3 => self.sbc(AddressMode::StackRelative),
                0xE5 => self.sbc(AddressMode::Direct(*opcode)),
                0xE7 => self.sbc(AddressMode::DirectIndirectLong),
                0xE9 => self.sbc(AddressMode::Immediate),
                0xED => self.sbc(AddressMode::Absolute(*opcode)),
                0xEF => self.sbc(AddressMode::AbsoluteLong),
                0xF1 => self.sbc(AddressMode::DirectIndirectIndexed),
                0xF2 => self.sbc(AddressMode::DirectIndirect),
                0xF3 => self.sbc(AddressMode::StackRelativeIndirectIndexed),
                0xF5 => self.sbc(AddressMode::DirectIndexedX),
                0xF7 => self.sbc(AddressMode::DirectIndirectIndexedLong),
                0xF9 => self.sbc(AddressMode::AbsoluteIndexedY),
                0xFD => self.sbc(AddressMode::AbsoluteIndexedX),
                0xFF => self.sbc(AddressMode::AbsoluteLongIndexedX),

                // compare
                0xC1 => self.cmp(AddressMode::DirectIndexedIndirect),
                0xC3 => self.cmp(AddressMode::StackRelative),
                0xC5 => self.cmp(AddressMode::Direct(*opcode)),
                0xC7 => self.cmp(AddressMode::DirectIndirectLong),
                0xC9 => self.cmp(AddressMode::Immediate),
                0xCD => self.cmp(AddressMode::Absolute(*opcode)),
                0xCF => self.cmp(AddressMode::AbsoluteLong),
                0xD1 => self.cmp(AddressMode::DirectIndirectIndexed),
                0xD2 => self.cmp(AddressMode::DirectIndirect),
                0xD3 => self.cmp(AddressMode::StackRelativeIndirectIndexed),
                0xD5 => self.cmp(AddressMode::DirectIndexedX),
                0xD7 => self.cmp(AddressMode::DirectIndirectIndexedLong),
                0xD9 => self.cmp(AddressMode::AbsoluteIndexedY),
                0xDD => self.cmp(AddressMode::AbsoluteIndexedX),
                0xDF => self.cmp(AddressMode::AbsoluteLongIndexedX),
                0xE0 => self.cpx(AddressMode::Immediate),
                0xE4 => self.cpx(AddressMode::Direct(*opcode)),
                0xEC => self.cpx(AddressMode::Absolute(*opcode)),
                0xC0 => self.cpy(AddressMode::Immediate),
                0xC4 => self.cpy(AddressMode::Direct(*opcode)),
                0xCC => self.cpy(AddressMode::Absolute(*opcode)),

                // decrement
                0x3A => self.dec(AddressMode::Accumulator),
                0xC6 => self.dec(AddressMode::Direct(*opcode)),
                0xCE => self.dec(AddressMode::Absolute(*opcode)),
                0xD6 => self.dec(AddressMode::DirectIndexedX),
                0xDE => self.dec(AddressMode::AbsoluteIndexedX),
                0xCA => self.dex(AddressMode::Implied),
                0x88 => self.dey(AddressMode::Implied),

                // increment
                0x3A => self.inc(AddressMode::Accumulator),
                0xC6 => self.inc(AddressMode::Direct(*opcode)),
                0xCE => self.inc(AddressMode::Absolute(*opcode)),
                0xD6 => self.inc(AddressMode::DirectIndexedX),
                0xDE => self.inc(AddressMode::AbsoluteIndexedX),
                0xCA => self.inx(AddressMode::Implied),
                0x88 => self.iny(AddressMode::Implied),

                // and
                0x21 => self.and(AddressMode::DirectIndexedIndirect),
                0x23 => self.and(AddressMode::StackRelative),
                0x25 => self.and(AddressMode::Direct(*opcode)),
                0x27 => self.and(AddressMode::DirectIndirectLong),
                0x29 => self.and(AddressMode::Immediate),
                0x2D => self.and(AddressMode::Absolute(*opcode)),
                0x2F => self.and(AddressMode::AbsoluteLong),
                0x31 => self.and(AddressMode::DirectIndirectIndexed),
                0x32 => self.and(AddressMode::DirectIndirect),
                0x33 => self.and(AddressMode::StackRelativeIndirectIndexed),
                0x35 => self.and(AddressMode::DirectIndexedX),
                0x37 => self.and(AddressMode::DirectIndirectIndexedLong),
                0x39 => self.and(AddressMode::AbsoluteIndexedY),
                0x3D => self.and(AddressMode::AbsoluteIndexedX),
                0x3F => self.and(AddressMode::AbsoluteLongIndexedX),

                // eor
                0x41 => self.eor(AddressMode::DirectIndexedIndirect),
                0x43 => self.eor(AddressMode::StackRelative),
                0x45 => self.eor(AddressMode::Direct(*opcode)),
                0x47 => self.eor(AddressMode::DirectIndirectLong),
                0x49 => self.eor(AddressMode::Immediate),
                0x4D => self.eor(AddressMode::Absolute(*opcode)),
                0x4F => self.eor(AddressMode::AbsoluteLong),
                0x51 => self.eor(AddressMode::DirectIndirectIndexed),
                0x52 => self.eor(AddressMode::DirectIndirect),
                0x53 => self.eor(AddressMode::StackRelativeIndirectIndexed),
                0x55 => self.eor(AddressMode::DirectIndexedX),
                0x57 => self.eor(AddressMode::DirectIndirectIndexedLong),
                0x59 => self.eor(AddressMode::AbsoluteIndexedY),
                0x5D => self.eor(AddressMode::AbsoluteIndexedX),
                0x5F => self.eor(AddressMode::AbsoluteLongIndexedX),

                // ora
                0x01 => self.ora(AddressMode::DirectIndexedIndirect),
                0x03 => self.ora(AddressMode::StackRelative),
                0x05 => self.ora(AddressMode::Direct(*opcode)),
                0x07 => self.ora(AddressMode::DirectIndirectLong),
                0x09 => self.ora(AddressMode::Immediate),
                0x0D => self.ora(AddressMode::Absolute(*opcode)),
                0x0F => self.ora(AddressMode::AbsoluteLong),
                0x11 => self.ora(AddressMode::DirectIndirectIndexed),
                0x12 => self.ora(AddressMode::DirectIndirect),
                0x13 => self.ora(AddressMode::StackRelativeIndirectIndexed),
                0x15 => self.ora(AddressMode::DirectIndexedX),
                0x17 => self.ora(AddressMode::DirectIndirectIndexedLong),
                0x19 => self.ora(AddressMode::AbsoluteIndexedY),
                0x1D => self.ora(AddressMode::AbsoluteIndexedX),
                0x1F => self.ora(AddressMode::AbsoluteLongIndexedX),

                // bit
                0x24 => self.bit(AddressMode::Direct(*opcode)),
                0x2C => self.bit(AddressMode::Absolute(*opcode)),
                0x34 => self.bit(AddressMode::DirectIndexedX),
                0x3C => self.bit(AddressMode::AbsoluteIndexedX),
                0x89 => self.bit(AddressMode::Immediate),

                // trb | tsb
                0x14 => self.trb(AddressMode::Direct(*opcode)),
                0x1C => self.trb(AddressMode::Absolute(*opcode)),
                0x04 => self.tsb(AddressMode::Direct(*opcode)),
                0x0C => self.tsb(AddressMode::Absolute(*opcode)),

                // asl
                0x06 => self.asl(AddressMode::Direct(*opcode)),
                0x0A => self.asl(AddressMode::Accumulator),
                0x0E => self.asl(AddressMode::Absolute(*opcode)),
                0x16 => self.asl(AddressMode::DirectIndexedX),
                0x1E => self.asl(AddressMode::AbsoluteIndexedX),

                // lsr
                0x46 => self.lsr(AddressMode::Direct(*opcode)),
                0x4A => self.lsr(AddressMode::Accumulator),
                0x4E => self.lsr(AddressMode::Absolute(*opcode)),
                0x56 => self.lsr(AddressMode::DirectIndexedX),
                0x5E => self.lsr(AddressMode::AbsoluteIndexedX),

                // rol
                0x26 => self.rol(AddressMode::Direct(*opcode)),
                0x2A => self.rol(AddressMode::Accumulator),
                0x2E => self.rol(AddressMode::Absolute(*opcode)),
                0x36 => self.rol(AddressMode::DirectIndexedX),
                0x3E => self.rol(AddressMode::AbsoluteIndexedX),

                // ror
                0x66 => self.ror(AddressMode::Direct(*opcode)),
                0x6A => self.ror(AddressMode::Accumulator),
                0x6E => self.ror(AddressMode::Absolute(*opcode)),
                0x76 => self.ror(AddressMode::DirectIndexedX),
                0x7E => self.ror(AddressMode::AbsoluteIndexedX),

                // branch
                0x90 => self.bcc(AddressMode::Relative8), 
                0xB0 => self.bcs(AddressMode::Relative8), 
                0xF0 => self.beq(AddressMode::Relative8), 
                0x30 => self.bmi(AddressMode::Relative8), 
                0xD0 => self.bne(AddressMode::Relative8), 
                0x10 => self.bpl(AddressMode::Relative8), 
                0x80 => self.bra(AddressMode::Relative8), 
                0x50 => self.bvc(AddressMode::Relative8), 
                0x70 => self.bvs(AddressMode::Relative8), 
                0x82 => self.brl(AddressMode::Relative16),

                // jump
                0x4C => self.jmp(AddressMode::Absolute(*opcode)),
                0x5C => self.jmp(AddressMode::AbsoluteLong),
                0x6C => self.jmp(AddressMode::AbsoluteIndirect),
                0x7C => self.jmp(AddressMode::AbsoluteIndexedIndirect),
                0xDC => self.jmp(AddressMode::AbsoluteIndirectLong),
                0x22 => self.jsl(AddressMode::AbsoluteLong),
                0x20 => self.jsr(AddressMode::Absolute(*opcode)),
                0xFC => self.jsr(AddressMode::AbsoluteIndexedIndirect),

                // return
                0x6B => self.rtl(AddressMode::Implied),
                0x60 => self.rts(AddressMode::Implied),
                0x40 => self.rti(AddressMode::Implied),

                // software interrupts
                0x00 => self.brk(AddressMode::Implied),
                0x02 => self.cop(AddressMode::Immediate),

                // clear | set
                0x18 => self.clc(AddressMode::Implied),
                0xD8 => self.cld(AddressMode::Implied),
                0x58 => self.cli(AddressMode::Implied),
                0xB8 => self.clv(AddressMode::Implied),
                0x38 => self.sec(AddressMode::Implied),
                0xF8 => self.sed(AddressMode::Implied),
                0x78 => self.sei(AddressMode::Implied),

                // reset / set processor status bits
                0xC2 => self.rep(AddressMode::Immediate),
                0xE2 => self.sep(AddressMode::Immediate),

                // load
                0xA1 => self.lda(AddressMode::DirectIndexedIndirect),
                0xA3 => self.lda(AddressMode::StackRelative),
                0xA5 => self.lda(AddressMode::Direct(*opcode)),
                0xA7 => self.lda(AddressMode::DirectIndirectLong),
                0xA9 => self.lda(AddressMode::Immediate), 
                0xAD => self.lda(AddressMode::Absolute(*opcode)),
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
                0xA6 => self.ldx(AddressMode::Direct(*opcode)),
                0xAE => self.ldx(AddressMode::Absolute(*opcode)),
                0xB6 => self.ldx(AddressMode::DirectIndexedY),
                0xBE => self.ldx(AddressMode::AbsoluteIndexedY),
                0xA0 => self.ldy(AddressMode::Immediate),        
                0xA4 => self.ldy(AddressMode::Direct(*opcode)),
                0xAC => self.ldy(AddressMode::Absolute(*opcode)),
                0xB4 => self.ldy(AddressMode::DirectIndexedX),
                0xBC => self.ldy(AddressMode::AbsoluteIndexedX),

                // store
                0x81 => self.sta(AddressMode::DirectIndexedIndirect),
                0x83 => self.sta(AddressMode::StackRelative),
                0x85 => self.sta(AddressMode::Direct(*opcode)),
                0x87 => self.sta(AddressMode::DirectIndirectLong),
                0x8D => self.sta(AddressMode::Absolute(*opcode)),
                0x8F => self.sta(AddressMode::AbsoluteLong),
                0x91 => self.sta(AddressMode::DirectIndirectIndexed),
                0x92 => self.sta(AddressMode::DirectIndirect),
                0x93 => self.sta(AddressMode::StackRelativeIndirectIndexed),
                0x95 => self.sta(AddressMode::DirectIndexedX),
                0x97 => self.sta(AddressMode::DirectIndirectIndexedLong),
                0x99 => self.sta(AddressMode::AbsoluteIndexedY),
                0x9D => self.sta(AddressMode::AbsoluteIndexedX),
                0x9F => self.sta(AddressMode::AbsoluteLongIndexedX),
                0x86 => self.stx(AddressMode::Direct(*opcode)),
                0x8E => self.stx(AddressMode::Absolute(*opcode)),
                0x96 => self.stx(AddressMode::DirectIndexedY),
                0x84 => self.sty(AddressMode::Direct(*opcode)),
                0x8C => self.sty(AddressMode::Absolute(*opcode)),
                0x94 => self.sty(AddressMode::DirectIndexedX),
                0x64 => self.stz(AddressMode::Direct(*opcode)),
                0x74 => self.stz(AddressMode::DirectIndexedX),
                0x9C => self.stz(AddressMode::Absolute(*opcode)),
                0x9E => self.stz(AddressMode::AbsoluteIndexedX),

                // move memory negative/positive
                0x54 => self.mvn(AddressMode::SourceDestination),
                0x44 => self.mvp(AddressMode::SourceDestination),

                // no op and the WDM JR. BABY
                0xEA => self.nop(AddressMode::Implied),
                0x42 => self.wdm(AddressMode::Immediate),

                // push effective
                0xF4 => self.pea(AddressMode::Immediate),
                0xD4 => self.pei(AddressMode::Direct(*opcode)),
                0x63 => self.per(AddressMode::Relative16),

                // push / pull
                0x48 => self.pha(AddressMode::Implied),
                0xDA => self.phx(AddressMode::Implied),
                0x5A => self.phy(AddressMode::Implied),
                0x68 => self.pla(AddressMode::Implied),
                0xFA => self.plx(AddressMode::Implied),
                0x7A => self.ply(AddressMode::Implied),

                0x8B => self.phb(AddressMode::Implied),
                0x0B => self.phd(AddressMode::Implied),
                0x4B => self.phk(AddressMode::Implied),
                0x08 => self.php(AddressMode::Implied),
                0xAB => self.plb(AddressMode::Implied),
                0x2B => self.pld(AddressMode::Implied),
                0x28 => self.plp(AddressMode::Implied),

                // stop / wait
                0xDB => self.stp(AddressMode::Implied),
                0xCB => self.wai(AddressMode::Implied),

                // transfer
                0xAA => self.tax(AddressMode::Implied),
                0xA8 => self.tay(AddressMode::Implied),
                0xBA => self.tsx(AddressMode::Implied),
                0x8A => self.txa(AddressMode::Implied),
                0x9A => self.txs(AddressMode::Implied),
                0x9B => self.txy(AddressMode::Implied),
                0x98 => self.tya(AddressMode::Implied),
                0xBB => self.tyx(AddressMode::Implied),
                0x5B => self.tcd(AddressMode::Implied),
                0x1B => self.tcs(AddressMode::Implied),
                0x7B => self.tdc(AddressMode::Implied),
                0x3B => self.tsc(AddressMode::Implied),

                // exchange
                0xEB => self.xba(AddressMode::Implied),
                0xFB => self.xce(AddressMode::Implied),

                _ => panic!("Opcode {:X} is not implemented", opcode)

                // address mode mappings
                // =====================
                // (dir, X)   => DirectIndexedIndirect
                // stk, S     => StackRelative
                // dir        => Direct
                // [dir]      => DirectIndirectLong
                // imm        => Immediate
                // abs        => Absolute
                // (dir), Y   => DirectIndirectIndexed
                // (dir)      => DirectIndirect
                // (stk,S), Y => StackRelativeIndirectIndexed
                // dir, X     => DirectIndexedX
                // [dir], Y   => DirectIndirectIndexedLong
                // abs, Y     => AbsoluteIndexedY
                // abs, X     => AbsoluteIndexedX
                // long       => AbsoluteLong
                // long, X    => AbsoluteLongIndexedX
                // (abs)      => AbsoluteIndirect
                // [abs]      => AbsoluteIndirectLong
                // (abs,X)    => AbsoluteIndexedIndirect

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

    fn adc(&mut self, am: AddressMode) {
        if self.p.accumulator_width {
            let to_add = self.load_8(&am);
            self.adc_8(to_add);
        } else {
            let to_add = self.load_16(&am);
            self.adc_16(to_add);
        }
    }

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

            // todo -> decide if ah() is a u16 with lower bits set or if its an 8 bit number and need to shift
            self.a = self.ah() | result_8 as u16;

            // TODO -> DETERMINE OVERFLOW

            self.p.set_negative_from_data_8(result_8);
            self.p.set_zero_from_data(result_8);
        } else {
            // binary mode
            let result_16 = self.a + to_add as u16 + self.p.carry as u16;

            if result_16 >= 0x100 {
                self.p.carry = true;
            } else {
                self.p.carry = false;
            }

            let result_8 = result_16 as u8;

            // TODO -> DETERMINE OVERFLOW

            self.a = self.ah() | result_8 as u16;

            self.p.set_negative_from_data_8(result_8);
            self.p.set_zero_from_data(result_8);
        }
    }

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

    fn sbc(&mut self, am: AddressMode) {
        if self.p.accumulator_width {
            let to_sub = self.load_8(&am);
            self.sbc_8(to_sub);
        } else {
            let to_sub = self.load_16(&am);
            self.sbc_16(to_sub);
        }
    }

    fn sbc_8(&self, to_sub: u8) {
        // accumulator -= data - 1 + carry;

        if self.p.decimal_mode {
            // bcd mode
        } else {
            // binary mode
        }
    }

    fn sbc_16(&self, to_sub: u16) {
        // accumulator -= data - 1 + carry;

        if self.p.decimal_mode {
            // bcd mode
        } else {
            // binary mode
        }
    }

    ////////////////////////////////////
    //
    //            COMPARE
    //
    ////////////////////////////////////

    // todo -> fix this
    fn set_flags_for_cmp(&mut self, a: u16, b: u16) {
        self.p.carry = a >= b;
        self.p.negative = a < b;
        self.p.zero = a == b;
    }

    fn cmp(&mut self, am: AddressMode) {
        if self.p.accumulator_width {
            // 8
            let to_compare = self.load_8(&am);
            let a = self.a as u8;
            self.set_flags_for_cmp(a as u16, to_compare as u16);
        } else {
            // 16
            let to_compare = self.load_16(&am);
            let a = self.a;
            self.set_flags_for_cmp(a, to_compare);
        }
    }

    fn cpx(&mut self, am: AddressMode) {
        if self.p.index_width {
            // 8
            let to_compare = self.load_8(&am);
            let a = self.x as u8;
            self.set_flags_for_cmp(a as u16, to_compare as u16);
        } else {
            // 16
            let to_compare = self.load_16(&am);
            let a = self.x;
            self.set_flags_for_cmp(a, to_compare);
        }
    }

    fn cpy(&mut self, am: AddressMode) {
        if self.p.index_width {
            // 8
            let to_compare = self.load_8(&am);
            let a = self.y as u8;
            self.set_flags_for_cmp(a as u16, to_compare as u16);
        } else {
            // 16
            let to_compare = self.load_16(&am);
            let a = self.y;
            self.set_flags_for_cmp(a, to_compare);
        }
    }

    ////////////////////////////////////
    //
    //      INCREMENT / DECREMENT
    //
    ////////////////////////////////////

    fn dec(&mut self, am: AddressMode){
        match am {
            AddressMode::Accumulator => {
                self.a -= 1;

                self.p.negative = if self.p.accumulator_width {
                    self.a & (1 << 7) != 0
                } else {
                    self.a & (1 << 15) != 0
                };

                self.p.zero = self.a == 0;
            },
            _ => {
                if self.p.accumulator_width {
                    let to_dec = self.load_8(&am) - 1;
                    self.p.negative = to_dec & (1 << 7) != 0;
                    self.p.zero = to_dec == 0;
                    self.store_8(&am, to_dec);
                } else {
                    let to_dec = self.load_16(&am) - 1;
                    self.p.negative = to_dec & (1 << 15) != 0;
                    self.p.zero = to_dec == 0;
                    self.store_16(&am, to_dec);
                }
            }
        };
    }

    fn dex(&mut self, _: AddressMode) {
        self.x -= 1;
    }

    fn dey(&mut self, _: AddressMode) {
        self.y -= 1;
    }

    fn inc(&mut self, am: AddressMode) {
        match am {
            AddressMode::Accumulator => {
                self.a += 1;

                self.p.negative = if self.p.accumulator_width {
                    self.a & (1 << 7) != 0
                } else {
                    self.a & (1 << 15) != 0
                };

                self.p.zero = self.a == 0;
            },
            _ => {
                if self.p.accumulator_width {
                    let to_dec = self.load_8(&am) + 1;
                    self.p.negative = to_dec & (1 << 7) != 0;
                    self.p.zero = to_dec == 0;
                    self.store_8(&am, to_dec);
                } else {
                    let to_dec = self.load_16(&am) + 1;
                    self.p.negative = to_dec & (1 << 15) != 0;
                    self.p.zero = to_dec == 0;
                    self.store_16(&am, to_dec);
                }
            }
        };
    }

    fn inx(&mut self, _: AddressMode) {
        self.x += 1;
    }

    fn iny(&mut self, _: AddressMode) {
        self.y += 1;
    }

    ////////////////////////////////////
    //
    //              BIT
    //
    ////////////////////////////////////

    fn bit(&mut self, am: AddressMode) {
        if self.p.index_width {
            let to_comp = self.load_8(&am);

            if am == AddressMode::Immediate {
                self.p.negative = to_comp & (1 << 7) != 0;
                self.p.overflow = to_comp & (1 << 6) != 0;
            }

            self.p.zero = to_comp == 0;
        } else {
            let to_comp = self.load_16(&am);

            if am == AddressMode::Immediate {
                self.p.negative = to_comp & (1 << 15) != 0;
                self.p.overflow = to_comp & (1 << 14) != 0;
            }

            self.p.zero = to_comp == 0;
        }
    }

    ////////////////////////////////////
    //
    //            TRB / TSB
    //
    ////////////////////////////////////

//    TSB: "Logically OR together the value in the accumulator with the data at the effective
//          address specified by the operand. Store the result at the memory location"
//
//    TRB: "Logically AND together the _complement_ of the value in the accumulator with the data
//          at the effective address specified by the operand. Store the result at the memory location"
    fn trb(&mut self, am: AddressMode) {
        if self.p.accumulator_width {
            // 8
            let data = self.load_8(&am);
            let result = self.al() | data;

            self.p.set_zero(result == 0);
            self.store_8(&am, result);
        } else {
            // 16
            let data = self.load_16(&am);
            let result = self.a | data;

            self.p.set_zero(result == 0);
            self.store_16(&am, result);
        }
    }

    fn tsb(&mut self, am: AddressMode) {
        if self.p.accumulator_width {
            let data = self.load_8(&am);
            let result = !self.al() & data;

            self.p.set_zero(result == 0);
            self.store_8(&am, result);
        } else {
            let data = self.load_16(&am);
            let result = (!self.a) & data;

            self.p.set_zero(result == 0);
            self.store_16(&am, result);
        }
    }

    ////////////////////////////////////
    //
    //             SHIFT
    //
    ////////////////////////////////////

    //The n flag reflects the high bit of the result.
    //The z flag reflects whether the result is zero.

    fn asl(&mut self, am: AddressMode) {
        //ASL shifts left; a zero is shifted into the low bit (bit 0); the high bit (bit 15 when the m flag is one, bit 7 when the m flag is 0) is shifted into the c flag.

        if self.p.accumulator_width {
            let to_and = 1 << 7 as u8;
            if am == AddressMode::Accumulator {
                let to_carry = (self.al() & to_and) >> 7;
                self.a = self.ah() | ((self.al() << 1) as u8);
                self.p.carry = to_carry == 1;
                self.p.set_negative_from_data_16(self.al());
                self.p.set_zero_from_data(self.al() as u16);
            } else {
                let addr = am.get_address_8(self, &self.mem);

                let mut data = self.load_8_from_addr(addr);
                let to_carry = (data & to_and) >> 7;
                data = (data << 1) as u8;

                self.store_8_from_addr(addr, data);

                self.p.set_negative_from_data_8(data);
                self.p.set_zero_from_data(data as u16);
            }
        } else {
            let to_and = 1 << 15 as u8;
            if am == AddressMode::Accumulator {
                let to_carry = (self.a & to_and) >> 15;
                self.a = (self.a << 1) as u16;
                self.p.carry = to_carry == 1;
                self.p.set_negative_from_data_16(self.a);
                self.p.set_zero_from_data(self.a);
            } else {
                let (lo_addr, hi_addr) = am.get_address_16(self, &self.mem);

                let mut data = self.load_16_from_addresses(lo_addr, hi_addr);
                let to_carry = (data & to_and) >> 15;
                self.p.carry = to_carry == 1;
                data = (data << 1) as u16;

                self.store_16_from_addr(lo_addr, hi_addr, data);

                self.p.set_negative_from_data_16(data);
                self.p.set_zero_from_data(data);
            }
        }
    }

    fn lsr(&mut self, am: AddressMode) {
        //LSR shifts right; a zero is shifted into the high bit; the low bit is shifted into the c flag.

        if self.p.accumulator_width {
            if am == AddressMode::Accumulator {
                let to_carry = (self.al() & 0b1);
                self.a = self.ah() | ((self.al() >> 1) as u8);
                self.p.carry = to_carry == 1;
                self.p.set_negative_from_data_16(self.al());
                self.p.set_zero_from_data(self.al() as u16);
            } else {
                let addr = am.get_address_8(self, &self.mem);

                let mut data = self.load_8_from_addr(addr);
                let to_carry = (data & 0b1);
                self.p.carry = to_carry == 1;
                data = (data >> 1) as u8;

                self.store_8_from_addr(addr, data);

                self.p.set_negative_from_data_8(data);
                self.p.set_zero_from_data(data as u16);
            }
        } else {
            if am == AddressMode::Accumulator {
                let to_carry = self.a & 0b1;
                self.a = (self.a >> 1) as u16;
                self.p.carry = to_carry == 1;
                self.p.set_negative_from_data_16(self.a);
                self.p.set_zero_from_data(self.a);
            } else {
                let (lo_addr, hi_addr) = am.get_address_16(self, &self.mem);

                let mut data = self.load_16_from_addresses(lo_addr, hi_addr);
                let to_carry = data & 0b1;
                self.p.carry = to_carry == 1;
                data = (data >> 1) as u16;

                self.store_16_from_addr(lo_addr, hi_addr, data);

                self.p.set_negative_from_data_16(data);
                self.p.set_zero_from_data(data);
            }
        }
    }

    ////////////////////////////////////
    //
    //             ROTATE
    //
    ////////////////////////////////////

    // for these the main thing is you
    // (a) grab the high/low bit
    // (b) set the carry flag to the bit you just grabbed
    // (c) shift the thing left/right
    // (d) set the low/high bit to the carry flag
    //      do this by 'or'ing. ie:
    //          if you have 0b1111_1111 and you << 1 then you have 0b1111_1110 and you | with the
    //          carry which will effectively shift it to the lower/higher bit

    fn rol(&mut self, am: AddressMode) {
        //ROL shifts left; the (input) c flag is shifted into the low bit; the high bit is shifted into the c flag (result).

        if self.p.accumulator_width {
            let to_and = 1 << 7 as u8;
            if am == AddressMode::Accumulator {
                let prev_carry = if self.p.carry { 0b1 } else { 0b0 };
                let to_carry = (self.al() & to_and) >> 7;
                self.a = self.ah() | ((self.al() << 1) as u8);
                self.p.carry = to_carry == 1;
                self.a = self.a | prev_carry;
                self.p.set_negative_from_data_8(self.al());
                self.p.set_zero_from_data(self.al() as u16);
            } else {
                let addr = am.get_address_8(self, &self.mem);

                let mut data = self.load_8_from_addr(addr);
                let prev_carry = if self.p.carry { 0b1 } else { 0b0 };
                let to_carry = (data & to_and) >> 7;
                data = (data << 1) as u8;
                data = data | prev_carry;
                self.store_8(&am, data);

                self.store_8_from_addr(addr, data);

                self.p.set_negative_from_data_8(data);
                self.p.set_zero_from_data(data as u16);
            }
        } else {
            let to_and = 1 << 15 as u8;
            if am == AddressMode::Accumulator {
                let prev_carry = if self.p.carry { 0b1 } else { 0b0 };
                let to_carry = (self.a & to_and) >> 15;
                self.a = (self.a << 1) as u16;
                self.a = self.a | prev_carry;
                self.p.carry = to_carry == 1;
                self.p.set_negative_from_data_16(self.a);
                self.p.set_zero_from_data(self.a);
            } else {
                let (lo_addr, hi_addr) = am.get_address_16(self, &self.mem);

                let mut data = self.load_16_from_addresses(lo_addr, hi_addr);
                let prev_carry = if self.p.carry { 0b1 } else { 0b0 };
                let to_carry = (data & to_and) >> 15;
                self.p.carry = to_carry == 1;
                data = (data << 1) as u16;
                data = data | prev_carry;

                self.store_16_from_addr(lo_addr, hi_addr, data);

                self.p.set_negative_from_data_16(data);
                self.p.set_zero_from_data(data);
            }
        }
    }

    fn ror(&mut self, am: AddressMode) {
        //ROR shifts right; the (input) c flag is shifted into the high bit; the low bit is shifted into the c flag (result).

        if self.p.accumulator_width {
            if am == AddressMode::Accumulator {
                let prev_carry = if self.p.carry { 0b1 } else { 0b0 };
                let to_carry = (self.al() & 0b1);
                self.a = self.ah() | ((self.al() >> 1) as u8);
                self.a = self.a | prev_carry;
                self.p.carry = to_carry == 1;

                self.p.set_negative_from_data_8(self.al());
                self.p.set_zero_from_data(self.al() as u16);
            } else {
                let addr = am.get_address_8(self, &self.mem);

                let mut data = self.load_8_from_addr(addr);
                let prev_carry = if self.p.carry { 0b1 } else { 0b0 };
                let to_carry = (data & 0b1);
                self.p.carry = to_carry == 1;
                data = (data >> 1) as u8;
                data = data | prev_carry;

                self.store_8_from_addr(addr, data);

                self.p.set_negative_from_data_8(data);
                self.p.set_zero_from_data(data as u16);
            }
        } else {
            if am == AddressMode::Accumulator {
                let prev_carry = if self.p.carry { 0b1 } else { 0b0 };
                let to_carry = self.a & 0b1;
                self.a = (self.a >> 1) as u16;
                self.a = self.a | prev_carry;
                self.p.set_carry(to_carry == 1);
                self.p.set_negative_from_data_16(self.a);
                self.p.set_zero_from_data(self.a);
            } else {
                let (lo_addr, hi_addr) = am.get_address_16(self, &self.mem);

                let mut data = self.load_16_from_addresses(lo_addr, hi_addr);
                let to_carry = data & 0b1;
                let prev_carry = if self.p.carry { 0b1 } else { 0b0 };
                self.p.set_carry(to_carry == 1);

                // todo -> don't think we're handling carries properly...
                data = (data >> 1) as u16;
                data = data | prev_carry;

                self.store_16_from_addr(lo_addr, hi_addr, data);

                self.p.set_negative_from_data_16(data);
                self.p.set_zero_from_data(data);
            }
        }
    }

    ////////////////////////////////////
    //
    //             BRANCH
    //
    ////////////////////////////////////

    // probably don't want to pass the address itself and instead just the addressmode
    fn branch(&mut self, am: AddressMode) {
        let address = self.get_address(am);

        // todo -> dont think we want to change the pbr? do we? do we? i dont know? i'm so confused.
        self.pbr = address.0; // first 8 bits
        self.pc = address.1;  // last 16 bits
    }

    fn bcc(&mut self, am: AddressMode) {
        if !self.p.carry {
            branch(am);
        }
    }

    fn bcs(&mut self, am: AddressMode) {
        if self.p.carry {
            branch(am);
        }
    }

    fn beq(&mut self, am: AddressMode) {
        if self.p.zero {
            branch(am);
        }
    }

    fn bmi(&mut self, am: AddressMode) {
        if self.p.negative {
            branch(am);
        }
    }

    fn bne(&mut self, am: AddressMode) {
        if !self.p.zero {
            branch(am);
        }
    }

    fn bpl(&mut self, am: AddressMode) {
        if !self.p.negative {
            branch(am);
        }
    }

    fn bvc(&mut self, am: AddressMode) {
        if !self.p.overflow {
            branch(am);
        }
    }

    fn bvs(&mut self, am: AddressMode) {
        if self.p.overflow {
            branch(am);
        }
    }

    fn bra(&mut self, am: AddressMode) {
        branch(am);
    }

    fn brl(&mut self, am: AddressMode) {
        branch(am);
    }

    ////////////////////////////////////
    //
    //              JUMP
    //
    ////////////////////////////////////

    fn jmp(&mut self, am: AddressMode) {
        branch(am);
    }

    // jumps within the same page
    fn jsl(&mut self, am: AddressMode) {
        let pbr = self.pbr;
        self.pushb(pbr);

        let pc = self.pc - 1;
        self.pushb((pc >> 8) as u8);
        self.pushb(pc as u8);

        let (pbr, pc) = am.address(self);
        self.pbr = pbr;
        self.pc = pc;
    }

    fn jsr(&mut self, am: AddressMode) {
        let pc = self.pc - 1;
        self.pushb((pc >> 8) as u8);
        self.pushb(pc as u8);

        self.pc = am.address(self).1;
    }

    ////////////////////////////////////
    //
    //             RETURN
    //
    ////////////////////////////////////

    fn rti(&mut self, am: AddressMode) {
        self.return_from_interrupt()
    }

    fn rts(&mut self, am: AddressMode) {
        let pcl = self.popb() as u16;
        let pch = self.popb() as u16;
        let pc = (pch << 8) | pcl;
        self.pc = pc + 1;   // +1 since the last byte of the JSR was saved
    }

    fn rtl(&mut self, am: AddressMode) {
        let pcl = self.popb() as u16;
        let pch = self.popb() as u16;
        let pbr = self.popb();
        let pc = (pch << 8) | pcl;
        self.pbr = pbr;
        self.pc = pc + 1; // +1 since the last byte of the JSR was saved
    }

    ////////////////////////////////////
    //
    //     BREAKPOINT / COPROCESSOR
    //
    ////////////////////////////////////

    fn brk(&mut self, am: AddressMode) {
        if native_mode {
            self.pushb(self.pbr);
            let pc = self.pc;
            self.pushb((pc >> 8) as u8);
            self.pushb(pc as u8);
            self.pushb(self.p);

            self.pbr = 0x00;
            self.pc = 0xFFE6;
        } else {
            // TODO
        }
    }

    fn cop(&mut self, am: AddressMode) {
        if native_mode {
            self.pushb(self.pbr);
            let pc = self.pc;
            self.pushb((pc >> 8) as u8);
            self.pushb(pc as u8);
            self.pushb(self.p);

            self.pbr = 0x00;
            self.pc = 0xFFE4;
        } else {
            // TODO
        }
    }

    ////////////////////////////////////
    //
    //          CLEAR / SET
    //
    ////////////////////////////////////

    fn clc(&mut self, addr: AddressMode) {
        self.p.set_carry(false);
    }

    fn cld(&mut self, addr: AddressMode) {
        self.p.set_decimal_mode(false);
    }

    fn cli(&mut self, addr: AddressMode) {
        self.p.set_interrupt_disable(false);
    }

    fn clv(&mut self, addr: AddressMode) {
        self.p.set_overflow(false);
    }

    fn sec(&mut self, addr: AddressMode) {
        self.p.set_carry(true);
    }

    fn sed(&mut self, addr: AddressMode) {
        self.p.set_decimal_mode(true);
    }

    fn sei(&mut self, addr: AddressMode) {
        self.p.set_interrupt_disable(true);
    }

    ////////////////////////////////////
    //
    //           SET / RESET
    //
    ////////////////////////////////////

    fn rep(&mut self, am: AddressMode) {
        // clear the bits in p register that are 1 in the op
        let to_comp = self.load_b(am);
        self.p = self.p & (!to_comp);

        if self.p.emulation_mode {
            // m and x flags are always 1
            self.p = self.p | 0b0011_0000;
        }
    }

    fn sep(&mut self, am: AddressMode) {
        // set the bits in p register that are 1 in the op
        let to_comp = self.load_b(am);
        self.p = self.p | to_comp;

        if self.p.emulation_mode {
            // m and x flags are always 1
            self.p = self.p | 0b0011_0000;
        }
    }

    ////////////////////////////////////
    //
    //       LOADING / STORING
    //
    ////////////////////////////////////

    fn lda(&mut self, am: AddressMode) {
        if self.p.accumulator_width {
            let val = self.load_8(&am);
            self.a = self.ah() << 8 | (val as u16);
        } else {
            let val = self.load_16(&am);
            self.a = val;
        }
    }

    fn ldx(&mut self, am: AddressMode) {
        if self.p.index_width {
            let val = self.load_8(&am);
            self.x = self.xh() << 8 | (val as u16);
        } else {
            let val = self.load_16(&am);
            self.x = val;
        }
    }

    fn ldy(&mut self, am: AddressMode) {
        if self.p.index_width {
            let val = self.load_8(&am);
            self.y = self.yh() << 8 | (val as u16);
        } else {
            let val = self.load_16(&am);
            self.y = val;
        }
    }

    fn sta(&mut self, am: AddressMode) {
        if self.p.accumulator_width {
            self.store_8(&am, self.al());
        } else {
            self.store_16(&am, self.a);
        }
    }

    fn stx(&mut self, am: AddressMode) {
        if self.p.index_width {
            self.store_8(&am, self.xl());
        } else {
            self.store_16(&am, self.x);
        }
    }

    fn sty(&mut self, am: AddressMode) {
        if self.p.index_width {
            self.store_8(&am, self.yl());
        } else {
            self.store_16(&am, self.y);
        }
    }

    fn stz(&mut self, am: AddressMode) {
        if self.p.accumulator_width{
            self.store_8(&am, 0);
        } else {
            self.store_16(&am, 0);
        }
    }

    ////////////////////////////////////
    //
    //              MOVE
    //
    ////////////////////////////////////

    fn mvn(&mut self, am: AddressMode) {
        self.a += 1;
        while self.a > 0 {
            swap(ss, tt, self.x, self.y);

            self.x += 1;
            self.y += 1;
            self.a -= 1;

            // todo -> dec 7 cycles
            // todo -> check for hardware interrupt
        }
        self.dbr = tt;
    }

    fn mvp(&mut self, am: AddressMode) {
        self.a += 1;
        while self.a > 0 {
            swap(ss, tt, self.x, self.y);

            self.x -= 1;
            self.y -= 1;
            self.a -= 1;

            // todo -> dec 7 cycles
            // todo -> check for hardware interrupt
        }
        self.dbr = tt;
    }

    ////////////////////////////////////
    //
    //            NOP/WDM
    //
    ////////////////////////////////////

    fn nop(&mut self, am: AddressMode) {
        // TODO cycles
    }

    fn wdm(&mut self, am: AddressMode) {
        self.next_b();
        // TODO cycles
    }

    ////////////////////////////////////
    //
    //              PUSH
    //
    ////////////////////////////////////

    fn push_effective(&mut self, am: AddressMode) {
        let to_push = am.address(self);
        self.push_b((to_push >> 8) as u8);
        self.push_b(to_push as u8);
    }

    fn pea(&mut self, am: AddressMode) {
        self.push_effective(am);
    }

    fn pei(&mut self, am: AddressMode) {
        self.push_effective(am);
    }

    fn per(&mut self, am: AddressMode) {
        self.push_effective(am);
    }

    fn pha(&mut self, am: AddressMode) {
        if self.p.accumulator_width {
            self.pusb_b(self.al());
        } else {
            self.pusb_b(self.ah());
            self.pusb_b(self.al());
        }
    }

    fn phx(&mut self, am: AddressMode) {
        if self.p.index_width {
            self.push_b(self.xl());
        } else {
            self.push_b(self.xh());
            self.push_b(self.xl());
        }
    }

    fn phy(&mut self, am: AddressMode) {
        if self.p.index_width {
            self.push_b(self.yl());
        } else {
            self.push_b(self.yh());
            self.push_b(self.yl());
        }
    }

    fn pla(&mut self, am: AddressMode) {
        if self.p.accumulator_width {
            let result = self.pull_b();

            self.p.set_negative_from_data_8(result as u16);
            self.p.set_zero_from_data(result);

            let high_bits = (self.ah() as u16) << 8;
            self.a = high_bits | result
        } else {
            let lo = self.pull_b();
            let hi = self.pull_b();

            let result = ((hi as u16) << 8) | lo;

            self.p.set_negative_from_data_16(result);
            self.p.set_zero_from_data(result);

            self.a = result;
        }
    }

    fn plx(&mut self, am: AddressMode) {
        if self.p.index_width {
            let result = self.pull_b();

            self.p.set_negative_from_data_8(result as u16);
            self.p.set_zero_from_data(result);

            let high_bits = (self.ah() as u16) << 8;
            self.x = high_bits | result
        } else {
            let lo = self.pull_b();
            let hi = self.pull_b();

            let result = ((hi as u16) << 8) | lo;

            self.p.set_negative_from_data_16(result);
            self.p.set_zero_from_data(result);

            self.x = result;
        }
    }

    fn ply(&mut self, am: AddressMode) {
        if self.p.index_width {
            let result = self.pull_b();

            self.p.set_negative_from_data_8(result as u16);
            self.p.set_zero_from_data(result);

            let high_bits = (self.ah() as u16) << 8;
            self.y = high_bits | result
        } else {
            let lo = self.pull_b();
            let hi = self.pull_b();

            let result = ((hi as u16) << 8) | lo;

            self.p.set_negative_from_data_16(result);
            self.p.set_zero_from_data(result);

            self.y = result;
        }
    }

    fn phb(&mut self, am: AddressMode) {
        self.push_b(self.dbr);
    }

    fn phd(&mut self, am: AddressMode) {
        self.push_b(self.dh());
        self.push_b(self.dl());
    }

    fn phk(&mut self, am: AddressMode) {
        self.push_b(self.pbr);
    }

    fn php(&mut self, am: AddressMode) {
        self.push_b(self.p);
    }

    fn plb(&mut self, am: AddressMode) {
        let result = self.pull_b();

        self.p.set_negative_from_data_8(result as u16);
        self.p.set_zero_from_data(result);

        let high_bits = (self.ah() as u16) << 8;
        self.dbr = high_bits | result
    }

    fn pld(&mut self, am: AddressMode) {
        let lo = self.pull_b();
        let hi = self.pull_b();

        let result = ((hi as u16) << 8) | lo;

        self.p.set_negative_from_data_16(result);
        self.p.set_zero_from_data(result);

        self.d = result;
    }

    fn plp(&mut self, am: AddressMode) {
        self.p = self.pull_b()
    }

    ////////////////////////////////////
    //
    //           STOP / WAIT
    //
    ////////////////////////////////////

    fn stp(&mut self, am: AddressMode) {
        // shut down until interrupt
    }

    fn wai(&mut self, am: AddressMode) {
        // shut down until interrupt
    }

    ////////////////////////////////////
    //
    //            TRANSFER
    //
    ////////////////////////////////////

    fn tax(&mut self, am: AddressMode) {
        if self.p.index_width {
            let hi_x = self.x >> 8;
            let lo_x = self.al();
            self.x = ((hi_x as u16) << 8) | (lo_x as u16);
        } else {
            self.x = self.a;
        }
    }

    fn tay(&mut self, am: AddressMode) {
        if self.p.index_width {
            let hi_y = self.y >> 8;
            let lo_y = self.al();
            self.y = ((hi_y as u16) << 8) | (lo_y as u16);
        } else {
            self.y = self.a;
        }
    }

    fn tsx(&mut self, am: AddressMode) {
        if self.p.index_width {
            let hi_x = self.x >> 8;
            let lo_x = self.spl();
            self.x = ((hi_x as u16) << 8) | (lo_x as u16);
        } else {
            self.x = self.sp;
        }
    }

    fn txa(&mut self, am: AddressMode) {
        if self.p.accumulator_width {
            let hi_a = self.a >> 8;
            let lo_a = self.xl();
            self.a = ((hi_a as u16) << 8) | (lo_a as u16);
        } else {
            self.a = self.x;
        }
    }

    fn txs(&mut self, am: AddressMode) {
        // However, when the e flag is 1, SH is forced to $01, so in effect, TXS is an 8-bit transfer in this case since XL is transferred to SL and SH remains $01.
        // Note that when the e flag is 0 and the x flag is 1 (i.e. 8-bit native mode), that XH is forced to zero,
        //      so after a TXS, SH will be $00, rather than $01. This is an important difference that must be accounted for if you want
        //      to run emulation mode code in (8-bit) native mode.

        if self.p.emulation_mode {
            let hi_s = 0x01;
            let lo_s = self.xl();

            self.sp = ((hi_s as u16) << 8) | (lo_s as u16);
        } else {
            if self.p.index_width {
                self.sp = self.xl() as u16;
            } else {
                self.sp = self.x;
            }
        }
    }

    fn txy(&mut self, am: AddressMode) {
        if self.p.index_width {
            let hi_y = self.y >> 8;
            let lo_y = self.xl();
            self.y = ((hi_y as u16) << 8) | (lo_y as u16);
        } else {
            self.y = self.x;
        }
    }

    fn tya(&mut self, am: AddressMode) {
        if self.p.accumulator_width {
            let hi_a = self.a >> 8;
            let lo_a = self.yl();
            self.a = ((hi_a as u16) << 8) | (lo_a as u16);
        } else {
            self.a = self.y;
        }
    }

    fn tyx(&mut self, am: AddressMode) {
        if self.p.index_width {
            let hi_x = self.x >> 8;
            let lo_x = self.yl();
            self.x = ((hi_x as u16) << 8) | (lo_x as u16);
        } else {
            self.x = self.y;
        }
    }

    fn tcd(&mut self, am: AddressMode) {
        self.d = self.a;
    }

    fn tcs(&mut self, am: AddressMode) {
        if self.p.emulation_mode {
            let hi_s = 0x01;
            let lo_s = self.al();

            self.sp = ((hi_s as u16) << 8) | (lo_s as u16);
        } else {
            self.sp = self.a;
        }
    }

    fn tdc(&mut self, am: AddressMode) {
        self.a = self.d;
    }

    fn tsc(&mut self, am: AddressMode) {
        self.a = self.s;
    }

    ////////////////////////////////////
    //
    //            EXCHANGE
    //
    ////////////////////////////////////

    fn xba(&mut self, am: AddressMode) {
        self.a = ((self.al() as u16) << 8) | (self.ah() as u16);
    }

    fn xce(&mut self, am: AddressMode) {
        let e_temp = self.p.emulation_mode;
        self.p.carry = e_temp;
        self.p.emulation_mode = self.p.carry;

        if self.p.emulation_mode {
            self.p.accumulator_width = true;
            self.p.index_width = true;
            self.x = 0x0011 & self.x;
            self.y = 0x0011 & self.y;
            self.sp = ((0x01 as u16) << 8) | (self.spl() as u16);
        }
    }

    ////////////////////////////////////
    //
    //             BITWISE
    //
    ////////////////////////////////////

    fn and(&mut self, am: AddressMode) {
        if self.p.index_width {
            let to_comp = self.load_8(&am);
            self.set_al(self.al() & to_comp);
            self.p.set_negative_from_data_8(self.al());
            self.p.set_zero_from_data(self.al() as u16);
        } else {
            let to_comp = self.load_16(&am);
            self.set_al(self.al() & to_comp);
            self.p.set_negative_from_data_16(self.a);
            self.p.set_zero_from_data(self.a);
        };
    }

    fn eor(&mut self, am: AddressMode) {
        if self.p.index_width {
            let to_comp = self.load_8(&am);
            self.set_al(self.al() ^ to_comp);
            self.p.set_negative_from_data_8(self.al());
            self.p.set_zero_from_data(self.al() as u16);
        } else {
            let to_comp = self.load_16(&am);
            self.set_al(self.al() ^ to_comp);
            self.p.set_negative_from_data_16(self.a);
            self.p.set_zero_from_data(self.a);
        };
    }

    fn ora(&mut self, am: AddressMode) {
        let to_comp = if self.p.index_width {
            let to_comp = self.load_8(&am);
            self.set_al(self.al() | to_comp);
            self.p.set_negative_from_data_8(self.al());
            self.p.set_zero_from_data(self.al() as u16);
        } else {
            let to_comp = self.load_16(&am);
            self.set_al(self.al() | to_comp);
            self.p.set_negative_from_data_16(self.a);
            self.p.set_zero_from_data(self.a);
        };
    }
}

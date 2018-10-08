use std::collections::HashMap;

use cpu::cpu::CPU;

type AddressModeFunction = fn(&CPU) -> u8;

pub enum AddressMode {
    Accumulator,
    Immediate,
    Implied,
    Relative,
    Absolute,
    ZeroPage,
    Indirect,
    AbsoluteIndexed,
    ZeroPageIndexed,
    IndexedIndirect,
    IndirectIndexed
}

pub static ADDRESSING_MODES: HashMap<AddressMode, AddressModeFunction> = {
    let mut am = HashMap::new();

    am.insert(AddressMode::Accumulator, accumulator);
    am.insert(AddressMode::Immediate, immediate);
    am.insert(AddressMode::Implied, implied);
    am.insert(AddressMode::Relative, relative);
    am.insert(AddressMode::Absolute, absolute);
    am.insert(AddressMode::ZeroPage, zero_page);
    am.insert(AddressMode::Indirect, indirect);
    am.insert(AddressMode::AbsoluteIndexed, absolute_indexed);
    am.insert(AddressMode::ZeroPageIndexed, zero_page_indexed);
    am.insert(AddressMode::IndexedIndirect, indexed_indirect);
    am.insert(AddressMode::IndirectIndexed, indirect_indexed);

    am
};

// non-indexed, non-memory

fn accumulator(cpu: &CPU) -> u16 {
    cpu.a()
}

fn immediate(cpu: &CPU) -> u32 {
    cpu.pc() + 1
}

fn implied() {
    //nothing
}

// non-indexed, memory

fn relative(cpu: &CPU, branch_offset: i8) -> u32 {
    cpu.pc() + branch_offset
}

fn absolute(cpu: &CPU, LL: u8, HH: u8) -> u32 {
    (cpu.dbr() << 4) | (HH << 2) | LL
}

fn zero_page(loc: u8) -> u8 {
    loc
}

fn indirect(cpu: &CPU, LL: u8, HH: u8) -> u32 {
    cpu.mem().load((HH << 2) | LL)
}

fn absolute_indexed(cpu: &CPU, LL: u8, HH: u8, reg: u8) -> u32 {
    absolute(cpu, LL, HH) + reg
}

fn zero_page_indexed(loc: u8, reg: u8) -> u8 {
    zero_page(loc) + reg
}

fn indexed_indirect(loc: u8) -> u32 {
    let addr = (loc + x) & 0xFF;
    let final_addr = load(addr);
    load(final_addr)
}

fn indirect_indexed(loc: u8) {
    let addr = load(loc);
    let final_addr = addr + y;
    load(final_addr)
}

use std::collections::HashMap;

use cpu::cpu::CPU;

pub enum AddressMode {
    Accumulator,
    Immediate,
    Implied,
    Relative8,
    Relative16,
    Absolute,
    ZeroPage,
    Indirect,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    IndexedIndirect,
    IndirectIndexed
}

// non-indexed, non-memory

pub fn accumulator(cpu: &CPU) -> u32 {
    cpu.a()
}

pub fn immediate(cpu: &CPU) -> u32 {
    cpu.pc() + 1
}

pub fn implied(_: &CPU) {
    //nothing
}

// non-indexed, memory

pub fn relative_8(cpu: &CPU) -> u32 {
    let pc = cpu.pc();
    let branch_offset = cpu.next_b();
    if branch_offset <= 0x7F {
        (cpu.pbr() << 16 | pc + 2 + branch_offset)
    } else {
        (cpu.pbr() << 16 | pc - 254 + branch_offset)
    }
}

pub fn relative_16(cpu: &CPU) -> u32 {
    let pc = cpu.pc();
    let lo = cpu.next_b();
    let hi = cpu.next_b();
    let offset = pc + 3 + (hi|lo);
    cpu.pbr() << 16 | offset 
}

pub fn absolute(cpu: &CPU, LL: u8, HH: u8) -> u32 {
    let lo = cpu.next_b();
    let hi = cpu.next_b();
    
    (cpu.dbr() << 16) | (hi << 8) | lo 
}

pub fn zero_page(cpu: &CPU) -> u32 {
    cpu.next_b() 
}

fn direct(cpu: &CPU) -> u32 {
    let bank = cpu.dbr();
    let ll = cpu.next_b();

    (0 | bank << 8 | ll)
}

fn indirect(cpu: &CPU, LL: u8, HH: u8) -> u32 {
    cpu.mem().load((HH << 2) | LL)
}

fn absolute_indexed_x(cpu: &CPU, LL: u8, HH: u8) -> u32 {
    absolute(cpu) + cpu.x() 
}

fn absolute_indexed_y(cpu: &CPU, LL: u8, HH: u8) -> u32 {
    absolute(cpu) + cpu.y() 
}

fn zero_page_indexed_x(cpu: &CPU) -> u8 {
    zero_page(cpu) + cpu.x() 
}

fn zero_page_indexed_y(cpu: &CPU) -> u8 {
    zero_page(cpu) + cpu.y() 
}

fn indexed_indirect(cpu: &CPU) -> u32 {
    let loc = cpu.next_b(); 
    
    let addr = (loc + cpu.x()) & 0xFF;
    let final_addr = cpu.load_8(addr);

    cpu.load_8(final_addr)
}

fn indirect_indexed(cpu: &CPU) {
    let loc = cpu.next_b(); 
    
    let addr = cpu.load(loc);
    let final_addr = addr + cpu.y();
    
    cpu.load_8(final_addr)
}

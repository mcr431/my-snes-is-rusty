use cpu::address_mode::*;
use cpu::cpu::CPU;

pub trait Mem {
    fn load_op(pc: u32) -> u16;
    fn load_byte_8(pc: u32, addr_mode: AddressMode) -> u8;
    fn load_byte_16(pc: u32, addr_mode: AddressMode) -> u16;
}

pub struct SimpleMemory {

}

impl SimpleMemory {
    pub fn new() -> SimpleMemory {
        SimpleMemory {}
    }
}

fn get_address(cpu: &CPU, addr_mode: AddressMode) -> u8 {
    let get_address_fn = ADDRESSING_MODES.get(addr_mode).unwrap();
    get_address_fn(cpu)
}

impl Mem for SimpleMemory {
    fn load_op(pc: u32) -> u16 {
        1_u16
    }

    fn load_byte_8(cpu: &CPU, addr_mode: AddressMode) -> u8 {
        let addr = Mem::get_address(cpu, addr_mode);
        load(addr)
    }

    fn load_byte_16(cpu: &CPU, addr_mode: AddressMode) -> u16 {
        let addr = Mem::get_address(cpu, addr_mode);
        (load(addr) << 8) | load(addr + 1)
    }
}
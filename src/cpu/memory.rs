use cpu::address_mode::*;
use cpu::cpu::CPU;

pub trait Mem {
    fn load(&self, cpu: &CPU, bank: u8, addr_mode: AddressMode) -> u8;
    fn store(&self, cpu: &CPU, bank: u8, addr_mode: AddressMode, to_store: u8); 
}

pub struct SimpleMemory {
    mem: [u8; 1 << 24] 
}

impl SimpleMemory {
    pub fn new() -> SimpleMemory {
        SimpleMemory {
            mem: [0; 1 << 24]
        }
    }

    fn load_from_store(&self, addr: u32) -> u8 {
        self.mem[addr]
    }

    fn store_value(&self, addr: u32, to_store: u8) {
        self.mem[addr] = to_store;
    }
}

impl Mem for SimpleMemory {
    fn load(&self, cpu: &CPU, address: u32) -> u8 {
        self.load_from_store(address)
    }

    fn store(&self, cpu: &CPU, address: u32, to_store: u16) {
        self.store_value(address, to_store);
    }
}
    

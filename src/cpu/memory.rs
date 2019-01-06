pub trait Mem {
    fn load(&self, bank: u8, address: u16) -> u8;
    fn store(&mut self, bank: u8, address: u16, to_store: u8);
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

    fn get_index(bank: u8, address: u16) -> usize {
        (bank as usize) << 16 | (address as usize)
    }

    fn load_from_store(&self, bank: u8, address: u16) -> u8 {
        self.mem[Self::get_index(bank, address)]
    }

    fn store_value(&mut self, bank: u8, address: u16, to_store: u8) {
        self.mem[Self::get_index(bank, address)] = to_store;
    }
}

impl Mem for SimpleMemory {
    fn load(&self, bank: u8, address: u16) -> u8 {
        self.load_from_store(bank, address)
    }

    fn store(&mut self, bank: u8, address: u16, to_store: u8) {
        self.store_value(bank, address, to_store);
    }
}
    
